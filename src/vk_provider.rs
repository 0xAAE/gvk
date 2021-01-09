use super::ui::{Message, NewsItem};
use chrono::prelude::*;
use serde::Deserialize;
use std::time::Duration;
use std::{fmt, str::FromStr};
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

impl FromStr for AccessTokenResponse {
    type Err = AccessTokenResponseParseError;

    fn from_str(uri: &str) -> Result<Self, Self::Err> {
        // https://oauth.vk.com/blank.html#
        // access_token=7de238693ce753e240abd1e1845480667bed442daf94e86df12d6c376322137fd2649661fbfd1352741fa&
        // expires_in=0&
        // user_id=184946538
        match uri.find("#") {
            None => Err(AccessTokenResponseParseError::Malformed),
            Some(pos) => {
                let (_, fragment) = uri.split_at(pos);
                if fragment.len() < 2 {
                    // "" and "=
                    return Err(AccessTokenResponseParseError::Malformed);
                }
                let fragment = fragment.get(1..).unwrap(); // skip "#"
                let mut access_token = None;
                let mut expires_in = None;
                let mut user_id = None;
                for part in fragment.split('&') {
                    match part.find("=") {
                        None => continue,
                        Some(pos) => {
                            let (name, value) = part.split_at(pos);
                            if value.len() < 2 {
                                // "" and "=
                                continue;
                            }
                            let value = value.get(1..).unwrap(); // skip '='
                            match name {
                                "access_token" => access_token = Some(value),
                                "expires_in" => {
                                    if let Ok(num) = value.parse::<u64>() {
                                        expires_in = Some(num);
                                    }
                                }
                                "user_id" => user_id = Some(value),
                                &_ => continue,
                            }
                        }
                    }
                }
                if access_token.is_none() {
                    Err(AccessTokenResponseParseError::NoAccessToken)
                } else if expires_in.is_none() {
                    Err(AccessTokenResponseParseError::NoExpires)
                } else if user_id.is_none() {
                    Err(AccessTokenResponseParseError::NoUserId)
                } else {
                    Ok(AccessTokenResponse {
                        access_token: access_token.unwrap().into(),
                        expires_in: expires_in.unwrap(),
                        user_id: user_id.unwrap().into(),
                    })
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum AccessTokenResponseParseError {
    Malformed,
    NoAccessToken,
    NoExpires,
    NoUserId,
}

impl fmt::Display for AccessTokenResponseParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Malformed => write!(f, "No fragment part (#) in the source URI"),
            Self::NoAccessToken => write!(f, "access_token is not set in URI fragment"),
            Self::NoExpires => write!(f, "expire_in is not set in URI fargment"),
            Self::NoUserId => write!(f, "user_id is not set in URI fragment"),
        }
    }
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

#[test]
fn access_token_response_from_str() {
    // correct
    let atr = "https://oauth.vk.com/blank.html#access_token=7de238693ce753e240abd1e1845480667bed442daf94e86df12d6c376322137fd2649661fbfd1352741fa&expires_in=86400&user_id=184946538"
        .parse::<AccessTokenResponse>();
    assert!(atr.is_ok());
    let atr = atr.unwrap();
    assert_eq!(
        &atr.access_token,
        "7de238693ce753e240abd1e1845480667bed442daf94e86df12d6c376322137fd2649661fbfd1352741fa"
    );
    assert_eq!(atr.expires_in, 86_400);
    assert_eq!(&atr.user_id, "184946538");
    // another correct variant
    let atr2 = "abc#access_token=1234&expires_in=1&user_id=1".parse::<AccessTokenResponse>();
    assert!(atr2.is_ok());
    let atr2 = atr2.unwrap();
    assert_eq!(&atr2.access_token, "1234");
    assert_eq!(atr2.expires_in, 1);
    assert_eq!(&atr2.user_id, "1");

    // incorrect variants
    assert!("abc#access_token=&expires_in=1&user_id=1"
        .parse::<AccessTokenResponse>()
        .is_err());
    assert!("abc#access_token_=1234&expires_in=1&user_id=1"
        .parse::<AccessTokenResponse>()
        .is_err());

    assert!("abc#access_token=1234&expires_in=&user_id=1"
        .parse::<AccessTokenResponse>()
        .is_err());
    assert!("abc#access_token=1234&expires_in_=1&user_id=1"
        .parse::<AccessTokenResponse>()
        .is_err());
    assert!("abc#access_token=1234&expires_in=_1&user_id=1"
        .parse::<AccessTokenResponse>()
        .is_err());

    assert!("abc#access_token=1234&expires_in=1&user_id="
        .parse::<AccessTokenResponse>()
        .is_err());
    assert!("abc#access_token=1234&expires_in=1&user_id_=1"
        .parse::<AccessTokenResponse>()
        .is_err());
}
