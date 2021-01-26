use crate::models::UserModel;
use crate::vk_provider::{AccessTokenProvider, AuthResponse, NewsUpdate, SourcesUpdate};
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    AdjustmentExt, ApplicationWindow, Builder, ContainerExt, Image, Label, ListBoxExt,
    ScrolledWindow, Stack, WidgetExt,
};
use std::cell::RefCell;
use tokio::sync::{
    mpsc::{Receiver, Sender},
    oneshot,
};
use webkit2gtk::{LoadEvent, WebContext, WebView, WebViewExt};

use crate::view_models::NewsItemVM;
use crate::view_models::NewsSourceVM;

type AuthResponseSender = oneshot::Sender<AuthResponse>;

mod news_list_box_row;
mod sources_list_box_row;

/// Communicating from VK provider to UI
pub enum Message {
    /// Request to display authentication page and get AuthResponse, argument is a send part of a oneshot channel
    /// to send back the response with access_token etc.
    Auth(AuthResponseSender),
    /// Updated own user info received
    OwnInfo(UserModel),
    /// New incoming message to display in the user's wall
    News(NewsUpdate),
    /// Older news to let user scroll back
    OlderNews(NewsUpdate),
    /// Updating news sources, friends and groups
    NewsSources(SourcesUpdate),
}

pub enum Request {
    // Stop working
    Stop,
    // Request a portion of news prior the oldest
    NewsOlder,
    // request a portion of news after the most recent
    NewsNext,
}

type MessageReceiver = Receiver<Message>;
type RequestSender = Sender<Request>;

pub fn build(application: &gtk::Application, rx_msg: MessageReceiver, tx_req: RequestSender) {
    let main_glade = include_str!("main.glade");
    let builder = Builder::from_string(main_glade);

    // main window
    let window: ApplicationWindow = builder
        .get_object("main_window")
        .expect("Couldn't get main_window");
    window.set_application(Some(application));

    // list news
    let news_item_model = gio::ListStore::new(NewsItemVM::static_type());
    let list_news: gtk::ListBox = builder
        .get_object("news_list")
        .expect("Couldn't get news_list widget");
    list_news.bind_model(
        Some(&news_item_model),
        clone!(@weak window => @default-panic, move |item| {
            let item = item.downcast_ref::<NewsItemVM>().expect("News item view model is of wrong type");
            let box_ = news_list_box_row::build(item);
            box_.upcast::<gtk::Widget>()
        }),
    );

    // sources list
    let sources_item_model = gio::ListStore::new(NewsSourceVM::static_type());
    let list_sources: gtk::ListBox = builder
        .get_object("news_sources")
        .expect("Couldn't get news_sources widget");
    list_sources.bind_model(
        Some(&sources_item_model),
        clone!(@weak window => @default-panic, move |item| {
            let item = item.downcast_ref::<NewsSourceVM>().expect("News source view model is of wrong type");
            let box_ = sources_list_box_row::build(item);
            box_.upcast::<gtk::Widget>()
        }),
    );

    // signals
    let tx_req_copy = tx_req.clone();
    builder.connect_signals(move |_, handler_name| {
        // This is the one-time callback to register signals.
        // Here we map each handler name to its handler.
        if handler_name == "news_edge_reached" {
            // Return the news scroll handler
            let tx_req_copy2 = tx_req_copy.clone();
            Box::new(move |values| {
                for val in values {
                    if let Some(pos) = val.downcast_ref::<gtk::PositionType>() {
                        if let Some(pos) = pos.get() {
                            match pos {
                                gtk::PositionType::Top => {
                                    log::debug!("reached top, requesting older news");
                                    let main_context = glib::MainContext::default();
                                    let tx_req_copy3 = tx_req_copy2.clone();
                                    main_context.spawn_local(async move {
                                        let _ = tx_req_copy3.send(Request::NewsOlder).await;
                                    });
                                }
                                gtk::PositionType::Bottom => {
                                    log::debug!("reached bottom, requesting more recent news");
                                    let main_context = glib::MainContext::default();
                                    let tx_req_copy3 = tx_req_copy2.clone();
                                    main_context.spawn_local(async move {
                                        let _ = tx_req_copy3.send(Request::NewsNext).await;
                                    });
                                }
                                _ => log::warn!("reached unreachable"),
                            }
                        };
                    }
                }
                None
            })
        } else if handler_name == "delete_main_window" {
            // Return the news exit handler
            let tx_req_copy2 = tx_req_copy.clone();
            Box::new(move |_| {
                log::debug!("sending stop request to vk_provider");
                let main_context = glib::MainContext::default();
                let tx_req_copy3 = tx_req_copy2.clone();
                main_context.spawn_local(async move {
                    let _ = tx_req_copy3.send(Request::Stop).await;
                });
                Some(glib::Value::from(&false))
            })
        } else {
            panic!("Unknown handler name {}", handler_name)
        }
    });

    // select visible right pane
    show_right_pane(&builder, "page_view_home");

    launch_msg_handler(news_item_model, builder, rx_msg);

    window.show_all();
}

