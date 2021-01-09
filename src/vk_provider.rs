use super::ui::{Message, NewsItem};
use chrono::prelude::*;
use serde::Deserialize;
use std::time::Duration;
use tokio::runtime::Builder;
use tokio::sync::mpsc::{error::TrySendError, Sender};
use tokio::time::delay_for;

type MessageSender = Sender<Message>;

const AUTH_URI: &str = "https://oauth.vk.com/authorize";
const AUTH_PARAMS: [(&str, &str); 6] = [
    ("client_id", "7720259"),
    ("display", "page"),
    ("redirect_uri", "https://oauth.vk.com/blank.html"),
    ("scope", "offline"), // "friends" is possible too
    ("response_type", "token"),
    ("v", "5.52"),
];

/// Spawn separate thread to handle communication.
pub fn launch_news_provider(mut tx: MessageSender) {
    // Note that blocking I/O with threads can be prevented
    // by using asynchronous code, which is often a better
    // choice. For the sake of this example, we showcase the
    // way to use a thread when there is no other option.

    // tokio-0.2.24
    let mut runtime = Builder::new()
        .threaded_scheduler()
        .enable_all()
        .core_threads(4)
        .thread_name("gvk-vk-provider")
        .thread_stack_size(3 * 1024 * 1024)
        .build()
        .unwrap();
    // tokio-1.0
    // let runtime = Builder::new_multi_thread()
    //     .worker_threads(2)
    //     .thread_name("gvk-vk-provider")
    //     .thread_stack_size(3 * 1024 * 1024)
    //     .build()
    //     .unwrap();

    runtime.block_on(async move {
        let access_token = match get_access_token().await {
            Ok(tmp) => tmp,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

        let _ = tx.try_send(Message::News(NewsItem {
            author: "Authorization result".to_string(),
            title: "Start".to_string(),
            datetime: Local::now(),
            content: format!("Aceess token:\n{}", access_token).to_string(),
        }));

        let mut counter: usize = 0;
        loop {
            // Instead of a counter, your application code will
            // block here on TCP or serial communications.
            let data = NewsItem {
                author: format!("Author {}", counter).to_string(),
                title: format!("Title {}", counter).to_string(),
                datetime: Local::now(),
                content: format!("Content {}:\n\tline 1\nline 2\nline 3", counter).to_string(),
            };

            match tx.try_send(Message::News(data)) {
                Ok(_) => {}
                Err(TrySendError::Full(_)) => {
                    println!("Data is produced too fast for GUI");
                }
                Err(TrySendError::Closed(_)) => {
                    println!("GUI stopped, stopping thread.");
                    break;
                }
            }
            counter += 1;

            delay_for(Duration::from_millis(100)).await;
        }
    });
}

#[derive(Deserialize, Default)]
struct AccessTokenResponse {
    access_token: String,
    expires_in: u64,
    user_id: String,
}

pub async fn get_access_token() -> Result<String, String> {
    // https://oauth.vk.com/authorize?client_id=7720259&display=page&redirect_uri=https://oauth.vk.com/blank.html&scope=offline&response_type=token&v=5.52
    // AUTH_URI?key=value&..
    let mut uri = AUTH_URI.to_string();
    uri.push('?');
    for (n, (name, value)) in AUTH_PARAMS.iter().enumerate() {
        if n > 0 {
            uri.push('&');
        }
        uri.push_str(name);
        uri.push('=');
        uri.push_str(value);
    }
    let req = reqwest::get(&uri).await;
    match req {
        Ok(_response) => {
            //Ok(format!("{:?}", response.header("Location").unwrap())),
            Ok("".into())
        }
        Err(e) => Err(format!("Auth get token error: {}", e)),
    }
}
