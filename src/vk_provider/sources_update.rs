use crate::models::NewsSourceModel;
use crate::storage::Storage;
use crate::utils::local_from_timestamp;
use crate::vk_provider;
use rvk::objects::newsfeed::NewsFeed;

pub struct SourcesUpdate {
    pub items: Vec<NewsSourceModel>,
}

impl SourcesUpdate {
    pub async fn new_async(newsfeed: &NewsFeed, storage: &Storage) -> Self {
        // prepare users
        let mut items = Vec::new();
        if let Some(src_users) = &newsfeed.profiles {
            for user in src_users {
                let name = vk_provider::User::get_full_name(user);
                let avatar = if let Ok(filename) = storage
                    .get_file(vk_provider::User::get_small_photo(user).as_str(), "")
                    .await
                {
                    filename
                } else {
                    String::new()
                };
                let comment = if let Some(last_seen) = &user.last_seen {
                    if let Some(ts) = last_seen.time {
                        local_from_timestamp(ts)
                            .format("%d.%m.%Y %H:%M")
                            .to_string()
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                };
                items.push(NewsSourceModel {
                    name,
                    avatar,
                    desc: "friend".to_string(),
                    comment,
                    uri: String::new(),
                });
            }
        }
        // prepare groups
        if let Some(src_groups) = &newsfeed.groups {
            for group in src_groups {
                let name = group.name.clone();
                let avatar =
                    if let Ok(filename) = storage.get_file(group.photo_50.as_str(), "").await {
                        filename
                    } else {
                        String::new()
                    };
                let desc = group.type_.clone();
                let comment = if let Some(description) = &group.description {
                    description.clone()
                } else {
                    String::new()
                };
                let uri = if let Some(links) = &group.links {
                    if !links.is_empty() {
                        links[0].url.clone()
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                };
                items.push(NewsSourceModel {
                    name,
                    avatar,
                    desc,
                    comment,
                    uri,
                });
            }
        }
        // construct sources items
        SourcesUpdate { items }
    }
}
