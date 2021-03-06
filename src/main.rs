
/* Todo:
  JOIN
    Add a signup page
    
  COMMENTS
    Add comments for logged in users
      -Users that are not logged in will just see a link to 
          Login to Comment instead of a post new comment form
      -List all comments below the article
      -Allow admin users to delete comments
      -Routes: Admin Users: can create comments and delete comments
               Regular users: can create comments and delete own comments
               Public: can view comments and see a link to login to post comments   

*/


#![feature(entry_and_modify)]
#![feature(custom_derive)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
// #![plugin(dotenv_macros)]

// extern crate multipart;
extern crate rocket;
extern crate rocket_contrib;
// extern crate rocket_simpleauth as auth;
extern crate serde;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate serde_yaml;
extern crate rmp_serde as rmps;

extern crate chrono;
#[macro_use] extern crate lazy_static;
extern crate regex;
// extern crate time;

extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;

// extern crate rocket_file_cache;
// extern crate concurrent_hashmap;
// extern crate chashmap;
// Markdown crates:
// - comrak has more features - allows inline html and autolink and tasklists and strikethrough and tagfilter
//                              and footnotes and superscript and tables and hardbreaks and more
// - pulldown-cmark is faster - allows inline html - can allow tables and footnotes
// - markdown - DO NOT USE THE MARKDOWN CRATE - it is extremely slow and doesn't support many features
extern crate comrak;
extern crate twoway;
// extern crate pulldown_cmark;
extern crate chashmap;

extern crate libflate;
extern crate brotli;
extern crate zopfli;

extern crate urlencoding;
extern crate titlecase;
// extern crate handlebars;
#[allow(unused_imports)]
extern crate htmlescape;
extern crate rss;
extern crate evmap;

// extern crate dotenv;

// #[macro_use] extern crate log;
// extern crate env_logger;
// #[macro_use] extern crate diesel_codegen;
// #[macro_use] extern crate diesel;

extern crate rocket_auth_login;

// mod vcache;
// mod counthits;
mod cache;
mod routes;
mod counter;
// mod static_pages;
mod content;
mod referrer;
mod location;
mod collate;
mod accept;
mod xpress;
mod layout;
mod blog;
mod data;
// mod hbs_templates;
mod templates;
mod pages;
mod sanitize;
mod ral_administrator;
mod ral_user;
// mod pages_administrator;
// mod templates;
// mod cookie_data;
// mod admin_auth;
// mod user_auth;
// mod users;
// mod login_form_status;

// use cache::*;
// use vcache::*;
// use counthits::*;
use routes::*;
use blog::*;
use cache::*;
use counter::*;
// use counter::*;
use xpress::*;
use accept::*;
use content::*;
use ral_administrator::*;
// use pages_administrator::*;
use data::*;
use pages::*;
// use templates::TemplateMenu;
use rocket_auth_login::authorization::*;
use rocket_auth_login::*;
use rocket_auth_login::sanitization::*;
use templates::*;

// use handlebars::Handlebars;
use regex::Regex;
use titlecase::titlecase;
use comrak::{markdown_to_html, ComrakOptions};

use std::time::{Instant, Duration, SystemTime};
use std::ffi::OsStr;
use std::{env, str, io};
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, Arc, RwLock};
use std::collections::HashMap;

// use std::cell::Cell;
// use std::rc::Rc;

// use chashmap::*;

// Rocket File Cache - Removed
// use rocket_file_cache::{Cache, CachedFile};
// use std::sync::Mutex;
// use std::path::{Path, PathBuf};
// use rocket::State;
// use concurrent_hashmap::*;


use rocket_contrib::Template;
use rocket::{Request, Data, Outcome, Response};
// use rocket::response::{content, NamedFile, Redirect, Flash, Responder, Content};
use rocket::response::{NamedFile, Redirect, Flash, Responder, Content};
use rocket::response::content::Html;
use rocket::data::FromData;
use rocket::request::{FlashMessage, Form, FromForm, FormItems, FromRequest};
use rocket::http::{Cookie, Cookies, MediaType, ContentType, Status};
use rocket::State;


// use auth::userpass::UserPass;
// use auth::status::{LoginStatus,LoginRedirect};
// use auth::dummy::DummyAuthenticator;
// use auth::authenticator::Authenticator;

// use chrono::prelude::*;
// use multipart::server::Multipart;


