use crate::models::NewsUpdate;
use crate::vk_provider::{AccessTokenProvider, AuthResponse, UserViewModel};
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    AdjustmentExt, ApplicationWindow, Builder, Image, Label, ListBoxExt, ScrolledWindow, Stack,
    WidgetExt,
};
use std::cell::RefCell;
use tokio::sync::{
    mpsc::{Receiver, Sender},
    oneshot,
};
use webkit2gtk::{LoadEvent, WebContext, WebView, WebViewExt};

use crate::view_models::RowData;
type AuthResponseSender = oneshot::Sender<AuthResponse>;

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
            let box_ = gtk::ListBoxRow::new();

            let item = item.downcast_ref::<RowData>().expect("Row data is of wrong type");

            let news_item_view_glade = include_str!("news_item_view.glade");
            let builder = Builder::from_string(news_item_view_glade);
            let news_item_view: gtk::Box = builder
                .get_object("news_item_view")
                .expect("Couldn't get news_item_view");

            let header: gtk::HeaderBar = builder
                .get_object("news_item_header")
                .expect("Couldn't get news_item_header");
            item.bind_property("itemtype", &header, "subtitle")
                .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
                .build();
            item.bind_property("author", &header, "title")
                .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
                .build();
            let avatar: gtk::Image = builder
                .get_object("news_item_avatar")
                .expect("Couldn't get news_item_avatar");
            //avatar.set_from_file(&item.avatar);
            item.bind_property("avatar", &avatar, "file")
                .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
                .build();

            // datetime
            let news_item_datetime: gtk::Label = builder
                .get_object("news_item_datetime")
                .expect("Couldn't get news_item_datetime");
            item.bind_property("datetime", &news_item_datetime, "label")
                .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
                .build();

            // content
            let news_item_content: gtk::Label = builder
                .get_object("news_item_content")
                .expect("Couldn't get news_item_content");
            item.bind_property("content", &news_item_content, "label")
                .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
                .build();

            box_.add(&news_item_view);
            box_.show_all();
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
        } else if handler_name == "news_adjustment_value_changed" {
            Box::new(move |values| {
                for val in values {
                    if let Some(adjustment) = val.downcast_ref::<gtk::Adjustment>() {
                        if let Some(adjustment) = adjustment.get() {
                            log::debug!("adjustment value {}", adjustment.get_value());
                        }
                    }
                }
                None
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
                        let is_first_filling = cnt_news >= 0;
                        let news_adjustment: gtk::Adjustment = ui_builder
                            .get_object("news_adjustment")
                            .expect("Couldn't get news_adjustment");
                        let news_list: gtk::ListBox = ui_builder
                            .get_object("news_list")
                            .expect("Couldn't get news_list");
                        // --
                        log::debug!("before adding news");
                        log::debug!(
                            "adjustment: <{} - {} - {}>",
                            news_adjustment.get_lower(),
                            news_adjustment.get_value(),
                            news_adjustment.get_upper(),
                        );
                        let list_height_before = news_list.get_preferred_height();
                        log::debug!(
                            "news_list: {}, {:?}",
                            news_list.get_allocated_height(),
                            list_height_before
                        );
                        // --
                        for view_model in update.into_iter().rev() {
                            model.append(&RowData::new(&view_model));
                            cnt_news += 1;
                        }
                        // --
                        log::debug!("after adding news");
                        log::debug!(
                            "adjustment: <{} - {} - {}>",
                            news_adjustment.get_lower(),
                            news_adjustment.get_value(),
                            news_adjustment.get_upper(),
                        );
                        let list_height_after = news_list.get_preferred_height();
                        log::debug!(
                            "news_list: {}, {:?}",
                            news_list.get_allocated_height(),
                            list_height_after
                        );
                        // --
                        if is_first_filling && cnt_news > 0 {
                            // srcroll down the list
                            let pos = list_height_after.0 as f64;
                            // news_adjustment.set_upper(pos);
                            // news_adjustment.set_value(pos);
                            news_adjustment.configure(
                                pos,
                                0f64,
                                pos,
                                news_adjustment.get_step_increment(),
                                news_adjustment.get_page_increment(),
                                0f64,
                            );
                            log::debug!("scroll news to {:?} for the first time", pos);
                            // let window: gtk::ScrolledWindow = ui_builder
                            //     .get_object("view_news")
                            //     .expect("Couldn't get view_news");
                            // window.show_all();
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
