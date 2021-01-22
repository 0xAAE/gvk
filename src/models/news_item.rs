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
use rvk::objects::{
    attachment::PostedPhoto,
    link::Link as NewsLink,
    newsfeed::{Item as NewsItem, NewsFeed},
    photo::Photo as NewsPhoto,
    video::Video,
};
use std::fmt;
use std::iter::{IntoIterator, Iterator};

pub struct Photo {
    pub uri: String,
    pub text: String,
}

impl fmt::Display for Photo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "photo {}", self.uri)
    }
}

pub struct Link {
    pub uri: String,
    pub text: String,
}

pub struct NewsItemModel {
    pub author: String,
    pub avatar: String,
    pub itemtype: String,
    pub datetime: String,
    pub content: String,
    pub photos: Option<Vec<Photo>>,
    pub links: Option<Vec<Link>>,
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
                // some items to ignore
                match src.type_.as_str() {
                    NEWS_TYPE_WALL_PHOTO => continue,
                    &_ => {}
                }
                // author & avatar
                let mut avatar = String::new(); // empty if failed finding
                let author = if src.source_id > 0 {
                    // author is user
                    if let Some(user) = users.iter().find(|u| u.id == src.source_id) {
                        if let Ok(filename) = storage
                            .get_file(user.photo_50.as_ref().unwrap().as_str(), "")
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
                        if let Ok(filename) = storage.get_file(grp.photo_50.as_str(), "").await {
                            avatar = filename;
                        }
                        grp.name.clone()
                    } else {
                        String::new()
                    }
                };
                // photos
                let photos = extract_photos(&src, storage).await;
                // links
                let links = extract_links(&src).await;
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
                    links,
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
            if let Some(photoset) = &item.photos {
                if let Some(photos) = &photoset.items {
                    for src_photo in photos {
                        if let Some(res_photo) =
                            select_photo(&src_photo, result.len(), storage).await
                        {
                            result.push(res_photo);
                        }
                    }
                }
            }
        }
        NEWS_TYPE_POST => {
            if let Some(copy_history) = &item.copy_history {
                for history_item in copy_history {
                    // for any type continue searching in attachments (WallAttachment)
                    if let Some(attachments) = &history_item.attachments {
                        // different attachment types might contain photos
                        for attachment in attachments {
                            // photo itself
                            if let Some(src_photo) = &attachment.photo {
                                if let Some(res_photo) =
                                    select_photo(src_photo, result.len(), storage).await
                                {
                                    result.push(res_photo);
                                }
                            }
                            // also, link might hold a photo
                            if let Some(link) = &attachment.link {
                                append_from_link(&mut result, link, storage).await;
                            }
                            // video
                            if let Some(video) = &attachment.video {
                                append_from_video(&mut result, video, storage).await;
                            }
                            // posted photo
                            if let Some(posted_photo) = &attachment.posted_photo {
                                append_from_posted_photo(&mut result, posted_photo, storage).await;
                            }
                        }
                    }
                }
            }
        }
        &_ => {}
    }
    // for any type continue searching in attachments (NewsAttachments)
    if let Some(attachments) = &item.attachments {
        // different attachment types might contain photos
        for attachment in attachments {
            // photo itself
            if let Some(src_photo) = &attachment.photo {
                if let Some(res_photo) = select_photo(src_photo, result.len(), storage).await {
                    result.push(res_photo);
                }
            }
            // also, link might hold a photo
            if let Some(link) = &attachment.link {
                append_from_link(&mut result, link, storage).await;
            }
            // video
            if let Some(video) = &attachment.video {
                append_from_video(&mut result, video, storage).await;
            }
            // posted photo
            if let Some(posted_photo) = &attachment.posted_photo {
                append_from_posted_photo(&mut result, posted_photo, storage).await;
            }
        }
    }

    if !result.is_empty() {
        Some(result)
    } else {
        None
    }
}

