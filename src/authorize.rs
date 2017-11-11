
use std::{env, str, io};
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::marker::PhantomData;

use rocket;
use rocket::{Request, Data, Outcome, Response};
use ::rocket::config::{Config, Environment};
use rocket::data::FromData;
use rocket::http::{Cookie, Cookies, MediaType, ContentType, Status, RawStr};
// use ::rocket::outcome::Outcome;
use rocket::request::{FlashMessage, Form, FromRequest,FromForm, FormItems, FromFormValue, FromParam};
use rocket::response::{content, NamedFile, Redirect, Flash, Responder, Content};
use rocket::response::content::Html;

use regex::Regex;
use titlecase::titlecase;
use ::serde::{Deserialize, Serialize};
use ::rmps::{Deserializer, Serializer};

use auth::userpass::UserPass;
use auth::status::{LoginStatus,LoginRedirect};
use auth::dummy::DummyAuthenticator;
use auth::authenticator::Authenticator;

use sanitize::*;

// use auth::userpass::UserPass;
// use auth::status::{LoginStatus,LoginRedirect};
// use auth::dummy::DummyAuthenticator;
// use auth::authenticator::Authenticator;


// FromRequest - retrieve cookie data
// FromForm - retrieve login data and authenticate and create cookie if authenticated

/// When using it for checking if a user is an administrator
/// use: AuthContainer::CookieData<Administrator> where Administrator is a
/// data structure that contains the administrator cookie data

#[derive(Debug, Clone)]
pub struct AuthContainer<'de, T: AuthorizeCookie + Serialize + Deserialize<'de> + 'de> {
    pub cookie: T,
    _marker: PhantomData<&'de T>,
}

#[derive(Debug, Clone)]
pub struct LoginContainer<'de, T: AuthorizeForm + Serialize + Deserialize<'de> + 'de> {
    pub form: T,
    _marker: PhantomData<&'de T>,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum AuthContainer<T: AuthorizeCookie+CookieId, U: CookieId> {
//     CookieData(T),
//     FormData(U),
// }


// pub trait StringConversion {
//     fn string_decode(s: String) -> Self;
//     fn string_encode(&self) -> String;
// }

pub trait CookieId {
    fn identifier(&self) -> &str {
        Self::cookie_id()
    }
    fn cookie_id<'a>() -> &'a str {
        "sid" // default identifier
    }
}

pub trait AuthorizeCookie : CookieId {
    // type FormType;
    // type CookieType;
    
    /// Convert a data structure to a string
    ///
    /// The default implementation uses Serde to serialize/deserialize the structure using MsgPack
    fn store(&self) -> String 
        where Self: ::serde::Serialize 
    {
        let mut buffer = Vec::new();
        self.serialize(&mut Serializer::new(&mut buffer)).expect("Could not store authorization data.");
        // str::from_utf8(&sparkle_heart).expect("Invalid UTF characters found in authorization data.")
        str::from_utf8(&buffer).expect("Invalid UTF characters found in authorization data.")
    }
    
    /// Takes a string and converts it to a useful data structure
    fn retrieve(data: String) -> Self
        where for <'de> Self: ::serde::Deserialize<'de> + ::serde::Serialize + ::std::marker::Sized
    {
        let buffer: Vec<u8> = data.as_bytes();
        let mut de = Deserializer::new(&buffer[..]);
        let output: Self = Deserialize::deserialize(&mut de).expect("Could not decode authorization data.");
        output
    }
    
}
pub trait AuthorizeForm : CookieId {
    
    /// The data structure to store as a cookie
    type CookieType;
    
    /// Authenticate user the trait method that authenticates a login form.  
    /// It determines whether the username and password field are valid credentials
    /// If so it returns a Ok<CookieType> otherwise it returns Err<String,String>
    /// 
    /// The Ok result is the cookie, so if your method queries a database and is successful 
    ///     it may return a data structure containing the user data like username email etc.
    /// 
    /// Otherwise Err<String,String> is returned; the first string is the Username the user entered
    ///     this is so that the login form may display the username that was submitted, 
    ///     this is to prevent the user from having to enter the username again
    /// The second string returned is the error message returned, or the reason why the
    ///     authentication failed.  An example could be that the username was not found
    ///     or perhaps invalid password for the specified user.
    /// 
    fn authenticate(&self) -> Result<Self::CookieType, (String, String)>;
    
