
use rocket::{Request, Outcome};
// use rocket::request::FromRequest;
use rocket::request::{FromRequest, FromForm, FormItems};
use std::collections::HashMap;
use std::str::{from_utf8};
use chrono::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};

use super::PGCONN;
// use password::*;
use rocket_auth_login::authorization::*;
use rocket_auth_login::sanitization::*;
// use auth::sanitization::*;


const MAX_ATTEMPTS: i16 = 8;
const LOCKOUT_DURATION: u32 = 6; // 900 seconds = 15 minutes


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCookie {
    pub userid: u32,
    pub username: String,
    pub display: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserForm {
    pub username: String,
    pub password: String,
}

impl CookieId for UserCookie {
    fn cookie_id<'a>() -> &'a str {
        "ucid"
    }
}

impl CookieId for UserForm {
    fn cookie_id<'a>() -> &'a str {
        "ucid"
    }
} 

impl AuthorizeCookie for UserCookie {
    fn store_cookie(&self) -> String {
        ::serde_json::to_string(self).expect("Could not serialize")
    }
    
    #[allow(unused_variables)]
    fn retrieve_cookie(string: String) -> Option<Self> {
        let mut des_buf = string.clone();
        let des: Result<UserCookie, _> = ::serde_json::from_str(&mut des_buf);
        if let Ok(cooky) = des {
            Some(cooky)
        } else {
            None
        }
    }
}

impl AuthorizeForm for UserForm {
    type CookieType = UserCookie;
    
    fn authenticate(&self) -> Result<Self::CookieType, AuthFail> {

        let conn = PGCONN.lock().unwrap();
        
        // Could make two queries: authstr and failstr
        // authstr is same as is, gets user data
        // failstr returns the username, is_admin, a bool indicating whether password was correct,
        //     and the attempts and lockout status/date
        
        let authstr = format!(r#"
            SELECT u.userid, u.username, u.display FROM users u WHERE u.username = '{username}' AND 
                u.hash_salt = crypt('{password}', u.hash_salt) AND lockout IS NULL"#, username=&self.username, password=&self.password);
        
        let lockout_qrystr = format!("SELECT u.username, u.attempts, u.lockout, LOCALTIMESTAMP as now, crypt('{pass}', u.hash_salt) = u.hash_salt as check FROM users u WHERE u.username = '{user}' AND u.lockout IS NOT NULL", user=&self.username, pass=&self.password);
        
        // let is_user_qrystr = format!("SELECT username, attempts, lockout, LOCALTIMESTAMP as now, LOCALTIMESTAMP + interval '{lock_duration} seconds' as lock_duration  FROM users WHERE username = '{user}'", user=&self.username, lock_duration=LOCKOUT_DURATION);
        let is_user_qrystr = format!("SELECT username, attempts, lockout FROM users WHERE username = '{}'", &self.username);
        // let is_admin_qrystr = format!("SELECT userid FROM users WHERE username = '{}' AND is_admin = '1'", &self.username);
        let password_qrystr = format!("SELECT username, attempts FROM users WHERE username = '{}' AND hash_salt = crypt('{}', hash_salt)", &self.username, &self.password);
        
        println!("Running: {}", authstr);
        
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
                
                let reset_attempts = format!("UPDATE users SET attempts = 0 WHERE username = '{}'", &self.username);
                conn.query(&reset_attempts, &[]);
                
                return Ok(UserCookie {
                    userid: row.get(0),
                    username: row.get(1),
                    display,
                });
            }
        }
        
        // Check if the user is locked out
        if let Ok(eqry) = conn.query(&lockout_qrystr, &[]) {
            if !eqry.is_empty() && eqry.len() != 0 {
                println!("User has been locked out!!  Query ran: {}", lockout_qrystr);
                let row = eqry.get(0);
                let username: String = row.get(0);
                let attempts: i16 = row.get(1);
                let lockout_opt: Option<NaiveDateTime> = row.get(2);
                let lockout = lockout_opt.expect("Error unwrapping lockout value");
                let now: NaiveDateTime = row.get(3);
                let valid: bool = row.get(4);
                
                // let now = Local::now().naive_local();
                
                // if the lockout has expired unlock the account but do not reset the attempts
                if now > lockout {
                    // do not increment attempt it will be incremented when calling authenticate() again
                    println!("Lockout has expired, valid: {}", valid);
                    
                    if valid {
                        let unlock_qrystr = format!("UPDATE users SET lockout = NULL, attempts = 0 WHERE username = '{}' RETURNING userid, username, display", &self.username);
                        println!("Lockout has expired, credentials valid, running: {}", unlock_qrystr);
                        if let Ok(aqry) = conn.query(&unlock_qrystr, &[]) {
                            if !aqry.is_empty() && aqry.len() == 1 {
                                let row = aqry.get(0);
                                let display_opt = row.get_opt(2);
                                let display = match display_opt {
                                    Some(Ok(d)) => Some(d),
                                    _ => None,
                                };
                                return Ok(UserCookie {
                                    userid: row.get(0),
                                    username: row.get(1),
                                    display,
                                });
                            } else {
                                return Err(AuthFail::new(self.username.clone(), "Account unlocked, unknown error occurred 1.".to_string()));
                            }
                        } else {
                            return Err(AuthFail::new(self.username.clone(), "Account unlocked, unknown error occurred 2.".to_string()));
                        }
                    } else {
                        let unlock_qrystr = format!("UPDATE users SET lockout = NULL WHERE username = '{}'", &self.username);
                        
                        println!("Lockout has expired. Running: {}", unlock_qrystr);
                        conn.query(&unlock_qrystr, &[]);
                        
                        return Err(AuthFail::new(self.username.clone(), "Account Unlocked.  Invalid username / password combination.".to_string()));
                    }
                } else {
                    println!("User account is still locked!");
                    return Err( AuthFail::new(self.username.clone(), "User has been locked due to excessive login attempts.  Please try again later.".to_string()) );
                }
                
            }
        }
        
        let mut attempts: i16 = 0;
        
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
        
        attempts += 1;
        println!("{} have made {} attempts to login.", &self.username, attempts);
        // Check the remainder of attempts divided by MAX_ATTEMPTS
        // If the result is 0 that means they have already tried the maximum number of attempts
        //   before the user account is locked
        // The remainder is used so the total attempts can be tracked without having to reset it
        //   after each lockout has ended
        let attempt_qrystr = if attempts % MAX_ATTEMPTS == 0 {
            // match attempts {
            //     
            // }
            
            let inc_qrystr = format!("UPDATE users SET attempts = attempts+1, lockout = LOCALTIMESTAMP + interval '{lockout}' WHERE username = '{user}'", user=&self.username, lockout=LOCKOUT_DURATION);
            println!("Running query to lockout the user and increment attempts: {}", &inc_qrystr);
            conn.query(&inc_qrystr, &[]);
        } else {
            let inc_qrystr = format!("UPDATE users SET attempts = attempts+1 WHERE username = '{}'", &self.username);
            println!("Running query to increment attempts: {}", &inc_qrystr);
            conn.query(&inc_qrystr, &[]);
        };
        // Check if the password is correct
        if let Ok(eqry) = conn.query(&password_qrystr, &[]) {
            if eqry.is_empty() || eqry.len() == 0 {
                return Err(AuthFail::new(self.username.clone(), "Invalid username / password combination.".to_string()));
            }
        }
        Err(AuthFail::new(self.username.clone(), "Unknown error..".to_string()))
    
        
        
        // let conn = PGCONN.lock().unwrap();
        // let authstr = format!(r#"
        //     SELECT u.userid, u.username, u.display FROM users u WHERE u.username = '{username}' AND 
        //         u.hash_salt = crypt('{password}', u.hash_salt)"#, username=&self.username, password=&self.password);
        // let is_user_qrystr = format!("SELECT userid FROM users WHERE username = '{}'", &self.username);
        // let password_qrystr = format!("SELECT userid FROM users WHERE username = '{}' AND hash_salt = crypt('{}', hash_salt)", &self.username, &self.password);
        // println!("Running: {}", authstr);
        // if let Ok(qry) = conn.query(&authstr, &[]) {
        //     if !qry.is_empty() && qry.len() == 1 {
        //         let row = qry.get(0);
                
        //         let display_opt = row.get_opt(2);
        //         let display = match display_opt {
        //             Some(Ok(d)) => Some(d),
        //             _ => None,
        //         };
        //         return Ok(UserCookie {
        //             userid: row.get(0),
        //             username: row.get(1),
        //             display,
        //         });
        //     }
        // }
        // if let Ok(eqry) = conn.query(&is_user_qrystr, &[]) {
        //     if eqry.is_empty() || eqry.len() == 0 {
        //         return Err(AuthFail::new(self.username.clone(), "Username was not found.".to_string()));
        //     }
        // }
        // if let Ok(eqry) = conn.query(&password_qrystr, &[]) {
        //     if eqry.is_empty() || eqry.len() == 0 {
        //         return Err(AuthFail::new(self.username.clone(), "Invalid username / password combination.".to_string()));
        //     }
        // }
        // Err(AuthFail::new(self.username.clone(), "Unknown error..".to_string()))
        
    }
    
    fn new_form(user: &str, pass: &str, _extras: Option<HashMap<String, String>>) -> Self {
        UserForm {
            username: user.to_string(),
            password: pass.to_string(),
        }
    }
    
}

impl<'a, 'r> FromRequest<'a, 'r> for UserCookie {
    type Error = ();
    
    fn from_request(request: &'a Request<'r>) -> ::rocket::request::Outcome<UserCookie,Self::Error>{
        let cid = UserCookie::cookie_id();
        let mut cookies = request.cookies();
        
        match cookies.get_private(cid) {
            Some(cookie) => {
                if let Some(cookie_deserialized) = UserCookie::retrieve_cookie(cookie.value().to_string()) {
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








