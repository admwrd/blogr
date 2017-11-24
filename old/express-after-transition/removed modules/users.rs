
use cookie_data::*;

use chrono::prelude::*;
// use diesel::prelude::*;


#[derive(Debug, Clone)]
pub struct User {
    pub userid: u32,
    pub username: String,
    pub display: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
}

impl User {
    // Accepts any type that implements CookieId, like:
    //   AdminAuth and UserAuth and AdminCookie and User Cookie
    pub fn fetchUser<A: CookieId>(cooky: A) -> User {
        // let user = cooky.cookie_username();
        unimplemented!()
    }
}




