
use auth::authenticator::Authenticator;
use rocket::config::{Config, Environment};
use cookie_data::{SECRET_KEY, CookieId};
use login_form_status::AuthFail;

pub struct UserAuth {
    pub username: String,
    password: String,
    pub failreason: Option<String>,
}

impl AuthFail for UserAuth {
    fn reason(&self) -> String {
        if let Some(ref msg) = self.failreason {
            msg.clone()
        } else {
            String::new()
        }
    }
    fn reason_str(&self) -> &str {
        if let Some(ref msg) = self.failreason {
            msg
        } else {
            ""
        }
    }
}

impl UserAuth {
    pub fn new(username: String, password: String) -> UserAuth {
        UserAuth {
            username,
            password,
            failreason: None,
        }
    }
    pub fn error(username: String, reason: String) -> UserAuth {
        UserAuth {
            username,
            password: String::new(),
            failreason: if &reason != "" { Some(reason) } else { None },
        }
    }
    pub fn authenticate(username: &str, password: &str) -> Result<Self, Self> {
        if username == "andrew" {
            Ok(UserAuth::new(username.to_string(), password.to_string()))
        } else {
            Err( UserAuth::error(username.to_string(), "Invalid username.".to_string()) )
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

