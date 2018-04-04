
use std::{thread, time};
use std::time::Instant;
use std::time::Duration;
use std::{env, str, io};
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs::{self, File};

use rocket_contrib::Template;
use rocket::response::{content, NamedFile, Redirect, Flash};
use rocket::{Request, Data, Outcome};
use rocket::request::{FlashMessage, Form, FromForm};
use rocket::data::FromData;
use rocket::response::content::Html;
use rocket::State;
// use rocket::request::{Form, FlashMessage};
use rocket::http::{Cookie, Cookies, RawStr};
// use auth::userpass::UserPass;
// use auth::status::{LoginStatus,LoginRedirect};
// use auth::dummy::DummyAuthenticator;
// use auth::authenticator::Authenticator;
use regex::Regex;
use titlecase::titlecase;

use chrono::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, Arc, RwLock};

use rocket::http::hyper::header::{Headers, ContentDisposition, DispositionType, DispositionParam, Charset};



use super::*;
// use routes::*;
// use routes::pages::*;
use cache::*;
// use content::*;
use content::{destruct_cache, destruct_context};
use cache::body::*;
use cache::pages::*;
use counter::*;
use location::*;
use referrer::*;
use collate::*;
use layout::*;
use blog::*;
use data::*;
use sanitize::*;
use rocket_auth_login::authorization::*;
use rocket_auth_login::sanitization::*;
use ral_administrator::*;
use ral_user::*;
use templates::*;
use xpress::*;
use accept::*;
// use ::templates::*;


use comrak::{markdown_to_html, ComrakOptions};


fn err_file_name(name: &str) -> PathBuf {
    if let Ok(mut dir) = env::current_exe() {
        dir.pop();
        // println!("Climbing directory tree into: {}", &dir.display());
        dir.pop();
        // println!("Loading into directory: {}", &dir.display());
        // dir.push("log");
        // dir.set_file_name(name);
        if cfg!(target_os = "windows") {
            dir.set_file_name(&format!("logs\\{}", name));
        } else {
            dir.set_file_name(&format!("logs/{}", name));
        }
        // println!("Load file is: {}", &dir.display());
        dir
    } else {
        PathBuf::from(name)
    }
}

#[cfg(PRODUCTION)]
fn debug_timer(_: &GenTimer) { }

#[cfg(not(PRODUCTION))]
fn debug_timer(start: &GenTimer) {
    let end = start.0.elapsed();
    println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
}

#[inline]
fn fmsg(flash: Option<FlashMessage>) -> Option<String> {
    if let Some(flashmsg) = flash {
        Some(alert_info( flashmsg.msg() ))
    } else {
        None
    }
}


pub mod tagcloud {
    use super::*;

    // replaces route pages::hbs_tags_all
    #[get("/all_tags")]
    pub fn cache_tagcloud(start: GenTimer, 
                        multi_aids: State<TagAidsLock>,
                        conn: DbConn,
                        admin: Option<AdministratorCookie>,
                        user: Option<UserCookie>,
                        encoding: AcceptCompression,
                        uhits: UniqueHits
                    ) -> Express 
    {
        // unimplemented!()
        let express: Express = cache::pages::tags::serve(&conn,
                                                        &multi_aids,
                                                        admin,
                                                        user,
                                                        Some(uhits),
                                                        Some(start.clone()),
                                                        Some(encoding),
                                                        None
                                                        );
        debug_timer(&start);
        express.compress( encoding )
    }
}

pub mod tag {
    use super::*;

    #[get("/tag?<tag>")]
    pub fn cache_tag_redirect(tag: Tag) -> Redirect {
        Redirect::to(&format!("/tag/{}", tag.tag))
    }

    #[get("/tag/<tag>")]
    pub fn cache_tag(start: GenTimer,
                            tag: String,
                            multi_aids: State<TagAidsLock>, 
                            article_state: State<ArticleCacheLock>, 
                            pagination: Page<Pagination>,
                            conn: DbConn, 
                            admin: Option<AdministratorCookie>, 
                            user: Option<UserCookie>, 
                            encoding: AcceptCompression, 
                            uhits: UniqueHits
                        ) -> Express 
    {
        
        let express: Express = cache::pages::tag::serve(&tag, 
                                                        &pagination, 
                                                        &*multi_aids, 
                                                        &*article_state, 
                                                        &conn, 
                                                        admin, 
                                                        user, 
                                                        Some(uhits), 
                                                        Some(start.clone()), 
                                                        Some(encoding), 
                                                        None
                                                    );
        debug_timer(&start);
        express.compress( encoding )
    }
}

pub mod article {
    use super::*;

    #[get("/article/<aid>/<title>")]
    pub fn cache_article_title(start: GenTimer,
                               aid: ArticleId, 
                               title: Option<&RawStr>,
                               article_lock: State<ArticleCacheLock>,
                               conn: DbConn,
                               admin: Option<AdministratorCookie>,
                               user: Option<UserCookie>,
                               encoding: AcceptCompression,
                               uhits: UniqueHits
                              ) -> Express 
    {
        routes::article::cache_article_view(start, aid, article_lock, conn, admin, user, encoding, uhits)
    }

