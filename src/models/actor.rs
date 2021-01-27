use crate::vk_provider::URI_BASE;
use std::fmt;

#[derive(Clone)]
pub struct ActorModel {
    pub id: i64,
    pub name: String,    // RT
    pub avatar: String,  // url
    pub desc: String,    // Интернет СМИ
    pub rel_uri: String, // url to page https://vk.com/rt_russian
    pub comment: String, // 1 266 277 подписчиков
}

impl ActorModel {
    pub fn get_uri(&self) -> String {
        URI_BASE.to_string() + self.rel_uri.as_str()
    }
}

impl fmt::Display for ActorModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "source {}", &self.name)
    }
}
