
use super::PGCONN;

use auth::authenticator::Authenticator;
use rocket::config::{Config, Environment};
use cookie_data::{SECRET_KEY, CookieId};
use login_form_status::AuthFail;

pub struct AdminAuth {
    pub username: String,
    password: String,
    pub userid: u32,
    pub failreason: Option<String>,
}

impl AuthFail for AdminAuth {
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

impl AdminAuth {
    pub fn new(username: String, password: String, userid: u32) -> AdminAuth {
        AdminAuth {
            username,
            password,
            userid,
            failreason: None,
        }
    }
    pub fn error(username: String, reason: String) -> AdminAuth {
        AdminAuth {
            username,
            password: String::new(),
            userid: 0,
            failreason: if &reason != "" { Some(reason) } else { None },
        }
    }
    pub fn authenticate(username: &str, password: &str) -> Result<Self, Self> {
        // if username == "andrew" {
        //     Ok(AdminAuth::new(username.to_string(), password.to_string()))
        // } else {
        //     Err( AdminAuth::error(username.to_string(), "Invalid username.".to_string()) )
        // }
        let qrystr = format!("SELECT userid FROM users WHERE username = '{user}' AND password = '{pass}' AND is_admin = true", user=username, pass=password);
        let conn = PGCONN.lock().unwrap();
        let qry = conn.query(&qrystr, &[]);
        if let Ok(result) = qry {
            if !result.is_empty() && result.len() == 1 {
                let uid = result.get(0).get(0);
                return Ok(AdminAuth::new(username.to_string(), password.to_string(), uid));
            }
        }
        let subqrystr = format!("SELECT userid FROM users WHERE username = '{user}'", user=username);
        if let Ok(subrst) = conn.query(&subqrystr, &[]) {
            if !subrst.is_empty() && subrst.len() == 1 {
                Err( AdminAuth::error(username.to_string(), "Incorrect password.".to_string()) )
            } else {
                Err( AdminAuth::error(username.to_string(), "The username does not exist.".to_string()) )
            }
        } else {
            Err( AdminAuth::error(username.to_string(), "The login request failed.".to_string()) )
        }
    }
}

impl Authenticator for  AdminAuth {
    type User = String;
    
    fn user(&self) -> String {
        self.username.clone() // Todo: remove clone?
    }
    
    fn check_credentials(username: String, password: String) -> Result<Self, Self> {
        AdminAuth::authenticate(&username, &password)
    }
}

impl CookieId for AdminAuth {
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
    // fn decode_cookie(string: String) -> AdminAuth {
    //     let userid = &string[..10];
    //     let username = &string[10..];
    //     AdminAuth {
    //         userid,
    //         password: String::new(),
    //         username,
    //         failreason: None,
    //     }
    // }
    // fn encode_cookie(&self) -> String {
    //     let mut encoded = String::with_capacity(self.username.len()+12);
    //     encoded.push_str(&format!("{:0>10}", self.userid));
    //     encoded.push_str(&self.username);
    //     encoded
    // }
}








