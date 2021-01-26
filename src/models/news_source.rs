use std::fmt;

pub struct NewsSourceModel {
    pub name: String,    // RT
    pub avatar: String,  // url
    pub desc: String,    // Интернет СМИ
    pub uri: String,     // url to page https://vk.com/rt_russian
    pub comment: String, // 1 266 277 подписчиков
}

impl fmt::Display for NewsSourceModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "source {}", &self.name)
    }
}
