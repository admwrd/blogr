
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


use super::*;
use blog::*;
use data::*;
use content::*;
use templates::*;
use xpress::*;


pub struct ArticleCache {
    articles: HashMap<u32, Article>,
}

pub struct PageCache {
    pages: HashMap<String, String>,
}

pub struct ArticleCacheLock {
    lock: RwLock<ArticleCache>,
}
pub struct PageCacheLock {
    lock: RwLock<PageCache>,
}


// Make the articles_cache a lazy_static wrapped in a mutex/rwlock/arc?
// make a request guard to retrieve a reference to the articles


pub fn load_all_articles(conn: &DbConn) -> Option<Vec<Article>> {
    // unimplemented!()
    if let Some(articles) = conn.articles("") {
        Some(articles)
    } else {
        None
    }
}

pub fn load_articles_map(conn: &DbConn) -> Option<HashMap<u32, Article>> {
    // unimplemented!()
    if let Some(articles) = conn.articles("") {
        let mut map: HashMap<u32, Article> = HashMap::new();
        for article in articles {
            map.insert(article.aid, article);
        }
        
        Some(map)
    } else {
        None
    }
}

// pub fn load_articles(&mut evmap::WriteHandle<String, Article>) -> Result<usize, String> {

// pub fn load_article_cache(articles: &Vec<Article>, writer: &mut WriteHandle<String, &Article>, conn: &DbConn) -> Result<usize, String> {
// pub fn load_article_cache(articles_arc: &Arc<Vec<Article>>, writer: &mut WriteHandle<u32, &Article>, conn: &DbConn) -> Result<usize, String> {
// pub fn load_article_cache(articles: &Vec<Article>, writer: &mut WriteHandle<u32, &Article>, conn: &DbConn) -> Result<usize, String> {
pub fn load_article_cache<'v, 'd, 'w>(articles: &'v Vec<Article>, writer: &'w mut WriteHandle<u32, &'v Article>, conn: &'d DbConn) -> Result<usize, String> {
    // unimplemented!()
    // let articles = articles_arc.clone();
    if articles.len() == 0 {
        Ok(0usize)
    } else {
        let mut count = 0usize;
        // for article in articles.iter() {
        for article in articles {
            writer.insert(article.aid, &article);
            count += 1;
        }
        writer.refresh();
        if count == articles.len() {
            Ok(count)
        } else {
            Err(format!("Error loading article cache: inconsistent vector sizes, {} inserted vs original", count/*, articles.len()*/))
        }
    }
}

pub fn load_pages(writer: &mut evmap::WriteHandle<String, String>, conn: &DbConn) -> Result<usize, String> {
    unimplemented!()
}






















