
use auth::authenticator::Authenticator;
use rocket::config::{Config, Environment};
use cookie_data::{SECRET_KEY, CookieId};
use login_form_status::AuthFail;

use super::PGCONN;

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
    // fn reason_str(&self) -> &str {
    //     if let Some(ref msg) = self.failreason {
    //         msg
    //     } else {
    //         ""
    //     }
    // }
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
        // if username == "andrew" {
        //     Ok(UserAuth::new(username.to_string(), password.to_string()))
        // } else {
        //     Err( UserAuth::error(username.to_string(), "Invalid username.".to_string()) )
        // }
        let qrystr = format!("SELECT userid FROM users WHERE username = '{user}' AND password = '{pass}'", user=username, pass=password);
        let conn = PGCONN.lock().unwrap();
        let qry = conn.query(&qrystr, &[]);
        if let Ok(result) = qry {
            if !result.is_empty() && result.len() == 1 {
                return Ok(UserAuth::new(username.to_string(), password.to_string()));
            }
        }
        let subqrystr = format!("SELECT userid FROM users WHERE username = '{user}'", user=username);
        if let Ok(subrst) = conn.query(&subqrystr, &[]) {
            if !subrst.is_empty() && subrst.len() == 1 {
                Err( UserAuth::error(username.to_string(), "Incorrect password.".to_string()) )
            } else {
                Err( UserAuth::error(username.to_string(), "The username does not exist.".to_string()) )
            }
        } else {
            Err( UserAuth::error(username.to_string(), "The login request failed.".to_string()) )
        }
    }
}

impl Authenticator for UserAuth {
    type User = String;
    
    fn user(&self) -> String {
        self.username.clone() // Todo: remove clone?
    }
    
    fn check_credentials(username: String, password: String) -> Result<Self, Self> {
        UserAuth::authenticate(&username, &password)
    }
}

impl CookieId for UserAuth {
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
    // fn decode_cookie(string: String) -> Self {
    //     Self::new(string, String::new())
    // }
    // fn encode_cookie(&self) -> String {
    //     self.username.clone()
    // }
}

