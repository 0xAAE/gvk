// Our GObject subclass for carrying an author, type and content of a news item for the list_news ListBox model
//
// (!) Store any property in a RefCell to allow for interior mutability
// Properties are exposed via normal GObject properties. This allows us to use property
// bindings below to bind the values with what widgets display in the UI
use crate::models::NewsItemModel;
use gio::prelude::*;
use glib::subclass;
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::ParamSpec as Param;

const FLAGS: glib::ParamFlags = glib::ParamFlags::READWRITE;

// Implementation sub-module of the GObject
mod imp {
    use super::*;
    use std::cell::RefCell;

    // The actual data structure that stores our values. This is not accessible
    // directly from the outside.
    pub struct RowData {
        // author name
        author: RefCell<Option<String>>,
        // author image / portrait
        avatar: RefCell<Option<String>>,
        // type: post, photo, etc.
        itemtype: RefCell<Option<String>>,
        // date and time
        datetime: RefCell<Option<String>>,
        // text
        content: RefCell<Option<String>>,
        // images
        image0: RefCell<Option<String>>,
        image0lbl: RefCell<Option<String>>,
        image0vis: RefCell<bool>,
        image1: RefCell<Option<String>>,
        image1lbl: RefCell<Option<String>>,
        image1vis: RefCell<bool>,
        image2: RefCell<Option<String>>,
        image2lbl: RefCell<Option<String>>,
        image2vis: RefCell<bool>,
        image3: RefCell<Option<String>>,
        image3lbl: RefCell<Option<String>>,
        image3vis: RefCell<bool>,
        image4: RefCell<Option<String>>,
        image4lbl: RefCell<Option<String>>,
        image4vis: RefCell<bool>,
        image5: RefCell<Option<String>>,
        image5lbl: RefCell<Option<String>>,
        image5vis: RefCell<bool>,
        image6: RefCell<Option<String>>,
        image6lbl: RefCell<Option<String>>,
        image6vis: RefCell<bool>,
        image7: RefCell<Option<String>>,
        image7lbl: RefCell<Option<String>>,
        image7vis: RefCell<bool>,
        image8: RefCell<Option<String>>,
        image8lbl: RefCell<Option<String>>,
        image8vis: RefCell<bool>,
        image9: RefCell<Option<String>>,
        image9lbl: RefCell<Option<String>>,
        image9vis: RefCell<bool>,
        // links
        link0txt: RefCell<Option<String>>,
        link0url: RefCell<Option<String>>,
        link0vis: RefCell<bool>,
        link1txt: RefCell<Option<String>>,
        link1url: RefCell<Option<String>>,
        link1vis: RefCell<bool>,
        link2txt: RefCell<Option<String>>,
        link2url: RefCell<Option<String>>,
        link2vis: RefCell<bool>,
        link3txt: RefCell<Option<String>>,
        link3url: RefCell<Option<String>>,
        link3vis: RefCell<bool>,
    }

