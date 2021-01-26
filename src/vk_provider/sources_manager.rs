use super::SourcesUpdate;
use crate::models::NewsSourceModel;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct SourcesManager {
    pub items: Mutex<HashMap<i64, NewsSourceModel>>,
}

impl SourcesManager {
    pub fn new() -> Self {
        SourcesManager {
            items: Mutex::new(HashMap::new()),
        }
    }

    pub fn add_new_sources(&self, update: SourcesUpdate) -> Option<SourcesUpdate> {
        match self.items.lock() {
            Ok(mut items) => {
                let mut new_items = Vec::new();
                for item in update.items {
                    if !items.contains_key(&item.id) {
                        items.insert(item.id, item.clone());
                        new_items.push(item);
                    }
                }
                if !new_items.is_empty() {
                    Some(SourcesUpdate { items: new_items })
                } else {
                    None
                }
            }
            Err(e) => {
                log::error!("failed getting access to items: {}", e);
                None
            }
        }
    }
}
