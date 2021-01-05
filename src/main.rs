#[macro_use]
extern crate glib;

use chrono::prelude::*;
use futures::channel::mpsc;
use gio::prelude::*;
use std::env::args;
use std::thread;

mod news_item_row_data;
mod ui;

use news_item_row_data::NewsItem;
type NewsItemSender = mpsc::Sender<NewsItem>;

fn main() {
    let application = gtk::Application::new(Some("com.aae.gvk.example"), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        // Create a channel between communication thread and main event loop:
        let (tx_news, rx_news) = mpsc::channel(1000);
        ui::build(app, rx_news);
        launch_news_provider(tx_news);
    });

    application.run(&args().collect::<Vec<_>>());
}

/// Spawn separate thread to handle communication.
fn launch_news_provider(mut tx: NewsItemSender) {
    // Note that blocking I/O with threads can be prevented
    // by using asynchronous code, which is often a better
    // choice. For the sake of this example, we showcase the
    // way to use a thread when there is no other option.

    thread::spawn(move || {
        let mut counter = 0;
        loop {
            // Instead of a counter, your application code will
            // block here on TCP or serial communications.
            let data = NewsItem {
                author: format!("Author {}", counter).to_string(),
                title: format!("Title {}", counter).to_string(),
                datetime: Local::now(),
                content: format!("Content {}:\n\tline 1\nline 2\nline 3", counter).to_string(),
            };

            match tx.try_send(data) {
                Ok(_) => {}
                Err(err) => {
                    if err.is_full() {
                        println!("Data is produced too fast for GUI");
                    } else if err.is_disconnected() {
                        println!("GUI stopped, stopping thread.");
                        break;
                    }
                }
            }
            counter += 1;
            thread::sleep(std::time::Duration::from_secs(2));
        }
    });
}