    // GObject property definitions for our three values
    static PROPERTIES: [subclass::Property; 47] = [
        subclass::Property("author", |val| {
            Param::string(val, "Author", "Author", None, FLAGS)
        }),
        subclass::Property("avatar", |val| {
            Param::string(val, "Avatar", "Avatar", None, FLAGS)
        }),
        subclass::Property("itemtype", |val| {
            Param::string(val, "ItemT", "ItemT", None, FLAGS)
        }),
        subclass::Property("datetime", |val| {
            Param::string(val, "Dt", "Dt", None, FLAGS)
        }),
        subclass::Property("content", |val| {
            Param::string(val, "Cont", "Cont", None, FLAGS)
        }),
        subclass::Property("image0", |val| {
            Param::string(val, "Image0", "Image0", None, FLAGS)
        }),
        subclass::Property("image0lbl", |val| {
            Param::string(val, "Im0lbl", "Im0lbl", None, FLAGS)
        }),
        subclass::Property("image0vis", |val| {
            Param::boolean(val, "Im0vis", "Im0vis", false, FLAGS)
        }),
        subclass::Property("image1", |val| {
            Param::string(val, "Image1", "Image1", None, FLAGS)
        }),
        subclass::Property("image1lbl", |val| {
            Param::string(val, "Im1lbl", "Im1lbl", None, FLAGS)
        }),
        subclass::Property("image1vis", |val| {
            Param::boolean(val, "Im1vis", "Im1vis", false, FLAGS)
        }),
        subclass::Property("image2", |val| {
            Param::string(val, "Image2", "Image2", None, FLAGS)
        }),
        subclass::Property("image2lbl", |val| {
            Param::string(val, "Im2lbl", "Im2lbl", None, FLAGS)
        }),
        subclass::Property("image2vis", |val| {
            Param::boolean(val, "Im2vis", "Im2vis", false, FLAGS)
        }),
        subclass::Property("image3", |val| {
            Param::string(val, "Image3", "Image3", None, FLAGS)
        }),
        subclass::Property("image3lbl", |val| {
            Param::string(val, "Im3lbl", "Im3lbl", None, FLAGS)
        }),
        subclass::Property("image3vis", |val| {
            Param::boolean(val, "Im3vis", "Im3vis", false, FLAGS)
        }),
        subclass::Property("image4", |val| {
            Param::string(val, "Image4", "Image4", None, FLAGS)
        }),
        subclass::Property("image4lbl", |val| {
            Param::string(val, "Im4lbl", "Im4lbl", None, FLAGS)
        }),
        subclass::Property("image4vis", |val| {
            Param::boolean(val, "Im4vis", "Im4vis", false, FLAGS)
        }),
        subclass::Property("image5", |val| {
            Param::string(val, "Image5", "Image5", None, FLAGS)
        }),
        subclass::Property("image5lbl", |val| {
            Param::string(val, "Im5lbl", "Im5lbl", None, FLAGS)
        }),
        subclass::Property("image5vis", |val| {
            Param::boolean(val, "Im5vis", "Im5vis", false, FLAGS)
        }),
        subclass::Property("image6", |val| {
            Param::string(val, "Image6", "Image6", None, FLAGS)
        }),
        subclass::Property("image6lbl", |val| {
            Param::string(val, "Im6lbl", "Im6lbl", None, FLAGS)
        }),
        subclass::Property("image6vis", |val| {
            Param::boolean(val, "Im6vis", "Im6vis", false, FLAGS)
        }),
        subclass::Property("image7", |val| {
            Param::string(val, "Image7", "Image7", None, FLAGS)
        }),
        subclass::Property("image7lbl", |val| {
            Param::string(val, "Im7lbl", "Im7lbl", None, FLAGS)
        }),
        subclass::Property("image7vis", |val| {
            Param::boolean(val, "Im7vis", "Im7vis", false, FLAGS)
        }),
        subclass::Property("image8", |val| {
            Param::string(val, "Image8", "Image8", None, FLAGS)
        }),
        subclass::Property("image8lbl", |val| {
            Param::string(val, "Im8lbl", "Im8lbl", None, FLAGS)
        }),
        subclass::Property("image8vis", |val| {
            Param::boolean(val, "Im8vis", "Im8vis", false, FLAGS)
        }),
        subclass::Property("image9", |val| {
            Param::string(val, "Image9", "Image9", None, FLAGS)
        }),
        subclass::Property("image9lbl", |val| {
            Param::string(val, "Im9lbl", "Im9lbl", None, FLAGS)
        }),
        subclass::Property("image9vis", |val| {
            Param::boolean(val, "Im9vis", "Im9vis", false, FLAGS)
        }),
        // links
        subclass::Property("link0txt", |val| {
            Param::string(val, "L0txt", "L0txt", None, FLAGS)
        }),
        subclass::Property("link0url", |val| {
            Param::string(val, "L0url", "L0url", None, FLAGS)
        }),
        subclass::Property("link0vis", |val| {
            Param::boolean(val, "L0vis", "L0vis", false, FLAGS)
        }),
        subclass::Property("link1txt", |val| {
            Param::string(val, "L1txt", "L1txt", None, FLAGS)
        }),
        subclass::Property("link1url", |val| {
            Param::string(val, "L1url", "L1url", None, FLAGS)
        }),
        subclass::Property("link1vis", |val| {
            Param::boolean(val, "L1vis", "L1vis", false, FLAGS)
        }),
        subclass::Property("link2txt", |val| {
            Param::string(val, "L2txt", "L2txt", None, FLAGS)
        }),
        subclass::Property("link2url", |val| {
            Param::string(val, "L2url", "L2url", None, FLAGS)
        }),
        subclass::Property("link2vis", |val| {
            Param::boolean(val, "L2vis", "L2vis", false, FLAGS)
        }),
        subclass::Property("link3txt", |val| {
            Param::string(val, "L3txt", "L3txt", None, FLAGS)
        }),
        subclass::Property("link3url", |val| {
            Param::string(val, "L3url", "L3url", None, FLAGS)
        }),
        subclass::Property("link3vis", |val| {
            Param::boolean(val, "L3vis", "L3vis", false, FLAGS)
        }),
    ];