    /// Creates a url that the user should be directed to when the login fails
    /// The authenticate method will return an error message, this is passed to fail_url()
    /// This only creates the "?some=thing" part, the "/page?" part is created by caller function
    // fn fail_url(&self, &str) -> String;
    fn fail_url(&str, &str) -> String;
    
    fn new_form(user: &str, pass: &str) -> Self;
}



impl<'de, 'a, 'r, T: AuthorizeCookie> FromRequest<'a, 'r> for AuthContainer<'de, T> where T: ::serde::Serialize + ::serde::Deserialize<'de> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<AuthContainer<'de, T>,Self::Error>{
        // let cookie_id = config::get_cookie_identifier();
        
        // let cid = AuthContainer.cookie::cookie_id();
        let cid = T::cookie_id();
        let mut cookies = request.cookies();

        match cookies.get_private(cid) {
            Some(cookie) => Outcome::Success(
                AuthContainer {
                    // Data: T::from_string( cookie.value().string_decode() )
                    // cookie: T::retrieve(cookie.value().string_decode()),
                    cookie: T::retrieve(cookie.value()),
                }
            ),
            None => Outcome::Forward(())
        }
    }
}




// pub enum AuthStatus<T> {
    
// }

// impl<A: AuthorizeCookie> AuthStatus<A> {
    
// }

impl<'de, A: AuthorizeForm+CookieId> LoginContainer<'de, A>  where A: ::serde::Serialize + ::serde::Deserialize<'de> {
    /// Redirect to different locations based on whether the LoginContainer data
    /// is valid credentials (as determined by authenticate() method)
    /// 
    /// If valid goes to first parameter
    /// Otherwise redirects to second parameter with the error message as a flash message
    pub fn redirect(&self, succeed_url: &str, fail_url: &str, mut cookies: Cookies) -> Result<Redirect, Flash<Redirect>> {
        let auth = self.form.authenticate();
        // First Err String is username, second is error message
        match auth {
            Ok(cooky) => {
                // add cookie here
                cookies.add_private(A::cookie_id(), cooky.store());
                
                Ok(Redirect::to(succeed_url))
            },
            Err(user, err) => {
                let mut fail = fail_url.to_string();
                fail.push_str(A::fail_url(err));
                Err(Flash::success(Redirect::to(), "Login Success"))
            },
        }
        
    }
}

impl<'de, 'f, A: AuthorizeForm> FromForm<'f> for LoginContainer<'de, A>  where A: ::serde::Serialize + ::serde::Deserialize<'de>{
    type Error = &'static str;
    
    fn from_form(form_items: &mut FormItems<'f>, _strict: bool) -> Result<Self, Self::Error> {
        // let mut user_pass = HashMap::new();
        let mut user: String = String::new();
        let mut pass: String = String::new();
        for (key,value) in form_items {
            match key.as_str(){
                // "username" => user_pass.insert("username", value).map_or((), |_v| ()),
                // "password" => user_pass.insert("password", value).map_or((), |_v| ()),
                "username" => sanitize(value.url_decode().unwrap_or(String::new())),
                "password" => sanitize_password(value.url_decode().unwrap_or(String::new())),
                _ => ()
            }
        }
        // if user_pass.get("username").is_none() || user_pass.get("password").is_none() {
        
        
        // if &user == "" || &pass == "" {
        //     Err("Invalid form: not all fields are specified.")
        // } else {
            
        Ok(
            LoginContainer {
                form: A::new_form(user, pass),
            }
        )
            
            
            // let result = A::check_credentials(
            //     user_pass.get("username").unwrap().to_string(), 
            //     user_pass.get("password").unwrap().to_string()
            // );
            // Ok(match result{
                // Ok(authenticator) => LoginStatus::Succeed(authenticator),
                // Err(authenticator) => LoginStatus::Failed(authenticator)
            // })
        // }
    }
}
// impl<'r> Responder<'r> for LoginRedirect{
//     fn respond_to(self, request: &Request) -> Result<Response<'r>, Status>{
//         self.0.respond_to(request)
//     }
// }



// impl AuthorizeCookie for AdminAuthCookie {
//     type FormType = AdminAuthForm;
    
// }

// pub trait Login {
//     type CookieType;
//     fn authenticate(&self) -> Result<Self, String>;
//     fn cookie_id() -> String;
    
// }


// impl Login for AdminAuthForm {
//     type CookieType = AdminAuthCookie;
// }
























