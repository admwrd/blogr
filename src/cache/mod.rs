
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
use collate::*;
use content::*;
use data::*;
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
    pub fn retrieve_articles(&self, aids: Vec<u32>) -> Option<Vec<Article>> {
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

// impl AidsCache {
//     pub fn load_cache(conn: &DbConn) -> Self {
//         // retrieve all distinct tags then call routes::pages::tags::tag_aids()
//         // find all tags - use the query for the tag cloud (get tag and number of times used)
//         // store tags and tag counts
//         // call load_tag_aids() on each tag
        
//         unimplemented!()
//     }
// }
impl TagsCache {
    pub fn load_cache(conn: &DbConn) -> Self {
        // Find all unique tags and store the number of times they are used
        // in a HashMap<String, u32>
        unimplemented!()
    }
}

impl TagAidsLock {
    // Returns the ArticleIds for the given page
    pub fn retrieve_aids(&self, page: &str) -> Option<Vec<u32>> {
        // unlock TagAidsLock
        // find the page
        // return the aids
        
        unimplemented!()
    }
    // Retrieve (from the cache) all tags and the number of times they have been used
    pub fn retrieve_tags(&self) -> Option<Vec<TagCount>> {
        unimplemented!()
    }
    
    // pub fn tag_aids(tag: &str) -> Option<Vec<u32>> {
    //     unimplemented!()
    // }
    
    // pub fn load_tag_cache(conn: &DbConn, tags: &HashMap<String, u32>) -> Vec<u32> {
    //     unimplemented!()
    // }
    
    // pub fn load_author_cache(conn: &DbConn) -> Vec<u32> {
    //     unimplemented!()
    // }
    pub fn load_cache(conn: &DbConn) -> Self {
        // Load tags then for each tag call 
        // cache::pages::tag::load_tag_aids(conn, tag) -> Option<Vec<u32>>
        // in order to find all articles attributed to that tag
        // 
        // Call cache::pages::author::load_authors(conn)
        // and call cache::pages::author::load_author_articles(conn, userid)
        // on each of the userids returned by the load_authors()
        
        let tag_cache = TagsCache::load_cache(&conn);
        let authors = cache::pages::author::load_authors(conn);
        
        let mut article_cache: HashMap<String, Vec<u32>> = HashMap::with_capacity(tag_cache.tags.len() + authors.len() + 10);
        
        for tag in tag_cache.tags.keys() {
            if let Some(aids) = cache::pages::tag::load_tag_aids(conn, &tag) {
                let key = format!("tag/{}", tag);
                article_cache.insert(key, aids);
            } else {
                println!("Error loading multi article cache on tag {} - no articles found", tag);
            }
        }
        
        for user in authors {
            if let Some(aids) = cache::pages::author::load_author_articles(conn, user) {
                let key = format!("author/{}", user);
                article_cache.insert(key, aids);
            } else {
                println!("Error loadign multi article cache on author {} - no articles found", user);
            }
        }
        
        TagAidsLock {
            aids_lock: RwLock::new( AidsCache{ pages: article_cache } ),
            tags_lock: RwLock::new( tag_cache ),
        }
        
    }
    #[inline]
    pub fn new(aids: AidsCache, tags: TagsCache) -> Self {
        TagAidsLock{ aids_lock: RwLock::new( aids), tags_lock: RwLock::new( tags ) }
    }
    pub fn tag_articles(&self, article_cache: &ArticleCacheLock, tag: &str, pagination: &Page<Pagination>) -> Option<(Vec<Article>, u32)> {
        let mut starting = pagination.cur_page as u32;
        let mut ending = pagination.cur_page as u32 + pagination.settings.ipp as u32;
        if let Some(aids) = self.retrieve_aids(&format!("tag/{}", tag)) {
            let total_items = aids.len() as u32;
            // the greater OR EQUALS part is equivelant to > total_items -1
            if starting >= total_items {
                // show last page
                let starting = total_items - 1 - (pagination.settings.ipp as u32);
                let ending = total_items - 1;
            // the greater OR EQUALS part is equivelant to > total_items -1
            } else if ending >= total_items { 
                let ending = total_items -1;
            }
            if starting - ending <= 0 {
                return None;
            }
            let ids: Vec<u32> = (starting..ending+1).into_iter().map(|i| i as u32).collect();
            if let Some(articles) = article_cache.retrieve_articles(ids) {
                let length = articles.len() as u32;
                if length != total_items {
                    println!("ERROR: Retrieving articles with tag `{}` yielded differing results.\nIt was supposed to return {} items but returned {} items.", tag, total_items, length);
                }
                Some( (articles, length) )
            } else {
                None
            }
        } else {
            None
        }
    }
    // Maybe add a function that executes a closure?
    // pub fn unlock<F, T>(f: F) -> T  where F: Fn(i32) -> T { unimplemented!() }
}

// Is this really needed??
// pub struct TextPages {
//     pub pages: HashMap<String, String>,
// }

pub fn template<T: BodyContext, U: BodyContext>(body_rst: Result<CtxBody<T>, CtxBody<U>>) -> Express where T: serde::Serialize, U: serde::Serialize {
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
}

// pub fn express<T: BodyContext>(body: CtxBody<T>) -> Express {
//     unimplemented!()
// }

// make a request guard to retrieve a reference to the articles?

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


