// BLOG_URL MUST HAVE A TRAILING FORWARD SLASH /
// pub const BLOG_URL: &'static str = "http://localhost:8000/";


// pub const BLOG_URL: &'static str = dotenv!("BLOG_URL");
// pub const USER_LOGIN_URL: &'static str = dotenv!("USER_LOGIN_URL");
// pub const ADMIN_LOGIN_URL: &'static str = dotenv!("ADMIN_LOGIN_URL");
// pub const TEST_LOGIN_URL: &'static str = dotenv!("TEST_LOGIN_URL");
// pub const CREATE_FORM_URL: &'static str = dotenv!("CREATE_FORM_URL");
// pub const EDIT_FORM_URL: &'static str = dotenv!("EDIT_FORM_URL");
// pub const MANAGE_URL: &'static str = dotenv!("MANAGE_URL");
// pub const MAX_CREATE_TITLE: usize = 120;
// pub const MAX_CREATE_DESCRIPTION: usize = 400;
// pub const MAX_CREATE_TAGS: usize = 250;
// pub const DATABASE_URL: &'static str = dotenv!("DATABASE_URL");
// const LOCKOUT_DURATION: u32 = 900; // 6 seconds // 900 seconds = 15 minutes
// const MAX_ATTEMPTS: i16 = 8; // 8


// #[cfg(production)]
// pub const BLOG_URL: &'static str = "http://127.0.0.1:8000/";
// #[cfg(not(production))]
// pub const BLOG_URL: &'static str = "http://localhost:8000/";


// Global settings are separated into a file called settings.rs
// This separation allows exclusion of the settings file from
//   things like git repos and other publicly viewable areas.
//   This allows passwords and server information to be kept
//   safe and secure while the rest of the project is uploaded
//   and can be viewed publicly.
include!("settings.rs");


// lazy_static! {
//     static ref B_URL: &'static str = ;
// }



#[get("/<file..>", rank=10)]
fn static_files(file: PathBuf, encoding: AcceptCompression) -> Option<Express> {
    // if let Some(named) = NamedFile::open(Path::new("static/").join(file)).ok() {
    if let Some(named) = NamedFile::open(Path::new("static/").join(file)).ok() {
        let exp: Express = named.into();
        Some( exp.compress(encoding) )
    } else {
        None
    }
}



// #[get("/<file..>", rank=10)]
// fn static_files(file: PathBuf, encoding: AcceptCompression) -> Option<Express> {
//     // if let Some(named) = NamedFile::open(Path::new("static/").join(file)).ok() {
//     if let Some(named) = NamedFile::open(Path::new("static/").join(file)).ok() {
//         let cached = CachedFile::open(named.path(), cache.inner());
        
//         // let exp: Express = named.into();
//         let exp: Express = cached.into();
//         Some( exp.comprses(encoding) )
//     } else {
//         None
//     }
// }


// }
// #[get("/<file..>")]
// fn files(file: PathBuf, cache: State<Cache> ) -> Option<CachedFile> {
//     CachedFile::open(Path::new("www/").join(file), cache.inner())
// }

// #[get("/<file..>", rank=10)]
// // fn static_files(file: PathBuf, encoding: AcceptCompression) -> Option<Express> {
// fn static_files(file: PathBuf, vcache: State<VCache>, encoding: AcceptCompression) -> Option<Express> {
// // fn static_files(file: PathBuf) -> Option<NamedFile> {
//     // Without Expiration header:
//     // NamedFile::open(Path::new("static/").join(file)).ok()
//     let start = Instant::now();
//     if let Some(named) = NamedFile::open(Path::new("static/").join(file)).ok() {
//         let result: Option<Express>;
//         if let Some(output) = VCache::retrieve(named.path().to_path_buf(), vcache) {
//             // let express: Express = output.into();
            
//             let ctype = ContentType::from_extension(named.path().extension().unwrap_or(OsStr::new("")).to_str().unwrap_or("")).unwrap_or(ContentType::Plain);
//             let express: Express = output.into();
//             // express.compress(encoding)
//             result = Some( express.set_content_type(ctype).compress(encoding) );
            
//         } else {
//             println!("Cache retrieve failed, falling back to named file");
//             let exp: Express = named.into();
//             result = Some( exp.compress(encoding) );
//         }
        
//         let end = start.elapsed();
//         println!("Served static file in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
        
//         result
//         // let exp: Express = named.into();
//         // Some( exp )
//     } else {
//         None
//     }
    
// }


