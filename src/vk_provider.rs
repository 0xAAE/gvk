use crate::storage::Storage;
use crate::ui::{Message, NewsItem};
use chrono::prelude::*;
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

type MessageSender = Sender<Message>;
type StopReceiver = oneshot::Receiver<()>;

/// Spawn separate thread to handle communication.
pub fn launch_vk_provider(
    rx_stop: StopReceiver,
    storage: Arc<Storage>,
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
