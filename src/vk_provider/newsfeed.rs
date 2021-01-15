use chrono::Utc;
use rvk::{methods::newsfeed, objects::newsfeed::NewsFeed, APIClient, Params};
use std::env;

// a maximal time interval to limit news updates
const MAX_UPDATE_DELTA_SEC: u64 = 1_200; // 20 minutes

/// <https://vk.com/dev/newsfeed.get>
pub struct NewsProvider {
    start_sec: u64,
    // end_sec: u64,
    start_from: String,
}

impl NewsProvider {
    pub fn new() -> Self {
        let end_sec = Utc::now().timestamp() as u64;
        let start_sec = end_sec - MAX_UPDATE_DELTA_SEC; // for the last hour
        NewsProvider {
            start_sec,
            // end_sec,
            start_from: String::new(),
        }
    }

    // returns pack of the news preceeding the current the most old one, i.e. start_sec - MAX_UPDATE_DELTA_SEC
    pub async fn prev_update(&mut self, api: &mut APIClient) -> Option<NewsFeed> {
        let upd_end_time = self.start_sec;
        let upd_start_time = upd_end_time - MAX_UPDATE_DELTA_SEC;
        let mut params = Params::new();
        params.insert("start_time".into(), format!("{}", upd_start_time).into());
        params.insert("end_time".into(), format!("{}", upd_end_time).into());
        self.do_update(api, params).await
    }

    // returrns next protion of the news, i.e. subsequent to the most recent ones
    pub async fn next_update(&mut self, api: &mut APIClient) -> Option<NewsFeed> {
        let mut params = Params::new();
        params.insert("start_from".into(), self.start_from.clone());
        self.do_update(api, params).await
    }

    async fn do_update(&mut self, api: &mut APIClient, params: Params) -> Option<NewsFeed> {
        let trace: u32 = if let Ok(s) = env::var("TRACE_NEWS") {
            s.as_str().parse::<u32>().unwrap_or(0)
        } else {
            0
        };
        api.trace_response(trace != 0);
        match newsfeed::get::<NewsFeed>(api, params).await {
            Ok(upd) => {
                env::set_var("TRACE_NEWS", "0");
                api.trace_response(false);
                self.start_from = if let Some(val) = upd.next_from.as_ref() {
                    val.clone()
                } else {
                    String::new()
                };
                Some(upd)
            }
            Err(e) => {
                env::set_var("TRACE_NEWS", "1");
                match e {
                    rvk::error::Error::API(e) => {
                        println!(
                            "Failed requesting news update: {}, extra {:?}",
                            e.msg(),
                            e.extra()
                        );
                    }
                    _ => println!("Failed requesting news update: {}", e),
                }
                None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::storage::Storage;
    use std::fs::read_to_string;
    use std::path::Path;

    #[test]
    fn deserialize_news_update() {
        let storage = Storage::new();
        let json_file = storage.get_cache_dir().to_string() + "/news.json";
        let path = Path::new(&json_file);
        let s = read_to_string(&path).unwrap();
        let result = serde_json::from_str::<NewsFeed>(&s);
        match result {
            Ok(_upd) => assert!(true),
            Err(e) => {
                let msg = format!("{}", e);
                println!("{}", msg);
                assert!(false);
            }
        }
    }
}
