
use rocket;

use ::rocket::request::FromRequest;
use ::rocket::Request;
use ::rocket::outcome::Outcome;
use ::rocket::config::{Config, Environment};
use login_form_status::AuthFail;

use auth::userpass::*;
use auth::authenticator::Authenticator;


pub trait UserType : Authenticator + CookieId + AuthFail + FromRequest +  {
    fn new(String, String) -> Self;
    fn error(String, String) -> Self;
    fn authenticate(&str, &str) -> Result<Self, Self>;
    fn new_cookie()
    // AuthFail
    // fn reason(&self) -> String;
    
    // CookieId
    // fn get_cookie_config() -> Config;
    // fn get_cookie_id() -> String;
    // fn cookie_username(&self) -> String;
    
    // Authenticator
    // fn user(&self) -> String;
    // fn check_credentials(String, String) -> Result<Self, Self>;
    
}

// Inevestigate using UserPass<RegularUser> instead
//      or maybe implementing ToString/FromString for RegularUser

pub enum UserRegular

pub struct RegularUser {
    pub username: String,
    pub userid: u32,
    password: String,
    pub failreason: Option<String>,
}

impl UserType for RegularUser {
    fn new(username: String, password: String) -> Self {
        RegularUser {
            username,
            userid: 666,
            password,
            failreason: None,
        }
    }
    fn new_cookie(username: String) -> Self {
        RegularUser {
            username,
            userid: 666,
            password: String::new(),
            failreason: None,
        }
    }
    fn error(username: String, failreason: String) -> Self {
        RegularUser {
            username,
            userid: 0,
            password: String::new(),
            failreason: Some(failreason),
        }
    }
    fn authenticate(username: &str, password: &str) -> Result<Self, Self> {
        if password.len() != 64 {
            Err( RegularUser::error(username.to_string(), format!("Invalid password: expected a SHA-256 hashed string, received: {} character long string", password.len())) )
        } else if &username == "andrew" {
            Ok(RegularUser::new(username.to_string(), password.to_string()))
        } else {
            Err( RegularUser::error(username.to_string(), format!("Invalid username")) )
        }
    }
}

impl Authenticator for RegularUser {
    type User = String;
    fn user(&self) -> String {
        self.username.clone()
    }
    fn check_credentials(username: String, password: String) -> Result<Self, Self> {
        RegularUser::authenticate(&username, &password)
    }
}

impl AutoFail for RegularUser {
    fn reason(&self) -> String {
        if let Some(ref msg) = self.failreason {
            msg.clone()
        } else {
            String::new()
        }
    }
}

impl CookieId for RegularUser {
    fn get_cookie_config() -> Config {
        Config::build(Environment::active().unwrap())
            .extra("regular_user_id", "ruid")
            .unwrap()
    }
    fn get_cookie_id() -> String {
        let config = Self::get_cookie_config();
        config.get_str("user_cookie_identifier").unwrap().to_owned()
    }
    fn cookie_username(&self) -> String {
        self.username.clone()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for RegularUser {
    type Error = ();
    
    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<RegularUser, Self::Error> {
        let cid = RegularUser::get_cookie_id();
        let mut cookies = request.cookies();
        
        match cookies.get_private(&cid) {
            Some(cookie) => Outcome::Success(RegularUser::new(cookie.value().to_string())),
            None => Outcome::Forward(()),
        }
    }
}




