#[macro_use]
extern crate glib;

use env_logger::{fmt::TimestampPrecision, Builder, Env, Target};
use gio::prelude::*;
use std::cell::RefCell;
use std::env::args;
use std::thread;
use tokio::sync::{mpsc, oneshot};

mod models;
mod storage;
mod ui;
mod utils;
mod view_models;
mod vk_provider;

fn main() {
    Builder::from_env(Env::default().default_filter_or("debug"))
        .target(Target::Stdout)
        .format_timestamp(Some(TimestampPrecision::Seconds))
        .init();

    // Create a channel from communication thread to main event loop (UI):
    let (tx_msg, rx_msg) = mpsc::channel(1_000);

    // Create a channel from the main loop (UI) to communication thread:
    let (tx_req, rx_req) = mpsc::channel(1_000);

    let application = gtk::Application::new(Some("com.aae.gvk.example"), Default::default())
        .expect("Initialization failed...");

    let rx_msg_ref = RefCell::new(Some(rx_msg));
    let tx_req_ref = RefCell::new(Some(tx_req));
    application.connect_activate(move |app| {
        ui::build(
            app,
            rx_msg_ref.borrow_mut().take().unwrap(),
            tx_req_ref.borrow_mut().take().unwrap(),
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
            vk_provider::run_with_own_runtime(
                rx_stop,
                tx_msg,
                rx_req,
                tokio_stack_size,
                tokio_thread_pool_size,
            );
        })
        .unwrap();
    application.run(&args().collect::<Vec<_>>());
    log::debug!("GUI has stopped");
    if tx_stop.send(()).is_ok() {
        let _ = handle.join();
    }
}
