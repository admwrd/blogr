
use auth::authenticator::Authenticator;
use rocket::config::{Config, Environment};
use cookie_data::{SECRET_KEY, CookieId};

pub struct UserAuth {
    pub username: String,
    password: String,
}

impl UserAuth {
    pub fn new(username: String, password: String) -> UserAuth {
        UserAuth {
            username,
            password,
        }
    }
    pub fn authenticate(username: &str, password: &str) -> Result<Self, Self> {
        if username == "andrew" {
            Ok(UserAuth::new(username.to_string(), password.to_string()))
        } else {
            Err( UserAuth::new("Invalid username.".to_string(), String::new()) )
        }
    }
}

impl Authenticator for  UserAuth {
    type User = String;
    
    fn user(&self) -> String {
        self.username.clone() // Todo: remove clone?
    }
    
    fn check_credentials(username: String, password: String) -> Result<Self, Self> {
        UserAuth::authenticate(&username, &password)
    }
}

impl CookieId for UserAuth {
    fn get_cookie_config() -> Config {
        Config::build(Environment::active().unwrap())
            .secret_key(SECRET_KEY)
            .extra("user_cookie_identifier", "usid")
            .unwrap()
    }
    fn get_cookie_id() -> String {
        let config = Self::get_cookie_config();
            config.get_str("user_cookie_identifier").unwrap().to_owned()
    }
}

