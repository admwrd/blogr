
use rocket;
use rocket::{Request, Data, Outcome, Response};
use ::rocket::config::{Config, Environment};
use rocket::data::FromData;
use rocket::http::{Cookie, Cookies, MediaType, ContentType, Status, RawStr};
// use ::rocket::outcome::Outcome;
use rocket::request::{FlashMessage, Form, FromRequest,FromForm, FormItems, FromFormValue, FromParam};
use rocket::response::{content, NamedFile, Redirect, Flash, Responder, Content};
use rocket::response::content::Html;


use authorize::*;


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
        "acid"
    }
}

impl CookieId for AdministratorForm {
    fn cookie_id<'a>() -> &'a str {
        "acid"
    }
} 


impl AuthorizeCookie for AdministratorCookie {
    // The store and retrieve methods are implemented automatically thanks to default methods
    
    // fn store(&self) -> String {
        
    // }
    // fn retrieve(data: String) -> Self {
        
    // }
}

impl AuthorizeForm for AdministratorForm {
    type CookieType = AdministratorCookie;
    // fn authenticate_user(username: &str, password: &str) -> Result<CookieType, (String, String)> {
    fn authenticate(&self) -> Result<Self::CookieType, (String, String)> {
        if &self.username == "andrew" && &self.password != "" {
            Ok(
                AdministratorCookie {
                    userid: 1,
                    username: "andrew",
                    display: "Andrew",
                }
            )
        } else {
            Err(self.username.to_string(), "Incorrect username".to_string())
        }
    }
    // fn fail_url(&self, msg: &str) -> String {
    fn new_form(user: &str, pass: &str) -> Self {
        AdministratorForm {
            username: user,
            password: pass, 
        }
    }
}

impl<'a,'r> FromRequest<'a, 'r> for AdministratorCookie {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<AdministratorCookie,Self::Error>{
        // let cookie_id = config::get_cookie_identifier();
        // let mut cookies = request.cookies();
        
        let cid = Self::cookie_id();
        let mut cookies = request.cookies();
        
        // match cookies.get_private(&cookie_id){
            // Some(cookie) => Outcome::Success(UserPass{user: T::from_string(cookie.value().to_string())}),
            // None => Outcome::Forward(())
        // }
        match cookies.get_private(&cid){
            Some(cookie) => Outcome::Success(
                // UserPass{user: T::from_string(cookie.value().to_string())}
                Self::retrieve(cookie.value().to_string())
                // AdministratorCookie {
                //     userid: 99,
                //     username: "andrews".to_string(),
                //     display: None,
                // }
            ),
            None => Outcome::Forward(())
        }
    }
}















