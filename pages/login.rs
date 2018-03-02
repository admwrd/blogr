
use rocket::{Request, Outcome};
use rocket::request::{FromRequest, FromForm, FormItems};
use std::collections::HashMap;
use std::str::{from_utf8};
use chrono::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};

use rocket::response::{Redirect, Flash};
use rocket::request::FlashMessage;
use rocket::http::{Cookie, Cookies, RawStr};

use rocket_auth_login::authorization::*;
use rocket_auth_login::sanitization::*;

// Specifies whether the app is running in dev mode or production mode
// Production mode will use HTTPS
const PRODUCTION: bool = false;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminCookie {
    pub userid: u32,
    pub username: String,
    pub display: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminForm {
    pub username: String,
    pub password: String,
}

impl CookieId for AdminCookie {
    fn cookie_id<'a>() -> &'a str {
        // The name the cookie should use.
        // If you have multiple user types this should be a 
        // different value for each user type.
        "acid"
    }
}

impl CookieId for AdminForm {
    fn cookie_id<'a>() -> &'a str {
        // This value should match the cookie's ID 
        // same value used in AdminCookie::cookie_id()
        "acid"
    }
} 

impl AuthorizeCookie for AdminCookie {
    fn store_cookie(&self) -> String {
        ::serde_json::to_string(self).expect("Could not serialize")
    }
    
    
    #[allow(unused_variables)]
    fn retrieve_cookie(string: String) -> Option<Self> {
        let mut des_buf = string.clone();
        let des: Result<AdminCookie, _> = ::serde_json::from_str(&mut des_buf);
        if let Ok(cooky) = des {
            Some(cooky)
        } else {
            None
        }
    }
}

impl AuthorizeForm for AdminForm {
    type CookieType = AdminCookie;
    
    fn authenticate(&self) -> Result<Self::CookieType, AuthFail> {
        let conn = PGCONN.lock().unwrap();
        
        let authstr = format!(r#"
            SELECT u.userid, u.username, u.display FROM users u WHERE u.username = '{username}' AND is_admin = true AND 
                u.hash_salt = crypt('{password}', u.hash_salt) AND lockout IS NULL"#, username=&self.username, password=&self.password);
        
        let is_user_qrystr = format!("SELECT username, attempts, lockout FROM users WHERE username = '{}'", &self.username);
        let is_admin_qrystr = format!("SELECT userid FROM users WHERE username = '{}' AND is_admin = '1'", &self.username);
        let password_qrystr = format!("SELECT username, attempts FROM users WHERE username = '{}' AND hash_salt = crypt('{}', hash_salt)", &self.username, &self.password);
        
        // Checking to see if user credentials are valid
        // does not work if user is locked out
        if let Ok(qry) = conn.query(&authstr, &[]) {
            if !qry.is_empty() && qry.len() == 1 {
                let row = qry.get(0);
                
                let display_opt = row.get_opt(2);
                let display = match display_opt {
                    Some(Ok(d)) => Some(d),
                    _ => None,
                };
                
                return Ok(AdminCookie {
                    userid: row.get(0),
                    username: row.get(1),
                    display,
                });
            }
        }
        
        // Check if the specified username is an actual user account
        // if not return an error
        // otherwise get the number of attempts that have been made to login to the account
        if let Ok(eqry) = conn.query(&is_user_qrystr, &[]) {
            if eqry.is_empty() || eqry.len() == 0 {
                return Err(AuthFail::new(self.username.clone(), "Username was not found.".to_string()));
            } else {
                let row = eqry.get(0);
                attempts = row.get(1);
            }
        }
        
        // Check if the user is an administrator
        if let Ok(eqry) = conn.query(&is_admin_qrystr, &[]) {
            if eqry.is_empty() || eqry.len() == 0 {
                // In production this message may be more harmful than useful as it
                // would be able to tell anyone who is an administrator and thus the
                // message should be changed to something like Unkown error or Invalid username/password
                return Err(AuthFail::new(self.username.clone(), "User does not have administrator priveleges.".to_string()));
            }
        }
        
        // Check if the password is correct
        if let Ok(eqry) = conn.query(&password_qrystr, &[]) {
            if eqry.is_empty() || eqry.len() == 0 {
                return Err(AuthFail::new(self.username.clone(), "Invalid username / password combination.".to_string()));
            }
        }
        Err(AuthFail::new(self.username.clone(), "Unknown error..".to_string()))
    }
    
    fn flash_redirect(&self, ok_redir: &str, err_redir: &str, cookies: &mut Cookies) -> Result<Redirect, Flash<Redirect>> {
        match self.authenticate() {
            Ok(cooky) => {
                let cid = Self::cookie_id();
                let contents = cooky.store_cookie();
                
                // Secure cookie while in production mode
                let new_cookie = if PRODUCTION == true {
                    Cookie::build(cid, contents)
                        .secure(true)
                        .finish()
                } else {
                    Cookie::build(cid, contents)
                        // .secure(true)
                        .finish()
                };
                cookies.add_private(
                    new_cookie
                );
                Ok(Redirect::to(ok_redir))
            },
            Err(fail) => {
                // let mut furl = String::from(err_redir);
                let mut furl = String::with_capacity(err_redir.len() + fail.user.len() + 20);
                furl.push_str(err_redir);
                if &fail.user != "" {
                    let furl_qrystr = if err_redir.contains("?") {
                        let mut fail_temp = String::with_capacity(fail.user.len() + 20);
                        fail_temp.push_str("&user=");
                        fail_temp.push_str(&fail.user);
                        fail_temp
                    } else {
                        Self::fail_url(&fail.user)
                    };
                    furl.push_str(&furl_qrystr);
                }
                Err( Flash::error(Redirect::to(&furl), &fail.msg) )
            },
        }
    }
    
    
    fn new_form(user: &str, pass: &str, _extras: Option<HashMap<String, String>>) -> Self {
        AdminForm {
            username: user.to_string(),
            password: pass.to_string(),
            referrer: if let Some(xtra) = _extras {
                if let Some(refer) = xtra.get("referrer") { 
                    refer.to_string()
                } else { 
                    String::new() 
                }
            } else {
                String::new()
            },
        }
    }
    
}

impl<'a, 'r> FromRequest<'a, 'r> for AdminCookie {
    type Error = ();
    
    fn from_request(request: &'a Request<'r>) -> ::rocket::request::Outcome<AdminCookie,Self::Error>{
        let cid = AdminCookie::cookie_id();
        let mut cookies = request.cookies();
        
        match cookies.get_private(cid) {
            Some(cookie) => {
                if let Some(cookie_deserialized) = AdminCookie::retrieve_cookie(cookie.value().to_string()) {
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






