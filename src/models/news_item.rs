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
use crate::vk_provider::constants::*;
use rvk::objects::newsfeed::{Item as NewsItem, NewsFeed};
use rvk::objects::photo::Photo as NewsPhoto;
use std::iter::{IntoIterator, Iterator};

pub struct Photo {
    pub uri: String,
    pub text: String,
}

pub struct NewsItemModel {
    pub author: String,
    pub avatar: String,
    pub itemtype: String,
    pub datetime: String,
    pub content: String,
    // 0 - primary photo, 1-2 second row photos, 3.. - other photos
    pub photos: Option<Vec<Photo>>,
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
                // photos
                let photos = extract_photos(&src, storage).await;
                // compose and return model
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
                    photos,
                })
            }
            //
            items
        } else {
            Vec::new()
        };
        NewsUpdate { items }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

async fn extract_photos(item: &NewsItem, storage: &Storage) -> Option<Vec<Photo>> {
    let mut result = Vec::new();
    // for photo types search in photos
    match item.type_.as_str() {
        NEWS_TYPE_PHOTO | NEWS_TYPE_PHOTO_TAG | NEWS_TYPE_WALL_PHOTO => {
            if let Some(ref photoset) = item.photos {
                if let Some(ref photos) = photoset.items {
                    for src_photo in photos {
                        if let Some(ref sizes) = src_photo.sizes {
                            if !sizes.is_empty() {
                                if let Some(res_photo) =
                                    select_uri(&src_photo, result.len(), storage).await
                                {
                                    result.push(res_photo);
                                }
                            }
                        }
                    }
                }
            }
        }
        &_ => {}
    }
    // for any type continue searching in attachments
    if let Some(ref attachments) = item.attachments {
        for attachemnt in attachments {
            if let Some(ref posted_photo) = attachemnt.posted_photo {
                if let Ok(uri) = storage.get_temp_file(posted_photo.photo_130.as_str()).await {
                    result.push(Photo {
                        text: String::new(),
                        uri,
                    })
                }
            }
            if let Some(ref src_photo) = attachemnt.photo {
                if let Some(ref sizes) = src_photo.sizes {
                    if !sizes.is_empty() {
                        if let Some(res_photo) = select_uri(&src_photo, result.len(), storage).await
                        {
                            result.push(res_photo);
                        }
                    }
                }
            }
        }
    }

    if !result.is_empty() {
        Some(result)
    } else {
        None
    }
}

static PRIO_0: [&str; 6] = ["r", "q", "x", "p", "o", "m"];
static PRIO_1: [&str; 4] = ["q", "p", "o", "m"];
static PRIO_2: [&str; 4] = ["q", "p", "o", "m"];
static PRIO_N: [&str; 2] = ["o", "m"];

async fn select_uri(src_photo: &NewsPhoto, idx: usize, storage: &Storage) -> Option<Photo> {
    let prio = match idx {
        0 => &PRIO_0[..],
        1 => &PRIO_1[..],
        2 => &PRIO_2[..],
        _ => &PRIO_N[..],
    };
    if let Some(ref sizes) = src_photo.sizes {
        for p in prio {
            if let Some(size) = sizes.iter().find(|s| s.type_.as_str() == *p) {
                if let Some(ref url) = size.url {
                    if let Ok(uri) = storage.get_temp_file(url).await {
                        let text = if let Some(ref val) = src_photo.text {
                            val.clone()
                        } else {
                            String::new()
                        };
                        return Some(Photo { uri, text });
                    }
                }
            }
        }
    }
    None
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
