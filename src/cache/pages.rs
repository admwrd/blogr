
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
        
        unimplemented!()
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
        
        macro_rules! ctx_info {
            ( $title:expr, $page:expr ) => {
                info::info(if $title == "" { None } else { Some($title.to_owned()) }, $page.to_owned(), admin, user, gen, uhits, encoding, javascript, msg)
            }
        }
        
        
        if let Some(article) = body {
            // let i = ctx_info!("Article", "/");
            let i = info::info(Some(article.title.clone()), "/article".to_owned(), admin, user, gen, uhits, encoding, javascript, msg);
            Ok(CtxBody( TemplateArticle::new(article, i) ))
        } else if let Some(article) = cache::pages::article::fallback(aid, conn) {
            let i = info::info(Some(article.title.clone()), "/article".to_owned(), admin, user, gen, uhits, encoding, javascript, msg);
            if !PRODUCTION {
                println!("Article {} served from fallacbk instead of cache", aid);
            }
            Ok(CtxBody( TemplateArticle::new(article, i) ))
        } else {
            // let i = ctx_info!("", "/");
            let i = info::info(Some(format!("Article {} not found", aid)), "/article".to_owned(), admin, user, gen, uhits, encoding, javascript, msg);
            Err(CtxBody( TemplateGeneral::new("The article could not be found.".to_owned(), i) ))
        }
    }
    // pub fn context_fallback(aid: u32, conn: &DbConn) -> Option<CtxBody<TemplateArticle>> {
    pub fn fallback(aid: u32, conn: &DbConn) -> Option<Article> {
        // unimplemented!()
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
    // pub fn fallback(aid: u32, start: GenTimer, article_state: State<ArticleCacheLock>, conn: &DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
    // pub fn fallback(aid: u32, 
    //                 start: GenTimer, 
    //                 article_state: State<ArticleCacheLock>, 
    //                 conn: &DbConn, 
    //                 admin: Option<AdministratorCookie>, 
    //                 user: Option<UserCookie>, 
    //                 encoding: AcceptCompression, 
    //                 uhits: UniqueHits
    //                ) -> Result<CtxBody<TemplateArticle>, CtxBody<TemplateGeneral>> {
    //     // unimplemented!()
    //     // Todo: Add an actual fallback implementation here
    //     //         it should query the database looking for
    //     //         the requested article and return it
    //     Err(CtxBody( TemplateGeneral::new("The article could not be found.".to_owned(), i) ))
    // }
    
}

