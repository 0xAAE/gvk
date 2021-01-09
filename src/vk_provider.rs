use super::ui::{Message, NewsItem};
use chrono::prelude::*;
use serde::Deserialize;
use std::time::Duration;
use tokio::runtime::Builder;
use tokio::sync::{
    mpsc::{error::TrySendError, Sender},
    oneshot,
    oneshot::error::TryRecvError,
};
use tokio::time::sleep;

type MessageSender = Sender<Message>;
type StopReceiver = oneshot::Receiver<()>;

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
pub fn launch_vk_provider(
    rx_stop: StopReceiver,
    tx: MessageSender,
    stack_size: usize,
    thread_pool_size: usize,
) {
    let runtime = Builder::new_multi_thread()
        .worker_threads(thread_pool_size)
        .enable_all()
        .thread_name("vk")
        .thread_stack_size(stack_size)
        .build()
        .unwrap();

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

        let mut rx_stop = rx_stop;

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
            sleep(Duration::from_millis(1000)).await;
            match rx_stop.try_recv() {
                Err(TryRecvError::Empty) => continue,
                _ => break,
            }
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
