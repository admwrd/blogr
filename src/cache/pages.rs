
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

use super::super::*;
use super::*;
use ::blog::*;
use ::data::*;
use ::content::*;
use ::templates::*;
use ::xpress::*;
use ::ral_user::*;
use ::ral_administrator::*;
use ::collate::*;

/*

    text        all_tags
    multi*      /tag/<tag>
                    /tag?<tag>
        
    article     /article?<aid>
                    /article/<aid>
                    /article/<aid>/<title>
                /article (hbs_article_not_found)
    text        /rss.xml
    multi*      /author/<authorid>
                    /author/<authorid>/<authorname>
    text        /about
        
        
    /pageviews
    /pagestats
    /pagestats/<show_errors>
    /manage/<sortstr>/<orderstr>
    /manage
    
*/


pub mod info {
    use super::*;
    pub fn info(title: Option<String>,
                page: String,
                admin: Option<AdministratorCookie>,
                user: Option<UserCookie>,
                gen: Option<GenTimer>,
                uhits: Option<UniqueHits>,
                encoding: Option<AcceptCompression>,
                msg: Option<String>,
                javascript: Option<String>,
               ) -> TemplateInfo
    {
        let js = if let Some(j) = javascript { j } else { "".to_string() };
        let (pages, admin_pages) = create_menu(&page, &admin, &user);
        let info = TemplateInfo::new(title, admin, user, js, gen.map(|g| g.0), page, pages, admin_pages, msg);
        info
        // unimplemented!()
    }
}


/// The article route module allows routes to serve up pages with
/// a single article as the content.
/// The article route module does not need a function to generate
/// the page, it only needs a serve function.
pub mod article {
    use super::*;
    pub fn context(aid: u32,
                   body: Option<Article>,
                   conn: &DbConn,
                   admin: Option<AdministratorCookie>, 
                   user: Option<UserCookie>, 
                   gen: Option<GenTimer>, 
                   uhits: Option<UniqueHits>, 
                   encoding: Option<AcceptCompression>, 
                   msg: Option<String>,
                   javascript: Option<String>
                  ) -> Result<CtxBody<TemplateArticle>, CtxBody<TemplateGeneral>>
    {
        let javascript: Option<String> = None;
        
        // macro_rules! ctx_info {
        //     ( $title:expr, $page:expr ) => {
        //         info::info(if $title == "" { None } else { Some($title.to_owned()) }, $page.to_owned(), admin, user, gen, uhits, encoding, javascript, msg)
        //     }
        // }
        // let i = ctx_info!("Article", "/");
        
        if let Some(article) = body {
            let i = info::info(Some(article.title.clone()), "/article".to_owned(), admin, user, gen, uhits, encoding, javascript, msg);
            Ok(CtxBody( TemplateArticle::new(article, i) ))
        } else if let Some(article) = cache::pages::article::fallback(aid, conn) {
            let i = info::info(Some(article.title.clone()), "/article".to_owned(), admin, user, gen, uhits, encoding, javascript, msg);
            if !PRODUCTION {
                println!("Article {} served from fallacbk instead of cache", aid);
            }
            Ok(CtxBody( TemplateArticle::new(article, i) ))
        } else {
            let i = info::info(Some(format!("Article {} not found", aid)), "/article".to_owned(), admin, user, gen, uhits, encoding, javascript, msg);
            Err(CtxBody( TemplateGeneral::new("The article could not be found.".to_owned(), i) ))
        }
    }
    pub fn fallback(aid: u32, conn: &DbConn) -> Option<Article> {
        let id = ArticleId { aid };
        id.retrieve()
    }
    pub fn serve(aid: u32, 
                 article_state: State<ArticleCacheLock>, 
                 conn: &DbConn, 
                 admin: Option<AdministratorCookie>, 
                 user: Option<UserCookie>, 
                 start: GenTimer, 
                 uhits: UniqueHits,
                 encoding: AcceptCompression,
                 msg: Option<String>
                ) -> Express 
    {
        let article_rst = article_state.retrieve_article(aid);
        
        let ctx: Result<CtxBody<TemplateArticle>, CtxBody<TemplateGeneral>>
             = cache::pages::article::context(aid,
                                              article_rst, 
                                              conn,
                                              admin, 
                                              user, 
                                              Some(start), 
                                              Some(uhits), 
                                              Some(encoding),
                                              None,
                                              None
                                             );
        
        let express: Express = cache::template(ctx);
        express
    }
}

