use rvk::{methods, objects, APIClient, Params};
use std::fmt;

pub struct AccountProvider;

pub struct Account(pub objects::account::Account);

impl AccountProvider {
    pub async fn query_async(api: &APIClient) -> Option<Account> {
        let params = Params::new();
        match methods::account::get_profile_info::<objects::account::Account>(api, params).await {
            Ok(a) => Some(Account(a)),
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
            self.0.first_name,
            self.0.last_name,
            self.0.status.as_ref().unwrap_or(&String::new())
        )
    }
}