    // Basic declaration of our type for the GObject type system
    impl ObjectSubclass for RowData {
        const NAME: &'static str = "NewsItemVM";
        type ParentType = glib::Object;
        type Instance = subclass::simple::InstanceStruct<Self>;
        type Class = subclass::simple::ClassStruct<Self>;

        glib_object_subclass!();

        // Called exactly once before the first instantiation of an instance. This
        // sets up any type-specific things, in this specific case it installs the
        // properties so that GObject knows about their existence and they can be
        // used on instances of our type
        fn class_init(klass: &mut Self::Class) {
            klass.install_properties(&PROPERTIES);
        }

        // Called once at the very beginning of instantiation of each instance and
        // creates the data structure that contains all our state
        fn new() -> Self {
            Self {
                author: RefCell::new(None),
                avatar: RefCell::new(None),
                itemtype: RefCell::new(None),
                datetime: RefCell::new(None),
                content: RefCell::new(None),
                // images
                image0: RefCell::new(None),
                image0lbl: RefCell::new(None),
                image0vis: RefCell::new(false),
                image1: RefCell::new(None),
                image1lbl: RefCell::new(None),
                image1vis: RefCell::new(false),
                image2: RefCell::new(None),
                image2lbl: RefCell::new(None),
                image2vis: RefCell::new(false),
                image3: RefCell::new(None),
                image3lbl: RefCell::new(None),
                image3vis: RefCell::new(false),
                image4: RefCell::new(None),
                image4lbl: RefCell::new(None),
                image4vis: RefCell::new(false),
                image5: RefCell::new(None),
                image5lbl: RefCell::new(None),
                image5vis: RefCell::new(false),
                image6: RefCell::new(None),
                image6lbl: RefCell::new(None),
                image6vis: RefCell::new(false),
                image7: RefCell::new(None),
                image7lbl: RefCell::new(None),
                image7vis: RefCell::new(false),
                image8: RefCell::new(None),
                image8lbl: RefCell::new(None),
                image8vis: RefCell::new(false),
                image9: RefCell::new(None),
                image9lbl: RefCell::new(None),
                image9vis: RefCell::new(false),
                // links
                link0txt: RefCell::new(None),
                link0url: RefCell::new(None),
                link0vis: RefCell::new(false),
                link1txt: RefCell::new(None),
                link1url: RefCell::new(None),
                link1vis: RefCell::new(false),
                link2txt: RefCell::new(None),
                link2url: RefCell::new(None),
                link2vis: RefCell::new(false),
                link3txt: RefCell::new(None),
                link3url: RefCell::new(None),
                link3vis: RefCell::new(false),
            }
        }
    }

    // The ObjectImpl trait provides the setters/getters for GObject properties.
    // Here we need to provide the values that are internally stored back to the
    // caller, or store whatever new value the caller is providing.
    //
    // This maps between the GObject properties and our internal storage of the
    // corresponding values of the properties.
    impl ObjectImpl for RowData {
        glib_object_impl!();

