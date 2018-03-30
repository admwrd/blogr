
use rocket_contrib::Template;
use rocket::{Request, Data, Outcome, Response};
// use rocket::response::{content, NamedFile, Redirect, Flash, Responder, Content};
use rocket::response::{NamedFile, Redirect, Flash, Responder, Content};
use rocket::response::content::Html;
use rocket::data::FromData;
use rocket::request::{FlashMessage, Form, FromForm, FormItems, FromRequest};
use rocket::http::{Cookie, Cookies, MediaType, ContentType, Status};
use rocket::State;

use std::fmt::Display;
use std::{env, str, thread};
use std::fs::{self, File, DirEntry};
use std::io::prelude::*;
use std::io::{self, Cursor, Read};
use std::path::{Path, PathBuf};
use std::time::{self, Instant, Duration};
use std::prelude::*;
use std::ffi::OsStr;
use std::collections::HashMap;
use std::sync::{Mutex, Arc, RwLock};
use std::sync::atomic::AtomicUsize;

use std::borrow::Cow;

use evmap::*;
use comrak::{markdown_to_html, ComrakOptions};


// mod body_options;
// mod page_routes;

pub mod body;
use body::*;
pub mod pages;
use pages::*;

use super::*;
use blog::*;
use data::*;
use content::*;
use templates::*;
use xpress::*;






pub struct ArticleCacheLock {
    pub lock: RwLock<ArticleCache>,
}

pub struct ArticleCache {
    pub articles: HashMap<u32, Article>,
}

impl ArticleCache {
    pub fn load_cache(conn: &DbConn) -> Self {
        if let Some(articles) = conn.articles_full("") {
            let mut map: HashMap<u32, Article> = HashMap::new();
            for article in articles {
                map.insert(article.aid, article);
            }
            ArticleCache{ articles: map }
        } else {
            ArticleCache{ articles: HashMap::new() }
        }
    }
}

impl ArticleCacheLock {
    pub fn new(cache: ArticleCache) -> Self {
        ArticleCacheLock{ lock: RwLock::new( cache ) }
    }
    pub fn retrieve_article(&self, aid: u32) -> Option<Article> {
        unimplemented!()
    }
    pub fn retrieve_articles(&self, aids: Vec<u32>) -> Option<Article> {
        unimplemented!()
        
    }
}



pub struct TextCacheLock {
    pub lock: RwLock<TextCache>,
}

pub struct TextCache {
    pub pages: HashMap<String, String>,
}
impl TextCache {
    pub fn load_cache(conn: &DbConn) -> Self {
        unimplemented!()
    }
    
}
impl TextCacheLock {
    pub fn new(cache: TextCache) -> Self {
        TextCacheLock{ lock: RwLock::new(cache) }
    }
    // For text retrieval maybe add a closure or function pointer parameter
    // that will be called in case the specified index(cached text) is not in the cache
    pub fn retrieve_text(&self, idx: &str) -> Option<String> {
        unimplemented!()
        
    }
}





// pub struct TagAids {
pub struct AidsCache {
    pub pages: HashMap<String, Vec<u32>>,
}
pub struct TagsCache {
    pub tags: HashMap<String, u32>,
}

pub struct TagAidsLock {
    pub aids_lock: RwLock<AidsCache>,
    pub tags_lock: RwLock<TagsCache>,
}

impl AidsCache {
    pub fn load_cache(conn: &DbConn) -> Self {
        // retrieve all distinct tags then call routes::pages::tags::tag_aids()
        // find all tags - use the query for the tag cloud (get tag and number of times used)
        // store tags and tag counts
        // call load_tag_aids() on each tag
        
        unimplemented!()
    }
}
impl TagsCache {
    pub fn load_cache(conn: &DbConn) -> Self {
        unimplemented!()
    }
}

impl TagAidsLock {
    // pub fn retrieve_tag_aids(&self, page: &str) -> Option<Vec<u32>> {
    pub fn retrieve_aids(&self, page: &str) -> Option<Vec<u32>> {
        // unlock TagAidsLock
        // find the page
        // return the aids
        
        unimplemented!()
    }
    pub fn retrieve_tags() -> Option<Vec<TagCount>> {
        unimplemented!()
    }
    pub fn tag_aids(tag: &str) -> Option<Vec<u32>> {
        unimplemented!()
    }
    pub fn new(aids: AidsCache, tags: TagsCache) -> Self {
        TagAidsLock{ aids_lock: RwLock::new( aids), tags_lock: RwLock::new( tags ) }
    }
}

// Is this really needed??
// pub struct TextPages {
//     pub pages: HashMap<String, String>,
// }



// pub fn express<T: BodyContext>(body: CtxBody<T>, info: CtxInfo) -> Express {
// pub fn express<T: BodyContext>(body: CtxBody<T>, info: CtxInfo) -> Express {

// pub fn template<T: BodyContext>(template_name: &str, body: CtxBody<T>) -> Express {
pub fn template<T: BodyContext, U: BodyContext>(body_rst: Result<CtxBody<T>, CtxBody<U>>) -> Express where T: serde::Serialize, U: serde::Serialize {
    // let template_name = body.0.template_name();
    // let template_name = routes::body::BodyContext::template_name(body.0);
    // let template_name = T::template_name();
    // let is_t = if body_rst.is_ok() { true } else { false };
    // let body = match body_rst {
    //     Ok(k) => k,
    //     Err(e) => e,
    // };
    match body_rst {
        Ok(body)  => {
            let template = Template::render(T::template_name(), body);
            let express: Express = template.into();
            express
        },
        Err(body) => {
            let template = Template::render(U::template_name(), body);
            let express: Express = template.into();
            express
        },
    }
    
    // let express: Express = String::new().into();
    // express
}
pub fn express<T: BodyContext>(body: CtxBody<T>) -> Express {
    unimplemented!()
}
// // pub fn render<T: BodyContext>(body: T, info: CtxInfo) -> Express {
 // pub fn render<T: BodyContext>(body: CtxBody<T>, info: CtxInfo) -> Express {
