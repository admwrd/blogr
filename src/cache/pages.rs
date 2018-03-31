
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
    pub fn context(body: Option<Article>, 
                   admin: Option<AdministratorCookie>, 
                   user: Option<UserCookie>, 
                   gen: Option<GenTimer>, 
                   uhits: Option<UniqueHits>, 
                   encoding: Option<AcceptCompression>, 
                   msg: Option<String>
                  ) -> Result<CtxBody<TemplateArticle>, CtxBody<TemplateGeneral>>
    {
        let javascript: Option<String> = None;
        // call info::info()
        // call TemplateArticle::new() with the result from info::info()s
        // let info = |title: Into<String>, page: Into<String>| {
        let info = |title: &str, page: &str| {
            // let t_opt: Option<String> = if title == "" { None } else { let temp: String = title.into(); Some(temp) };
            let t_opt: Option<String> = if title == "" { None } else { Some(title.to_owned()) };
            // let p_opt: Option<String> = if page == "" { None } else { let temp: String = page.into(); Some(temp) };
            // let p_opt: Option<String> = if page == "" { None } else { Some(page.to_owned()) };
            let p: String = page.to_owned();
            // let t: String = title.into();
            // let p: String = page.into();
            info::info(t_opt, p, admin, user, gen, uhits, encoding, javascript, msg)
        };
        
        let i = info("Article", "Blah");
        if let Some(article) = body {
            Ok(CtxBody( TemplateArticle::new(article, i) ))
        } else {
            Err(CtxBody( TemplateGeneral::new("The article could not be found.".to_owned(), i) ))
        }
        
        // unimplemented!()
    }
    pub fn serve(aid: u32, 
                 start: GenTimer, 
                 article_state: State<ArticleCacheLock>, 
                 conn: &DbConn, 
                 admin: Option<AdministratorCookie>, 
                 user: Option<UserCookie>, 
                 encoding: AcceptCompression, 
                 uhits: UniqueHits
                ) -> Express 
    {
        let article_rst = article_state.retrieve_article(aid);
        
        // Is this really needed?  Maybe just inline the article() here instead of a call to it
        let ctx: Result<CtxBody<TemplateArticle>, CtxBody<TemplateGeneral>>
            //  = cache::body::article(article_rst, 
             = cache::pages::article::context(article_rst, 
                                              admin, 
                                              user, 
                                              Some(start), 
                                              Some(uhits), 
                                              Some(encoding),
                                              None
                                             );
        
        let express: Express = cache::template(ctx);
        express
    }
    pub fn fallback(aid: u32, start: GenTimer, article_state: State<ArticleCacheLock>, conn: &DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
        unimplemented!()
    }
    
}

pub mod tag {
    use super::*;
    pub fn context(body: Option<Vec<Article>>,
                   pagination: Page<Pagination>,
                   admin: Option<AdministratorCookie>, 
                   user: Option<UserCookie>, 
                   uhits: Option<UniqueHits>, 
                   gen: Option<GenTimer>, 
                   msg: Option<String>
                  ) -> Result<CtxBody<TemplateArticlesPages>, CtxBody<TemplateGeneral>>
    {
        unimplemented!()
        
    }
    pub fn serve(tag: &str, start: GenTimer, multi_aids: State<TagAidsLock>, article_state: State<ArticleCacheLock>, conn: &DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
        // let aids = 
        // if let Some() = lookup_aids() {
            
        // }
        // let articles_rst = article_state.retrieve_articles(aids);
        unimplemented!()
    }
    // pub fn db_tag_aids(conn: &DbConn, tag: &str) -> Option<Vec<u32>> {
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
    pub fn lookup_aids(tag: &str, multi_aids: &TagAidsLock) -> Option<Vec<u32>> {
        // multi_aids.retrieve_tag_aids(&format!("tag/{}", tag))
        multi_aids.retrieve_aids(&format!("tag/{}", tag))
    }
}

pub mod tags {
    use super::*;
    pub fn context(body: Option<Vec<TagCount>>, 
                   admin: Option<AdministratorCookie>, 
                   user: Option<UserCookie>, 
                   uhits: Option<UniqueHits>, 
                   gen: Option<GenTimer>, 
                   msg: Option<String>
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
                   uhits: Option<UniqueHits>, 
                   gen: Option<GenTimer>, 
                   msg: Option<String>
                  ) -> Result<CtxBody<TemplateArticlesPages>, CtxBody<TemplateGeneral>>
    {
        unimplemented!()
        
    }
}






































