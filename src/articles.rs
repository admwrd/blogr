
use rocket;
use ::rocket::request::FromRequest;
use ::rocket::Request;
use ::rocket::outcome::Outcome;
use ::rocket::config::{Config, Environment};

use view::*;
use regex::Regex;
use chrono::prelude::*;


#[derive(Debug, Clone)]
pub struct Aid {
    aid: u64,
}

#[derive(Debug, Clone)]
pub struct Comment {
    pub author: String,
    pub author_display: String,
    pub body: String,
}

#[derive(Debug, Clone)]
pub struct Article {
    pub aid: u64,
    pub author: String,
    pub author_display: String,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
    pub title: String,
    pub body: String,
    // pub hits: u64,
    // pub tags: Option<Vec<Tag>>,
    // pub comments: Option<Vec<Comment>>,
}

#[derive(Debug, Clone)]
pub struct ArticleForm {
    pub author: String,
    pub title: String,
    pub body: String,
    pub tags: Option<String>, // split based on comma
}

#[derive(Debug, Clone)]
pub struct ArticleDescription {
    pub author_display: String,
    pub created: DateTime<Utc>,
    pub title: String, // Max 128 characters
    pub description: String, // Max 512 characters
    pub tags: String,
    pub category: Category,
}

pub fn sanitize_body(string: String) -> String {
    unimplemented!()
}

pub fn sanitize_title(string: String) -> String {
    unimplemented!()
}

pub fn sanitize_tags(string: String) -> String {
    unimplemented!()
}

impl Article {
    pub fn retrieve(aid: u64) -> Option<Article> {
        unimplemented!()
    }
    pub fn sanitize_body() -> String {
        unimplemented!()
    }
    pub fn sanitize_title() -> String {
        unimplemented!()
    }
}

impl ArticleId {
    pub fn find(&self) -> Option<Article> {
        unimplemented!()
    }
    
    pub fn retrieve(&self) -> Article {
        // this should only be used after a request guard ensures that the aid exists
        // if aid is not found panic?
        unimplemented!()
    }
    pub fn short_description(&self) -> ArticleDescription {
        unimplemented!()
    }
}

// A request guard to ensure that an article exists for a given ArticleId or aid
impl<'a, 'r> FromRequest<'a, 'r> for ArticleId {
    type Error = ();
    
    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<Article, Self::Error> {
        
        
        // match Article::retrieve() {
        //     Some(cookie) => Outcome::Success(UserCookie::new(cookie.value().to_string())),
        //     None => Outcome::Forward(()),
        // }
        
        unimplemented!()
    }
}

impl ArticleForm {
    pub fn new(title: String, body: String, tags: String) -> ArticleForm {
        ArticleForm {
            author: String::new(),
            title,
            body,
            tags,
        }
    }
    pub fn author(&self, author: String) -> ArticleForm {
        let new = self.clone();
        ArticleForm {
            author,
            .. new
        }
    }
    pub fn save(&self) -> Result<Article, String> {
        unimplemented!()
    }
}

impl<'f> FromForm<'f> for ArticleForm {
    type Error = &'static str;
    
    fn from_form(form_items: &mut FormItems<'f>, _strict: bool) -> Result<Self, Self::Error> {
        
        // Author should be blank string here, when saving the author can be identified from cookies
        let mut author = String::new();
        
        let mut title = String::new();
        let mut body: String::new();
        let mut tags: String::new();
        
        for (field, value) in form_items {
            match field.as_str() {
                "title" => { title = sanitize_title(value.to_string()) },
                "body" => { body = sanitize_body(value.to_string()) },
                "tags" => { tags = sanitize_tags(value.to_string()) },
            }
        }
        if title == "" || body == "" {
            Err("Missing a required field.")
        } else {
            Ok( ArticleForm::new(title, body, tags) )
        }
    }
}


