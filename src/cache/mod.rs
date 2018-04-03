
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
use titlecase::titlecase;

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



pub fn make_descriptions(articles: Vec<Article>) -> Vec<Article> {
    let output: Vec<Article> = articles.into_iter()
        .map(|mut article| {
            article.body = if article.description != "" {
                article.description.clone()
            } else {
                article.body[..DESC_LIMIT].to_owned()
            };
            article
        }
    ).collect();
    output
}




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
        if let Ok(article_cache) = self.lock.read() {
            if let Some(article) = article_cache.articles.get(&aid) {
                Some(article.clone())
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn retrieve_articles(&self, aids: Vec<u32>) -> Option<Vec<Article>> {
        if let Ok(article_cache) = self.lock.read() {
            let mut articles: Vec<Article> = Vec::new();
            for aid in aids {
                if let Some(article) = article_cache.articles.get(&aid) {
                    articles.push(article.clone());
                } else {
                    println!("Failed to retrieve article {} from collection", aid);
                }
            }
            if articles.len() != 0 {
                Some(articles)
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn all_articles(&self, pagination: &Page<Pagination>) -> Option<(Vec<Article>, u32)> {
        let mut starting = pagination.cur_page as u32;
        let mut ending = pagination.cur_page as u32 + pagination.settings.ipp as u32;
        
        if let Ok(article_lock) = self.lock.read() {
            let aids: Vec<u32> = article_lock.articles.keys().map(|i| *i).collect();
        // if let Some(aids) = self.retrieve_aids(&format!("author/{}", &author)) {
            // println!("Attempting to retrieve author articles: {:#?}", &aids);
            let total_items = aids.len() as u32;
            if total_items <= pagination.settings.ipp as u32 {
                starting = 0;
                ending = total_items;
            } else {
                if starting >= total_items {
                    // show last page
                    let starting = total_items - (pagination.settings.ipp as u32);
                    let ending = total_items;
                // the greater OR EQUALS part is equivelant to > total_items -1
                } else if ending >= total_items { 
                    let ending = total_items;
                }
                if starting - ending <= 0 {
                    println!("Pagination error!  Start-Ending <= 0");
                    return None;
                }
            }
            // println!("Attempting to grab articles ({}, {}]", starting, ending);
            let slice: &[u32] = &aids[starting as usize..ending as usize];
            // println!("which are: {:?}", &slice);
            let ids = slice.to_owned();
            // The retireve_articles() opens another reader in the RwLock,
            // but this shouldn't cause any major issues, maybe a little slower
            if let Some(mut articles) = self.retrieve_articles(ids) {
                articles = make_descriptions(articles);
                let length = articles.len() as u32;
                if length != total_items {
                    println!("ERROR: Retrieving all articles yielded differing results.\nIt was supposed to return {} items but returned {} items.", total_items, length);
                }
                Some( (articles, length) )
            } else {
                println!("Could not retrieve all articles - retrieve_articles failed");
                None
            }
        } else {
            None
        }
    }
}



pub struct TextCacheLock {
    pub lock: RwLock<TextCache>,
}

pub struct TextCache {
    pub pages: HashMap<String, String>,
}
impl TextCache {
    pub fn load_cache(conn: &DbConn, multi_aids: &TagAidsLock) -> Self {
        let mut pages: HashMap<String, String> = HashMap::new();
        
        let rss = cache::pages::rss::load_rss(conn);
        pages.insert("rss".to_owned(), rss);
        
        // TagCloud is not text! It is a Vec<TagCount>
        // // let tagcloud = cache::pages::tags::load_tagcloud(multi_aids);
        // if let Some(tagcloud) = cache::pages::tags::load_tagcloud(multi_aids) {
        //     pages.insert("tagcloud".to_owned(), tagcloud);
        // }
        
        TextCache {
            pages
        }
        // unimplemented!()
    }
    
}
impl TextCacheLock {
    pub fn new(cache: TextCache) -> Self {
        TextCacheLock{ lock: RwLock::new(cache) }
    }
    // For text retrieval maybe add a closure or function pointer parameter
    // that will be called in case the specified index(cached text) is not in the cache
    pub fn retrieve_text(&self, idx: &str) -> Option<String> {
        if let Ok(text_cache) = self.lock.read() {
            text_cache.pages.get(idx).map(|s| s.clone())
        } else {
            None
        }
        
        // unimplemented!()
    }
}





// pub struct TagAids {
pub struct AidsCache {
    pub pages: HashMap<String, Vec<u32>>,
}
pub struct TagsCache {
    // pub tags: HashMap<String, u32>,
    pub tags: Vec<TagCount>,
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
        
        let qrystr = "SELECT COUNT(*) as cnt, unnest(tag) as untag FROM articles GROUP BY untag ORDER BY cnt DESC;";
        let qry = conn.query(qrystr, &[]);
        if let Ok(result) = qry {
            // let mut pages: HashMap<String, u32> = HashMap::new();
            let mut tags: Vec<TagCount> = Vec::new();
            for row in &result {
                let c: i64 = row.get(0);
                let t: String = row.get(1);
                let t2: String = t.trim_matches('\'').to_string();
                let tc: TagCount = TagCount {
                    tag: titlecase(&t2),
                    url: t2,
                    count: c as u32,
                    size: 0,
                };
                tags.push(tc);
                // pages.insert(t, c as u32);
            }
            
            if tags.len() > 4 {
                if tags.len() > 7 {
                    let mut i = 0u16;
                    for mut v in &mut tags[0..6] {
                        v.size = 6-i;
                        i += 1;
                    }
                } else {
                    let mut i = 0u16;
                    for mut v in &mut tags[0..3] {
                        v.size = (3-i)*2;
                    }
                }
                tags.sort_by(|a, b| a.tag.cmp(&b.tag));
            }
            
            TagsCache {
                tags,
            }
        } else {
            TagsCache {
                // tags: HashMap::new(),
                tags: Vec::new(),
            }
        }
        
        
        // unimplemented!()
    }
}

impl TagAidsLock {
    // Returns the ArticleIds for the given page
    pub fn retrieve_aids(&self, page: &str) -> Option<Vec<u32>> {
        // unlock TagAidsLock
        // find the page
        // return the aids
        if let Ok(multi_aids) = self.aids_lock.read() {
            if let Some(aids) = multi_aids.pages.get(page) {
                Some(aids.clone())
            } else {
                None
            }
        } else {
            None
        }
        
        
        // unimplemented!()
    }
    // Retrieve (from the cache) all tags and the number of times they have been used
    pub fn retrieve_tags(&self) -> Option<Vec<TagCount>> {
        // unimplemented!()
        if let Ok(all_tags) = self.tags_lock.read() {
            Some(all_tags.tags.clone())
            
            // let mut tags: Vec<TagCount> = Vec::new();
            // for (tag, count) in &all_tags.tags {
            //     let t = TagCount {
            //         // tag: tag.clone(),
            //         tag: titlecase(tag),
            //         // url: titlecase(tag.trim_matches('\'')),
            //         url: tag.trim_matches('\'').to_owned(),
            //         count: *count,
            //         size: 0u16,
            //     };
            //     tags.push(t);
            // }
            // Some(tags)
        } else {
            None
        }
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
        
        // for tag in tag_cache.tags.keys() {
        for tag in tag_cache.tags.iter() {
            if let Some(aids) = cache::pages::tag::load_tag_aids(conn, &tag.tag.to_lowercase()) {
                let key = format!("tag/{}", &tag.tag);
                if !PRODUCTION { println!("Loading tag {}\n\t{:?}\n\trelated articles:\n\t{:#?}", &tag.url, &tag, &aids); }
                article_cache.insert(key, aids);
            } else {
                println!("Error loading multi article cache on tag {} - no articles found", tag.url);
            }
        }
        
        for user in authors {
            if let Some(aids) = cache::pages::author::load_author_articles(conn, user) {
                let key = format!("author/{}", user);
                if !PRODUCTION { println!("Loading user {}\n\trelated articles:\n\t{:#?}", &user, &aids); }
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
    // Could make author_articles() and tag_articles generic, 
    // take a page name &str to lookup the multi articles page.
    // And change error message to a generic message
    pub fn author_articles(&self, article_cache: &ArticleCacheLock, author: u32, pagination: &Page<Pagination>) -> Option<(Vec<Article>, u32)> {
        let mut starting = pagination.cur_page as u32;
        let mut ending = pagination.cur_page as u32 + pagination.settings.ipp as u32;
        if let Some(aids) = self.retrieve_aids(&format!("author/{}", &author)) {
            // println!("Attempting to retrieve author articles: {:#?}", &aids);
            let total_items = aids.len() as u32;
            if total_items <= pagination.settings.ipp as u32 {
                starting = 0;
                ending = total_items;
            } else {
                if starting >= total_items {
                    // show last page
                    let starting = total_items - (pagination.settings.ipp as u32);
                    let ending = total_items;
                // the greater OR EQUALS part is equivelant to > total_items -1
                } else if ending >= total_items { 
                    let ending = total_items;
                }
                if starting - ending <= 0 {
                    println!("Pagination error!  Start-Ending <= 0");
                    return None;
                }
            }
            // println!("Attempting to grab articles ({}, {}]", starting, ending);
            let slice: &[u32] = &aids[starting as usize..ending as usize];
            // println!("which are: {:?}", &slice);
            let ids = slice.to_owned();
            if let Some(mut articles) = article_cache.retrieve_articles(ids) {
                articles = make_descriptions(articles);
                let length = articles.len() as u32;
                if length != total_items {
                    println!("ERROR: Retrieving articles with author `{}` yielded differing results.\nIt was supposed to return {} items but returned {} items.", author, total_items, length);
                }
                Some( (articles, length) )
            } else {
                println!("Could not retrieve articles in author_articles() - retrieve_articles failed");
                None
            }
            
        } else {
            println!("Error retrieving articles for author, retrieve_aids() failed for `author/{}`", &author);
            None
        }
    }
    // Could make this generic, see author_articles() comments above.
    pub fn tag_articles(&self, article_cache: &ArticleCacheLock, tag: &str, pagination: &Page<Pagination>) -> Option<(Vec<Article>, u32)> {
        let mut starting = pagination.cur_page as u32;
        let mut ending = pagination.cur_page as u32 + pagination.settings.ipp as u32;
        if let Some(aids) = self.retrieve_aids(&format!("tag/{}", tag)) {
            let total_items = aids.len() as u32;
            // the greater OR EQUALS part is equivelant to > total_items -1
            if total_items <= pagination.settings.ipp as u32 {
                starting = 0;
                ending = total_items;
            } else {
                if starting >= total_items {
                    // show last page
                    let starting = total_items - (pagination.settings.ipp as u32);
                    let ending = total_items ;
                // the greater OR EQUALS part is equivelant to > total_items -1
                } else if ending >= total_items { 
                    let ending = total_items;
                }
                if starting - ending <= 0 {
                    return None;
                }
            }
            // println!("Attempting to grab articles ({}, {}]", starting, ending);
            let slice: &[u32] = &aids[starting as usize..ending as usize];
            let ids = slice.to_owned();
            if let Some(mut articles) = article_cache.retrieve_articles(ids) {
                articles = make_descriptions(articles);
                let length = articles.len() as u32;
                if length != total_items {
                    println!("ERROR: Retrieving articles with tag `{}` yielded differing results.\nIt was supposed to return {} items but returned {} items.", tag, total_items, length);
                }
                Some( (articles, length) )
            } else {
                println!("Could not retrieve articles in tag_articles() - retrieve_articles failed");
                None
            }
            
        } else {
            println!("Error retrieving articles for tag, retrieve_aids() failed for `tag/{}`", &tag);
            None
        }
        //     let ids: Vec<u32> = (starting..ending).into_iter().map(|i| i as u32).collect();
        //     if let Some(mut articles) = article_cache.retrieve_articles(ids) {
        //         articles = make_descriptions(articles);
        //         let length = articles.len() as u32;
        //         if length != total_items {
        //             println!("ERROR: Retrieving articles with tag `{}` yielded differing results.\nIt was supposed to return {} items but returned {} items.", tag, total_items, length);
        //         }
        //         Some( (articles, length) )
        //     } else {
        //         None
        //     }
        // } else {
        //     None
        // }
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



















