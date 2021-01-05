use chrono::prelude::*;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder};

use super::news_item_row_data::RowData;

pub fn build(application: &gtk::Application) {
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

    let local: chrono::DateTime<Local> = Local::now();
    for i in 0..40 {
        news_item_model.append(&RowData::new(
            &format!("Author {}", i),
            &format!("Title {}", i),
            &format!("{}", local.format("%d.%m.%Y %H:%M (%a)")),
            &format!("Content {}:\n\tline 1\nline 2\nline 3", i),
        ));
    }

    window.show_all();
}
