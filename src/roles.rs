
use ral::authorization;
use ral::authorization::*;
use ral::sanitization::*;

use rocket::{Request, Outcome};
use rocket::request::{FromRequest, FromForm, FormItems};
// use rocket::http::{Cookie, Cookies};
use rocket::http::Cookies;
use std::collections::HashMap;
use std::str::{from_utf8};
use rocket::response::{Redirect, Flash};

use super::PGCONN;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Admin {
    pub userid: u32,
    pub username: String,
    pub display: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminLogin {
    pub username: String,
    password: String,
}

impl CookieId for Admin {
    fn cookie_id<'a>() -> &'a str {
        "acid"
    }
}

impl CookieId for AdminLogin {
    fn cookie_id<'a>() -> &'a str {
        "acid"
    }
}

impl AuthorizeCookie for Admin {
    
    fn store_cookie(&self) -> String {
        ::serde_json::to_string(self).expect("Could not serialize")
    }
    
    #[allow(unused_variables)]
    fn retrieve_cookie(string: String) -> Option<Self> {
        let mut des_buf = string.clone();
        let des: Result<Admin, _> = ::serde_json::from_str(&mut des_buf);
        if let Ok(cooky) = des {
            Some(cooky)
        } else {
            None
        }
    }
}

impl AuthorizeForm for AdminLogin {
    type CookieType = Admin;
    
    /// Authenticate the credentials inside the login form
    fn authenticate(&self) -> Result<Self::CookieType, AuthFail> {
        let conn = PGCONN.lock().unwrap();
        let authstr = format!(r#"
            SELECT u.userid, u.username, u.display FROM users u WHERE u.username = '{username}' AND 
                u.hash_salt = crypt('{password}', u.hash_salt)"#, username=&self.username, password=&self.password);
            // , 'LATIN1')"#, username=&self.username, password=sanitize_password(from_utf8(&self.password).unwrap_or("")));
        // let qrystr = format!("SELECT userid, username, display,  FROM users WHERE username = '{}' AND password = '{}' AND is_admin = '1'", &self.username, &self.password);
        let is_user_qrystr = format!("SELECT userid FROM users WHERE username = '{}'", &self.username);
        let is_admin_qrystr = format!("SELECT userid FROM users WHERE username = '{}' AND is_admin = '1'", &self.username);
        // let password_qrystr = format!("SELECT userid FROM users WHERE username = '{}' AND password = '{}'", &self.username, &self.password);
        let password_qrystr = format!("SELECT u.userid FROM users u WHERE u.username = '{}' AND u.hash_salt = crypt('{}', u.hash_salt)", &self.username, &self.password);
        // let password_qrystr = format!("SELECT userid FROM users WHERE username = '{}' AND password = '{}'", &self.username, from_utf8(&self.password).unwrap_or(""));
        println!("Attempting query: {}", authstr);
        // if let Ok(qry) = conn.query(&qrystr, &[]) {
        if let Ok(qry) = conn.query(&authstr, &[]) {
            if !qry.is_empty() && qry.len() == 1 {
                let row = qry.get(0);
                
                let display_opt = row.get_opt(2);
                let display = match display_opt {
                    Some(Ok(d)) => Some(d),
                    _ => None,
                };
                
                return Ok(Admin {
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
    
    /// Create a new login form instance
    fn new_form(user: &str, pass: &str, _extras: Option<HashMap<String, String>>) -> Self {
        AdminLogin {
            username: user.to_string(),
            password: pass.to_string(),
        }
    }
    
    // /// Define a custom flash_redirect() method that overrides the default
    // /// implementation in authorization::AuthorizeForm trait.
    // /// This allows the cookie to be made secure
    // fn flash_redirect(&self, ok_redir: &str, err_redir: &str, mut cookies: Cookies) -> Result<Redirect, Flash<Redirect>> {
    //     match self.authenticate() {
    //         Ok(cooky) => {
    //             let cid = Self::cookie_id();
    //             let contents = cooky.store_cookie();
    //             cookies.add_private(
    //                 Cookie::build(cid, contents)
    //                     // .secure(true)
    //                     .finish()
    //             );
    //             Ok(Redirect::to(ok_redir))
    //         },
    //         Err(fail) => {
    //             let mut furl = String::from(err_redir);
    //             if &fail.user != "" {
    //                 let furl_qrystr = Self::fail_url(&fail.user);
    //                 furl.push_str(&furl_qrystr);
    //             }
    //             Err( Flash::error(Redirect::to(&furl), &fail.msg) )
    //         },
    //     }
    // }
}


impl<'a, 'r> FromRequest<'a, 'r> for Admin {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> ::rocket::request::Outcome<Admin,Self::Error>{
        let cid = Admin::cookie_id();
        let mut cookies = request.cookies();
        
        match cookies.get_private(cid) {
            Some(cookie) => {
                if let Some(cookie_deserialized) = Admin::retrieve_cookie(cookie.value().to_string()) {
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


impl<'f> FromForm<'f> for AdminLogin {
    type Error = &'static str;
    
    fn from_form(form_items: &mut FormItems<'f>, _strict: bool) -> Result<AdminLogin, Self::Error> {
        let mut user: String = String::new();
        let mut pass: String = String::new();
        let mut extras: HashMap<String, String> = HashMap::new();
        
        for (key,value) in form_items {
            match key.as_str(){
                "username" => {
                    user = AdminLogin::clean_username(&value.url_decode().unwrap_or(String::new()));
                },
                "password" => {
                    pass = AdminLogin::clean_password(&value.url_decode().unwrap_or(String::new()));
                },
                a => {
                    extras.insert( a.to_string(), AdminLogin::clean_extras( &value.url_decode().unwrap_or(String::new()) ) );
                },
            }
        }
        
        if extras.len() == 0 {
            AdminLogin::new_form(&user, &pass, None)
        } else {
            AdminLogin::new_form(&user, &pass, Some(extras))
        }
        
        // Ok(
        //     LoginCont {
        //         form: if extras.len() == 0 {
        //                   A::new_form(&user, &pass, None)
        //                } else {
        //                    A::new_form(&user, &pass, Some(extras))
        //                },
        //     }
        // )
    }
}






















