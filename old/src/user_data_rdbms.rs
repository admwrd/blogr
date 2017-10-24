
use cookie_data::*;

use chrono::prelude::*;

#[derive(Debug, Clone)]
pub struct UserData {
    pub username: String,
    pub display_name: String,
    pub email: String,
    pub show_email: bool,
    // pub can_post: bool,
    pub admin: bool,
    // pub joined: DateTime<Utc>,
    
}

impl UserData {
    // Accepts any type that implements CookieId, like:
    //   AdminAuth and UserAuth and AdminCookie and User Cookie
    pub fn fetchUser<A: CookieId>(cooky: A) -> UserData {
        // let user = cooky.cookie_username();
        unimplemented!()
    }
}




