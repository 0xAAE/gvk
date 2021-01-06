#[macro_use]
extern crate glib;

use async_std::channel::bounded;
use gio::prelude::*;
use std::env::args;

mod ui;
mod vk_provider;

use vk_provider::launch_news_provider;

fn main() {
    let application = gtk::Application::new(Some("com.aae.gvk.example"), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        // Create a channel between communication thread and main event loop:
        let (tx_news, rx_news) = bounded(1000);
        ui::build(app, rx_news);
        launch_news_provider(tx_news);
    });

    application.run(&args().collect::<Vec<_>>());
}
