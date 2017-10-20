// use cookies::*;

use std::env;

use rocket::Response;
use rocket::response::Redirect;
use rocket::response::Responder;
use rocket::request::{FormItems, FromForm, Request};
use rocket::http::{Status, Cookie, Cookies};
use std::collections::HashMap;
use auth::authenticator::Authenticator;

use regex::Regex;

use ::cookie_data::*;



#[derive(FromForm)]
// #[form(lenient)]
pub struct AuthFailure {
    pub user: String,
    pub msg: String,
}

/// Login state is used after the user has typed its username and password. It checks with an
/// authenticator if given credentials are valid and returns InvalidCredentials or Succeed based
/// on the validality of the username and password.
///
/// It does that by implementing the FromForm trait that takes the form submitted by your login
/// page
///
/// It expects a form like this on your page:
///
///```
///<form>
/// <input type="text" name="username" />
/// <input type="password" name="password" />
///</form>
/// ```
#[derive(Clone)]
pub enum LoginFormStatus<A>{
    Succeed(A),
    Failed(A)
}


pub trait AuthFail {
    fn reason(&self) -> String;
    // fn reason_str(&self) -> &str;
}

// #[derive(Clone)]
pub struct LoginFormRedirect(Redirect);

// pub struct FallbackRedirect<'a>(&'a str);



pub fn sanitize(string: &str) -> String {
    lazy_static! {
        static ref SANITARY: Regex = Regex::new(r#"^\w+$"#).unwrap();
        static ref SANITIZE: Regex = Regex::new(r#"\W+"#).unwrap();
    }
    if SANITARY.is_match(string) {
        string.to_string()
    } else {
        SANITIZE.replace_all(string, "").to_string()
    }
}

pub fn sanitize_password(string: &str) -> String {
    lazy_static! {
        static ref SANITARY_PASSWORD: Regex = Regex::new(r#"^[A-Fa-f0-9]+$"#).unwrap();
        static ref SANITIZE_PASSWORD: Regex = Regex::new(r#"[^A-Fa-f0-9]+"#).unwrap();
    }
    if SANITARY_PASSWORD.is_match(string) {
        string.to_string()
    } else {
        SANITIZE_PASSWORD.replace_all(string, "").to_string()
    }
}

impl<'a, A: 'a> LoginFormStatus<A> where A: Authenticator + CookieId + AuthFail {
    pub fn fail_str(&self) -> String {
        // let 
        match self {
            &LoginFormStatus::Succeed(ref inside) => inside.reason(),
            &LoginFormStatus::Failed(ref inside) => inside.reason(),
        }
    }
    pub fn user_str(&self) -> String {
        match self {
            &LoginFormStatus::Succeed(ref inside) => inside.user().to_string(),
            &LoginFormStatus::Failed(ref inside) => inside.user().to_string(),
        }
    }
    /// Returns the user id from an instance of Authenticator
    pub fn get_authenticator (&self) -> &A{
        match self{
            &LoginFormStatus::Succeed(ref authenticator) => authenticator,
            &LoginFormStatus::Failed(ref authenticator) => authenticator
        }
    }

    /// Generates a succeed response
    // fn succeed(self, url: &str, mut cookies: Cookies) -> Redirect {
    fn succeed(self, url: &'static str, mut cookies: Cookies) -> Redirect {
        let cookie_identifier = A::get_cookie_id();

        cookies.add_private(Cookie::new(cookie_identifier, self.get_authenticator().user().to_string()));
        Redirect::to(url)
    }

    /// Generates a failed response
    // fn failed(self, url: &str) -> Redirect {
    fn failed(self, url: &'static str) -> Redirect {
        // match env::var(url) {
        //     Ok(val) => Redirect::to(&val),
        //     Err(er) => Redirect::to(&fallback),
        // }
        Redirect::to(url)
    }

    // pub fn redirect(self, success_url: &'static str, failure_url: &'static str, cookies: Cookies) -> LoginFormRedirect{
    pub fn redirect(self, success_url: &'static str, cookies: Cookies) -> Option<LoginFormRedirect> {
        // let redirect = match self {
        /*match self {
          // LoginFormStatus::Succeed(_) => self.succeed(success_url, cookies),
          // LoginFormStatus::Failed(_) => self.failed(failure_url)
          // LoginFormStatus::Succeed(_) => Some(LoginFormRedirect(self.succeed(success_url, cookies))),
          LoginFormStatus::Succeed(_) => Some( LoginFormRedirect::new(self.succeed(success_url, cookies)) ),
          LoginFormStatus::Failed(_) => None,
          // LoginFormStatus::Failed(_) => self.failed(failure_url, fallback)
        }*/
        
        if let LoginFormStatus::Succeed(_) = self {
            let redir = self.succeed(success_url, cookies);
            Some(LoginFormRedirect(redir))
        } else {
            None
        }
        
        // Some(LoginFormRedirect(redirect))
    }
}

impl<'f,A: Authenticator> FromForm<'f> for LoginFormStatus<A>{
    type Error = &'static str;
    
    fn from_form(form_items: &mut FormItems<'f>, _strict: bool) -> Result<Self, Self::Error> {
        // let mut user_pass = HashMap::new();
        let mut user = String::new();
        let mut pass = String::new();
        
        
        for (key,value) in form_items{
            match key.as_str(){
                // "username" => user_pass.insert("username", value).map_or((), |_v| ()),
                // "password" => user_pass.insert("password", value).map_or((), |_v| ()),
                "username" => { user = sanitize(&value.to_string()) },
                // "username" => { user = sanitize(&value.url_decode().to_string()) },
                "password" => { pass = sanitize_password(&value.to_string()) },
                _ => ()
            }
        }

        // if user_pass.get("username").is_none() || user_pass.get("password").is_none() {
        if user == "" || pass == "" {
            Err("Authentication error: Blank credential fields")
        } else {
            // let result = A::check_credentials(user_pass.get("username").unwrap().to_string(), user_pass.get("password").unwrap().to_string());
            let result = A::check_credentials(user, pass);

            Ok(match result{
                Ok(authenticator) => LoginFormStatus::Succeed(authenticator),
                Err(authenticator) => LoginFormStatus::Failed(authenticator),
            })
        }
    }
}

impl LoginFormRedirect {
    pub fn new(r: Redirect) -> LoginFormRedirect {
        LoginFormRedirect(r)
    }
}

impl<'r> Responder<'r> for LoginFormRedirect{
    fn respond_to(self, request: &Request) -> Result<Response<'r>, Status>{
        self.0.respond_to(request)
    }
}

// impl<'r, 'a> Responder<'r> for FallbackRedirect<'a> {
//     fn respond_to(self, request: &Request) -> Result<Response<'r>, Status> {
//         self.0.
//     }
// }
