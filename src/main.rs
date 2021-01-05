#[macro_use]
extern crate glib;

use gio::prelude::*;
//use glib::clone;
use chrono::prelude::*;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder};
use std::env::args;

mod news_item_row_data;
use news_item_row_data::RowData;

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("main.glade");
    let builder = Builder::from_string(glade_src);

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
    //let list_news = gtk::ListBox::new();
    list_news.bind_model(
        Some(&news_item_model),
        clone!(@weak window => @default-panic, move |item| {
            let box_ = gtk::ListBoxRow::new();

            let item = item.downcast_ref::<RowData>().expect("Row data is of wrong type");
            let vbox = gtk::Box::new(gtk::Orientation::Vertical, 5);

            // let header = gtk::HeaderBar::new();
            // item.bind_property("author", &header, "title")
            //     .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            //     .build();
            // author
            let lbl_author = gtk::Label::new(None);
            item.bind_property("author", &lbl_author, "label")
                .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
                .build();
            vbox.pack_start(&lbl_author, true, true, 0);
            // title
            let lbl_title = gtk::Label::new(None);
            item.bind_property("title", &lbl_title, "label")
                .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
                .build();
            vbox.pack_start(&lbl_title, true, true, 0);
            // content
            let lbl_content = gtk::Label::new(None);
            item.bind_property("content", &lbl_content, "label")
                .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
                .build();
            vbox.pack_start(&lbl_content, true, true, 0);

            box_.add(&vbox);
            box_.show_all();
            box_.upcast::<gtk::Widget>()
        }),
    );

    let local: chrono::DateTime<Local> = Local::now();
    for i in 0..40 {
        news_item_model.append(&RowData::new(
            &format!("Author {} @ {}", i, local),
            &format!("Title {}", i),
            &format!("Content {}", i),
        ));
    }

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(Some("com.aae.gvk.example"), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
