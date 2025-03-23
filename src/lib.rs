#![allow(dead_code, unused_variables)]
mod database;

mod auth_utils{

    pub fn login(creds: models::Credentials) {
        crate::database::get_user();
    }

    fn logout(){}

    pub mod models{
        pub struct Credentials {
            username: String,
            password: String,
        }
    }
}

use auth_utils::login;
use auth_utils::models::Credentials;
use database::{Status, connect_to_database};

pub fn authentication(creds: Credentials) {
    if let Status::Connected = connect_to_database() {
        login(creds);
    }
}
