use super::ui::{Message, NewsItem};
use async_std::{channel::Sender, task};
use chrono::prelude::*;
use serde::Deserialize;
use std::thread;
use surf::middleware::{Middleware, Next};
use surf::{Client, Request, Response};

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
pub fn launch_news_provider(tx: MessageSender) {
    // Note that blocking I/O with threads can be prevented
    // by using asynchronous code, which is often a better
    // choice. For the sake of this example, we showcase the
    // way to use a thread when there is no other option.

    task::block_on(async move {
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

            match tx.try_send(Message::News(data)) {
                Ok(_) => {}
                Err(err) => {
                    if err.is_full() {
                        println!("Data is produced too fast for GUI");
                    } else if err.is_closed() {
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
    let req = surf::get(&uri);
    let client = surf::client()
        .with(surf::middleware::Redirect::new(10))
        .with(AccessTokenResponse::default());
    match client.send(req).await {
        Ok(response) => {
            //Ok(format!("{:?}", response.header("Location").unwrap())),
            Ok("".into())
        }
        Err(e) => Err(format!("Auth get token error: {}", e)),
    }
}

#[surf::utils::async_trait]
impl Middleware for AccessTokenResponse {
    async fn handle(
        &self,
        mut req: Request,
        client: Client,
        next: Next<'_>,
    ) -> surf::Result<Response> {
        // Note(Jeremiah): This is not ideal.
        //
        // HttpClient is currently too limiting for efficient redirects.
        // We do not want to make unnecessary full requests, but it is
        // presently required due to how Body streams work.
        //
        // Ideally we'd have methods to send a partial request stream,
        // without sending the body, that would potnetially be able to
        // get a server status before we attempt to send the body.
        //
        // As a work around we clone the request first (without the body),
        // and try sending it until we get some status back that is not a
        // redirect.

        let r: Request = req.clone();
        let res: Response = client.send(r).await?;
        if let Some(location) = res.header(surf::http::headers::LOCATION) {
            println!("{:?}", location);
        } else {
        }

        Ok(next.run(req, client).await?)
    }
}
