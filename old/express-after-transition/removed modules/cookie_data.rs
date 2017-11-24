
// use auth::config::*;
// use ::cookies::*;

use auth::userpass::*;

use rocket;
use ::rocket::request::FromRequest;
use ::rocket::Request;
use ::rocket::outcome::Outcome;
use ::rocket::config::{Config, Environment};

// Todo: possibly: allow AdminCookie to take a generic parameter that
//         will be a struct containing the user data
//       make a trait that will create the struct
//         it should take the username from the Cookie and look
//         up the username and retrieve the user data into the 
//         generic parameter

pub(crate) const SECRET_KEY: &'static str = "8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=";

pub trait CookieId {
    // fn get_cookie_config(&self) -> Config;
    // fn get_cookie_id(&self) -> String;
    fn get_cid() -> &'static str;
    fn get_cookie_config() -> Config;
    fn get_cookie_id() -> String;
    fn cookie_username(&self) -> String;
}

pub struct AdminCookie {
    pub username: String,
}

impl AdminCookie {
    pub fn new(username: String) -> AdminCookie {
        AdminCookie {
            username,
        }
    }
}

impl CookieId for AdminCookie {
    fn get_cid() -> &'static str {
        "asid"
    }
    fn get_cookie_config() -> Config {
        Config::build(Environment::active().unwrap())
            // .secret_key(SECRET_KEY)
            .extra("admin_cookie_identifier", "asid")
            .unwrap()
    }
    fn get_cookie_id() -> String {
        let config = Self::get_cookie_config();
            config.get_str("admin_cookie_identifier").unwrap().to_owned()
    }
    fn cookie_username(&self) -> String {
        self.username.clone()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AdminCookie {
    type Error = ();
    
    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<AdminCookie, Self::Error> {
        // let cid = cookies::get_cookie_identifier();
        let cid = AdminCookie::get_cookie_id();
        let mut cookies = request.cookies();
        
        match cookies.get_private(&cid) {
            Some(cookie) => Outcome::Success(AdminCookie::new(cookie.value().to_string())),
            None => Outcome::Forward(()),
        }
    }
}



pub struct UserCookie {
    pub username: String,
}

impl UserCookie {
    pub fn new(username: String) -> UserCookie {
        UserCookie {
            username,
        }
    }
}



impl CookieId for UserCookie {
    fn get_cid() -> &'static str {
        "usid"
    }
    fn get_cookie_config() -> Config {
        Config::build(Environment::active().unwrap())
            // .secret_key(SECRET_KEY)
            .extra("user_cookie_identifier", "usid")
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
impl<'a, 'r> FromRequest<'a, 'r> for UserCookie {
    type Error = ();
    
    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<UserCookie, Self::Error> {
        // let cid = cookies::get_cookie_identifier();
        let cid = UserCookie::get_cookie_id();
        let mut cookies = request.cookies();
        
        match cookies.get_private(&cid) {
            Some(cookie) => Outcome::Success(UserCookie::new(cookie.value().to_string())),
            None => Outcome::Forward(()),
        }
    }
}



