use crate::storage::Storage;
use rvk::{methods::users, objects::user::User as VKUser, APIClient, Params};
use std::fmt;

pub struct User {
    pub data: VKUser,
}

pub struct UserViewModel {
    pub name: String,
    pub image: String,
    pub status: String,
}

impl User {
    pub async fn query_async(api: &APIClient, user_id: &str) -> Option<Self> {
        let mut params = Params::new();
        params.insert("user_id".into(), user_id.into());
        params.insert("fields".into(), "photo_50".into());
        match users::get::<Vec<VKUser>>(api, params).await {
            Ok(mut users) => {
                if users.len() > 0 {
                    Some(User {
                        data: users.pop().unwrap(),
                    })
                } else {
                    None
                }
            }
            Err(e) => {
                println!("Failed query user info: {}", e);
                None
            }
        }
    }

    pub async fn get_view_model(&self, storage: &Storage) -> UserViewModel {
        let uri = if let Some(uri) = self.data.photo_50.as_ref() {
            uri.clone()
        } else {
            String::new()
        };
        let image = if let Ok(s) = storage.get_file(uri.as_str()).await {
            s
        } else {
            String::new()
        };
        UserViewModel {
            name: self.data.first_name.clone() + " " + &self.data.last_name,
            image,
            status: self.data.status.as_ref().unwrap_or(&String::new()).clone(),
        }
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.data.first_name,
            self.data.last_name,
            self.data.status.as_ref().unwrap_or(&String::new())
        )
    }
}

impl fmt::Display for UserViewModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} image: {}", self.name, self.status, self.image)
    }
}
