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
        // primary image
        image0: RefCell<Option<String>>,
        image0vis: RefCell<bool>,
        // secondary images: 1
        image1: RefCell<Option<String>>,
        image1vis: RefCell<bool>,
        // secondary images: 2
        image2: RefCell<Option<String>>,
        image2vis: RefCell<bool>,
        // other messages
        image3: RefCell<Option<String>>,
        image3vis: RefCell<bool>,
        image4: RefCell<Option<String>>,
        image4vis: RefCell<bool>,
        image5: RefCell<Option<String>>,
        image5vis: RefCell<bool>,
        image6: RefCell<Option<String>>,
        image6vis: RefCell<bool>,
        image7: RefCell<Option<String>>,
        image7vis: RefCell<bool>,
        image8: RefCell<Option<String>>,
        image8vis: RefCell<bool>,
        image9: RefCell<Option<String>>,
        image9vis: RefCell<bool>,
    }

    // GObject property definitions for our three values
    static PROPERTIES: [subclass::Property; 25] = [
        subclass::Property("author", |author| {
            glib::ParamSpec::string(
                author,
                "Author",
                "Author",
                // Default value
                None,
                glib::ParamFlags::READWRITE,
            )
        }),
        subclass::Property("avatar", |avatar| {
            glib::ParamSpec::string(
                avatar,
                "Avatar",
                "Avatar",
                // Default value
                None,
                glib::ParamFlags::READWRITE,
            )
        }),
        subclass::Property("itemtype", |itemtype| {
            glib::ParamSpec::string(
                itemtype,
                "ItemType",
                "ItemType",
                // Default value
                None,
                glib::ParamFlags::READWRITE,
            )
        }),
        subclass::Property("datetime", |datetime| {
            glib::ParamSpec::string(
                datetime,
                "Datetime",
                "Datetime",
                // Default value
                None,
                glib::ParamFlags::READWRITE,
            )
        }),
        subclass::Property("content", |content| {
            glib::ParamSpec::string(
                content,
                "Content",
                "Content",
                // Default value
                Some("empty".into()),
                glib::ParamFlags::READWRITE,
            )
        }),
        subclass::Property("image0", |image0| {
            glib::ParamSpec::string(
                image0,
                "Image0",
                "Image0",
                // Default value
                Some("empty".into()),
                glib::ParamFlags::READWRITE,
            )
        }),
        subclass::Property("image0vis", |image0vis| {
            glib::ParamSpec::boolean(
                image0vis,
                "Image0vis",
                "Image0vis",
                // Default value
                false,
                glib::ParamFlags::READWRITE,
            )
        }),
        subclass::Property("image1", |image1| {
            glib::ParamSpec::string(
                image1,
                "Image1",
                "Image1",
                // Default value
                Some("empty".into()),
                glib::ParamFlags::READWRITE,
            )
        }),
        subclass::Property("image1vis", |image1vis| {
            glib::ParamSpec::boolean(
                image1vis,
                "Image1vis",
                "Image1vis",
                // Default value
                false,
                glib::ParamFlags::READWRITE,
            )
        }),
        subclass::Property("image2", |image2| {
            glib::ParamSpec::string(
                image2,
                "Image2",
                "Image2",
                // Default value
                Some("empty".into()),
                glib::ParamFlags::READWRITE,
            )
        }),
        subclass::Property("image2vis", |image2vis| {
            glib::ParamSpec::boolean(
                image2vis,
                "Image2vis",
                "Image2vis",
                // Default value
                false,
                glib::ParamFlags::READWRITE,
            )
        }),
        subclass::Property("image3", |image3| {
            glib::ParamSpec::string(
                image3,
                "Image3",
                "Image3",
                // Default value
                Some("empty".into()),
                glib::ParamFlags::READWRITE,
            )
        }),
        subclass::Property("image3vis", |image3vis| {
            glib::ParamSpec::boolean(
                image3vis,
                "Image3vis",
                "Image3vis",
                // Default value
                false,
                glib::ParamFlags::READWRITE,
            )
        }),
        subclass::Property("image4", |image4| {
            glib::ParamSpec::string(
                image4,
                "Image4",
                "Image4",
                // Default value
                Some("empty".into()),
                glib::ParamFlags::READWRITE,
            )
        }),
        subclass::Property("image4vis", |image4vis| {
            glib::ParamSpec::boolean(
                image4vis,
                "Image4vis",
                "Image4vis",
                // Default value
                false,
                glib::ParamFlags::READWRITE,
            )
        }),
        subclass::Property("image5", |image5| {
            glib::ParamSpec::string(
                image5,
                "Image5",
                "Image5",
                // Default value
                Some("empty".into()),
                glib::ParamFlags::READWRITE,
            )
        }),
        subclass::Property("image5vis", |image5vis| {
            glib::ParamSpec::boolean(
                image5vis,
                "Image5vis",
                "Image5vis",
                // Default value
                false,
                glib::ParamFlags::READWRITE,
            )
        }),
        subclass::Property("image6", |image6| {
            glib::ParamSpec::string(
                image6,
                "Image6",
                "Image6",
                // Default value
                Some("empty".into()),
                glib::ParamFlags::READWRITE,
            )
        }),
        subclass::Property("image6vis", |image6vis| {
            glib::ParamSpec::boolean(
                image6vis,
                "Image6vis",
                "Image6vis",
                // Default value
                false,
                glib::ParamFlags::READWRITE,
            )
        }),
        subclass::Property("image7", |image7| {
            glib::ParamSpec::string(
                image7,
                "Image7",
                "Image7",
                // Default value
                Some("empty".into()),
                glib::ParamFlags::READWRITE,
            )
        }),
        subclass::Property("image7vis", |image7vis| {
            glib::ParamSpec::boolean(
                image7vis,
                "Image7vis",
                "Image7vis",
                // Default value
                false,
                glib::ParamFlags::READWRITE,
            )
        }),
        subclass::Property("image8", |image8| {
            glib::ParamSpec::string(
                image8,
                "Image8",
                "Image8",
                // Default value
                Some("empty".into()),
                glib::ParamFlags::READWRITE,
            )
        }),
        subclass::Property("image8vis", |image8vis| {
            glib::ParamSpec::boolean(
                image8vis,
                "Image8vis",
                "Image8vis",
                // Default value
                false,
                glib::ParamFlags::READWRITE,
            )
        }),
        subclass::Property("image9", |image9| {
            glib::ParamSpec::string(
                image9,
                "Image9",
                "Image9",
                // Default value
                Some("empty".into()),
                glib::ParamFlags::READWRITE,
            )
        }),
        subclass::Property("image9vis", |image9vis| {
            glib::ParamSpec::boolean(
                image9vis,
                "Image9vis",
                "Image9vis",
                // Default value
                false,
                glib::ParamFlags::READWRITE,
            )
        }),
    ];

    // Basic declaration of our type for the GObject type system
    impl ObjectSubclass for RowData {
        const NAME: &'static str = "RowData";
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
                image0: RefCell::new(None),
                image0vis: RefCell::new(false),
                image1: RefCell::new(None),
                image1vis: RefCell::new(false),
                image2: RefCell::new(None),
                image2vis: RefCell::new(false),
                image3: RefCell::new(None),
                image3vis: RefCell::new(false),
                image4: RefCell::new(None),
                image4vis: RefCell::new(false),
                image5: RefCell::new(None),
                image5vis: RefCell::new(false),
                image6: RefCell::new(None),
                image6vis: RefCell::new(false),
                image7: RefCell::new(None),
                image7vis: RefCell::new(false),
                image8: RefCell::new(None),
                image8vis: RefCell::new(false),
                image9: RefCell::new(None),
                image9vis: RefCell::new(false),
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
                    let author = value
                        .get()
                        .expect("author type conformity checked by `Object::set_property`");
                    self.author.replace(author);
                }
                subclass::Property("avatar", ..) => {
                    let avatar = value
                        .get()
                        .expect("avatar type conformity checked by `Object::set_property`");
                    self.avatar.replace(avatar);
                }
                subclass::Property("itemtype", ..) => {
                    let itemtype = value
                        .get()
                        .expect("itemtype type conformity checked by `Object::set_property`");
                    self.itemtype.replace(itemtype);
                }
                subclass::Property("datetime", ..) => {
                    let datetime = value
                        .get()
                        .expect("datetime type conformity checked by `Object::set_property`");
                    self.datetime.replace(datetime);
                }
                subclass::Property("content", ..) => {
                    let content = value
                        .get()
                        .expect("content type conformity checked by `Object::set_property`");
                    self.content.replace(content);
                }
                subclass::Property("image0", ..) => {
                    let image0 = value
                        .get()
                        .expect("image0 type conformity checked by `Object::set_property`");
                    self.image0.replace(image0);
                }
                subclass::Property("image0vis", ..) => {
                    let image0vis = value
                        .get()
                        .expect("image0vis type conformity checked by `Object::set_property`")
                        .unwrap_or(false);
                    self.image0vis.replace(image0vis);
                }
                subclass::Property("image1", ..) => {
                    let image1 = value
                        .get()
                        .expect("image1 type conformity checked by `Object::set_property`");
                    self.image1.replace(image1);
                }
                subclass::Property("image1vis", ..) => {
                    let image1vis = value
                        .get()
                        .expect("image1vis type conformity checked by `Object::set_property`")
                        .unwrap_or(false);
                    self.image1vis.replace(image1vis);
                }
                subclass::Property("image2", ..) => {
                    let image2 = value
                        .get()
                        .expect("image2 type conformity checked by `Object::set_property`");
                    self.image2.replace(image2);
                }
                subclass::Property("image2vis", ..) => {
                    let image2vis = value
                        .get()
                        .expect("image2vis type conformity checked by `Object::set_property`")
                        .unwrap_or(false);
                    self.image2vis.replace(image2vis);
                }
                subclass::Property("image3", ..) => {
                    let image3 = value
                        .get()
                        .expect("image3 type conformity checked by `Object::set_property`");
                    self.image3.replace(image3);
                }
                subclass::Property("image3vis", ..) => {
                    let image3vis = value
                        .get()
                        .expect("image3vis type conformity checked by `Object::set_property`")
                        .unwrap_or(false);
                    self.image3vis.replace(image3vis);
                }
                subclass::Property("image4", ..) => {
                    let image4 = value
                        .get()
                        .expect("image4 type conformity checked by `Object::set_property`");
                    self.image4.replace(image4);
                }
                subclass::Property("image4vis", ..) => {
                    let image4vis = value
                        .get()
                        .expect("image4vis type conformity checked by `Object::set_property`")
                        .unwrap_or(false);
                    self.image4vis.replace(image4vis);
                }
                subclass::Property("image5", ..) => {
                    let image5 = value
                        .get()
                        .expect("image5 type conformity checked by `Object::set_property`");
                    self.image5.replace(image5);
                }
                subclass::Property("image5vis", ..) => {
                    let image5vis = value
                        .get()
                        .expect("image5vis type conformity checked by `Object::set_property`")
                        .unwrap_or(false);
                    self.image5vis.replace(image5vis);
                }
                subclass::Property("image6", ..) => {
                    let image6 = value
                        .get()
                        .expect("image6 type conformity checked by `Object::set_property`");
                    self.image6.replace(image6);
                }
                subclass::Property("image6vis", ..) => {
                    let image6vis = value
                        .get()
                        .expect("image6vis type conformity checked by `Object::set_property`")
                        .unwrap_or(false);
                    self.image6vis.replace(image6vis);
                }
                subclass::Property("image7", ..) => {
                    let image7 = value
                        .get()
                        .expect("image7 type conformity checked by `Object::set_property`");
                    self.image7.replace(image7);
                }
                subclass::Property("image7vis", ..) => {
                    let image7vis = value
                        .get()
                        .expect("image7vis type conformity checked by `Object::set_property`")
                        .unwrap_or(false);
                    self.image7vis.replace(image7vis);
                }
                subclass::Property("image8", ..) => {
                    let image8 = value
                        .get()
                        .expect("image8 type conformity checked by `Object::set_property`");
                    self.image8.replace(image8);
                }
                subclass::Property("image8vis", ..) => {
                    let image8vis = value
                        .get()
                        .expect("image8vis type conformity checked by `Object::set_property`")
                        .unwrap_or(false);
                    self.image8vis.replace(image8vis);
                }
                subclass::Property("image9", ..) => {
                    let image9 = value
                        .get()
                        .expect("image9 type conformity checked by `Object::set_property`");
                    self.image9.replace(image9);
                }
                subclass::Property("image9vis", ..) => {
                    let image9vis = value
                        .get()
                        .expect("image9vis type conformity checked by `Object::set_property`")
                        .unwrap_or(false);
                    self.image9vis.replace(image9vis);
                }
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
                subclass::Property("image0vis", ..) => Ok(self.image0vis.borrow().to_value()),
                subclass::Property("image1", ..) => Ok(self.image1.borrow().to_value()),
                subclass::Property("image1vis", ..) => Ok(self.image1vis.borrow().to_value()),
                subclass::Property("image2", ..) => Ok(self.image2.borrow().to_value()),
                subclass::Property("image2vis", ..) => Ok(self.image2vis.borrow().to_value()),
                subclass::Property("image3", ..) => Ok(self.image3.borrow().to_value()),
                subclass::Property("image3vis", ..) => Ok(self.image3vis.borrow().to_value()),
                subclass::Property("image4", ..) => Ok(self.image4.borrow().to_value()),
                subclass::Property("image4vis", ..) => Ok(self.image4vis.borrow().to_value()),
                subclass::Property("image5", ..) => Ok(self.image5.borrow().to_value()),
                subclass::Property("image5vis", ..) => Ok(self.image5vis.borrow().to_value()),
                subclass::Property("image6", ..) => Ok(self.image6.borrow().to_value()),
                subclass::Property("image6vis", ..) => Ok(self.image6vis.borrow().to_value()),
                subclass::Property("image7", ..) => Ok(self.image7.borrow().to_value()),
                subclass::Property("image7vis", ..) => Ok(self.image7vis.borrow().to_value()),
                subclass::Property("image8", ..) => Ok(self.image8.borrow().to_value()),
                subclass::Property("image8vis", ..) => Ok(self.image8vis.borrow().to_value()),
                subclass::Property("image9", ..) => Ok(self.image9.borrow().to_value()),
                subclass::Property("image9vis", ..) => Ok(self.image9vis.borrow().to_value()),
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
        let mut image: [String; 10] = Default::default(); // [String::new(); 10];
        let mut vis: [bool; 10] = [false; 10];

        if let Some(ref photos) = model.photos {
            for (i, photo) in photos.iter().enumerate() {
                if !photo.uri.is_empty() {
                    image[i] = photo.uri.clone();
                    vis[i] = true;
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
                ("image0", &image[0]),
                ("image0vis", &vis[0]),
                ("image1", &image[1]),
                ("image1vis", &vis[1]),
                ("image2", &image[2]),
                ("image2vis", &vis[2]),
                ("image3", &image[3]),
                ("image3vis", &vis[3]),
                ("image4", &image[4]),
                ("image4vis", &vis[4]),
                ("image5", &image[5]),
                ("image5vis", &vis[5]),
                ("image6", &image[6]),
                ("image6vis", &vis[6]),
                ("image7", &image[7]),
                ("image7vis", &vis[7]),
                ("image8", &image[8]),
                ("image8vis", &vis[8]),
                ("image9", &image[9]),
                ("image9vis", &vis[9]),
            ],
        )
        .expect("Failed to create row data")
        .downcast()
        .expect("Created row data is of wrong type")
    }
}