        fn set_property(&self, _obj: &glib::Object, id: usize, value: &glib::Value) {
            let prop = &PROPERTIES[id];

            match *prop {
                subclass::Property("author", ..) => {
                    self.author
                        .replace(value.get().expect("author set_property"));
                }
                subclass::Property("avatar", ..) => {
                    self.avatar
                        .replace(value.get().expect("avatar set_property"));
                }
                subclass::Property("itemtype", ..) => {
                    self.itemtype
                        .replace(value.get().expect("itemtype set_property"));
                }
                subclass::Property("datetime", ..) => {
                    self.datetime
                        .replace(value.get().expect("datetime set_property"));
                }
                subclass::Property("content", ..) => {
                    self.content
                        .replace(value.get().expect("content set_property"));
                }
                subclass::Property("image0", ..) => {
                    self.image0
                        .replace(value.get().expect("image0 set_property"));
                }
                subclass::Property("image0lbl", ..) => {
                    self.image0lbl
                        .replace(value.get().expect("image0lbl set_property"));
                }
                subclass::Property("image0vis", ..) => {
                    self.image0vis.replace(
                        value
                            .get()
                            .expect("image0vis set_property")
                            .unwrap_or(false),
                    );
                }
                subclass::Property("image1", ..) => {
                    self.image1
                        .replace(value.get().expect("image1 set_property"));
                }
                subclass::Property("image1lbl", ..) => {
                    self.image1lbl
                        .replace(value.get().expect("image1lbl set_property"));
                }
                subclass::Property("image1vis", ..) => {
                    self.image1vis.replace(
                        value
                            .get()
                            .expect("image1vis set_property")
                            .unwrap_or(false),
                    );
                }
                subclass::Property("image2", ..) => {
                    self.image2
                        .replace(value.get().expect("image2 set_property"));
                }
                subclass::Property("image2lbl", ..) => {
                    self.image2lbl
                        .replace(value.get().expect("image2lbl set_property"));
                }
                subclass::Property("image2vis", ..) => {
                    self.image2vis.replace(
                        value
                            .get()
                            .expect("image2vis set_property")
                            .unwrap_or(false),
                    );
                }
                subclass::Property("image3", ..) => {
                    self.image3
                        .replace(value.get().expect("image3 set_property"));
                }
                subclass::Property("image3lbl", ..) => {
                    self.image3lbl
                        .replace(value.get().expect("image3lbl set_property"));
                }
                subclass::Property("image3vis", ..) => {
                    self.image3vis.replace(
                        value
                            .get()
                            .expect("image3vis set_property")
                            .unwrap_or(false),
                    );
                }
                subclass::Property("image4", ..) => {
                    self.image4
                        .replace(value.get().expect("image4 set_property"));
                }
                subclass::Property("image4lbl", ..) => {
                    self.image4lbl
                        .replace(value.get().expect("image4lbl set_property"));
                }
                subclass::Property("image4vis", ..) => {
                    self.image4vis.replace(
                        value
                            .get()
                            .expect("image4vis set_property")
                            .unwrap_or(false),
                    );
                }
                subclass::Property("image5", ..) => {
                    self.image5
                        .replace(value.get().expect("image5 set_property"));
                }
                subclass::Property("image5lbl", ..) => {
                    self.image5lbl
                        .replace(value.get().expect("image5lbl set_property"));
                }
                subclass::Property("image5vis", ..) => {
                    self.image5vis.replace(
                        value
                            .get()
                            .expect("image5vis set_property")
                            .unwrap_or(false),
                    );
                }
                subclass::Property("image6", ..) => {
                    self.image6
                        .replace(value.get().expect("image6 set_property"));
                }
                subclass::Property("image6lbl", ..) => {
                    self.image6lbl
                        .replace(value.get().expect("image6lbl set_property"));
                }
                subclass::Property("image6vis", ..) => {
                    self.image6vis.replace(
                        value
                            .get()
                            .expect("image6vis set_property")
                            .unwrap_or(false),
                    );
                }
                subclass::Property("image7", ..) => {
                    self.image7
                        .replace(value.get().expect("image7 set_property"));
                }
                subclass::Property("image7lbl", ..) => {
                    self.image7lbl
                        .replace(value.get().expect("image7lbl set_property"));
                }
                subclass::Property("image7vis", ..) => {
                    self.image7vis.replace(
                        value
                            .get()
                            .expect("image7vis set_property")
                            .unwrap_or(false),
                    );
                }
                subclass::Property("image8", ..) => {
                    self.image8
                        .replace(value.get().expect("image8 set_property"));
                }
                subclass::Property("image8lbl", ..) => {
                    self.image8lbl
                        .replace(value.get().expect("image8lbl set_property"));
                }
                subclass::Property("image8vis", ..) => {
                    self.image8vis.replace(
                        value
                            .get()
                            .expect("image8vis set_property")
                            .unwrap_or(false),
                    );
                }
                subclass::Property("image9", ..) => {
                    self.image9
                        .replace(value.get().expect("image9 set_property"));
                }
                subclass::Property("image9lbl", ..) => {
                    self.image9lbl
                        .replace(value.get().expect("image9lbl set_property"));
                }
                subclass::Property("image9vis", ..) => {
                    self.image9vis.replace(
                        value
                            .get()
                            .expect("image9vis set_property")
                            .unwrap_or(false),
                    );
                }
                // links
                subclass::Property("link0txt", ..) => {
                    self.link0txt
                        .replace(value.get().expect("link0txt set_property"));
                }
                subclass::Property("link0url", ..) => {
                    self.link0url
                        .replace(value.get().expect("link0url set_property"));
                }
                subclass::Property("link0vis", ..) => {
                    self.link0vis
                        .replace(value.get().expect("link0vis set_property").unwrap_or(false));
                }
                subclass::Property("link1txt", ..) => {
                    self.link1txt
                        .replace(value.get().expect("link1txt set_property"));
                }
                subclass::Property("link1url", ..) => {
                    self.link1url
                        .replace(value.get().expect("link1url set_property"));
                }
                subclass::Property("link1vis", ..) => {
                    self.link1vis
                        .replace(value.get().expect("link1vis set_property").unwrap_or(false));
                }
                subclass::Property("link2txt", ..) => {
                    self.link2txt
                        .replace(value.get().expect("link2txt set_property"));
                }
                subclass::Property("link2url", ..) => {
                    self.link2url
                        .replace(value.get().expect("link2url set_property"));
                }
                subclass::Property("link2vis", ..) => {
                    self.link2vis
                        .replace(value.get().expect("link2vis set_property").unwrap_or(false));
                }
                subclass::Property("link3txt", ..) => {
                    self.link3txt
                        .replace(value.get().expect("link3txt set_property"));
                }
                subclass::Property("link3url", ..) => {
                    self.link3url
                        .replace(value.get().expect("link3url set_property"));
                }
                subclass::Property("link3vis", ..) => {
                    self.link3vis
                        .replace(value.get().expect("link3vis set_property").unwrap_or(false));
                }
                //
                _ => unimplemented!(),
            }
        }

