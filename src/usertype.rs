
use rocket;

use ::rocket::request::FromRequest;
use ::rocket::Request;
use ::rocket::outcome::Outcome;
use ::rocket::config::{Config, Environment};
use login_form_status::AuthFail;

use auth::userpass::*;
use auth::authenticator::Authenticator;


pub trait UserType : Authenticator + CookieId + AuthFail {
    // AuthFail
    // fn reason(&self) -> String;
    
    // CookieId
    // fn get_cookie_config() -> Config;
    // fn get_cookie_id() -> String;
    // fn cookie_username(&self) -> String;
    fn new(String, String) -> Self;
    fn error(String, String) -> Self;
    fn authenticate(&str, &str) -> Result<Self, Self>;
    
    // Authenticator
    // fn user(&self) -> String;
    // fn check_credentials(String, String) -> Result<Self, Self>;
}


pub struct RegularUser {
    pub username: String,
    pub userid: u32,
    password: String,
    pub failreason: Option<String>,
}








