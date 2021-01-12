use rvk::{
    methods::account,
    objects::{
        geo::{City, Country},
        user::User,
    },
    APIClient, Params,
};
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Clone, Default)]
pub struct Account {
    // имя пользователя
    pub first_name: String,
    // фамилия пользователя
    pub last_name: String,
    // девичья фамилия пользователя (только для женского пола)
    pub maiden_name: Option<String>,
    // короткое имя пользователя (если есть)
    pub screen_name: Option<String>,
    // пол: 1 — женский, 2 — мужской, 0 — пол не указан
    pub sex: Option<u32>,
    // семейное положение: 1 — не женат/не замужем, 2 — есть друг/есть подруга, 3 — помолвлен/помолвлена, 4 — женат/замужем
    // 5 — всё сложно, 6 — в активном поиске, 7 — влюблён/влюблена, 8 — в гражданском браке, 0 — не указано
    pub relation: Option<u32>,
    // объект пользователя, с которым связано семейное положение (если есть)
    pub relation_partner: Option<User>,
    // 1, если пользователь, указанный в relation_partner, не подтвердил отношения
    pub relation_pending: Option<u32>,
    // список объектов пользователей, которые указали, что состоят в отношениях с данным пользователем (если есть)
    pub relation_requests: Option<Vec<User>>,
    // дата рождения пользователя, возвращается в формате D.M.YYYY.
    pub bdate: Option<String>,
    // видимость даты рождения: 1 — показывать дату рождения, 2 — показывать только месяц и день, 0 — не показывать дату рождения
    pub bdate_visibility: Option<u32>,
    // название родного города
    pub home_town: Option<String>,
    // страна. Объект, содержащий поля:
    //  id (integer) — идентификатор страны
    //  title (string) — название страны
    pub country: Option<Country>,
    // город. Объект, содержащий поля:
    //  id (integer) — идентификатор города
    //  title (string) — название города
    pub city: Option<City>,
    // информация о заявке на смену имени, если она была подана. Объект, содержащий поля:
    //  id (integer) – идентификатор заявки, необходимый для её отмены (только если status равен processing)
    //  status (string) – статус заявки. Возможные значения:
    //      processing – заявка рассматривается
    //      declined – заявка отклонена
    //      response – общий ответ по статусу обработки заявки
    //      response_with_link – общий ответ по статусу обработки заявки, содержащий ссылку в поле link
    //  first_name (string) – имя пользователя, указанное в заявке
    //  last_name (string) – фамилия пользователя, указанная в заявке
    //pub name_request: NameRequest,
    // статус пользователя
    pub status: Option<String>,
    // номер телефона
    pub phone: Option<String>,
}

impl Account {
    pub async fn query_async(api: &APIClient) -> Option<Self> {
        let params = Params::new();
        match account::get_profile_info::<Account>(api, params).await {
            Ok(a) => Some(a),
            Err(e) => {
                println!("Failed query account info: {}", e);
                None
            }
        }
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.first_name,
            self.last_name,
            self.status.as_ref().unwrap_or(&String::new())
        )
    }
}
