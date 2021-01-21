use crate::models::NewsUpdate;
use crate::vk_provider::{AccessTokenProvider, AuthResponse, UserViewModel};
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

use crate::view_models::RowData;
type AuthResponseSender = oneshot::Sender<AuthResponse>;

mod news_list_box_row;

/// Communicating from VK provider to UI
pub enum Message {
    /// Request to display authentication page and get AuthResponse, argument is a send part of a oneshot channel
    /// to send back the response with access_token etc.
    Auth(AuthResponseSender),
    /// Updated own user info received
    OwnInfo(UserViewModel),
    /// New incoming message to display in the user's wall
    News(NewsUpdate),
    /// Older news to let user scroll back
    OlderNews(NewsUpdate),
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
    let news_item_model = gio::ListStore::new(RowData::static_type());
    let list_news: gtk::ListBox = builder
        .get_object("news_list")
        .expect("Couldn't get list_news");
    list_news.bind_model(
        Some(&news_item_model),
        clone!(@weak window => @default-panic, move |item| {

            let item = item.downcast_ref::<RowData>().expect("Row data is of wrong type");
            let box_ = news_list_box_row::build(item);
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
    show_right_pane(&builder, "page_view_news");

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
                            model.append(&RowData::new(&view_model));
                            cnt_news += 1;
                        }
                        if scroll_to_end && cnt_news > 0 {
                            let news_list: gtk::ListBox = ui_builder
                                .get_object("news_list")
                                .expect("Couldn't get news_list");
                            if let Some(news_adjustment) = news_list.get_adjustment() {
                                // srcroll down the list
                                let h = if let Some(ref last_row) =
                                    news_list.get_row_at_index(cnt_news as i32 - 1)
                                {
                                    last_row.get_preferred_height().0
                                } else {
                                    0
                                };
                                let list_height_after = news_list.get_preferred_height();
                                let pos = list_height_after.0 as f64;
                                news_adjustment.set_upper(pos);
                                news_adjustment.set_value(pos - h as f64);
                                log::debug!("scroll news to {:?} for the first time", pos);
                            }
                        }
                    }
                }
                Message::OlderNews(update) => {
                    // natural news order is from most recent to oldest,
                    // so insert every next prior previous:
                    for view_model in update.into_iter() {
                        model.insert(0, &RowData::new(&view_model));
                        cnt_news += 1;
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
                            show_right_pane(&ui_builder, "page_view_news");
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

fn show_user_info(ui_builder: &Builder, view_model: &UserViewModel) {
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
