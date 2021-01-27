// Our GObject subclass for carrying an author, type and content of a news item for the list_news ListBox model
//
// (!) Store any property in a RefCell to allow for interior mutability
// Properties are exposed via normal GObject properties. This allows us to use property
// bindings below to bind the values with what widgets display in the UI
use crate::models::ActorModel;
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
        name: RefCell<Option<String>>,
        // author image / portrait
        avatar: RefCell<Option<String>>,
        // type: post, photo, etc.
        desc: RefCell<Option<String>>,
        // date and time
        uri: RefCell<Option<String>>,
        // text
        comment: RefCell<Option<String>>,
    }

    // GObject property definitions for our three values
    static PROPERTIES: [subclass::Property; 5] = [
        subclass::Property("name", |val| {
            Param::string(val, "Name", "Name", None, FLAGS)
        }),
        subclass::Property("avatar", |val| {
            Param::string(val, "Avatar", "Avatar", None, FLAGS)
        }),
        subclass::Property("desc", |val| {
            Param::string(val, "Desc", "Desc", None, FLAGS)
        }),
        subclass::Property("uri", |val| Param::string(val, "URI", "URI", None, FLAGS)),
        subclass::Property("comment", |val| {
            Param::string(val, "Coment", "Comment", None, FLAGS)
        }),
    ];

    // Basic declaration of our type for the GObject type system
    impl ObjectSubclass for RowData {
        const NAME: &'static str = "NewsSourceVM";
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
                name: RefCell::new(None),
                avatar: RefCell::new(None),
                desc: RefCell::new(None),
                uri: RefCell::new(None),
                comment: RefCell::new(None),
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
                subclass::Property("name", ..) => {
                    self.name.replace(value.get().expect("name set_property"));
                }
                subclass::Property("avatar", ..) => {
                    self.avatar
                        .replace(value.get().expect("avatar set_property"));
                }
                subclass::Property("desc", ..) => {
                    self.desc.replace(value.get().expect("desc set_property"));
                }
                subclass::Property("uri", ..) => {
                    self.uri.replace(value.get().expect("uri set_property"));
                }
                subclass::Property("comment", ..) => {
                    self.comment
                        .replace(value.get().expect("comment set_property"));
                }
                //
                _ => unimplemented!(),
            }
        }

        fn get_property(&self, _obj: &glib::Object, id: usize) -> Result<glib::Value, ()> {
            let prop = &PROPERTIES[id];

            match *prop {
                subclass::Property("name", ..) => Ok(self.name.borrow().to_value()),
                subclass::Property("avatar", ..) => Ok(self.avatar.borrow().to_value()),
                subclass::Property("desc", ..) => Ok(self.desc.borrow().to_value()),
                subclass::Property("uri", ..) => Ok(self.uri.borrow().to_value()),
                subclass::Property("comment", ..) => Ok(self.comment.borrow().to_value()),
                //
                _ => unimplemented!(),
            }
        }
    }
}

// Public part of the NewsSourceVM type. This behaves like a normal gtk-rs-style GObject
// binding
glib_wrapper! {
    pub struct NewsSourceVM(
        Object<subclass::simple::InstanceStruct<imp::RowData>,
        subclass::simple::ClassStruct<imp::RowData>, NewsSourceVMClass>
    );

    match fn {
        get_type => || imp::RowData::get_type().to_glib(),
    }
}

// Constructor for new instances. This simply calls glib::Object::new() with
// initial values for our two properties and then returns the new instance
impl NewsSourceVM {
    pub fn new(model: &ActorModel) -> NewsSourceVM {
        glib::Object::new(
            Self::static_type(),
            &[
                ("name", &model.name),
                ("avatar", &model.avatar),
                ("desc", &model.desc),
                ("uri", &model.rel_uri),
                ("comment", &model.comment),
            ],
        )
        .expect("Failed to create row data")
        .downcast()
        .expect("Created row data is of wrong type")
    }
}
