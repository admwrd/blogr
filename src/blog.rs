
use rocket;
use ::rocket::request::{FromRequest, FromForm, FormItems};
use ::rocket::Request;
use ::rocket::outcome::Outcome;
use ::rocket::config::{Config, Environment};

use regex::Regex;
use chrono::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};

use users::*;


// type ArticleId = u32;
#[derive(Debug, Clone, FromForm)]
pub struct ArticleId {
    aid: u32,
}

#[derive(Debug, Clone)]
pub struct Article {
    pub aid: u32,
    pub title: String,
    pub posted: NaiveDateTime,
    pub body: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ArticleForm {
    // pub userid: u32,
    pub title: String,
    pub body: String,
    pub tags: String,
}

#[derive(Debug, Clone)]
pub struct User {
    pub userid: u32,
    pub username: String,
    pub display: Option<String>,
    pub email: Option<String>,
    pub password: String,
    pub is_admin: bool,
}

impl ArticleForm {
    pub fn to_article(&self) -> Article {
        // get next aid
        let next_aid = 0;
        Article {
            aid: next_aid,
            title: sanitize_title(self.title),
            posted: Local::now().naive_local(), // This fn is only used when saving new articles
            body: sanitize_body(self.body),
            tags: split_tags(sanitize_tags(self.tags)),
        }
    }
}


impl ArticleId {
    pub fn exists(&self) -> bool {
        unimplemented!()
    }
    pub fn retrieve(&self) -> Article {
        unimplemented!()
    }
    pub fn last_id() -> u32 {
        unimplemented!()
    }
    pub fn next_id() -> u32 {
        unimplemented!()
    }
    
}

pub fn sanitize_body(string: String) -> String {
    // escape html entities/elements
    unimplemented!()
}

pub fn sanitize_title(string: String) -> String {
    // set max length to 120 characters
    unimplemented!()
}

pub fn sanitize_tags(string: String) -> String {
    unimplemented!()
}
pub fn split_tags(string: String) -> Vec<String> {
    let tags: Vec<String> = string.split(',').filter(|t| t != &"").map(|t| t.to_string()).collect();
    tags
}

impl Article {
    pub fn retrieve(aid: u64) -> Option<Article> {
        unimplemented!()
    }
}

impl ArticleForm {
    pub fn new(title: String, body: String, tags: String) -> ArticleForm {
        ArticleForm {
            // userid: 0,
            title,
            body,
            tags,
        }
    }
    // Removed userid from ArticleForm and no userid exists in Article
    // pub fn author(self, userid: u32) -> ArticleForm {
    //     let new: ArticleForm = self.clone();
    //     ArticleForm {
    //         userid,
    //         .. new
    //     }
    // }
    pub fn save(&self) -> Result<Article, String> {
        unimplemented!()
    }
}



// A request guard to ensure that an article exists for a given ArticleId or aid
impl<'a, 'r> FromRequest<'a, 'r> for ArticleId {
    type Error = ();
    
    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<ArticleId, Self::Error> {
        
        
        // match Article::retrieve() {
        //     Some(cookie) => Outcome::Success(UserCookie::new(cookie.value().to_string())),
        //     None => Outcome::Forward(()),
        // }
        
        unimplemented!()
    }
}


impl<'f> FromForm<'f> for ArticleForm {
    type Error = &'static str;
    
    fn from_form(form_items: &mut FormItems<'f>, _strict: bool) -> Result<Self, Self::Error> {
        
        // Author should be blank string here, when saving the author can be identified from cookies
        // this prevents user from altering the userid in submitted form data when using a hidden field to save the userid
        
        let mut title: String = String::new();
        let mut body: String = String::new();
        let mut tags: String = String::new();
        
        for (field, value) in form_items {
            match field.as_str() {
                "title" => { title = sanitize_title(value.to_string()) },
                "body" => { body = sanitize_body(value.to_string()) },
                "tags" => { tags = sanitize_tags(value.to_string()) },
                _ => {},
            }
        }
        if title == "" || body == "" {
            Err("Missing a required field.")
        } else {
            Ok( ArticleForm::new(title, body, tags) )
        }
    }
}