async fn extract_links(item: &NewsItem) -> Option<Vec<Link>> {
    let mut result = Vec::new();
    match item.type_.as_str() {
        NEWS_TYPE_POST => {
            if let Some(copy_history) = &item.copy_history {
                for history_item in copy_history {
                    // for any type continue searching in attachments (WallAttachment)
                    if let Some(attachments) = &history_item.attachments {
                        // history attachment might contain link
                        for attachment in attachments {
                            if let Some(src_link) = &attachment.link {
                                append_link_model(&mut result, src_link);
                            }
                        }
                    }
                }
            }
        }
        &_ => {}
    }
    // NewsAttachments might contain link
    if let Some(attachments) = &item.attachments {
        for attachment in attachments {
            if let Some(link) = &attachment.link {
                append_link_model(&mut result, link);
            }
        }
    }

    if !result.is_empty() {
        Some(result)
    } else {
        None
    }
}

fn append_link_model(cont: &mut Vec<Link>, link: &NewsLink) {
    if link.url.is_empty() {
        return;
    }
    let uri = format!(r#"<a href="{}">{}</a>"#, &link.url, &link.url);
    let text = if let Some(desc) = &link.description {
        desc.clone()
    } else if !link.title.is_empty() {
        link.title.clone()
    } else if let Some(cap) = &link.caption {
        cap.clone()
    } else {
        String::new()
    };
    cont.push(Link { uri, text });
}

async fn append_from_link(cont: &mut Vec<Photo>, link: &NewsLink, storage: &Storage) {
    if let Some(src_photo) = &link.photo {
        if let Some(mut res_photo) = select_photo(src_photo, cont.len(), storage).await {
            if res_photo.text.is_empty() {
                res_photo.text = if let Some(text) = &link.description {
                    if !text.is_empty() {
                        text.clone()
                    } else {
                        link.title.clone()
                    }
                } else {
                    link.title.clone()
                }
            };
            cont.push(res_photo);
        }
    }
}

async fn append_from_video(cont: &mut Vec<Photo>, video: &Video, storage: &Storage) {
    let mut found = false;
    if cont.len() < 3 {
        if let Some(src_uri) = &video.photo_640 {
            if let Ok(uri) = storage.get_temp_file(src_uri, "vx").await {
                found = true;
                cont.push(Photo {
                    text: String::new(),
                    uri,
                });
            }
        }
    } else {
        if let Some(src_uri) = &video.photo_130 {
            if let Ok(uri) = storage.get_temp_file(src_uri, "vp").await {
                found = true;
                cont.push(Photo {
                    text: String::new(),
                    uri,
                });
            }
        }
    }
    if !found {
        if let Some(images) = &video.image {
            // find thru unsorted image collection
            if !images.is_empty() {
                let desired = 832;
                let mut idx_best = 0;
                let mut wid_best = 0;
                for (i, img) in images.iter().enumerate() {
                    if img.width < desired && img.width > wid_best {
                        idx_best = i;
                        wid_best = img.width;
                    }
                }
                if let Ok(uri) = storage.get_temp_file(&images[idx_best].url, "v").await {
                    cont.push(Photo {
                        text: String::new(),
                        uri,
                    });
                }
            }
        }
    }
}

async fn append_from_posted_photo(
    cont: &mut Vec<Photo>,
    posted_photo: &PostedPhoto,
    storage: &Storage,
) {
    if cont.len() < 3 {
        if let Ok(uri) = storage
            .get_temp_file(posted_photo.photo_604.as_str(), "ppx")
            .await
        {
            cont.push(Photo {
                text: String::new(),
                uri,
            });
        }
    } else {
        if let Ok(uri) = storage
            .get_temp_file(posted_photo.photo_130.as_str(), "ppp")
            .await
        {
            cont.push(Photo {
                text: String::new(),
                uri,
            });
        }
    }
}

static PRIO_0: [&str; 8] = ["y", "x", "r", "q", "p", "o", "m", "s"];
//static PRIO_N: [&str; 2] = ["o", "m"];

async fn select_photo(src_photo: &NewsPhoto, idx: usize, storage: &Storage) -> Option<Photo> {
    let prio = match idx {
        //0 | 1 => &PRIO_0[..],
        _ => &PRIO_0[..],
    };
    if let Some(sizes) = &src_photo.sizes {
        for p in prio {
            if let Some(size) = sizes.iter().find(|s| s.type_.as_str() == *p) {
                if let Some(url) = &size.url {
                    if let Ok(uri) = storage.get_temp_file(url, *p).await {
                        let text = if let Some(val) = &src_photo.text {
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
