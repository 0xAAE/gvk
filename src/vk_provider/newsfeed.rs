use crate::utils::local_from_timestamp;
use chrono::Utc;
use rvk::{methods::newsfeed, objects::newsfeed::NewsFeed, APIClient, Params};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Mutex,
};

// a maximal time interval to limit news updates
const MAX_UPDATE_DELTA_SEC: u64 = 3_600; // 60 minutes

/// <https://vk.com/dev/newsfeed.get>
/// Multi-threaded, callef from a couple of tasks
pub struct NewsProvider {
    received_from: AtomicU64,
    received_to: AtomicU64,
    last_next_from: Mutex<String>,
}

impl NewsProvider {
    pub fn new() -> Self {
        // on start get news for the last hour:
        let received_from = Utc::now().timestamp() as u64 - MAX_UPDATE_DELTA_SEC;
        let received_to = received_from; // for the last hour
        NewsProvider {
            received_from: AtomicU64::new(received_from),
            received_to: AtomicU64::new(received_to),
            last_next_from: Mutex::new(String::new()),
        }
    }

    // returns pack of the news preceeding the current the most old one, i.e. start_sec - MAX_UPDATE_DELTA_SEC
    pub async fn prev_update(&self, api: &APIClient) -> Option<NewsFeed> {
        let end_time = self.received_from.load(Ordering::SeqCst);
        let start_time = end_time - MAX_UPDATE_DELTA_SEC;
        let mut params = Params::new();
        params.insert("start_time".into(), format!("{}", start_time).into());
        params.insert("end_time".into(), format!("{}", end_time).into());
        params.insert("count".into(), "100".into());
        self.received_from.store(start_time, Ordering::SeqCst);
        self.do_update(api, params)
            .await
            .map(|upd| {
                // if success update next_from
                let next_from = if let Some(val) = upd.next_from.as_ref() {
                    val.clone()
                } else {
                    String::new()
                };
                if let Ok(mut s) = self.last_next_from.lock() {
                    *s = next_from;
                }
                upd
            })
            .or_else(|| {
                // if failed log warning
                log::warn!(
                    "drop news from {} to {} due to error",
                    format!(
                        "{}",
                        local_from_timestamp(start_time as i64).format("%d.%m.%Y %H:%M")
                    )
                    .as_str(),
                    format!(
                        "{}",
                        local_from_timestamp(end_time as i64).format("%d.%m.%Y %H:%M")
                    )
                    .as_str()
                );
                None
            })
    }

    // returrns next protion of the news, i.e. subsequent to the most recent ones
    pub async fn next_update(&self, api: &APIClient) -> Option<NewsFeed> {
        let mut params = Params::new();
        let start_time = self.received_to.load(Ordering::SeqCst);
        params.insert("start_time".into(), format!("{}", start_time));
        params.insert("count".into(), "100".into());
        self.received_to
            .store(Utc::now().timestamp() as u64, Ordering::SeqCst);
        self.do_update(api, params).await
    }

    async fn do_update(&self, api: &APIClient, params: Params) -> Option<NewsFeed> {
        match newsfeed::get::<NewsFeed>(api, params).await {
            Ok(upd) => Some(upd),
            Err(e) => {
                match e {
                    rvk::error::Error::API(e) => {
                        log::error!(
                            "failed requesting news update: {}, extra {:?}",
                            e.msg(),
                            e.extra()
                        );
                    }
                    _ => log::error!("failed requesting news update: {}", e),
                }
                None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::{read_dir, read_to_string};
    use std::path::Path;

    #[test]
    fn deserialize_news_update() {
        if let Ok(mut file_list) = read_dir("resources/tests/newsfeed") {
            while let Some(file) = file_list.next() {
                assert!(file.is_ok());
                let file_name = file.unwrap().path().to_string_lossy().to_string();
                let path = Path::new(file_name.as_str());
                let json = read_to_string(&path).unwrap();
                let result = serde_json::from_str::<NewsFeed>(&json);
                match result {
                    Ok(upd) => {
                        assert!(
                            upd.items.is_some() || upd.profiles.is_some() || upd.groups.is_some()
                        );
                    }
                    Err(e) => {
                        println!("test failed for {}", file_name);
                        let msg = format!("{}", e);
                        println!("{}", msg);
                        assert!(false);
                    }
                }
            }
        }
    }
}