pub mod tag {
    use super::*;
    pub fn context(tag: &str,
                   conn: &DbConn,
                   pagination: &Page<Pagination>,
                   article_cache: &ArticleCacheLock,
                   multi_aids: &TagAidsLock,
                   admin: Option<AdministratorCookie>, 
                   user: Option<UserCookie>, 
                   uhits: Option<UniqueHits>, 
                   gen: Option<GenTimer>, 
                   encoding: Option<AcceptCompression>,
                   msg: Option<String>,
                   javascript: Option<String>
                  ) -> Result<CtxBody<TemplateArticlesPages>, CtxBody<TemplateGeneral>>
    {
        if CACHE_ENABLED {
            if let Some((articles, total_items)) = multi_aids.tag_articles(article_cache, tag, &pagination) {
                let javascript: Option<String> = None;
                let info_opt: Option<String> = None;
                let i = info::info( Some(format!("Showing articles with tag '{}'", &tag)), "/tag".to_owned(), admin, user, gen, uhits, encoding, javascript, msg );
                Ok(CtxBody( TemplateArticlesPages::new(articles, pagination.clone(), total_items, info_opt, i) ))
            } else {
                let i = info::info( Some(format!("No articles to display for tag '{}'", &tag)), "/tag".to_owned(), admin, user, gen, uhits, encoding, javascript, msg );
                Err(CtxBody( TemplateGeneral::new(format!("No artiles displayed for tag {}", tag), i) ))
            }
            
        } else if CACHE_FALLBACK {
            if let Some((articles, total_items)) = cache::pages::tag::fallback(tag, &pagination, conn) {
                let javascript: Option<String> = None;
                let info_opt: Option<String> = None;
                let i = info::info( Some(format!("Showing articles with tag '{}'", &tag)), "/tag".to_owned(), admin, user, gen, uhits, encoding, javascript, msg );
                Ok(CtxBody( TemplateArticlesPages::new(articles, pagination.clone(), total_items, info_opt, i) ))
            } else {
                let i = info::info( Some(format!("No articles to display for tag '{}'", &tag)), "/tag".to_owned(), admin, user, gen, uhits, encoding, javascript, msg );
                Err(CtxBody( TemplateGeneral::new(format!("No artiles displayed for tag {}", tag), i) ))
            }
        } else {
            println!("SUPER ERROR: Cache disabled and cache fallback disabled");
            let i = info::info( Some("Error".to_owned()), "/tag".to_owned(), admin, user, gen, uhits, encoding, javascript, msg );
            Err(CtxBody( TemplateGeneral::new("Error retrieving articles.".to_owned(), i) ))
        }
    }
    pub fn serve(tag: &str, 
                 pagination: &Page<Pagination>,
                 multi_aids: &TagAidsLock, 
                 article_state: &ArticleCacheLock, 
                 conn: &DbConn, 
                 admin: Option<AdministratorCookie>, 
                 user: Option<UserCookie>, 
                 uhits: Option<UniqueHits>, 
                 gen: Option<GenTimer>, 
                 encoding: Option<AcceptCompression>,
                 msg: Option<String>,
                ) -> Express {
        use ::sanitize::sanitize_tag;
        let t = sanitize_tag(tag);
        let javascript: Option<String> = None;
        cache::template( cache::pages::tag::context(&t, conn, &pagination, article_state, multi_aids, admin, user, uhits, gen, encoding, msg, javascript) )
    }
    // pub fn db_tag_aids(conn: &DbConn, tag: &str) -> Option<Vec<u32>> {
    // This function is used to fill the multi article cache.  
    // This is used to cache what articles correspond with each tag
    pub fn load_tag_aids(conn: &DbConn, tag: &str) -> Option<Vec<u32>> {
        // look up all ArticleId's for the given tag
        let result = conn.query(&format!("SELECT aid FROM articles WHERE '{}' = ANY(tag)", tag), &[]);
        if let Ok(rst) = result {
            let aids: Vec<u32> = rst.iter().map(|row| row.get(0)).collect();
            if aids.len() != 0 {
                Some(aids)
            } else {
                None
            }
        } else {
            None
        }
    }
    // pub fn lookup_articles(tag: &str, pagination: Page<Pagination>, multi_aids: &TagAidsLock, article_cache: ArticleCacheLock) -> Option<(Vec<u32>, u32)> {
    //     // multi_aids.retrieve_tag_aids(&format!("tag/{}", tag))
    //     // multi_aids.retrieve_aids(&format!("tag/{}", tag))
    //     // multi_aids.tag_articles(tag, starting, ending, multi_aids)
    //     multi_aids.tag_articles(article_cache, tag, pagination)
    // }
    // The fallback() should return the current page of articles and the total number of articles
    pub fn fallback(tag: &str, pagination: &Page<Pagination>, conn: &DbConn) -> Option<(Vec<Article>, u32)> {
        // conn.articles(&format!("SELECT a.aid, a.title, a.posted, a.body, a.tag, a.description, u.userid, u.display, u.username, a.image, a.markdown, a.modified  FROM articles a JOIN users u ON (a.author = u.userid) WHERE '{}' = ANY(tag)", tag))
        // Need to use collate's methods to help generate the SQL
        // use ArticleId.retrieve_with_conn(conn)
        unimplemented!()
    }
}