#[error(404)]
pub fn error_not_found(req: &Request) -> Express {
    // let hits = req.guard::<Hits>();
    ErrorHits::error404(req);
    
    let content = format!( "The request page `{}` could not be found.", sanitize_text(req.uri().as_str()) );
    let output = hbs_template(TemplateBody::General(content), None, Some("404 Not Found.".to_string()), String::from("/404"), None, None, None, None);
    let express: Express = output.into();
    express
}
#[error(500)]
pub fn error_internal_error(req: &Request) -> Express {
    // let hits = req.guard::<Hits>();
    ErrorHits::error500(req);
    
    
    let content = format!( "An internal server error occurred procesing the page `{}`.", sanitize_text(req.uri().as_str()) );
    let output = hbs_template(TemplateBody::General(content), None, Some("500 Internal Error.".to_string()), String::from("/500"), None, None, None, None);
    let express: Express = output.into();
    express
}

lazy_static! {
    static ref PGCONN: Mutex<DbConn> = Mutex::new( DbConn(init_pg_pool().get().expect("Could not connect to database.")) );
}


fn main() {
    // env_logger::init();
    // init_pg_pool();
    // if let Ok(pg_conn) = init_pg_pool().get() {
    //     pg_conn;
    //     // (*pg_conn).connect();
    // }
    
    // dotenv().ok();
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    if PRODUCTION {
        println!("Production mode");
    } else {
        println!("Dev mode");
    }
    // let hitcount: Counter = Counter::new();
    // let views: TotalHits = TotalHits::new();
    let hitcount: Counter = Counter::load();
    let views: TotalHits = TotalHits::load();
    let uhits: UStatsWrapper = UStatsWrapper( RwLock::new( UniqueStats::load() ) );
    
    // let init_pg_pool().get().unwrap();
    let mut pg_pool = data::init_pg_pool();
    
    let conn;
    if let Ok(pooled_conn) = pg_pool.get() {
        conn = DbConn(pooled_conn);
    } else {
        panic!("Connection could not be retrieved from db connection pool")
    }
    
    
    let rock = rocket::ignite();
    // let rock = Rc::new(rocket::ignite());
    // let mut rock = Rc::new(rocket::ignite());
    // let mut rock = Cell::new(rocket::ignite());
    
    // let statics: Mutex<PageMap> = Mutex::new(PageMap::load_all());
    // let statics: PagesMutex = PagesMutex( Mutex::new(  PageMap::load_all()  ) );
    // let statics: PagesMutex = PagesMutex( RwLock::new(  PageContextMap::load_all()  ) );
    let content_context: ContentContext = ContentContext::load(STATIC_PAGES_DIR);
    let content_cache: ContentCacheLock = ContentCacheLock::new();
    
    
    // let map_articles: HashMap<u32, Article>;
    // if let Some(articles) = routes::load_articles_map(&conn) {
    //     map_articles = articles;
    // } else {
    //     panic!("Could not load articles map from database.");
    // }
    
    // let article_map_cache = ArticleCacheLock{ lock: RwLock::new( ArticleCache{ articles: map_articles } ) };
    
    // Cache Todo:: if CACHE_ENABLED is false, make blank structs
    let article_map_cache = ArticleCacheLock::new( ArticleCache::load_cache(&conn) );
    // let multi_aids = TagAidsLock::new( TagAids::load_cache(&conn) );
    // let multi_aids = TagAidsLock::new( AidsCache::load_cache(&conn), TagsCache::load_cache(&conn) );
    let multi_aids = TagAidsLock::load_cache(&conn);
    let text_cache = TextCacheLock::new( TextCache::load_cache(&conn, &multi_aids) );
    let num_articles = NumArticles( article_map_cache.num_articles() );
    /*
    
    all_tags
    /tag/<tag>
        /tag?<tag>
    
    /article?<aid>
        /article/<aid>
        /article/<aid>/<title>
    /article (hbs_article_not_found)
    /rss.xml
    /author/<authorid>
        /author/<authorid>/<authorname>
    /about
    
    
    /pageviews
    /pagestats
    /pagestats/<show_errors>
    /manage/<sortstr>/<orderstr>
    /manage
    
    
    
    */
    
    // let content_cache: ContentCacheLock = ContentCacheLock::cache(rock, STATIC_PAGES_DIR);
    
    // let hitcount: PageCount = PageCount::new();
    // let views: ViewsTotal = ViewsTotal::new();
    
    // let hitcount: PageCount = PageCount::load();
    // let views: ViewsTotal = ViewsTotal::load();
    
    
    // let vcache: VCache = VCache(CHashMap::new());
    
    // init_pg_pool().get().unwrap();
    
    // rocket::ignite()
    if CACHE_ENABLED {
        println!("Starting blog.  Cache enabled.");
    } else {
        println!("Starting blog.  Cache is DISABLED.");
    }
        
    rock
        // .manage(vcache)
        
        // .manage(data::init_pg_pool())
        .manage(pg_pool)
        .manage(hitcount)
        .manage(views)
        .manage(uhits)
        .manage(content_context)
        .manage(content_cache)
        
        .manage(article_map_cache)
        .manage(text_cache)
        .manage(multi_aids)
        .manage(num_articles)
        
        .attach(Template::fairing())
        
        .mount("/", routes![
            
            
            pages::test_article,
            pages::test_tag,
            pages::test_author,
            pages::test_rss,
            pages::test_tagcloud,
            pages::test_home,
            
            pages::static_pages,
            pages::code_download,
            pages::refresh_content,
            
            // pages::hbs_view_articles,
            // old_pages::hbs_tags_all,
            routes::tagcloud::cache_tagcloud,
            // old_pages::hbs_articles_tag_redirect,
            routes::tag::cache_tag_redirect,
            // old_pages::hbs_articles_tag,
            routes::tag::cache_tag,
            // old_pages::hbs_article_title,
            routes::article::cache_article_title,
            // old_pages::hbs_article_id,
            routes::article::cache_article_id,
            // old_pages::hbs_article_view,
            routes::article::cache_article_view,
            // old_pages::hbs_article_not_found,
            routes::article::hbs_article_not_found,
            
            pages::hbs_article_process,
            pages::hbs_create_unauthorized,
            pages::hbs_create_form,
            pages::hbs_edit,
            pages::hbs_edit_process,
            pages::hbs_delete_confirm,
            pages::hbs_process_delete,
            
            // pages::hbs_search_page,
            pages::hbs_search_redirect,
            pages::hbs_search_results,
            // old_pages::hbs_author_display,
            routes::author::cache_author_seo,
            // old_pages::hbs_author,
            routes::author::cache_author,
            pages::hbs_about,
            // old_pages::rss_page,
            routes::rss::cache_rss,
            // old_pages::hbs_index,
            routes::articles::cache_index,
            
            pages::hbs_manage_basic,
            pages::hbs_manage_full,
            
            pages::hbs_dashboard_admin_authorized,
            pages::hbs_dashboard_admin_flash,
            pages::hbs_dashboard_admin_retry_user,
            pages::hbs_process_admin_login,
            pages::hbs_logout_admin,
            
            pages::hbs_admin_test,
            // pages::hbs_admin_test_unauthorized,
            pages::hbs_dashboard_admin_retry_redir,
            pages::hbs_dashboard_admin_retry_redir_only,
            
            pages::backup,
            pages::hbs_pageviews,
            
            pages::hbs_dashboard_user_authorized,
            pages::hbs_dashboard_user_retry_user,
            pages::hbs_dashboard_user_flash,
            pages::hbs_process_user_login,
            pages::hbs_logout_user,
            
            pages::hbs_edit_unauthorized,
            pages::hbs_manage_unauthorized,
            pages::hbs_delete_unauthorized,
            pages::hbs_pageviews_unauthorized,
            pages::hbs_backup_unauthorized,
            
            pages::hbs_pagestats,
            pages::hbs_pagestats_unauthorized,
            pages::hbs_pagestats_no_errors,
            
            // pages::test_cache,,
            
            // pages_administrator::resp_test,
            // pages_administrator::uncompressed,
            // pages_administrator::compress_test,
            // pages_administrator::compress_test2,
            // pages_administrator::compress_test3,
            // pages_administrator::compress_test4,
            // pages_administrator::compress_gzip,
            // pages_administrator::compress_deflate,
            // pages_administrator::compress_brotli,
            
            // pages::hit_count,
            // pages::hit_count2,
            // pages::hit_count3,
            
            static_files
        ])
        .catch(errors![ error_internal_error, error_not_found ])
        
    // DOES NOT WORK - moved values and stuff :(
    //     ;
    //     {
    //         ContentCacheLock::cache(&mut rock, STATIC_PAGES_DIR);
    //     }
    // rock
        .launch();
}


