
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


pub mod tags {
    use super::*;
    // pub fn 
    
}

pub mod article {
    use super::*;
    pub fn serve(aid: u32, start: GenTimer, article_state: State<ArticleCacheLock>, conn: &DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
        let article_rst = article_state.retrieve_article(aid);
        
        // Is this really needed?  Maybe just inline the article() here instead of a call to it
        let ctx: Result<CtxBody<TemplateArticle>, CtxBody<TemplateGeneral>> = cache::body::article(article_rst, admin, user, Some(uhits), Some(start), None);
        
        let express: Express = cache::template(ctx);
        express
    }
}

pub mod tag {
    use super::*;
    pub fn serve(tag: &str, start: GenTimer, multi_aids: State<MultiArticlePagesLock>, article_state: State<ArticleCacheLock>, conn: &DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
        // let aids = 
        // if let Some() = lookup_aids() {
            
        // }
        // let articles_rst = article_state.retrieve_articles(aids);
        unimplemented!()
    }
    pub fn db_tag_aids(conn: &DbConn, tag: &str) -> Option<Vec<u32>> {
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
    pub fn lookup_aids(tag: &str, multi_aids: &MultiArticlePagesLock) -> Option<Vec<u32>> {
        multi_aids.retrieve_tag_aids(&format!("tag/{}", tag))
    }
}


















