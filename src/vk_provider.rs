use crate::storage::{SharedStorage, Storage};
use crate::ui::{Message, Request};
use rvk::APIClient;
use std::sync::Arc;
use tokio::runtime::Builder;
use tokio::sync::{
    mpsc::{error::TrySendError, Receiver, Sender},
    oneshot,
};
use tokio::time::{sleep, Duration};

pub const URI_BASE: &str = "https://vk.com/";
pub const CLEAR_TEMP_ON_START: bool = false;
pub const CLEAR_TEMP_ON_EXIT: bool = false;

mod access_token_provider;
pub use access_token_provider::AccessTokenProvider;
pub use access_token_provider::AuthResponse;
mod account;
pub use account::{Account, AccountProvider};
pub mod constants;
mod user;
pub use user::User;
mod news_provider;
pub use news_provider::NewsProvider;
mod news_update;
pub use news_update::NewsUpdate;
mod sources_update;
pub use sources_update::SourcesUpdate;
mod sources_manager;
use sources_manager::SourcesManager;

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
        log::info!("starting main worker");
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
            let vk_api = Arc::new(APIClient::new(auth.get_access_token()));
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
            let news = Arc::new(NewsProvider::new());
            let news_sources = Arc::new(SourcesManager::new());

            // start task handling rx_req
            let vk_api_copy = vk_api.clone();
            let news_copy = news.clone();
            let news_sources_copy = news_sources.clone();
            let storage_copy = storage.clone();
            let tx_msg_copy = tx_msg.clone();
            tokio::spawn(async move {
                log::info!("starting UI requests handler");
                let mut rx_req = rx_req;
                loop {
                    if let Some(req) = rx_req.recv().await {
                        match req {
                            // more news requested vy UI
                            Request::NewsNext => {
                                log::debug!("UI requested more news, please wait")
                            }
                            // older news requested by UI
                            Request::NewsOlder => {
                                if let Some(news_feed) = news_copy.prev_update(&vk_api_copy).await {
                                    if let Some(items) = &news_feed.items {
                                        log::debug!("got {} older news items", items.len());
                                    }
                                    let update =
                                        NewsUpdate::new_async(&news_feed, &storage_copy).await;
                                    if !do_send(&tx_msg_copy, Message::OlderNews(update)) {
                                        break;
                                    }
                                    // prepare sources update
                                    if let Some(new_items) = news_sources_copy.add_new_sources(
                                        SourcesUpdate::new_async(&news_feed, &storage_copy).await,
                                    ) {
                                        // send sources update
                                        if !do_send(&tx_msg_copy, Message::NewsSources(new_items)) {
                                            break;
                                        }
                                    }
                                }
                            }
                            Request::Stop => {
                                storage_copy.prepare_to_stop();
                                break;
                            }
                        }
                    } else {
                        log::warn!(
                            "request channel has closed by sender(s), there are no more requests"
                        );
                        break;
                    }
                }
                log::info!("UI requests handler has stopped");
            });

            loop {
                // periodically query news
                if let Some(news_feed) = news.next_update(&vk_api).await {
                    if let Some(items) = &news_feed.items {
                        log::debug!("got {} news items", items.len());
                    }
                    // prepare news update
                    let update = NewsUpdate::new_async(&news_feed, &storage).await;
                    // send news update
                    if !do_send(&tx_msg, Message::News(update)) {
                        break;
                    }
                    // prepare sources update
                    if let Some(new_items) = news_sources
                        .add_new_sources(SourcesUpdate::new_async(&news_feed, &storage).await)
                    {
                        // send sources update
                        if !do_send(&tx_msg, Message::NewsSources(new_items)) {
                            break;
                        }
                    }
                }
                if let Err(e) = storage.save_state_async().await {
                    log::warn!("saving storage state failed: {}", e);
                }

                // pause main provider task until time to get next update from vk.com
                sleep(Duration::from_millis(60_000)).await;
            }
        };

        // wait on two futures: the stop signal and the main task (worker)
        tokio::select! {
            _ = rx_stop => log::debug!("get command stop, exitting"),
            _ = worker => log::debug!("has stopped itself"),
        }
        log::info!("main worker has stopped");
    });
}

fn do_send(tx: &MessageSender, msg: Message) -> bool {
    // send sources update
    match tx.try_send(msg) {
        Ok(_) => true,
        Err(TrySendError::Full(_)) => {
            log::warn!("data is being produced too fast for UI");
            true
        }
        Err(TrySendError::Closed(_)) => {
            log::info!("UI has stopped, also stopping");
            false
        }
    }
}
