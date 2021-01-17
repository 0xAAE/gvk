use crate::storage::SharedStorage;
use crate::utils::local_from_timestamp;
use chrono::Utc;
use rvk::{methods::newsfeed, objects::newsfeed::NewsFeed, APIClient, Params};

// a maximal time interval to limit news updates
const MAX_UPDATE_DELTA_SEC: u64 = 3_600; // 60 minutes

/// <https://vk.com/dev/newsfeed.get>
pub struct NewsProvider {
    storage: SharedStorage,
    received_from: u64,
    received_to: u64,
    last_next_from: String,
}

impl NewsProvider {
    pub fn new(storage: SharedStorage) -> Self {
        // on start get news for the last hour:
        let received_from = Utc::now().timestamp() as u64 - MAX_UPDATE_DELTA_SEC;
        let received_to = received_from; // for the last hour
        NewsProvider {
            storage,
            received_from,
            received_to,
            last_next_from: String::new(),
        }
    }

    // returns pack of the news preceeding the current the most old one, i.e. start_sec - MAX_UPDATE_DELTA_SEC
    pub async fn prev_update(&mut self, api: &mut APIClient) -> Option<NewsFeed> {
        let end_time = self.received_from;
        let start_time = end_time - MAX_UPDATE_DELTA_SEC;
        let mut params = Params::new();
        params.insert("start_time".into(), format!("{}", start_time).into());
        params.insert("end_time".into(), format!("{}", end_time).into());
        params.insert("count".into(), "100".into());
        self.received_from = start_time;
        self.do_update(api, params)
            .await
            .map(|upd| {
                self.last_next_from = if let Some(val) = upd.next_from.as_ref() {
                    val.clone()
                } else {
                    String::new()
                };
                upd
            })
            .or_else(|| {
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
    pub async fn next_update(&mut self, api: &mut APIClient) -> Option<NewsFeed> {
        let mut params = Params::new();
        //params.insert("start_from".into(), self.last_next_from.clone());
        params.insert("start_time".into(), format!("{}", self.received_to));
        params.insert("count".into(), "100".into());
        self.received_to = Utc::now().timestamp() as u64;
        self.do_update(api, params).await
    }

    async fn do_update(&self, api: &mut APIClient, params: Params) -> Option<NewsFeed> {
        match newsfeed::get::<NewsFeed>(api, params).await {
            Ok(mut upd) => {
                // replace URLs in photo_50 with downloaded local files or clear them if failed
                // in profiles and groups
                if let Some(profiles) = upd.profiles.as_mut() {
                    for u in profiles {
                        if u.photo_50.is_some() {
                            let res = self
                                .storage
                                .get_file(u.photo_50.as_ref().unwrap().as_str())
                                .await;
                            if let Ok(file) = res {
                                u.photo_50 = Some(file);
                            } else {
                                u.photo_50 = None;
                            }
                        }
                    }
                }
                if let Some(groups) = upd.groups.as_mut() {
                    for g in groups {
                        let res = self.storage.get_file(g.photo_50.as_str()).await;
                        if let Ok(file) = res {
                            g.photo_50 = file;
                        } else {
                            g.photo_50 = String::new();
                        }
                    }
                }
                Some(upd)
            }
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
