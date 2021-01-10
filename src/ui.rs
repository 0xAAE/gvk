use crate::vk_provider::{AccessTokenProvider, AuthResponse};
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, ScrolledWindow, Stack};
use tokio::sync::mpsc::Receiver;
use webkit2gtk::{LoadEvent, WebContext, WebView, WebViewExt};

mod news_item_row_data;
pub use news_item_row_data::NewsItem;
use news_item_row_data::RowData;

pub enum Message {
    Auth(String),
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
    let right_pane: Stack = builder
        .get_object("right_pane")
        .expect("Couldn't get right_pane");
    right_pane.set_visible_child_name("page_view_auth");

    // create WebKit2GTK view
    let context = WebContext::get_default().unwrap();
    //context.set_web_extensions_directory("webkit2gtk-tmp/");
    let webview = WebView::with_context(&context);
    webview.load_uri("https://oauth.vk.com/authorize?client_id=7720259&display=page&redirect_uri=https://oauth.vk.com/blank.html&scope=offline&response_type=token&v=5.52");
    let web_auth: ScrolledWindow = builder
        .get_object("web_auth")
        .expect("Couldn't get view_auth");
    webview.connect_load_changed(clone!(@weak right_pane => move |view, evt| {
        //todo: logging
        println!("{}: {}", evt, view.get_uri().unwrap());
        if evt == LoadEvent::Finished {
            if let Some(uri) = view.get_uri() {
                if AccessTokenProvider::is_auth_succeeded_uri(uri.as_str()) {
                    // parse auth response
                    if let Ok(auth) = uri.as_str().parse::<AuthResponse>() {
                        //todo: logging
                        println!("Authenticated {}", auth);
                        //todo: store auth response
                        //todo: update user info view
                        // show news
                        right_pane.set_visible_child_name("page_view_news");
                    }
                }
            }
        }
    }));
    web_auth.add(&webview);

    // let proceed_auth: Button = builder
    //     .get_object("proceed_auth")
    //     .expect("Couldn't get proceed_auth");
    // proceed_auth.connect_clicked(clone!(@weak right_pane, @strong webview => move |_| {
    //     //todo: logging
    //     println!("Url: {}", webview.get_uri().unwrap());
    //     right_pane.set_visible_child_name("page_view_news");
    // }));

    launch_msg_handler(news_item_model, rx);

    window.show_all();
}

/// Spawns message handler as a task on the main event loop
fn launch_msg_handler(model: gio::ListStore, mut rx: MessageReceiver) {
    let main_context = glib::MainContext::default();
    let future = async move {
        while let Some(item) = rx.recv().await {
            match item {
                Message::Auth(_access_token) => {}
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
