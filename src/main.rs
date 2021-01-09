#[macro_use]
extern crate glib;

use gio::prelude::*;
use std::env::args;
use tokio::runtime::Builder;
use tokio::sync::mpsc;

mod ui;
mod vk_provider;

use vk_provider::launch_news_provider;

fn main() {
    let application = gtk::Application::new(Some("com.aae.gvk.example"), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        // Create a channel between communication thread and main event loop:
        let (tx_msg, rx_msg) = mpsc::channel(1000);
        ui::build(app, rx_msg);
        launch_news_provider(tx_msg);
    });

    application.run(&args().collect::<Vec<_>>());
}