pub mod tags {
    use super::*;
    pub fn context(body: Option<Vec<TagCount>>, 
                   admin: Option<AdministratorCookie>, 
                   user: Option<UserCookie>, 
                   gen: Option<GenTimer>, 
                   uhits: Option<UniqueHits>, 
                   msg: Option<String>,
                   javascript: Option<String>
                  ) -> Result<CtxBody<TemplateTags>, CtxBody<TemplateGeneral>> 
    {
        unimplemented!()
    }
    pub fn serve(start: GenTimer, 
                 tag_lock: State<TagAidsLock>, 
                 conn: &DbConn, 
                 admin: Option<AdministratorCookie>, 
                 user: Option<UserCookie>, 
                 encoding: AcceptCompression, 
                 uhits: UniqueHits
                ) -> Express 
    {
        unimplemented!()
    }
    pub fn load_all_tags(conn: &DbConn) -> Option<Vec<TagCount>> {
        unimplemented!()
    }
    pub fn lookup_tags(cache: TagAidsLock) -> Option<Vec<TagCount>> {
        unimplemented!()
    }
    pub fn load_tagcloud(cache: &TagAidsLock) -> Option<String> {
        // unimplemented!()
        if let Some(mut tags) = cache.retrieve_tags() {
            // let mut tagcounts: Vec<TagCount> = Vec::new();
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
            
            for tag in all_tags {
                
                
                
            }
            
        } else {
            "Could not load tags.".to_owned()
        }
        
        
    }
    
    
}

pub mod author {
    use super::*;
    pub fn context(body: Option<Vec<Article>>, 
                   pagination: Page<Pagination>,
                   admin: Option<AdministratorCookie>, 
                   user: Option<UserCookie>, 
                   gen: Option<GenTimer>, 
                   uhits: Option<UniqueHits>, 
                   msg: Option<String>,
                   javascript: Option<String>
                  ) -> Result<CtxBody<TemplateArticlesPages>, CtxBody<TemplateGeneral>>
    {
        unimplemented!()
        
    }
    // Find all authors, their user id, their username, and display name
    // pub fn load_authors() -> Vec<(u32, String, String)> {
    // Find all authors' user ids
    // pub fn load_author_articles(conn: &DbConn, userid: u32) -> Option<Vec<u32>> {
    pub fn load_author_articles(conn: &DbConn, userid: u32) -> Option<Vec<u32>> {
        unimplemented!()
    }
    pub fn load_authors(conn: &DbConn) -> Vec<u32> {
        unimplemented!()
    }
}

