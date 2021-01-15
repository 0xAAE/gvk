use chrono::prelude::*;
use rvk::objects::newsfeed::{Item as NewsItem, NewsFeed};
use rvk::objects::{group, user};
use std::iter::{IntoIterator, Iterator};

pub struct NewsItemModel {
    pub author: String,
    pub itemtype: String,
    pub datetime: String,
    pub content: String,
}

pub struct NewsUpdate(pub NewsFeed);

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
            items: src.0.items.unwrap_or(Vec::new()),
            users: src.0.profiles.unwrap_or(Vec::new()),
            groups: src.0.groups.unwrap_or(Vec::new()),
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
                    Some(grp.name.clone())
                } else {
                    None
                }
            };
            let naive = NaiveDateTime::from_timestamp(item.date as i64, 0);
            let datetime: DateTime<Local> =
                DateTime::<Utc>::from_utc(naive, Utc).with_timezone(&Local);
            Some(NewsItemModel {
                author: source.unwrap_or_default(),
                itemtype: item.type_.clone(),
                datetime: format!("{}", datetime.format("%d.%m.%Y %H:%M (%a)")),
                content: if let Some(text) = item.text.as_ref() {
                    process_text(text)
                } else {
                    String::new()
                },
            })
        }
    }
}

fn process_text(text: &String) -> String {
    // replace & symbol
    //text.replace('&', "&amp;")

    let mut result = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => result.push_str("&amp;"),
            //'A'..='Z' => 'X',
            '<' => result.push_str("&lt;"),
            '>' => result.push_str("&gt;"),
            _ => result.push(c),
        }
    }
    result
}
