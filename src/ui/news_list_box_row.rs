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
    if test_property(&item.get_property("image0vis"), true) {
        let pic_0: gtk::Box = builder.get_object("pic_0").expect("Couldn't get pic_0");
        pic_0.set_visible(true);
        let image_0: gtk::Image = builder.get_object("image_0").expect("Couldn't get image_0");
        item.bind_property("image0", &image_0, "file")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
        let label_0: gtk::Label = builder.get_object("label_0").expect("Couldn't get label_0");
        item.bind_property("image0lbl", &label_0, "label")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
    }
    if test_property(&item.get_property("image1vis"), true) {
        let pic_1: gtk::Box = builder.get_object("pic_1").expect("Couldn't get image_1");
        pic_1.set_visible(true);
        let image_1: gtk::Image = builder.get_object("image_1").expect("Couldn't get pic_1");
        item.bind_property("image1", &image_1, "file")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
        let label_1: gtk::Label = builder.get_object("label_1").expect("Couldn't get label_1");
        item.bind_property("image1lbl", &label_1, "label")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
    }
    if test_property(&item.get_property("image2vis"), true) {
        let pic_2: gtk::Box = builder.get_object("pic_2").expect("Couldn't get pic_2");
        pic_2.set_visible(true);
        let image_2: gtk::Image = builder.get_object("image_2").expect("Couldn't get image_2");
        item.bind_property("image2", &image_2, "file")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
        let label_2: gtk::Label = builder.get_object("label_2").expect("Couldn't get label_2");
        item.bind_property("image2lbl", &label_2, "label")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
    }
    if test_property(&item.get_property("image3vis"), true) {
        let pic_3: gtk::Box = builder.get_object("pic_3").expect("Couldn't get pic_3");
        pic_3.set_visible(true);
        let image_3: gtk::Image = builder.get_object("image_3").expect("Couldn't get image_3");
        item.bind_property("image3", &image_3, "file")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
        let label_3: gtk::Label = builder.get_object("label_3").expect("Couldn't get label_3");
        item.bind_property("image3lbl", &label_3, "label")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
    }
    if test_property(&item.get_property("image4vis"), true) {
        let pic_4: gtk::Box = builder.get_object("pic_4").expect("Couldn't get pic_4");
        pic_4.set_visible(true);
        let image_4: gtk::Image = builder.get_object("image_4").expect("Couldn't get image_4");
        item.bind_property("image4", &image_4, "file")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
        let label_4: gtk::Label = builder.get_object("label_4").expect("Couldn't get label_4");
        item.bind_property("image4lbl", &label_4, "label")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
    }
    if test_property(&item.get_property("image5vis"), true) {
        let pic_5: gtk::Box = builder.get_object("pic_5").expect("Couldn't get pic_5");
        pic_5.set_visible(true);
        let image_5: gtk::Image = builder.get_object("image_5").expect("Couldn't get image_5");
        item.bind_property("image5", &image_5, "file")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
        let label_5: gtk::Label = builder.get_object("label_5").expect("Couldn't get label_5");
        item.bind_property("image5lbl", &label_5, "label")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
    }
    if test_property(&item.get_property("image6vis"), true) {
        let pic_6: gtk::Box = builder.get_object("pic_6").expect("Couldn't get pic_6");
        pic_6.set_visible(true);
        let image_6: gtk::Image = builder.get_object("image_6").expect("Couldn't get image_6");
        item.bind_property("image6", &image_6, "file")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
        let label_6: gtk::Label = builder.get_object("label_6").expect("Couldn't get label_6");
        item.bind_property("image6lbl", &label_6, "label")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
    }
    if test_property(&item.get_property("image7vis"), true) {
        let pic_7: gtk::Box = builder.get_object("pic_7").expect("Couldn't get pic_7");
        pic_7.set_visible(true);
        let image_7: gtk::Image = builder.get_object("image_7").expect("Couldn't get image_7");
        item.bind_property("image7", &image_7, "file")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
        let label_7: gtk::Label = builder.get_object("label_7").expect("Couldn't get label_7");
        item.bind_property("image7lbl", &label_7, "label")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
    }
    if test_property(&item.get_property("image8vis"), true) {
        let pic_8: gtk::Box = builder.get_object("pic_8").expect("Couldn't get pic_8");
        pic_8.set_visible(true);
        let image_8: gtk::Image = builder.get_object("image_8").expect("Couldn't get image_8");
        item.bind_property("image8", &image_8, "file")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
        let label_8: gtk::Label = builder.get_object("label_8").expect("Couldn't get label_8");
        item.bind_property("image8lbl", &label_8, "label")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
    }
    if test_property(&item.get_property("image9vis"), true) {
        let pic_9: gtk::Box = builder.get_object("pic_9").expect("Couldn't get pic_9");
        pic_9.set_visible(true);
        let image_9: gtk::Image = builder.get_object("image_9").expect("Couldn't get image_9");
        item.bind_property("image9", &image_9, "file")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
        let label_9: gtk::Label = builder.get_object("label_9").expect("Couldn't get label_9");
        item.bind_property("image9lbl", &label_9, "label")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
    }

    // links
    if test_property(&item.get_property("link0vis"), true) {
        let link_0: gtk::Box = builder.get_object("link_0").expect("Couldn't get link_0");
        link_0.set_visible(true);
        let link_text_0: gtk::Label = builder
            .get_object("link_text_0")
            .expect("Couldn't get link_text_0");
        item.bind_property("link0txt", &link_text_0, "label")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
        let link_url_0: gtk::Label = builder
            .get_object("link_url_0")
            .expect("Couldn't get link_url_0");
        item.bind_property("link0url", &link_url_0, "label")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
    }
    if test_property(&item.get_property("link1vis"), true) {
        let link_1: gtk::Box = builder.get_object("link_1").expect("Couldn't get link_1");
        link_1.set_visible(true);
        let link_text_1: gtk::Label = builder
            .get_object("link_text_1")
            .expect("Couldn't get link_text_1");
        item.bind_property("link1txt", &link_text_1, "label")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
        let link_url_1: gtk::Label = builder
            .get_object("link_url_1")
            .expect("Couldn't get link_url_1");
        item.bind_property("link1url", &link_url_1, "label")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
    }
    if test_property(&item.get_property("link2vis"), true) {
        let link_2: gtk::Box = builder.get_object("link_2").expect("Couldn't get link_2");
        link_2.set_visible(true);
        let link_text_2: gtk::Label = builder
            .get_object("link_text_2")
            .expect("Couldn't get link_text_2");
        item.bind_property("link2txt", &link_text_2, "label")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
        let link_url_2: gtk::Label = builder
            .get_object("link_url_2")
            .expect("Couldn't get link_url_2");
        item.bind_property("link2url", &link_url_2, "label")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
    }
    if test_property(&item.get_property("link3vis"), true) {
        let link_3: gtk::Box = builder.get_object("link_3").expect("Couldn't get link_3");
        link_3.set_visible(true);
        let link_text_3: gtk::Label = builder
            .get_object("link_text_3")
            .expect("Couldn't get link_text_3");
        item.bind_property("link3txt", &link_text_3, "label")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
        let link_url_3: gtk::Label = builder
            .get_object("link_url_3")
            .expect("Couldn't get link_url_3");
        item.bind_property("link3url", &link_url_3, "label")
            .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
            .build();
    }

    box_.add(&news_item_view);
    box_.show();
    box_
}
