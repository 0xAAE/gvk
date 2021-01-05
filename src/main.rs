#[macro_use]
extern crate glib;

use gio::prelude::*;
use std::env::args;

mod news_item_row_data;
mod ui;

fn main() {
    let application = gtk::Application::new(Some("com.aae.gvk.example"), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        ui::build(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
