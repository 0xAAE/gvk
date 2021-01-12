use crate::storage::Storage;
use crate::ui::{Message, NewsItem};
use chrono::prelude::*;
use rvk::APIClient;
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Builder;
use tokio::sync::{
    mpsc::{error::TrySendError, Sender},
    oneshot,
    oneshot::error::TryRecvError,
};
use tokio::time::sleep;

mod access_token_provider;
pub use access_token_provider::AccessTokenProvider;
pub use access_token_provider::AuthResponse;
mod account;
pub use account::Account;
mod user;
pub use user::{User, UserViewModel};

type MessageSender = Sender<Message>;
type StopReceiver = oneshot::Receiver<()>;

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
        let storage = Arc::new(Storage::new());
        let mut rx_stop = rx_stop;

        // test access to vk.com account
        // test stored auth
        let mut auth: Option<AuthResponse> = None;
        let mut account: Option<Account> = None;
        let access_token_valid = if let Ok(a) = storage.load_auth_async().await {
            //todo: logging
            //test auth
            // create VK client
            let vk_api = APIClient::new(a.get_access_token());
            account = Account::query_async(&vk_api).await;
            auth = Some(a);
            account.is_some()
        } else {
            false
        };
        if !access_token_valid {
            let (tx_response, rx_response) = oneshot::channel::<AuthResponse>();
            if let Ok(_) = tx.send(Message::Auth(tx_response)).await {
                if let Ok(a) = rx_response.await {
                    if let Err(e) = storage.save_auth_async(&a).await {
                        println!("Failed to store auth data: {}", e);
                    }
                    // create VK client
                    let vk_api = APIClient::new(a.get_access_token());
                    account = Account::query_async(&vk_api).await;
                    auth = Some(a);
                }
            }
        }
        if auth.is_none() {
            println!("Authentication is not available, cannot continue");
            return;
        }
        if account.is_none() {
            println!("Authentication succeded but account is unreachable, cannot continue");
            return;
        }
        let auth = auth.unwrap();
        let account = account.unwrap();
        println!("Authentication: {}", auth);
        println!("Account: {}", account);
        // create VK client
        let vk_api = APIClient::new(auth.get_access_token());
        // request own user info
        let user = User::query_async(&vk_api, auth.get_user_id()).await;
        if user.is_none() {
            println!("Failed to get user infoe, cannot continue");
            return;
        }
        let user = user.unwrap();
        println!("User: {}", user);
        let _ = tx.send(Message::OwnInfo(user.get_view_model())).await;

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
                    //todo: logging
                    println!("Data is produced too fast for GUI");
                }
                Err(TrySendError::Closed(_)) => {
                    //todo: logging
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
