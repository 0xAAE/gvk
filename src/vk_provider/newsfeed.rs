use chrono::Utc;
use rvk::{
    methods::newsfeed,
    objects::{
        attachment, audio, document, geo, group, link, market_album, market_item, note, page,
        photo, podcast, poll, post, post_source, sticker, user, video,
    },
    APIClient, Params,
};
use serde::Deserialize;
use std::env;

// a maximal time interval to limit news updates
const MAX_UPDATE_DELTA_SEC: u64 = 3_600; // 1 hour

/// <https://vk.com/dev/newsfeed.get>
pub struct NewsFeed {
    start_sec: u64,
    // end_sec: u64,
    start_from: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PhotoSet {
    // информация о количестве объектов
    pub count: i64,
    // и до 5 последних объектов, связанных с данной новостью
    pub items: Option<Vec<photo::Photo>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PhotoTags {
    // информация о количестве объектов
    pub count: i64,
    // и до 5 последних объектов, связанных с данной новостью
    //pub items: Option<Vec<?>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NoteSet {
    // информация о количестве объектов
    pub count: i64,
    // и до 5 последних объектов, связанных с данной новостью
    pub items: Option<Vec<note::Note>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FriendSet {
    // информация о количестве объектов
    pub count: i64,
    // и до 5 последних объектов, связанных с данной новостью
    pub items: Option<Vec<i64>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HistoryItem {
    pub date: u64,
    pub from_id: i64,
    pub id: i64,
    pub owner_id: i64,
    // находится в записях со стен и содержит массив объектов, которые прикреплены к текущей новости (фотография, ссылка и т.п.).
    // Более подробная информация представлена на странице <https://vk.com/dev/objects/attachments_w>
    pub attachments: Option<Vec<attachment::WallAttachment>>,
    // тип
    pub post_type: Option<String>,
    // находится в записях со стен и содержит текст записи
    pub text: Option<String>,
    // source
    pub post_source: Option<post_source::PostSource>,
}

/// undocumented, differs from the photo::Album <https://vk.com/dev/objects/attachments_w> by id: String
#[derive(Deserialize, Clone, Debug)]
pub struct Album {
    pub id: String,
    pub thumb: photo::Photo,
    pub owner_id: i64,
    pub title: String,
    pub description: String,
    pub created: u64,
    pub updated: u64,
    pub size: u64,
}

/// undocumented, differs from WallAttachment <https://vk.com/dev/objects/attachments_w> by album
/// which does not equal to album::Album (id: String)
#[derive(Deserialize, Clone, Debug)]
pub struct NewsAttachment {
    #[serde(rename = "type")]
    pub type_: String,

    // type = photo
    pub photo: Option<photo::Photo>,

    // type = posted_photo
    pub posted_photo: Option<attachment::PostedPhoto>,

    // type = video
    pub video: Option<video::Video>,

    // type = audio
    pub audio: Option<audio::Audio>,

    // type = doc
    pub doc: Option<document::Document>,

    // type = graffiti
    pub graffiti: Option<attachment::Graffiti>,

    // type = link
    pub link: Option<link::Link>,

    // type = note
    pub note: Option<note::Note>,

    // type = app
    pub app: Option<attachment::App>,

    // type = poll
    pub poll: Option<poll::Poll>,

    // type = page
    pub page: Option<page::Page>,

    // type = album
    pub album: Option<Album>,

    // type = photos_list
    pub photos_list: Option<Vec<String>>,

    // type = market
    pub market: Option<market_item::MarketItem>,

    // type = market_album
    pub market_album: Option<market_album::MarketAlbum>,

    // type = sticker
    pub sticker: Option<sticker::Sticker>,

    // type = pretty_cards
    pub cards: Option<Vec<attachment::Card>>,

    // type = event
    pub event: Option<attachment::Event>,

    // type = podcast
    pub podcast: Option<podcast::Podcast>,
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
    pub post_id: Option<u64>,
    // находится в записях со стен, содержит тип новости (post или copy)
    pub post_type: Option<String>,
    // передается в случае, если этот пост сделан при удалении
    pub final_post: Option<String>,
    // находится в записях со стен, если сообщение является копией сообщения с чужой стены,
    // и содержит идентификатор владельца стены, у которого было скопировано сообщение
    pub copy_owner_id: Option<i64>,
    // находится в записях со стен, если сообщение является копией сообщения с чужой стены,
    // и содержит идентификатор скопированного сообщения на стене его владельца
    pub copy_post_id: Option<String>,
    // массив, содержащий историю репостов для записи. Возвращается только в том случае,
    // если запись является репостом. Каждый из объектов массива, в свою очередь,
    // является объектом-записью стандартного формата (wtf?)
    pub copy_history: Option<Vec<HistoryItem>>,
    // находится в записях со стен, если сообщение является копией сообщения с чужой стены,
    // и содержит дату скопированного сообщения
    pub copy_post_date: Option<String>,
    // находится в записях со стен и содержит текст записи
    pub text: Option<String>,
    // содержит 1, если текущий пользователь может редактировать запись
    pub can_edit: Option<u64>,
    // возвращается, если пользователь может удалить новость, всегда содержит 1
    pub can_delete: Option<u64>,
    // находится в записях со стен и содержит информацию о комментариях к записи,
    pub comments: Option<post::Comments>,
    //  находится в записях со стен и содержит информацию о числе людей, которым понравилась данная запись
    pub likes: Option<post::Likes>,
    // находится в записях со стен и содержит информацию о числе людей, которые скопировали данную запись на свою страницу
    pub reposts: Option<post::Reposts>,
    // находится в записях со стен и содержит массив объектов, которые прикреплены к текущей новости (фотография, ссылка и т.п.).
    // Более подробная информация представлена на странице <https://vk.com/dev/objects/attachments_w>
    pub attachments: Option<Vec<NewsAttachment>>,
    // geo — находится в записях со стен, в которых имеется информация о местоположении
    pub geo: Option<geo::Geo>,
    // (кроме записей со стен) содержат информацию о количестве объектов и до 5 последних объектов, связанных с данной новостью
    pub photos: Option<PhotoSet>,
    // (кроме записей со стен) содержат информацию о количестве объектов и до 5 последних объектов, связанных с данной новостью
    pub photo_tags: Option<PhotoTags>,
    // (кроме записей со стен) содержат информацию о количестве объектов и до 5 последних объектов, связанных с данной новостью
    pub notes: Option<NoteSet>,
    // (кроме записей со стен) содержат информацию о количестве объектов и до 5 последних объектов, связанных с данной новостью
    pub friends: Option<FriendSet>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct NewsUpdate {
    // массив новостей для текущего пользователя
    pub items: Option<Vec<Item>>,
    // информация о пользователях (<https://vk.com/dev/objects/user>), которые находятся в списке новостей
    pub profiles: Option<Vec<user::User>>,
    // содержит массив объектов сообществ (<https://vk.com/dev/objects/groups>), которые присутствуют в новостях
    pub groups: Option<Vec<group::Group>>,
    // offset, который необходимо передать, для того, чтобы получить следующую часть новостей (в более старых версиях API)
    //pub new_offset: u64,
    // start_from, который необходимо передать, для того, чтобы получить следующую часть новостей.
    // Позволяет избавиться от дубликатов, которые могут возникнуть при появлении новых новостей между вызовами этого метода.
    pub next_from: Option<String>,
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
    pub async fn prev_update(&mut self, api: &mut APIClient) -> Option<NewsUpdate> {
        let upd_end_time = self.start_sec;
        let upd_start_time = upd_end_time - MAX_UPDATE_DELTA_SEC;
        let mut params = Params::new();
        params.insert("start_time".into(), format!("{}", upd_start_time).into());
        params.insert("end_time".into(), format!("{}", upd_end_time).into());
        self.do_update(api, params).await
    }

    // returrns next protion of the news, i.e. subsequent to the most recent ones
    pub async fn next_update(&mut self, api: &mut APIClient) -> Option<NewsUpdate> {
        let mut params = Params::new();
        params.insert("start_from".into(), self.start_from.clone());
        self.do_update(api, params).await
    }

    async fn do_update(&mut self, api: &mut APIClient, params: Params) -> Option<NewsUpdate> {
        let trace = env::var("TRACE_NEWS").unwrap_or_default();
        api.trace_response(trace == "1");
        match newsfeed::get::<NewsUpdate>(api, params).await {
            Ok(upd) => {
                api.trace_response(false);
                self.start_from = if let Some(val) = upd.next_from.as_ref() {
                    val.clone()
                } else {
                    String::new()
                };
                Some(upd)
            }
            Err(e) => {
                api.trace_response(false);
                match e {
                    rvk::error::Error::API(e) => {
                        println!(
                            "Failed requesting news update: {}, extra {:?}",
                            e.msg(),
                            e.extra()
                        );
                    }
                    _ => println!("Failed requesting news update: {}", e),
                }
                None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::storage::Storage;
    use std::fs::read_to_string;
    use std::path::Path;

    #[test]
    fn deserialize_news_update() {
        let storage = Storage::new();
        let json_file = storage.get_cache_dir().to_string() + "/news.json";
        let path = Path::new(&json_file);
        let s = read_to_string(&path).unwrap();
        let result = serde_json::from_str::<NewsUpdate>(&s);
        match result {
            Ok(_upd) => assert!(true),
            Err(e) => {
                let msg = format!("{}", e);
                println!("{}", msg);
                assert!(false);
            }
        }
    }
}