pub mod tag {
    use super::*;
    pub fn context(tag: &str,
                //    body: Option<Vec<Article>>,
                   pagination: Page<Pagination>,
                //    total_items: u32, // 0 if tag not found
                   article_state: State<ArticleCacheLock>,
                   admin: Option<AdministratorCookie>, 
                   user: Option<UserCookie>, 
                   uhits: Option<UniqueHits>, 
                   gen: Option<GenTimer>, 
                   encoding: Option<AcceptCompression>,
                   msg: Option<String>,
                   javascript: Option<String>
                  ) -> Result<CtxBody<TemplateArticlesPages>, CtxBody<TemplateGeneral>>
    {
        // unimplemented!()
        // let output: Result<Vec<Article>, String>;
        
        if CACHE_ENABLED {
            if let Some((articles, total_items)) = multi_aids.tag_articles(tag, pagination) {
                let javascript: Option<String> = None;
                let info_opt: Option<String> = None;
                let i = info::info( Some(format!("Showing articles with tag '{}'", &tag)), "/tag".to_owned(), admin, user, gen, uhits, encoding, javascript, msg );
                Ok(CtxBody( TemplateArticlesPages::new(articles, pagination, total_items, info_opt, i) ))
            } else {
                let i = info::info( Some(format!("No articles to display for tag '{}'", &tag)), "/tag".to_owned(), admin, user, gen, uhits, encoding, javascript, msg );
                Err(CtxBody( TemplateGeneral::new(format!("No artiles displayed for tag {}", tag), i) ))
            }
            
        } else if CACHE_FALLBACK {
            if let Some((articles, total_items)) = cache::pages::tag::fallback(tag, pagination, conn) {
                let javascript: Option<String> = None;
                let info_opt: Option<String> = None;
                let i = info::info( Some(format!("Showing articles with tag '{}'", &tag)), "/tag".to_owned(), admin, user, gen, uhits, encoding, javascript, msg );
                Ok(CtxBody( TemplateArticlesPages::new(articles, pagination, total_items, info_opt, i) ))
            } else {
                let i = info::info( Some(format!("No articles to display for tag '{}'", &tag)), "/tag".to_owned(), admin, user, gen, uhits, encoding, javascript, msg );
                Err(CtxBody( TemplateGeneral::new(format!("No artiles displayed for tag {}", tag), i) ))
            }
        } else {
            println!("SUPER ERROR: Cache disabled and cache fallback disabled");
            let i = info::info( Some("Error".to_owned()), "/tag".to_owned(), admin, user, gen, uhits, encoding, javascript, msg );
            Err(CtxBody( TemplateGeneral::new("Error retrieving articles.".to_owned(), i) ))
        }
        
        // let i = info::info( Some(format!("Showing Articles with tag '{}'", sanitize_tag(&tag))), "/tag".to_owned(), admin, user, gen, uhits, encoding, javascript, msg );
        // return CtxBody( TemplateGeneral("No articles were found for the given tag.".to_owned(), i) );
        // let mut total_items: u32 = 0;
        // // if CACHE_ENABLED {
        //     if let Some(articles) = body {
        //         output = Ok(articles);
        //     } else {
        //         output = Err("No articles found for the given tag.".to_owned());
        //     }
        // } else if CACHE_FALLBACK {
        //     if let Some(articles) = cache::pages::tag::fallback(tag, conn) {
        //         output = Ok(articles);
        //     } else {
        //         if !PRODUCTION { println!(); }
        //         output = Err("No articles found for the given tag.".to_owned());
        //     }
        // } else {
        //     println!("Really bad error: cache and fallback are both disabled.");
        //     output = Err("Error: cache and database lookup disabled.".to_owned();
        // }
        // match output {
        //     Ok(articles) => {
        //         Ok(CtxBody( TemplateArticlesPages::new(articles, pagination, total) ))
        //     },
        //     Err(err) => {
        //         let i = ;
        //         Err(CtxBody( TemplateGeneral::new(err.to_owned(), i) ))
        //     },
        // }
    }
    // ADD ARTICLE CACHE TO SERVE() AND CONTEXT()
    pub fn serve(tag: &str, 
                 start: GenTimer, 
                 multi_aids: State<TagAidsLock>, 
                 article_state: State<ArticleCacheLock>, 
                 conn: &DbConn, 
                 admin: Option<AdministratorCookie>, 
                 user: Option<UserCookie>, 
                 uhits: Option<UniqueHits>, 
                 gen: Option<GenTimer>, 
                 encoding: Option<AcceptCompression>,
                 msg: Option<String>,
                ) -> Express {
        use ::sanitize::sanitize_tag;
        // let output: Result<(Vec<Article>, Page<Pagination>, u32), String>;
        let t = sanitize_tag(tag);
        cache::template( cache::pages::tag::context(t, pagination, admin, user, uhits, gen, encoding, msg, javascript) )
    }
    // pub fn db_tag_aids(conn: &DbConn, tag: &str) -> Option<Vec<u32>> {
    // This function is used to fill the multi article cache.  
    // This is used to cache what articles correspond with each tag
    pub fn load_tag_aids(conn: &DbConn, tag: &str) -> Option<Vec<u32>> {
        // unimplemented!()
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
    // pub fn lookup_aids(tag: &str, starting: u32, ending: u32, multi_aids: &TagAidsLock) -> Option<(Vec<u32>, u32)> {
    //     // multi_aids.retrieve_tag_aids(&format!("tag/{}", tag))
    //     // multi_aids.retrieve_aids(&format!("tag/{}", tag))
    //     multi_aids.tag_articles(tag, starting, ending, multi_aids)
    // }
    pub fn fallback(tag: &str, pagination: Page<Pagination>, conn: &DbConn) -> Option<Vec<Article>> {
        // conn.articles(&format!("SELECT a.aid, a.title, a.posted, a.body, a.tag, a.description, u.userid, u.display, u.username, a.image, a.markdown, a.modified  FROM articles a JOIN users u ON (a.author = u.userid) WHERE '{}' = ANY(tag)", tag))
        // Need to use collate's methods to help generate the SQL
        // use ArticleId.retrieve_with_conn(conn)
        
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
    pub fn serve(start: GenTimer, tag_lock: State<TagAidsLock>, conn: &DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
        unimplemented!()
    }
    pub fn load_all_tags(conn: &DbConn) -> Option<Vec<TagCount>> {
        unimplemented!()
    }
    pub fn lookup_tags(cache: TagAidsLock) -> Option<Vec<TagCount>> {
        unimplemented!()
    }
    pub fn load_tagcloud(cache: TagAidsLock) -> String {
        unimplemented!()
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
        
    }
    pub fn load_authors(conn: &DbConn) -> Vec<u32> {
        unimplemented!()
    }
}






































