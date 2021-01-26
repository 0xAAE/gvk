use chrono::{DateTime, Local, NaiveDateTime, Utc};

pub fn local_from_timestamp(timestamp: i64) -> DateTime<Local> {
    utc_from_timestamp(timestamp).with_timezone(&Local)
}

pub fn utc_from_timestamp(timestamp: i64) -> DateTime<Utc> {
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);
    DateTime::<Utc>::from_utc(naive, Utc)
}

// perform desired text processing before display it
pub fn process_text(text: &str) -> String {
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

    #[test]
    fn test_link_formatting() {
        let src_url = "https://habr.com/ru/post/538874/?utm_campaign=538874&utm_source=habrahabr&utm_medium=rss";
        let uri = format!(r#"<a href="{}">{}</a>"#, &src_url, &src_url);
        let expected = r#"<a href="https://habr.com/ru/post/538874/?utm_campaign=538874&utm_source=habrahabr&utm_medium=rss">https://habr.com/ru/post/538874/?utm_campaign=538874&utm_source=habrahabr&utm_medium=rss</a>"#;
        assert_eq!(uri, expected);

        let uri = format!(
            r#"<a href="{}">{}</a>"#,
            &src_url,
            glib::markup_escape_text(src_url).to_string()
        );
        let expected = r#"<a href="https://habr.com/ru/post/538874/?utm_campaign=538874&utm_source=habrahabr&utm_medium=rss">https://habr.com/ru/post/538874/?utm_campaign=538874&amp;utm_source=habrahabr&amp;utm_medium=rss</a>"#;
        assert_eq!(uri, expected);
    }
}