        fn get_property(&self, _obj: &glib::Object, id: usize) -> Result<glib::Value, ()> {
            let prop = &PROPERTIES[id];

            match *prop {
                subclass::Property("author", ..) => Ok(self.author.borrow().to_value()),
                subclass::Property("avatar", ..) => Ok(self.avatar.borrow().to_value()),
                subclass::Property("itemtype", ..) => Ok(self.itemtype.borrow().to_value()),
                subclass::Property("datetime", ..) => Ok(self.datetime.borrow().to_value()),
                subclass::Property("content", ..) => Ok(self.content.borrow().to_value()),
                subclass::Property("image0", ..) => Ok(self.image0.borrow().to_value()),
                subclass::Property("image0lbl", ..) => Ok(self.image0lbl.borrow().to_value()),
                subclass::Property("image0vis", ..) => Ok(self.image0vis.borrow().to_value()),
                subclass::Property("image1", ..) => Ok(self.image1.borrow().to_value()),
                subclass::Property("image1lbl", ..) => Ok(self.image1lbl.borrow().to_value()),
                subclass::Property("image1vis", ..) => Ok(self.image1vis.borrow().to_value()),
                subclass::Property("image2", ..) => Ok(self.image2.borrow().to_value()),
                subclass::Property("image2lbl", ..) => Ok(self.image2lbl.borrow().to_value()),
                subclass::Property("image2vis", ..) => Ok(self.image2vis.borrow().to_value()),
                subclass::Property("image3", ..) => Ok(self.image3.borrow().to_value()),
                subclass::Property("image3lbl", ..) => Ok(self.image3lbl.borrow().to_value()),
                subclass::Property("image3vis", ..) => Ok(self.image3vis.borrow().to_value()),
                subclass::Property("image4", ..) => Ok(self.image4.borrow().to_value()),
                subclass::Property("image4lbl", ..) => Ok(self.image4lbl.borrow().to_value()),
                subclass::Property("image4vis", ..) => Ok(self.image4vis.borrow().to_value()),
                subclass::Property("image5", ..) => Ok(self.image5.borrow().to_value()),
                subclass::Property("image5lbl", ..) => Ok(self.image5lbl.borrow().to_value()),
                subclass::Property("image5vis", ..) => Ok(self.image5vis.borrow().to_value()),
                subclass::Property("image6", ..) => Ok(self.image6.borrow().to_value()),
                subclass::Property("image6lbl", ..) => Ok(self.image6lbl.borrow().to_value()),
                subclass::Property("image6vis", ..) => Ok(self.image6vis.borrow().to_value()),
                subclass::Property("image7", ..) => Ok(self.image7.borrow().to_value()),
                subclass::Property("image7lbl", ..) => Ok(self.image7lbl.borrow().to_value()),
                subclass::Property("image7vis", ..) => Ok(self.image7vis.borrow().to_value()),
                subclass::Property("image8", ..) => Ok(self.image8.borrow().to_value()),
                subclass::Property("image8lbl", ..) => Ok(self.image8lbl.borrow().to_value()),
                subclass::Property("image8vis", ..) => Ok(self.image8vis.borrow().to_value()),
                subclass::Property("image9", ..) => Ok(self.image9.borrow().to_value()),
                subclass::Property("image9lbl", ..) => Ok(self.image9lbl.borrow().to_value()),
                subclass::Property("image9vis", ..) => Ok(self.image9vis.borrow().to_value()),
                // links
                subclass::Property("link0txt", ..) => Ok(self.link0txt.borrow().to_value()),
                subclass::Property("link0url", ..) => Ok(self.link0url.borrow().to_value()),
                subclass::Property("link0vis", ..) => Ok(self.link0vis.borrow().to_value()),
                subclass::Property("link1txt", ..) => Ok(self.link1txt.borrow().to_value()),
                subclass::Property("link1url", ..) => Ok(self.link1url.borrow().to_value()),
                subclass::Property("link1vis", ..) => Ok(self.link1vis.borrow().to_value()),
                subclass::Property("link2txt", ..) => Ok(self.link2txt.borrow().to_value()),
                subclass::Property("link2url", ..) => Ok(self.link2url.borrow().to_value()),
                subclass::Property("link2vis", ..) => Ok(self.link2vis.borrow().to_value()),
                subclass::Property("link3txt", ..) => Ok(self.link3txt.borrow().to_value()),
                subclass::Property("link3url", ..) => Ok(self.link3url.borrow().to_value()),
                subclass::Property("link3vis", ..) => Ok(self.link3vis.borrow().to_value()),
                //
                _ => unimplemented!(),
            }
        }
    }
}

