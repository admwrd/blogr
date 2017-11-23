
use rocket::{Request, Outcome};
// use rocket::request::FromRequest;
use rocket::request::{FromRequest, FromForm, FormItems};
use std::collections::HashMap;
use std::str::{from_utf8};


use super::PGCONN;
// use password::*;
use rocket_auth_login::authorization::*;
use rocket_auth_login::sanitization::*;
// use auth::sanitization::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdministratorCookie {
    pub userid: u32,
    pub username: String,
    pub display: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdministratorForm {
    pub username: String,
    pub password: String,
}

impl CookieId for AdministratorCookie {
    fn cookie_id<'a>() -> &'a str {
        "ascid"
    }
}

impl CookieId for AdministratorForm {
    fn cookie_id<'a>() -> &'a str {
        "ascid"
    }
} 

impl AuthorizeCookie for AdministratorCookie {
    fn store_cookie(&self) -> String {
        ::serde_json::to_string(self).expect("Could not serialize")
    }
    
    
    #[allow(unused_variables)]
    fn retrieve_cookie(string: String) -> Option<Self> {
        let mut des_buf = string.clone();
        let des: Result<AdministratorCookie, _> = ::serde_json::from_str(&mut des_buf);
        if let Ok(cooky) = des {
            Some(cooky)
        } else {
            None
        }
    }
}

impl AuthorizeForm for AdministratorForm {
    type CookieType = AdministratorCookie;
    
    fn authenticate(&self) -> Result<Self::CookieType, AuthFail> {
        let conn = PGCONN.lock().unwrap();
        let authstr = format!(r#"
            SELECT u.userid, u.username, u.display FROM users u WHERE u.username = '{username}' AND is_admin = true AND 
                u.hash_salt = crypt('{password}', u.hash_salt)"#, username=&self.username, password=&self.password);
        let is_user_qrystr = format!("SELECT userid FROM users WHERE username = '{}'", &self.username);
        let is_admin_qrystr = format!("SELECT userid FROM users WHERE username = '{}' AND is_admin = '1'", &self.username);
        let password_qrystr = format!("SELECT userid FROM users WHERE username = '{}' AND hash_salt = crypt('{}', hash_salt)", &self.username, &self.password);
        println!("Running: {}", authstr);
        if let Ok(qry) = conn.query(&authstr, &[]) {
            if !qry.is_empty() && qry.len() == 1 {
                let row = qry.get(0);
                
                let display_opt = row.get_opt(2);
                let display = match display_opt {
                    Some(Ok(d)) => Some(d),
                    _ => None,
                };
                return Ok(AdministratorCookie {
                    userid: row.get(0),
                    username: row.get(1),
                    display,
                });
            }
        }
        if let Ok(eqry) = conn.query(&is_user_qrystr, &[]) {
            if eqry.is_empty() || eqry.len() == 0 {
                return Err(AuthFail::new(self.username.clone(), "Username was not found.".to_string()));
            }
        }
        if let Ok(eqry) = conn.query(&is_admin_qrystr, &[]) {
            if eqry.is_empty() || eqry.len() == 0 {
                // In production this message may be more harmful than useful as it
                // would be able to tell anyone who is an administrator and thus the
                // message should be changed to something like Unkown error or Invalid username/password
                return Err(AuthFail::new(self.username.clone(), "User does not have administrator priveleges.".to_string()));
            }
        }
        if let Ok(eqry) = conn.query(&password_qrystr, &[]) {
            if eqry.is_empty() || eqry.len() == 0 {
                return Err(AuthFail::new(self.username.clone(), "Invalid username / password combination.".to_string()));
            }
        }
        Err(AuthFail::new(self.username.clone(), "Unknown error..".to_string()))
    }
    
    fn new_form(user: &str, pass: &str, _extras: Option<HashMap<String, String>>) -> Self {
        AdministratorForm {
            username: user.to_string(),
            password: pass.to_string(),
        }
    }
    
}

impl<'a, 'r> FromRequest<'a, 'r> for AdministratorCookie {
    type Error = ();
    
