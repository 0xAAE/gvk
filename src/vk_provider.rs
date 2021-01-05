use chrono::prelude::*;
use futures::channel::mpsc::Sender;
use std::thread;

use super::ui::NewsItem;
type NewsItemSender = Sender<NewsItem>;

/// Spawn separate thread to handle communication.
pub fn launch_news_provider(mut tx: NewsItemSender) {
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
