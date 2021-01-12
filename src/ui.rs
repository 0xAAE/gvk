use crate::vk_provider::{AccessTokenProvider, AuthResponse, UserViewModel};
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, Image, Label, ScrolledWindow, Stack};
use std::cell::RefCell;
use tokio::sync::{mpsc::Receiver, oneshot};
use webkit2gtk::{LoadEvent, WebContext, WebView, WebViewExt};

mod news_item_row_data;
pub use news_item_row_data::NewsItem;
use news_item_row_data::RowData;

type AuthResponseSender = oneshot::Sender<AuthResponse>;

/// Communicating from VK provider to UI
pub enum Message {
    /// Request to display authentication page and get AuthResponse, argument is a send part of a oneshot channel
    /// to send back the response with access_token etc.
    Auth(AuthResponseSender),
    /// Updated own user info received
    OwnInfo(UserViewModel),
    /// New incoming message to display in the user's wall
    News(NewsItem),
}

type MessageReceiver = Receiver<Message>;

pub fn build(application: &gtk::Application, rx: MessageReceiver) {
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
            item.bind_property("title", &header, "title")
                .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
                .build();
            item.bind_property("author", &header, "subtitle")
                .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
                .build();

            // datetime
            let news_item_datetime: gtk::Label = builder
                .get_object("news_item_datetime")
                .expect("Couldn't get news_item_datetime");
            news_item_datetime.set_halign(gtk::Align::Start);
            item.bind_property("datetime", &news_item_datetime, "label")
                .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
                .build();

            // content
            let news_item_content: gtk::Label = builder
                .get_object("news_item_content")
                .expect("Couldn't get news_item_content");
            news_item_content.set_halign(gtk::Align::Start);
            item.bind_property("content", &news_item_content, "label")
                .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
                .build();

            box_.add(&news_item_view);
            box_.show_all();
            box_.upcast::<gtk::Widget>()
        }),
    );

    // select visible right pane
    show_right_pane(&builder, "page_view_news");

    launch_msg_handler(news_item_model, builder, rx);

    window.show_all();
}

/// Spawns message handler as a task on the main event loop
fn launch_msg_handler(model: gio::ListStore, ui_builder: Builder, mut rx: MessageReceiver) {
    let main_context = glib::MainContext::default();
    let future = async move {
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
                Message::News(item) => model.append(&RowData::new(
                    &item.author,
                    &item.title,
                    &format!("{}", item.datetime.format("%d.%m.%Y %H:%M (%a)")),
                    &item.content,
                )),
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
            //todo: logging
            println!("{}: {}", evt, view.get_uri().unwrap());
            if evt == LoadEvent::Finished {
                if let Some(uri) = view.get_uri() {
                    if AccessTokenProvider::is_auth_succeeded_uri(uri.as_str()) {
                        // parse auth response
                        if let Ok(auth) = uri.as_str().parse::<AuthResponse>() {
                            //todo: logging
                            println!("Authentication is successful: {}", auth);
                            let tx_response = tx_holder.borrow_mut().take().unwrap();
                            if let Err(e) = tx_response.send(auth) {
                                println!("Failed sending auth_response: {}", e);
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
