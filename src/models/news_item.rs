//! Produces news item model objects from rvk::objects::newsfeed::* implementing all required operations and types:
//! * NewsItemModel - the result data, also the source for the NewsItemViewModel
//! * NewsUpdateIterator - iterates over NewsUpdate providing NewsItemModel objects
//! * NewsUpdate - the wrapper over rvk::objects::newsfeed::NewsFeed, implements IntoIterator for NewsUpdateIterator
//! So, having the NewsUpdate(rvk::objects::newsfeed::NewsFeed) one can turn it into NewsUpdateIterator which
//! in its turn produces NewsItemModel objects from underlying rvk::objects::newsfeed::Item objects.
//! While producing models from siurce items there are some transformations applied:
//! * all markdown control symbols are "preserved", examples: '&' --> &amp; or '>' --> &gt;
//! * all URLs are surrounded by <a href=> tag
use crate::utils::local_from_timestamp;
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
            Some(NewsItemModel {
                author: source.unwrap_or_default(),
                itemtype: item.type_.clone(),
                datetime: format!(
                    "{}",
                    local_from_timestamp(item.date).format("%d.%m.%Y %H:%M (%a)")
                ),
                content: if let Some(text) = item.text.as_ref() {
                    process_text(text)
                } else {
                    String::new()
                },
            })
        }
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
