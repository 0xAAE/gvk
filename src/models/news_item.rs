use std::fmt;

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