// Public part of the RowData type. This behaves like a normal gtk-rs-style GObject
// binding
glib_wrapper! {
    pub struct RowData(Object<subclass::simple::InstanceStruct<imp::RowData>, subclass::simple::ClassStruct<imp::RowData>, RowDataClass>);

    match fn {
        get_type => || imp::RowData::get_type().to_glib(),
    }
}

// Constructor for new instances. This simply calls glib::Object::new() with
// initial values for our two properties and then returns the new instance
impl RowData {
    pub fn new(model: &NewsItemModel) -> RowData {
        const MAX_IMAGES: usize = 10;
        let mut image_file: [String; MAX_IMAGES] = Default::default(); // [String::new(); MAX_IMAGES];
        let mut image_text: [String; MAX_IMAGES] = Default::default(); // [String::new(); MAX_IMAGES];
        let mut image_vis: [bool; MAX_IMAGES] = [false; MAX_IMAGES];
        if let Some(ref photos) = model.photos {
            for (i, photo) in photos.iter().enumerate() {
                if !photo.uri.is_empty() {
                    image_file[i] = photo.uri.clone();
                    image_text[i] = photo.text.clone();
                    image_vis[i] = true;
                }
            }
        };

        const MAX_LINKS: usize = 4;
        let mut link_url: [String; MAX_LINKS] = Default::default(); // [String::new(); MAX_LINKS];
        let mut link_txt: [String; MAX_LINKS] = Default::default(); // [String::new(); MAX_LINKS];
        let mut link_vis: [bool; MAX_LINKS] = [false; MAX_LINKS];
        if let Some(ref links) = model.links {
            for (i, link) in links.iter().enumerate() {
                if !link.uri.is_empty() {
                    link_url[i] = link.uri.clone();
                    link_txt[i] = link.text.clone();
                    link_vis[i] = true;
                }
            }
        };

        glib::Object::new(
            Self::static_type(),
            &[
                ("author", &model.author),
                ("avatar", &model.avatar),
                ("itemtype", &model.itemtype),
                ("datetime", &model.datetime),
                ("content", &model.content),
                ("image0", &image_file[0]),
                ("image0lbl", &image_text[0]),
                ("image0vis", &image_vis[0]),
                ("image1", &image_file[1]),
                ("image1lbl", &image_text[1]),
                ("image1vis", &image_vis[1]),
                ("image2", &image_file[2]),
                ("image2lbl", &image_text[2]),
                ("image2vis", &image_vis[2]),
                ("image3", &image_file[3]),
                ("image3lbl", &image_text[3]),
                ("image3vis", &image_vis[3]),
                ("image4", &image_file[4]),
                ("image4lbl", &image_text[4]),
                ("image4vis", &image_vis[4]),
                ("image5", &image_file[5]),
                ("image5lbl", &image_text[5]),
                ("image5vis", &image_vis[5]),
                ("image6", &image_file[6]),
                ("image6lbl", &image_text[6]),
                ("image6vis", &image_vis[6]),
                ("image7", &image_file[7]),
                ("image7lbl", &image_text[7]),
                ("image7vis", &image_vis[7]),
                ("image8", &image_file[8]),
                ("image8lbl", &image_text[8]),
                ("image8vis", &image_vis[8]),
                ("image9", &image_file[9]),
                ("image9lbl", &image_text[9]),
                ("image9vis", &image_vis[9]),
                // links
                ("link0url", &link_url[0]),
                ("link0txt", &link_txt[0]),
                ("link0vis", &link_vis[0]),
                ("link1url", &link_url[1]),
                ("link1txt", &link_txt[1]),
                ("link1vis", &link_vis[1]),
                ("link2url", &link_url[2]),
                ("link2txt", &link_txt[2]),
                ("link2vis", &link_vis[2]),
                ("link3url", &link_url[3]),
                ("link3txt", &link_txt[3]),
                ("link3vis", &link_vis[3]),
            ],
        )
        .expect("Failed to create row data")
        .downcast()
        .expect("Created row data is of wrong type")
    }
}
