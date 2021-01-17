//! Produces news item model objects from rvk::objects::newsfeed::* implementing all required operations and types:
//! * NewsItemModel - the result data, also the source for the NewsItemViewModel
//! * NewsUpdate - is constracted from rvk::objects::newsfeed::NewsFeed and performs any desired transformations on data:
//!       * all markdown control symbols are "preserved", examples: '&' --> &amp; or '>' --> &gt;
//!       * all URLs are surrounded by <a href=> tag
//! NewsUpdate implements trait IntoIterator to allow transparent iterating over NewsItemModel objects
//! * NewsUpdate.into_iter() - iterates over NewsUpdate providing NewsItemModel objects
//! So, having the NewsUpdate::new(&rvk::objects::newsfeed::NewsFeed) one can turn it into iterator byy into_iter() which
//! in its turn produces NewsItemModel objects from underlying collection.
use crate::storage::Storage;
use crate::utils::local_from_timestamp;
use rvk::objects::newsfeed::NewsFeed;
use std::iter::{IntoIterator, Iterator};

pub struct NewsItemModel {
    pub author: String,
    pub avatar: String,
    pub itemtype: String,
    pub datetime: String,
    pub content: String,
}

pub struct NewsUpdate {
    items: Vec<NewsItemModel>,
}

impl NewsUpdate {
    // Perform all desired transformations while constructing NewsItemModel collecton
    pub async fn new_async(newsfeed: &NewsFeed, storage: &Storage) -> Self {
        // prepare users
        let users_stub = Vec::new();
        let users = if let Some(ref src_users) = newsfeed.profiles {
            src_users
        } else {
            &users_stub
        };
        // prepare groups
        let groups_stub = Vec::new();
        let groups = if let Some(ref src_groups) = newsfeed.groups {
            src_groups
        } else {
            &groups_stub
        };
        // construct news items
        let items = if let Some(ref src_items) = newsfeed.items {
            let mut items = Vec::with_capacity(src_items.len());
            for src in src_items {
                // author & avatar
                let mut avatar = String::new(); // empty if failed finding
                let author = if src.source_id > 0 {
                    // author is user
                    if let Some(user) = users.iter().find(|u| u.id == src.source_id) {
                        if let Ok(filename) = storage
                            .get_file(user.photo_50.as_ref().unwrap().as_str())
                            .await
                        {
                            avatar = filename;
                        }
                        user.last_name.clone() + " " + user.last_name.as_str()
                    } else {
                        String::new()
                    }
                } else {
                    // source is group, source_id is *negative* as defined in VK.com API doc
                    // see https://vk.com/dev/newsfeed.get description of source_id in description of items
                    if let Some(grp) = groups.iter().find(|g| g.id == -src.source_id) {
                        if let Ok(filename) = storage.get_file(grp.photo_50.as_str()).await {
                            avatar = filename;
                        }
                        grp.name.clone()
                    } else {
                        String::new()
                    }
                };
                items.push(NewsItemModel {
                    author,
                    avatar,
                    itemtype: src.type_.clone(),
                    datetime: format!(
                        "{}",
                        local_from_timestamp(src.date).format("%d.%m.%Y %H:%M (%a)")
                    ),
                    content: if let Some(text) = src.text.as_ref() {
                        process_text(text)
                    } else {
                        String::new()
                    },
                })
            }
            //
            items
        } else {
            Vec::new()
        };
        NewsUpdate { items }
    }
}

impl IntoIterator for NewsUpdate {
    type Item = NewsItemModel;
    type IntoIter = std::vec::IntoIter<NewsItemModel>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

// perform desired text processing before display it
fn process_text(text: &str) -> String {
    let result = glib::markup_escape_text(text).to_string();
    markup_html_links(result.as_str())
}

fn markup_html_links(text: &str) -> String {
    let mut result = String::with_capacity(text.len()); // at least of equal size
    let mut src = text;
    while let Some(pos) = src.find("http") {
        result.push_str(&src[..pos]);
        result.push_str("<a href=\"");
        let link = src[pos..].split_ascii_whitespace().next();
        match link {
            None => return text.into(),
            Some(l) => {
                let new_pos = pos + l.len();
                result.push_str(l);
                result.push_str("\">");
                result.push_str(l);
                result.push_str("</a>");
                src = &src[new_pos..];
            }
        }
    }
    result.push_str(src);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_markup_escape_text() {
        assert_eq!(
            glib::markup_escape_text("Justerini & Brooks")
                .to_string()
                .as_str(),
            r"Justerini &amp; Brooks"
        );
        assert_eq!(
            glib::markup_escape_text("I <...> you").to_string().as_str(),
            r"I &lt;...&gt; you"
        );
    }

    #[test]
    fn test_markup_html_links() {
        assert_eq!(markup_html_links(""), "");
        assert_eq!(
            markup_html_links("http://www.google.com"),
            r#"<a href="http://www.google.com">http://www.google.com</a>"#
        );
        assert_eq!(
            markup_html_links(
                "Google recommends visiting the site http://www.google.com in the morning"
            ),
            r#"Google recommends visiting the site <a href="http://www.google.com">http://www.google.com</a> in the morning"#
        );
        assert_eq!(
            markup_html_links("There are links: https://www.gvk.com and https://gvk.com"),
            r#"There are links: <a href="https://www.gvk.com">https://www.gvk.com</a> and <a href="https://gvk.com">https://gvk.com</a>"#
        );
        assert_eq!(
            markup_html_links(
                "https://www.gvk.com https://gvk.com http://www.gvk.com http://gvk.com"
            ),
            r#"<a href="https://www.gvk.com">https://www.gvk.com</a> <a href="https://gvk.com">https://gvk.com</a> <a href="http://www.gvk.com">http://www.gvk.com</a> <a href="http://gvk.com">http://gvk.com</a>"#
        );
    }
}
