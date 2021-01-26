use super::*;

pub fn build(item: &NewsSourceVM) -> gtk::ListBoxRow {
    let box_ = gtk::ListBoxRow::new();

    let item_view_glade = include_str!("../source_item_view.glade");
    let builder = Builder::from_string(item_view_glade);
    let item_view: gtk::Box = builder
        .get_object("source_item_view")
        .expect("Couldn't get source_item_view");

    let name: gtk::Label = builder
        .get_object("src_name")
        .expect("Couldn't get src_name");
    item.bind_property("name", &name, "label")
        .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
        .build();

    let avatar: gtk::Image = builder
        .get_object("src_avatar")
        .expect("Couldn't get src_avatar");
    item.bind_property("avatar", &avatar, "file")
        .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
        .build();

    let desc: gtk::Label = builder
        .get_object("src_desc")
        .expect("Couldn't get src_desc");
    item.bind_property("desc", &desc, "label")
        .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
        .build();

    let uri: gtk::Label = builder.get_object("src_uri").expect("Couldn't get src_uri");
    item.bind_property("uri", &uri, "label")
        .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
        .build();

    let comment: gtk::Label = builder
        .get_object("src_comment")
        .expect("Couldn't get src_comment");
    item.bind_property("comment", &comment, "label")
        .flags(glib::BindingFlags::DEFAULT | glib::BindingFlags::SYNC_CREATE)
        .build();

    box_.add(&item_view);
    box_.show();
    box_
}
