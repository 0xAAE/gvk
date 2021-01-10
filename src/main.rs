#[macro_use]
extern crate glib;

use gio::prelude::*;
use std::cell::RefCell;
use std::env::args;
use std::sync::Arc;
use std::thread;
use tokio::sync::{mpsc, oneshot};

mod ui;
mod vk_provider;
use vk_provider::launch_vk_provider;
mod storage;
use storage::Storage;

fn main() {
    // Create a channel between communication thread and main event loop:
    let (tx_msg, rx_msg) = mpsc::channel(1000);

    let application = gtk::Application::new(Some("com.aae.gvk.example"), Default::default())
        .expect("Initialization failed...");

    let storage = Arc::new(Storage::new());
    let storage_ref = RefCell::new(Some(storage.clone()));
    let rx_msg_ref = RefCell::new(Some(rx_msg));
    application.connect_activate(move |app| {
        ui::build(
            app,
            rx_msg_ref.borrow_mut().take().unwrap(),
            storage_ref.borrow_mut().take().unwrap(),
        );
    });

    // Create a channel to send stop to communication thread aftre the ui main loop stops
    let (tx_stop, rx_stop) = oneshot::channel();
    let tokio_stack_size = 3 * 1024 * 1024; // taken from some tokio doc example
    let tokio_thread_pool_size = 2;
    let handle = thread::Builder::new()
        .stack_size(tokio_stack_size)
        .name("vkhost".into())
        .spawn(move || {
            launch_vk_provider(
                rx_stop,
                storage,
                tx_msg,
                tokio_stack_size,
                tokio_thread_pool_size,
            );
        })
        .unwrap();
    application.run(&args().collect::<Vec<_>>());
    if tx_stop.send(()).is_ok() {
        let _ = handle.join();
    }
}
