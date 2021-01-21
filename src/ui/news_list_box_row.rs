use super::*;
//use crate::view_models::RowData;

pub fn build(item: &RowData) -> gtk::ListBoxRow {
    let box_ = gtk::ListBoxRow::new();

    let news_item_view_glade = include_str!("../news_item_view.glade");
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

    // photos
    let image_0: gtk::Image = builder.get_object("image_0").expect("Couldn't get image_0");
    if test_property(&item.get_property("image0vis"), true) {
        item.bind_property("image0", &image_0, "file")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
    }
    let image_1: gtk::Image = builder.get_object("image_1").expect("Couldn't get image_1");
    if test_property(&item.get_property("image1vis"), true) {
        item.bind_property("image1", &image_1, "file")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
    }
    let image_2: gtk::Image = builder.get_object("image_2").expect("Couldn't get image_2");
    if test_property(&item.get_property("image2vis"), true) {
        item.bind_property("image2", &image_2, "file")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
    }
    let image_3: gtk::Image = builder.get_object("image_3").expect("Couldn't get image_3");
    if test_property(&item.get_property("image3vis"), true) {
        item.bind_property("image3", &image_3, "file")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
    }
    let image_4: gtk::Image = builder.get_object("image_4").expect("Couldn't get image_4");
    if test_property(&item.get_property("image4vis"), true) {
        item.bind_property("image4", &image_4, "file")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
    }
    let image_5: gtk::Image = builder.get_object("image_5").expect("Couldn't get image_5");
    if test_property(&item.get_property("image5vis"), true) {
        item.bind_property("image5", &image_5, "file")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
    }
    let image_6: gtk::Image = builder.get_object("image_6").expect("Couldn't get image_6");
    if test_property(&item.get_property("image6vis"), true) {
        item.bind_property("image6", &image_6, "file")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
    }
    let image_7: gtk::Image = builder.get_object("image_7").expect("Couldn't get image_7");
    if test_property(&item.get_property("image7vis"), true) {
        item.bind_property("image7", &image_7, "file")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
    }
    let image_8: gtk::Image = builder.get_object("image_8").expect("Couldn't get image_8");
    if test_property(&item.get_property("image8vis"), true) {
        item.bind_property("image8", &image_8, "file")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
    }
    let image_9: gtk::Image = builder.get_object("image_9").expect("Couldn't get image_9");
    if test_property(&item.get_property("image9vis"), true) {
        item.bind_property("image9", &image_9, "file")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
    }

    box_.add(&news_item_view);
    box_.show_all();
    box_
}