    #[get("/article/<aid>")]
    pub fn cache_article_id(start: GenTimer,
                            aid: ArticleId, 
                            article_lock: State<ArticleCacheLock>,
                            conn: DbConn,
                            admin: Option<AdministratorCookie>,
                            user: Option<UserCookie>,
                            encoding: AcceptCompression,
                            uhits: UniqueHits
                           ) -> Express 
    {
        routes::article::cache_article_view(start, aid, article_lock, conn, admin, user, encoding, uhits)
    }

    #[get("/article?<aid>")]
    pub fn cache_article_view(start: GenTimer, 
                              aid: ArticleId,
                              article_state: State<ArticleCacheLock>, 
                              conn: DbConn, 
                              admin: Option<AdministratorCookie>, 
                              user: Option<UserCookie>, 
                              encoding: AcceptCompression, 
                              uhits: UniqueHits
                          ) -> Express 
    {
        let express: Express = cache::pages::article::serve(aid.aid, 
                                                            article_state, 
                                                            &conn, 
                                                            admin, 
                                                            user, 
                                                            start.clone(),
                                                            uhits,
                                                            encoding, 
                                                            None
                                                        );
        debug_timer(&start);
        express.compress( encoding )
    }

    #[get("/article")]
    pub fn hbs_article_not_found(start: GenTimer, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
        // let start = Instant::now();
        let output: Template = hbs_template(TemplateBody::General(alert_danger("Article not found")), None, Some("Article not found".to_string()), String::from("/article"), admin, user, None, Some(start.0));
        let end = start.0.elapsed();
        // println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
        let express: Express = output.into();
        express.compress(encoding)
    }
}

pub mod rss {
    use super::*;

    #[get("/rss.xml")]
    pub fn cache_rss(start: GenTimer,
                    text_lock: State<TextCacheLock>,
                    conn: DbConn,
                    admin: Option<AdministratorCookie>,
                    user: Option<UserCookie>,
                    encoding: AcceptCompression,
                    uhits: UniqueHits
                ) -> Express {
        
        let express: Express = cache::pages::rss::serve(&conn,
                                                        &*text_lock,
                                                        admin,
                                                        user,
                                                        Some(uhits),
                                                        Some(start.clone()),
                                                        Some(encoding),
                                                        None
                                                    );
        
        debug_timer(&start);
        express.compress( encoding )
    }
}

pub mod author {
    use super::*;

    #[get("/author/<author>/<authorname>")]
    pub fn cache_author_seo(start: GenTimer,
                            author: u32, 
                            authorname: &RawStr,
                            pagination: Page<Pagination>,
                            multi_aids: State<TagAidsLock>,
                            article_lock: State<ArticleCacheLock>,
                            conn: DbConn,
                            admin: Option<AdministratorCookie>,
                            user: Option<UserCookie>,
                            encoding: AcceptCompression,
                            uhits: UniqueHits
                           ) -> Express {
        routes::author::cache_author(start, author, pagination, multi_aids, article_lock, conn, admin, user, encoding, uhits)
    }

    #[get("/author/<author>")]
    pub fn cache_author(start: GenTimer,
                        author: u32,
                        pagination: Page<Pagination>,
                        multi_aids: State<TagAidsLock>,
                        article_lock: State<ArticleCacheLock>,
                        conn: DbConn,
                        admin: Option<AdministratorCookie>,
                        user: Option<UserCookie>,
                        encoding: AcceptCompression,
                        uhits: UniqueHits
                       ) -> Express {
        
        let express: Express = cache::pages::author::serve(author, 
                                                        &pagination, 
                                                        &conn, 
                                                        &multi_aids, 
                                                        &article_lock,
                                                        admin,
                                                        user,
                                                        Some(uhits),
                                                        Some(start.clone()),
                                                        Some(encoding),
                                                        None
                                                        );
        
        debug_timer(&start);
        express.compress( encoding )
        
    }
}

pub mod articles {
    use super::*;

    #[get("/")]
    pub fn cache_index(start: GenTimer, 
                       pagination: Page<Pagination>,
                       article_lock: State<ArticleCacheLock>,
                       conn: DbConn, 
                       admin: Option<AdministratorCookie>, 
                       user: Option<UserCookie>, 
                       flash: Option<FlashMessage>, 
                       encoding: AcceptCompression, 
                       uhits: UniqueHits
                      ) -> Express 
    {
        let fmsg = fmsg(flash);
        let page_info: Option<String> = None;
        
        let express: Express = cache::pages::articles::serve(&*article_lock, 
                                                             pagination, 
                                                             &conn, 
                                                             admin, 
                                                             user, 
                                                             start.clone(), 
                                                             uhits, 
                                                             encoding, 
                                                             fmsg, 
                                                             page_info
                                                            );
        
        debug_timer(&start);
        express.compress( encoding )
        
    }
}

































