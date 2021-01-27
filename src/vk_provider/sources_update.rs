use crate::models::ActorModel;
use crate::storage::Storage;
use crate::utils::local_from_timestamp;
use crate::vk_provider;
use rvk::objects::newsfeed::NewsFeed;
use std::iter::IntoIterator;

pub struct SourcesUpdate {
    pub items: Vec<ActorModel>,
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
                let rel_uri = if let Some(screen_name) = &user.screen_name {
                    screen_name.clone()
                } else {
                    String::new()
                };
                items.push(ActorModel {
                    id: user.id,
                    name,
                    avatar,
                    desc: "profile".to_string(),
                    comment,
                    rel_uri,
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
                let desc = if let Some(is_member) = &group.is_member {
                    match is_member {
                        1 => "subscription".to_string(),
                        &_ => group.type_.clone(),
                    }
                } else {
                    if let Some(is_advertiser) = &group.is_advertiser {
                        match is_advertiser {
                            1 => "advertiser".to_string(),
                            &_ => group.type_.clone(),
                        }
                    } else {
                        group.type_.clone()
                    }
                };
                let comment = if let Some(description) = &group.description {
                    description.clone()
                } else {
                    String::new()
                };
                items.push(ActorModel {
                    id: group.id,
                    name,
                    avatar,
                    desc,
                    comment,
                    rel_uri: group.screen_name.clone(),
                });
            }
        }
        // construct sources items
        SourcesUpdate { items }
    }
}

impl IntoIterator for SourcesUpdate {
    type Item = ActorModel;
    type IntoIter = std::vec::IntoIter<ActorModel>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}
