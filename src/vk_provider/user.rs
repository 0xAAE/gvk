use crate::models;
use crate::storage::Storage;
use rvk::{methods::users, objects::user::User as VKUser, APIClient, Params};
use std::boxed::Box;
use std::fmt;

pub struct User(pub Box<VKUser>);

impl User {
    pub async fn query_async(api: &APIClient, user_id: &str) -> Option<Self> {
        let mut params = Params::new();
        params.insert("user_id".into(), user_id.into());
        params.insert("fields".into(), "photo_50".into());
        match users::get::<Vec<VKUser>>(api, params).await {
            Ok(mut users) => {
                if users.len() > 0 {
                    Some(User(Box::new(users.pop().unwrap())))
                } else {
                    None
                }
            }
            Err(e) => {
                log::error!("failed query user info: {}", e);
                None
            }
        }
    }

    pub async fn get_view_model(&self, storage: &Storage) -> models::UserModel {
        let uri = if let Some(uri) = self.0.photo_50.as_ref() {
            uri.clone()
        } else {
            String::new()
        };
        let image = if let Ok(s) = storage.get_file(uri.as_str(), "").await {
            s
        } else {
            String::new()
        };
        models::UserModel {
            name: self.0.first_name.clone() + " " + &self.0.last_name,
            image,
            status: self.0.status.as_ref().unwrap_or(&String::new()).clone(),
        }
    }

    pub fn get_max_photo(user: &VKUser) -> String {
        if let Some(photo) = &user.photo_400_orig {
            photo.clone()
        } else if let Some(photo) = &user.photo_200_orig {
            photo.clone()
        } else if let Some(photo) = &user.photo_200 {
            photo.clone()
        } else if let Some(photo) = &user.photo_100 {
            photo.clone()
        } else if let Some(photo) = &user.photo_50 {
            photo.clone()
        } else {
            String::new()
        }
    }

    pub fn get_small_photo(user: &VKUser) -> String {
        if let Some(photo) = &user.photo_50 {
            photo.clone()
        } else {
            String::new()
        }
    }

    pub fn get_full_name(user: &VKUser) -> String {
        user.first_name.clone() + " " + user.last_name.as_str()
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.0.first_name,
            self.0.last_name,
            self.0.status.as_ref().unwrap_or(&String::new())
        )
    }
}
