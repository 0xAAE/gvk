use crate::models::NewsUpdate;
use crate::storage::{SharedStorage, Storage};
use crate::ui::{Message, Request};
use rvk::APIClient;
use std::sync::Arc;
use tokio::runtime::Builder;
use tokio::sync::{
    mpsc::{error::TrySendError, Receiver, Sender},
    oneshot,
};
use tokio::time::sleep;
use tokio::time::{timeout, Duration};

mod access_token_provider;
pub use access_token_provider::AccessTokenProvider;
pub use access_token_provider::AuthResponse;
mod account;
pub use account::{Account, AccountProvider};
mod user;
pub use user::{User, UserViewModel};
mod newsfeed;
pub use newsfeed::NewsProvider;

type MessageSender = Sender<Message>;
type RequestReceiver = Receiver<Request>;
type StopReceiver = oneshot::Receiver<()>;

/// Spawn separate thread to handle communication.
pub fn run_with_own_runtime(
    rx_stop: StopReceiver,
    tx_msg: MessageSender,
    rx_req: RequestReceiver,
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
        // main task, executes until inner error or rx_stop is received
        let worker = async move {
            let storage: SharedStorage = Arc::new(Storage::new());

            // test access to vk.com account
            // test stored auth
            let mut auth: Option<AuthResponse> = None;
            let mut account: Option<Account> = None;
            let access_token_valid = if let Ok(a) = storage.load_auth_async().await {
                //todo: logging
                //test auth
                // create VK client
                let vk_api = APIClient::new(a.get_access_token());
                account = AccountProvider::query_async(&vk_api).await;
                auth = Some(a);
                account.is_some()
            } else {
                false
            };
            if !access_token_valid {
                let (tx_response, rx_response) = oneshot::channel::<AuthResponse>();
                if let Ok(_) = tx_msg.send(Message::Auth(tx_response)).await {
                    if let Ok(a) = rx_response.await {
                        if let Err(e) = storage.save_auth_async(&a).await {
                            log::warn!("failed to store auth data: {}", e);
                        }
                        // create VK client
                        let vk_api = APIClient::new(a.get_access_token());
                        account = AccountProvider::query_async(&vk_api).await;
                        auth = Some(a);
                    }
                }
            }
            if auth.is_none() {
                log::error!("authentication is not available");
                return;
            }
            if account.is_none() {
                log::error!("authentication succeded but account is unreachable");
                return;
            }
            let auth = auth.unwrap();
            let account = account.unwrap();
            log::debug!("authentication: {}", auth);
            log::info!("account: {}", account);
            // create VK client
            let mut vk_api = APIClient::new(auth.get_access_token());
            // request own user info
            let user = User::query_async(&vk_api, auth.get_user_id()).await;
            if user.is_none() {
                log::error!("failed to get user info");
                return;
            }
            let user = user.unwrap();
            log::debug!("user is {}", user);
            let view_model = user.get_view_model(&storage).await;
            log::debug!("user view is {}", &view_model);
            if let Err(e) = tx_msg.send(Message::OwnInfo(view_model)).await {
                log::error!("failed updating user info, {}", e);
            }

            let mut news = NewsProvider::new();
            let mut rx_req = rx_req;
            loop {
                // periodically query news
                if let Some(news_feed) = news.next_update(&mut vk_api).await {
                    if let Some(items) = &news_feed.items {
                        log::debug!("got {} news items", items.len());
                    }
                    let update = NewsUpdate::new_async(&news_feed, &storage).await;
                    match tx_msg.try_send(Message::News(update)) {
                        Ok(_) => {}
                        Err(TrySendError::Full(_)) => {
                            log::warn!("data is being produced too fast for GUI");
                        }
                        Err(TrySendError::Closed(_)) => {
                            log::info!("GUI has stopped, stopping also");
                            break;
                        }
                    }
                }
                if let Err(e) = storage.save_state_async().await {
                    log::warn!("saving storage state failed: {}", e);
                }
                // todo: select rx_req + timeout:
                if let Ok(res) =
                    tokio::time::timeout(tokio::time::Duration::from_secs(60), rx_req.recv()).await
                {
                    match res {
                        Some(req) => match req {
                            // more news requested vy UI
                            Request::NewsNext => log::info!("UI requested more news, please wait"),
                            // older news requested by UI
                            Request::NewsOlder => {
                                if let Some(news_feed) = news.prev_update(&mut vk_api).await {
                                    if let Some(items) = &news_feed.items {
                                        log::debug!("got {} older news items", items.len());
                                    }
                                    let update = NewsUpdate::new_async(&news_feed, &storage).await;
                                    match tx_msg.try_send(Message::OlderNews(update)) {
                                        Ok(_) => {}
                                        Err(TrySendError::Full(_)) => {
                                            log::warn!("data is being produced too fast for GUI");
                                        }
                                        Err(TrySendError::Closed(_)) => {
                                            log::info!("GUI has stopped, stopping also");
                                            break;
                                        }
                                    }
                                }
                            }
                        },
                        None => log::warn!(
                            "request channel has closed bysender(s), there are no more requests"
                        ),
                    }
                }
                sleep(Duration::from_millis(60_000)).await;
            }
        };

        // wait on two futures: the stop signal and the main task (worker)
        tokio::select! {
            _ = rx_stop => log::debug!("get command stop, exitting"),
            _ = worker => log::debug!("has stopped itself"),
        }
    });
}