    fn from_request(request: &'a Request<'r>) -> ::rocket::request::Outcome<AdministratorCookie,Self::Error>{
        let cid = AdministratorCookie::cookie_id();
        let mut cookies = request.cookies();
        
        match cookies.get_private(cid) {
            Some(cookie) => {
                if let Some(cookie_deserialized) = AdministratorCookie::retrieve_cookie(cookie.value().to_string()) {
                    Outcome::Success(
                        cookie_deserialized
                    )
                } else {
                    Outcome::Forward(())
                }
            },
            None => Outcome::Forward(())
        }
    }
}




// impl<'f> FromForm<'f> for AdministratorForm {
//     type Error = &'static str;
    
//     fn from_form(form_items: &mut FormItems<'f>, _strict: bool) -> Result<Self, Self::Error> {
//         // let mut user_pass = HashMap::new();
//         let mut user: String = String::new();
//         let mut pass: String = String::new();
//         // let mut pass: Vec<u8> = Vec::new();
//         let mut extras: HashMap<String, String> = HashMap::new();
        
//         for (key,value) in form_items {
//             match key.as_str(){
//                 "username" => {
//                     // user = sanitize(&value.url_decode().unwrap_or(String::new()));
//                     user = AdministratorForm::clean_username(&value.url_decode().unwrap_or(String::new()));
//                 },
//                 "password" => {
//                     // pass = sanitize_password(&value.url_decode().unwrap_or(String::new()));
//                     pass = AdministratorForm::clean_password(&value.url_decode().unwrap_or(String::new()));
//                     // pass = value.bytes().collect();
//                 },
//                 // _ => {},
//                 a => {
//                     // extras.insert( a.to_string(), sanitize( &value.url_decode().unwrap_or(String::new()) ) );
//                     extras.insert( a.to_string(), AdministratorForm::clean_extras( &value.url_decode().unwrap_or(String::new()) ) );
//                 },
//             }
//         }
        
//         // println!("Creating login form data structure with:\nUser: {}\nPass: {}\nExtras: {:?}", user, pass, extras);
        
//         // Do not need to check for username / password here,
//         // if the authentication method requires them it will
//         // fail at that point.
//         Ok(
//             if extras.len() == 0 {
//               AdministratorForm::new_form(&user, &pass, None)
//            } else {
//                AdministratorForm::new_form(&user, &pass, Some(extras))
//            }
//         )
//     }
// }


// impl<'f> FromForm<'f> for LoginCont<AdministratorForm> {
//     type Error = &'static str;
//     
//     fn from_form(form_items: &mut FormItems<'f>, _strict: bool) -> Result<Self, Self::Error> {
//         // let mut user_pass = HashMap::new();
//         let mut user: String = String::new();
//         let mut pass: String = String::new();
//         // let mut pass: Vec<u8> = Vec::new();
//         let mut extras: HashMap<String, String> = HashMap::new();
//         
//         for (key,value) in form_items {
//             match key.as_str(){
//                 "username" => {
//                     // user = sanitize(&value.url_decode().unwrap_or(String::new()));
//                     user = AdministratorForm::clean_username(&value.url_decode().unwrap_or(String::new()));
//                 },
//                 "password" => {
//                     // pass = sanitize_password(&value.url_decode().unwrap_or(String::new()));
//                     pass = AdministratorForm::clean_password(&value.url_decode().unwrap_or(String::new()));
//                     // pass = value.bytes().collect();
//                 },
//                 // _ => {},
//                 a => {
//                     // extras.insert( a.to_string(), sanitize( &value.url_decode().unwrap_or(String::new()) ) );
//                     extras.insert( a.to_string(), AdministratorForm::clean_extras( &value.url_decode().unwrap_or(String::new()) ) );
//                 },
//             }
//         }
//         
//         // println!("Creating login form data structure with:\nUser: {}\nPass: {}\nExtras: {:?}", user, pass, extras);
//         
//         // Do not need to check for username / password here,
//         // if the authentication method requires them it will
//         // fail at that point.
//         Ok(
//             LoginCont {
//                 form: if extras.len() == 0 {
//                           AdministratorForm::new_form(&user, &pass, None)
//                        } else {
//                            AdministratorForm::new_form(&user, &pass, Some(extras))
//                        },
//             }
//         )
//     }
// }







