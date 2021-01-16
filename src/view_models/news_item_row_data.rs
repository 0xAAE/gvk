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
    }

    // GObject property definitions for our three values
    static PROPERTIES: [subclass::Property; 5] = [
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
        glib::Object::new(
            Self::static_type(),
            &[
                ("author", &model.author),
                ("avatar", &model.avatar),
                ("itemtype", &model.itemtype),
                ("datetime", &model.datetime),
                ("content", &model.content),
            ],
        )
        .expect("Failed to create row data")
        .downcast()
        .expect("Created row data is of wrong type")
    }
}
