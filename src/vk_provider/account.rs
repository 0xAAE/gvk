use rvk::objects::account::Account as VKAccount;
use rvk::{methods, objects, APIClient, Params};
use std::boxed::Box;
use std::fmt;

pub struct Account(pub Box<VKAccount>);

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

pub struct AccountProvider;

impl AccountProvider {
    pub async fn query_async(api: &APIClient) -> Option<Account> {
        let params = Params::new();
        match methods::account::get_profile_info::<objects::account::Account>(api, params).await {
            Ok(a) => Some(Account(Box::new(a))),
            Err(e) => {
                log::error!("failed query account info: {}", e);
                None
            }
        }
    }
}
