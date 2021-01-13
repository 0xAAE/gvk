use chrono::Utc;
use rvk::{
    methods::newsfeed,
    objects::{attachment, geo, group, note, photo, post, user},
    APIClient, Params,
};
use serde::Deserialize;

// a maximal time interval to limit news updates
const MAX_UPDATE_DELTA_SEC: u64 = 3_600; // 1 hour

pub enum NewsFeedError {
    Request,
}

/// <https://vk.com/dev/newsfeed.get>
pub struct NewsFeed {
    start_sec: u64,
    // end_sec: u64,
    start_from: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Item {
    // тип списка новости, соответствующий одному из значений параметра filters
    #[serde(rename = "type")]
    pub type_: String,
    // идентификатор источника новости (положительный — новость пользователя, отрицательный — новость группы)
    pub source_id: i64,
    // время публикации новости в формате unixtime
    pub date: u64,
    // находится в записях со стен и содержит идентификатор записи на стене владельца
    pub post_id: Option<String>,
    // находится в записях со стен, содержит тип новости (post или copy)
    pub post_type: Option<String>,
    // передается в случае, если этот пост сделан при удалении
    pub final_post: Option<String>,
    // находится в записях со стен, если сообщение является копией сообщения с чужой стены,
    // и содержит идентификатор владельца стены, у которого было скопировано сообщение
    pub copy_owner_id: Option<String>,
    // находится в записях со стен, если сообщение является копией сообщения с чужой стены,
    // и содержит идентификатор скопированного сообщения на стене его владельца
    pub copy_post_id: Option<String>,
    // массив, содержащий историю репостов для записи. Возвращается только в том случае,
    // если запись является репостом. Каждый из объектов массива, в свою очередь,
    // является объектом-записью стандартного формата (wtf?)
    pub copy_history: Option<Vec<post::Post>>,
    // находится в записях со стен, если сообщение является копией сообщения с чужой стены,
    // и содержит дату скопированного сообщения
    pub copy_post_date: Option<String>,
    // находится в записях со стен и содержит текст записи
    pub text: Option<String>,
    // содержит 1, если текущий пользователь может редактировать запись
    pub can_edit: u64,
    // возвращается, если пользователь может удалить новость, всегда содержит 1
    pub can_delete: u64,
    // находится в записях со стен и содержит информацию о комментариях к записи,
    pub comments: Option<post::Comments>,
    //  — находится в записях со стен и содержит информацию о числе людей, которым понравилась данная запись
    pub likes: Option<post::Likes>,
    // находится в записях со стен и содержит информацию о числе людей, которые скопировали данную запись на свою страницу
    pub reposts: Option<post::Reposts>,
    // аходится в записях со стен и содержит массив объектов, которые прикреплены к текущей новости (фотография, ссылка и т.п.).
    // Более подробная информация представлена на странице <https://vk.com/dev/objects/attachments_w>
    pub attachments: Option<Vec<attachment::WallAttachment>>,
    // geo — находится в записях со стен, в которых имеется информация о местоположении
    pub geo: Option<geo::Geo>,
    // (кроме записей со стен) содержат информацию о количестве объектов и до 5 последних объектов, связанных с данной новостью
    pub photos: Option<Vec<photo::Photo>>,
    // (кроме записей со стен) содержат информацию о количестве объектов и до 5 последних объектов, связанных с данной новостью
    pub photo_tags: Option<Vec<photo::Photo>>,
    // (кроме записей со стен) содержат информацию о количестве объектов и до 5 последних объектов, связанных с данной новостью
    pub notes: Option<Vec<note::Note>>,
    // (кроме записей со стен) содержат информацию о количестве объектов и до 5 последних объектов, связанных с данной новостью
    pub friends: Option<Vec<i64>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct NewsUpdate {
    // массив новостей для текущего пользователя
    pub items: Option<Vec<Item>>,
    // информация о пользователях (<https://vk.com/dev/objects/user>), которые находятся в списке новостей
    pub profiles: Option<Vec<user::User>>,
    // содержит массив объектов сообществ (<https://vk.com/dev/objects/groups>), которые присутствуют в новостя
    pub groups: Option<Vec<group::Group>>,
    // offset, который необходимо передать, для того, чтобы получить следующую часть новостей (в более старых версиях API)
    pub new_offset: u64,
    // start_from, который необходимо передать, для того, чтобы получить следующую часть новостей.
    // Позволяет избавиться от дубликатов, которые могут возникнуть при появлении новых новостей между вызовами этого метода.
    pub next_from: String,
}

impl NewsFeed {
    pub fn new() -> Self {
        let end_sec = Utc::now().timestamp() as u64;
        let start_sec = end_sec - MAX_UPDATE_DELTA_SEC; // for the last hour
        NewsFeed {
            start_sec,
            // end_sec,
            start_from: String::new(),
        }
    }

    // returns pack of the news preceeding the current the most old one, i.e. start_sec - MAX_UPDATE_DELTA_SEC
    pub async fn prev_update(&mut self, api: &APIClient) -> Option<NewsUpdate> {
        let upd_end_time = self.start_sec;
        let upd_start_time = upd_end_time - MAX_UPDATE_DELTA_SEC;
        let mut params = Params::new();
        params.insert("start_time".into(), format!("{}", upd_start_time).into());
        params.insert("end_time".into(), format!("{}", upd_end_time).into());
        match newsfeed::get::<NewsUpdate>(api, params).await {
            Ok(upd) => {
                self.start_sec = upd_start_time;
                Some(upd)
            }
            Err(e) => {
                println!("Failed requesting news update: {}", e);
                None
            }
        }
    }

    // returrns next protion of the news, i.e. subsequent to the most recent ones
    pub async fn next_update(&mut self, api: &APIClient) -> Option<NewsUpdate> {
        let mut params = Params::new();
        params.insert("start_from".into(), self.start_from.clone());
        match newsfeed::get::<NewsUpdate>(api, params).await {
            Ok(upd) => {
                self.start_from = upd.next_from.clone();
                Some(upd)
            }
            Err(e) => {
                println!("Failed requesting news update: {}", e);
                None
            }
        }
    }
}