// // pub fn render(body: CtxBody, info: CtxInfo) -> Express {
//     unimplemented!()
// }


// Make the articles_cache a lazy_static wrapped in a mutex/rwlock/arc?
// make a request guard to retrieve a reference to the articles

/*
    General - body text String
    Article - Article
    ArticlesPages - Vec<Article>, Page<Pagination>, u32 (total num items), Option<String> (page info - "Showing page x of y - z items found")
    Search - Vec<Article>, Option<Search>
    Login - String action url, Option<String> username
    LoginData - String action url, Option<String> username, HashMap<String, String> hidden form fields
    Create - String action url
    Edit - String action url, Article
    Manage - Vec<Article>, Page<Pagination>, u32 (num items total), Sort
    Tags - Vec<TagCount> lists tags and their counts
*/

pub fn load_all_articles(conn: &DbConn) -> Option<Vec<Article>> {
    // unimplemented!()
    if let Some(articles) = conn.articles_full("") {
        Some(articles)
    } else {
        None
    }
}

pub fn load_articles_map(conn: &DbConn) -> Option<HashMap<u32, Article>> {
    // unimplemented!()
    if let Some(articles) = conn.articles_full("") {
        let mut map: HashMap<u32, Article> = HashMap::new();
        for article in articles {
            map.insert(article.aid, article);
        }
        
        Some(map)
    } else {
        None
    }
}

/* 
pub trait Cachable {
    type Index;
    type StateType;
    // type Output;
    fn new(Self::Index) -> Self;
    // fn retrieve(self, State<Self::StateType>, Option<&DbConn>) -> Option<Self::Output>;
    fn retrieve(self, State<Self::StateType>, Option<&DbConn>) -> Express;
    
}

pub struct SingleArticle(u32);

pub struct MultiArticles(Vec<u32>);

pub struct GenericInfo{
    
}

impl Cachable for SingleArticle {
    type Index = u32;
    type StateType = ArticleCacheLock;
    // type Output = Express;
    
    fn new(aid: u32) -> SingleArticle {
        SingleArticle(aid)
    }
    // pub fn retrieve(self, conn_opt: Option<&DbConn>) -> Option<Article> {
    fn retrieve(self, articles_state: State<Self::StateType>, conn_opt: Option<&DbConn>) -> Express {
        let aid = self.0;
        
        
        // let output: Template;
        // // let output = 
        // if let Ok(a) = articles_state.lock.read() {
        //     if let Some(article) = a.articles.get(&aid) {
        //         let title = article.title.clone();
        //         // println!("Article {}\n{:?}", article.aid, &article);
        //         // output = hbs_template(TemplateBody::Article(article.clone()), None, Some(title), String::from("/article"), admin, user, Some("enable_toc(true);".to_owned()), Some(start.0));
        //         hbs_template(TemplateBody::Article(article.clone()), None, Some(title), String::from("/article"), admin, user, Some("enable_toc(true);".to_owned()), Some(start.0))
        //     } else {
        //         // output = hbs_template(TemplateBody::General(alert_danger(&format!("Article {} not found.", aid))), None, Some("Article Not Found".to_string()), String::from("/article"), admin, user, None, Some(start.0));
        //         hbs_template(TemplateBody::General(alert_danger(&format!("Article {} not found.", aid))), None, Some("Article Not Found".to_string()), String::from("/article"), admin, user, None, Some(start.0))
        //     }
        // } else {
        //     // output =  hbs_template(TemplateBody::General(alert_danger(&format!("Failed to acquire cache lock for article {}.", aid))), None, Some("Internal Error".to_string()), String::from("/article"), admin, user, None, Some(start.0));
        //     hbs_template(TemplateBody::General(alert_danger(&format!("Failed to acquire cache lock for article {}.", aid))), None, Some("Internal Error".to_string()), String::from("/article"), admin, user, None, Some(start.0))
        // }
        
        
        // let output: Template;
        // // let output = 
        // if let Ok(a) = articles_state.lock.read() {
        //     if let Some(article) = a.articles.get(&aid) {
        //         let title = article.title.clone();
        //         // println!("Article {}\n{:?}", article.aid, &article);
        //         // output = hbs_template(TemplateBody::Article(article.clone()), None, Some(title), String::from("/article"), admin, user, Some("enable_toc(true);".to_owned()), Some(start.0));
        //         hbs_template(TemplateBody::Article(article.clone()), None, Some(title), String::from("/article"), admin, user, Some("enable_toc(true);".to_owned()), Some(start.0))
        //     } else {
        //         // output = hbs_template(TemplateBody::General(alert_danger(&format!("Article {} not found.", aid))), None, Some("Article Not Found".to_string()), String::from("/article"), admin, user, None, Some(start.0));
        //         hbs_template(TemplateBody::General(alert_danger(&format!("Article {} not found.", aid))), None, Some("Article Not Found".to_string()), String::from("/article"), admin, user, None, Some(start.0))
        //     }
        // } else {
        //     // output =  hbs_template(TemplateBody::General(alert_danger(&format!("Failed to acquire cache lock for article {}.", aid))), None, Some("Internal Error".to_string()), String::from("/article"), admin, user, None, Some(start.0));
        //     hbs_template(TemplateBody::General(alert_danger(&format!("Failed to acquire cache lock for article {}.", aid))), None, Some("Internal Error".to_string()), String::from("/article"), admin, user, None, Some(start.0))
        // }
        // // };
        // output
        
        let express: Express = String::new().into();
        express
    }
}

 */


