/// Spawns message handler as a task on the main event loop
fn launch_msg_handler(model: gio::ListStore, ui_builder: Builder, mut rx: MessageReceiver) {
    let main_context = glib::MainContext::default();
    let future = async move {
        let mut cnt_news = 0;
        while let Some(item) = rx.recv().await {
            match item {
                Message::Auth(tx_response) => {
                    let webview = build_auth_view(&ui_builder, tx_response);
                    let web_auth: ScrolledWindow = ui_builder
                        .get_object("web_auth")
                        .expect("Couldn't get view_auth");
                    web_auth.add(&webview);
                    web_auth.show_all();
                    show_right_pane(&ui_builder, "page_view_auth");
                }
                Message::OwnInfo(vm) => {
                    show_user_info(&ui_builder, &vm);
                }
                Message::News(update) => {
                    if !update.is_empty() {
                        let scroll_to_end = cnt_news == 0;
                        for view_model in update.into_iter().rev() {
                            model.append(&NewsItemVM::new(&view_model));
                            cnt_news += 1;
                        }
                        if scroll_to_end && cnt_news > 0 {
                            let news_list: gtk::ListBox = ui_builder
                                .get_object("news_list")
                                .expect("Couldn't get news_list");
                            if let Some(news_adjustment) = news_list.get_adjustment() {
                                // srcroll down the list
                                let new_height = news_list.get_preferred_height().1 as f64;
                                news_adjustment.set_upper(new_height);
                                let pos = new_height - news_adjustment.get_page_size();
                                news_adjustment.set_value(pos);
                                log::debug!(
                                    "scroll news to {} of {} for the first time",
                                    pos,
                                    new_height
                                );
                            }
                        }
                    }
                }
                Message::OlderNews(update) => {
                    let news_list: gtk::ListBox = ui_builder
                        .get_object("news_list")
                        .expect("Couldn't get news_list");
                    let stored_height = news_list.get_preferred_height().1;
                    // natural news order is from most recent to oldest,
                    // so insert every next prior previous i.e. always at 0 position:
                    for view_model in update.into_iter() {
                        model.insert(0, &NewsItemVM::new(&view_model));
                        cnt_news += 1;
                    }
                    if let Some(news_adjustment) = news_list.get_adjustment() {
                        let new_height = news_list.get_preferred_height().1;
                        // scroll to delta height to position of previous start item
                        let pos = new_height as f64 - stored_height as f64;
                        news_adjustment.set_value(pos);
                        log::debug!("scroll news to {} after inserting older news", pos);
                    }
                }
                Message::NewsSources(update) => {
                    // update sources pane from incoming data
                    for view_model in update.into_iter() {
                        model.insert(0, &NewsSourceVM::new(&view_model));
                    }
                }
            };
        }
    };
    main_context.spawn_local(future);
}

fn test_property<'t, T, E>(prop: &'t Result<glib::Value, E>, value: T) -> bool
where
    T: glib::value::FromValueOptional<'t> + PartialEq,
{
    if let Ok(glib_value) = prop {
        if let Ok(prop) = glib_value.get::<T>() {
            if let Some(val) = prop {
                return val == value;
            }
        }
    }
    false
}

fn build_auth_view(ui_builder: &Builder, tx_response: AuthResponseSender) -> WebView {
    // create WebKit2GTK view
    let context = WebContext::get_default().unwrap();
    //context.set_web_extensions_directory("webkit2gtk-tmp/");
    let webview = WebView::with_context(&context);
    webview.load_uri(AccessTokenProvider::get_auth_uri().as_str());
    let tx_holder = RefCell::new(Some(tx_response));
    webview.connect_load_changed(
        clone!(@strong webview, @strong ui_builder => move |view, evt| {
            log::debug!("{} {}", evt, view.get_uri().unwrap());
            if evt == LoadEvent::Finished {
                if let Some(uri) = view.get_uri() {
                    if AccessTokenProvider::is_auth_succeeded_uri(uri.as_str()) {
                        // parse auth response
                        if let Ok(auth) = uri.as_str().parse::<AuthResponse>() {
                            log::debug!("authentication is successful: {}", auth);
                            let tx_response = tx_holder.borrow_mut().take().unwrap();
                            if let Err(e) = tx_response.send(auth) {
                                log::error!("failed sending auth_response: {}", e);
                            }
                            // remove child to prevemt from using it more than once!
                            let parent: ScrolledWindow = ui_builder
                                .get_object("web_auth")
                                .expect("Couldn't get view_auth");
                            parent.remove(&webview);
                            // view news page
                            //todo: view previous page
                            show_right_pane(&ui_builder, "page_view_home");
                        }
                    }
                }
            }
        }),
    );
    webview
}

fn show_right_pane(ui_builder: &Builder, name: &str) {
    let right_pane: Stack = ui_builder
        .get_object("right_pane")
        .expect("Couldn't get right_pane");
    right_pane.set_visible_child_name(name);
}

fn show_user_info(ui_builder: &Builder, view_model: &UserModel) {
    if !view_model.image.is_empty() {
        let user_image: Image = ui_builder
            .get_object("user_image")
            .expect("Couldn't get user_image widget");
        user_image.set_from_file(&view_model.image);
    }
    let user_name: Label = ui_builder
        .get_object("user_name")
        .expect("Couldn't get user_name widget");
    user_name.set_label(&view_model.name);
    let user_status: Label = ui_builder
        .get_object("user_status")
        .expect("Couldn't get user_status widget");
    user_status.set_label("online");
}
