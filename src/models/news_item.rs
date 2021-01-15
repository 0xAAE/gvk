use crate::vk_provider::{NewsItem, NewsUpdate};
use chrono::prelude::*;
use rvk::objects::{group, user};
use std::iter::{IntoIterator, Iterator};

pub struct NewsItemModel {
    pub author: String,
    pub title: String,
    pub datetime: String,
    pub content: String,
}

impl IntoIterator for NewsUpdate {
    type Item = NewsItemModel;
    type IntoIter = NewsUpdateIterator;

    fn into_iter(self) -> Self::IntoIter {
        NewsUpdateIterator::new(self)
    }
}

pub struct NewsUpdateIterator {
    current: usize,
    items: Vec<NewsItem>,
    users: Vec<user::User>,
    groups: Vec<group::Group>,
}

impl NewsUpdateIterator {
    fn new(src: NewsUpdate) -> Self {
        NewsUpdateIterator {
            current: 0,
            items: src.items.unwrap_or(Vec::new()),
            users: src.profiles.unwrap_or(Vec::new()),
            groups: src.groups.unwrap_or(Vec::new()),
        }
    }
}

impl Iterator for NewsUpdateIterator {
    type Item = NewsItemModel;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.items.len() {
            None
        } else {
            let item = self.items.get(self.current).unwrap();
            self.current += 1;
            let source = if item.source_id > 0 {
                // source is user
                if let Some(user) = self.users.iter().find(|u| u.id == item.source_id) {
                    Some(user.last_name.clone() + " " + user.last_name.as_str())
                } else {
                    None
                }
            } else {
                // source is group
                if let Some(grp) = self.groups.iter().find(|g| g.id == -item.source_id) {
                    Some(grp.screen_name.clone())
                } else {
                    None
                }
            };
            let naive = NaiveDateTime::from_timestamp(item.date as i64, 0);
            let datetime: DateTime<Local> =
                DateTime::<Utc>::from_utc(naive, Utc).with_timezone(&Local);
            Some(NewsItemModel {
                author: source.unwrap_or_default(),
                title: item.type_.clone(),
                datetime: format!("{}", datetime.format("%d.%m.%Y %H:%M (%a)")),
                content: item.text.as_ref().unwrap_or(&String::new()).clone(),
            })
        }
    }
}
