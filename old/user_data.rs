
use cookie_data::*;

use chrono::prelude::*;

#[derive(Debug, Clone)]
pub struct UserData {
    pub userid: u32,
    pub username: String,
    pub display: String,
    pub email: String,
    pub password: String,
    pub is_admin: bool,
}

impl UserData {
    // Accepts any type that implements CookieId, like:
    //   AdminAuth and UserAuth and AdminCookie and User Cookie
    pub fn fetchUser<A: CookieId>(cooky: A) -> UserData {
        // let user = cooky.cookie_username();
        unimplemented!()
    }
}