pub mod rss {
    use super::*;
    // pub fn context(conn: &DbConn,
    //                article_cache: &ArticleCacheLock,
    //                multi_aids: &TagAidsLock,
    //                admin: Option<AdministratorCookie>, 
    //                user: Option<UserCookie>, 
    //                uhits: Option<UniqueHits>, 
    //                gen: Option<GenTimer>, 
    //                encoding: Option<AcceptCompression>,
    //                msg: Option<String>,
    //                javascript: Option<String>
    //               ) -> Result<CtxBody<TemplateArticlesPages>, CtxBody<TemplateGeneral>>
    // {
    //     unimplemented!()
    // }
    pub fn serve(conn: &DbConn, 
                //  pagination: &Page<Pagination>,
                //  multi_aids: &TagAidsLock, 
                //  article_state: &ArticleCacheLock, 
                 text_lock: TextCacheLock,
                 admin: Option<AdministratorCookie>, 
                 user: Option<UserCookie>, 
                 uhits: Option<UniqueHits>, 
                 gen: Option<GenTimer>, 
                 encoding: Option<AcceptCompression>,
                 msg: Option<String>,
                ) -> Express
    {
        let javascript: Option<String> = None;
        let content = text_lock.retrieve_text("rss").unwrap_or("Could not load RSS feed.".to_owned());
        let express: Express = content.into();
        // express.set_content_type(ContentType::XML).compress(encoding)
        express.set_content_type(ContentType::XML)
        
        // if let Some(rss) = text_lock.retrieve_text("rss") {
        // } else {
        //     
        // }
        // let i = info::info( Some("".to_owned(), "/rss".to_owned(), admin, user, gen, uhits, encoding, javascript, msg );
        // let ctx_body = CtxBody( TemplateGeneral::new(rss, i) )
        
        // unimplemented!()
    }
    pub fn load_rss(conn: &DbConn) -> String {
        use rss::{Channel, ChannelBuilder, Guid, GuidBuilder, Item, ItemBuilder, Category, CategoryBuilder, TextInput, TextInputBuilder, extension};
        use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
        use urlencoding::encode;
        
        // let rss: Express;
        
        
        let result = conn.articles("");
        if let Some(articles) = result {
            let mut article_items: Vec<Item> = Vec::new();
            for article in &articles {
                let mut link = String::with_capacity(BLOG_URL.len()+20);
                link.push_str(BLOG_URL);
                // link.push_str("article?aid=");
                link.push_str("article/");
                link.push_str(&article.aid.to_string());
                link.push_str("/");
                // let encoded = encode("This string will be URL encoded.");
                link.push_str( &encode(&article.title) );
                
                let desc: &str = if &article.description != "" {
                    &article.description
                } else {
                    if article.body.len() > DESC_LIMIT {
                        &article.body[..200]
                    } else {
                        &article.body[..]
                    }
                };
                
                let guid = GuidBuilder::default()
                    .value(link.clone())
                    .build()
                    .expect("Could not create article guid.");
                
                let date_posted = DateTime::<Utc>::from_utc(article.posted, Utc).to_rfc2822();
                
                let item =ItemBuilder::default()
                    .title(article.title.clone())
                    .link(link)
                    .description(desc.to_string())
                    // .author("Andrew Prindle".to_string())
                    .author(article.username.clone())
                    // .categories()
                    .guid(guid)
                    .pub_date(date_posted)
                    .build();
                    
                match item {
                    Ok(i) => article_items.push(i),
                    Err(e) => println!("Could not create rss article {}.  Error: {}", article.aid, e),
                }
            }
            // Items:
            // title    link    description author  categories  guid    pub_date
            // Channels:
            // title    link    description categories  language    copyright   rating  ttl
            let mut search_link = String::with_capacity(BLOG_URL.len()+10);
            search_link.push_str(BLOG_URL);
            search_link.push_str("search");
            
            let searchbox = TextInputBuilder::default()
                .title("Search")
                .name("q")
                .description("Search articles")
                .link(search_link)
                .build()
                .expect("Could not create text input item in RSS channel.");
            
            let channel = ChannelBuilder::default()
                .title("Vishus Blog")
                .link(BLOG_URL)
                .description("A programming and development blog about Rust, Javascript, and Web Development.")
                .language("en-us".to_string())
                .copyright("2017 Andrew Prindle".to_string())
                .ttl(720.to_string()) // half a day, 1440 minutes in a day
                .items(article_items)
                .text_input(searchbox)
                .build()
                .expect("Could not create RSS channel.");
            
            let rss_output = channel.to_string();
            let mut output = String::with_capacity(rss_output.len() + 30);
            output.push_str(r#"<?xml version="1.0"?>"#);
            output.push_str(&rss_output);
            output
            // let express: Express = output.into();
            // rss = express.set_content_type(ContentType::XML).compress(encoding);
            //     // set_ttl(-1)
            //     // .add_extra("Content-Type".to_string(), "application/rss+xml".to_string())
        } else {
            let output = String::from("Could not create RSS feed.");
            output
            // let express: Express = output.into();
            // // Do not need to compress output with such a small string.
            // // express.compress(encoding).set_content_type(ContentType::XML
            // rss = express;
        }
    }
}




































