
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

// use super::{BLOG_URL, ADMIN_LOGIN_URL, USER_LOGIN_URL, CREATE_FORM_URL, TEST_LOGIN_URL};

// use super::RssContent;
// use cookie_data::*;
// use cookie_data::CookieId;
// use admin_auth::*;
// use user_auth::*;
// use users::*;
// use login_form_status::*;
// use login_form_status::LoginFormRedirect;
// use templates::*;
// use authorize::*;
// use administrator::*;
// use roles::*;
// use counter::*;
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

// pub const COMRAK_OPTIONS: ComrakOptions = ComrakOptions {
//     hardbreaks: true,            // \n => <br>\n
//     width: 120usize,             
//     github_pre_lang: false,      
//     ext_strikethrough: true,     // hello ~world~ person.
//     ext_tagfilter: true,         // filters out certain html tags
//     ext_table: true,             // | a | b |\n|---|---|\n| c | d |
//     ext_autolink: true,          
//     ext_tasklist: true,          // * [x] Done\n* [ ] Not Done
//     ext_superscript: true,       // e = mc^2^
//     ext_header_ids: None,        // None / Some("some-id-prefix-".to_string())
//     ext_footnotes: true,         // Hi[^x]\n\n[^x]: A footnote here\n
// };


// TODO: Collate: make a route that takes a number in the route (not query string)
//                use this number to determine how many pages to list
//                on each page say the page number as determined in the Page structure










// output\.into\(\)\.compress\(encoding\)
// let express: Express = output.into();
//   express.compress(encoding)

// #[get("/login-user")]
// fn hbs_login_form_admin(start: GenTimer, conn: DbConn, user: Option<UserCookie>, flash_msg_opt: Option<FlashMessage>, encoding: AcceptCompression, referrer: Referrer) -> Express {
    
    // let mut fields: HashMap<String, String> = HashMap::new();
    
    // if let Referrer(Some(refer)) = referrer {
    //     println!("Referrer: {}", &refer);
    //     fields.insert("");
    // }
    
    
//     let express: Express = String::new().into();
//     express.compress(encoding)
// }
// #[post("/login-user", data = "<form>")]
// fn hbs_login_process_admin() -> Redirect {
    
// }





// DOESN'T WORK
// #[get("/init")]
// pub fn initialize(admin: Option<AdministratorCookie>) {
//     ContentCacheLock::cache(rock, STATIC_PAGES_DIR);
// }




/* 
#[get("/article/<aid>")]
pub fn hbs_article_id(start: GenTimer, aid: ArticleId, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
    hbs_article_view(start, aid, conn, admin, user, encoding, uhits)
}

#[get("/article?<aid>")]
pub fn hbs_article_view(start: GenTimer, aid: ArticleId, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
    // let start = Instant::now();
    let rst = aid.retrieve_with_conn(conn); // retrieve result
    let mut output: Template; 
    if let Some(article) = rst {
        let title = article.title.clone();
        output = hbs_template(TemplateBody::Article(article), None, Some(title), String::from("/article"), admin, user, Some("enable_toc(true);".to_owned()), Some(start.0));
    } else {
        output = hbs_template(TemplateBody::General(alert_danger(&format!("Article {} not found.", aid.aid))), None, Some("Article Not Found".to_string()), String::from("/article"), admin, user, None, Some(start.0));
    }
    let end = start.0.elapsed();
    // println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
} 
*/

// #[get("/test_articles")]
// pub fn test_articles(start: GenTimer, article_state: State<ArticleCacheLock>, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
//     // Need to either 
//     //   1. specify here to retrieve the cached aids with a fallback
//     //      generating the aids for the specified tag
//     //    OR
//     //   2. make the function take a function pointer or closure that
//     //      will be called if aids for the specified tag could not be
//     //      found in the cache
    
    
//     unimplemented!()
// }

#[get("/test_tagcloud")]
pub fn test_tagcloud(start: GenTimer, 
                multi_aids: State<TagAidsLock>, 
                // text_cache: State<TextCacheLock>, 
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
    let end = start.0.elapsed();
    println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    express.compress( encoding )
    
}

#[get("/test_rss")]
pub fn test_rss(start: GenTimer, 
                text_lock: State<TextCacheLock>, 
                conn: DbConn, 
                admin: Option<AdministratorCookie>, 
                user: Option<UserCookie>, 
                encoding: AcceptCompression, 
                uhits: UniqueHits
               ) -> Express
{
    let express: Express = cache::pages::rss::serve(&conn,
                                                    &*text_lock,
                                                    admin,
                                                    user,
                                                    Some(uhits),
                                                    Some(start.clone()),
                                                    Some(encoding),
                                                    None
                                                   );
    let end = start.0.elapsed();
    println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    express.compress( encoding )
}

#[get("/test_author")]
pub fn test_author(start: GenTimer, 
                   pagination: Page<Pagination>,
                   multi_aids: State<TagAidsLock>, 
                   article_lock: State<ArticleCacheLock>, 
                   conn: DbConn, 
                   admin: Option<AdministratorCookie>, 
                   user: Option<UserCookie>, 
                   encoding: AcceptCompression, 
                   uhits: UniqueHits
                  ) -> Express 
{
    let author: u32 = 1;
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
    let end = start.0.elapsed();
    println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    express.compress( encoding )
}

#[get("/test_tag")]
pub fn test_tag(start: GenTimer, 
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
    
    let tag = "code";
    let express: Express = cache::pages::tag::serve(tag, 
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
    let end = start.0.elapsed();
    println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    express.compress( encoding )
    
}

#[get("/test_article")]
pub fn test_article(start: GenTimer, article_state: State<ArticleCacheLock>, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
    let aid = 21u32;
    
    // routes::pages::article::serve(aid, start, article_state, &conn, admin, user, encoding, uhits)
    // cache::pages::article::serve(aid, start, article_state, &conn, admin, user, encoding, uhits)
    let express: Express = cache::pages::article::serve(aid, 
                                                        article_state, 
                                                        &conn, 
                                                        admin, 
                                                        user, 
                                                        start.clone(),
                                                        uhits,
                                                        encoding, 
                                                        None
                                );
    let end = start.0.elapsed();
    println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    express.compress( encoding )
    
    // let article_rst = article_state.retrieve_article(aid);
    // let ctx: Result<CtxBody<TemplateArticle>, CtxBody<TemplateGeneral>> = routes::body::article(article_rst, admin, user, Some(uhits), Some(start), None);
    // // let express: Express = routes::template(template_name, ctx);
    // let express: Express = routes::template(ctx);
    
    // // let express: Express = String::new().into();
    // express
}

#[get("/test_home")]
pub fn test_home(start: GenTimer, pagination: Page<Pagination>, article_lock: State<ArticleCacheLock>, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
    let page_info: Option<String> = None;
    
    let express: Express = cache::pages::articles::serve(&*article_lock, 
                                                         pagination, 
                                                         &conn, 
                                                         admin, 
                                                         user, 
                                                         start.clone(), 
                                                         uhits, 
                                                         encoding, 
                                                         None, 
                                                         page_info
                                                        );
    let end = start.0.elapsed();
    println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    express.compress( encoding )
    
}

// #[get("/test_cache")]
// // pub fn test_cache(articles: State<Vec<Article>>) -> Express {
// pub fn test_cache(start: GenTimer, articles_state: State<ArticleCacheLock>, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
//     // unimplemented!()
//     let aid = 21;
    
//     let output: Template;
//     if let Ok(a) = articles_state.lock.read() {
//         if let Some(article) = a.articles.get(&aid) {
//             let title = article.title.clone();
//             // println!("Article {}\n{:?}", article.aid, &article);
//             output = hbs_template(TemplateBody::Article(article.clone()), None, Some(title), String::from("/article"), admin, user, Some("enable_toc(true);".to_owned()), Some(start.0));
//         } else {
//              output = hbs_template(TemplateBody::General(alert_danger(&format!("Article {} not found.", aid))), None, Some("Article Not Found".to_string()), String::from("/article"), admin, user, None, Some(start.0));
//         }
        
        
//     } else {
//         output =  hbs_template(TemplateBody::General(alert_danger(&format!("Failed to acquire cache lock for article {}.", aid))), None, Some("Internal Error".to_string()), String::from("/article"), admin, user, None, Some(start.0));
//     }
//     // let express: Express = String::new().into();
//     let end = start.0.elapsed();
//     println!("Processed in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
//     let express: Express = output.into();
//     express.compress( encoding )
// }










#[get("/refresh_content")]
pub fn refresh_content(start: GenTimer, admin: AdministratorCookie, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits, context_state: State<ContentContext>, cache_state: State<ContentCacheLock>) -> Express {
    
    let mut ctx_writer;
    if let Ok(ctx) = context_state.pages.write() {
        ctx_writer = ctx;
    } else {
        let template = hbs_template(TemplateBody::General(alert_danger("An error occurred attempting to access content.")), None, Some("Content not available.".to_string()), String::from("/error404"), Some(admin), user, None, Some(start.0));
        let express: Express = template.into();
        return express.compress(encoding);
    }
    
    let mut cache_writer;
    if let Ok(cache) = cache_state.pages.write() {
        cache_writer = cache;
    } else {
        let template = hbs_template(TemplateBody::General(alert_danger("An error occurred attempting to access content.")), None, Some("Content not available.".to_string()), String::from("/error404"), Some(admin), user, None, Some(start.0));
        let express: Express = template.into();
        return express.compress(encoding);
    }
    
    // let content_context: ContentContext = ContentContext::load(STATIC_PAGES_DIR);
    // let content_cache: ContentCacheLock = ContentCacheLock::new();
    
    let (ctx_pages, ctx_size) = destruct_context(ContentContext::load(STATIC_PAGES_DIR));
    *ctx_writer = ctx_pages;
    context_state.size.store(ctx_size, Ordering::SeqCst);
    
    // let cache = ContentCacheLock::new();
    let (cache_pages, cache_size) = destruct_cache(ContentCacheLock::new());
    *cache_writer = cache_pages;
    cache_state.size.store(cache_size, Ordering::SeqCst);
    
    
    // // load template contexts for all content files in the pages directory
    // *ctx_writer = *ctx.pages.read().unwrap();
    
    // // ctx_writer = ctx.pages.read();
    // context_state.size.store(ctx.size.load(Ordering::SeqCst), Ordering::SeqCst);
    
    // // reset cache back to nothing
    // *cache_writer = *cache.pages.read().unwrap();
    // cache_state.size.store(cache.size.load(Ordering::SeqCst), Ordering::SeqCst);
    
    let template = hbs_template(TemplateBody::General(alert_success("Content has been refreshed successfully.")), None, Some("Content refreshed.".to_string()), String::from("/error404"), Some(admin), user, None, Some(start.0));
    let express: Express = template.into();
    express.compress(encoding)
}




//
#[get("/content/<uri..>")]
pub fn static_pages(start: GenTimer, 
                    uri: PathBuf, 
                    admin: Option<AdministratorCookie>, 
                    user: Option<UserCookie>, 
                    encoding: AcceptCompression, 
                    uhits: UniqueHits, 
                    context: State<ContentContext>, 
                    // cache_lock: State<ContentCacheLock>
                   ) -> Result<ContentRequest, Express> {
    // could also prevent hotlinking by checking the referrer
    //   and sending an error for referring sites other than BASE or blank
    
    // look for the uri in the context, if it exists then make a ContextRequest
    //   which will be passed as the output
    //   before passing ContextRequest as the output, check for admin/user in the context
    //     if the context has user or admin set to true then make sure the admin/user var is_some()
    // if it does not exist then return an Express instance with an error message
    //   use hbs_template's General template
    
    // Could also move context out of the ContentReuqest and in the Responder use
    // let cache = req.guard::<State<HitCount>>().unwrap();
    
    let page = uri.to_string_lossy().into_owned();
    
    if let Ok(ctx_reader) = context.pages.read() {
        // if let Some(ctx) = context.pages.get(&page) {
        if let Some(ctx) = ctx_reader.get(&page) {
            // Permissions check
            if (ctx.admin && admin.is_none()) || (ctx.user && user.is_none()) {
                let template = hbs_template(TemplateBody::General(alert_danger("You do not have sufficient privileges to view this content.")), None, Some("Insufficient Privileges".to_string()), String::from("/error403"), admin, user, None, Some(start.0));
                let express: Express = template.into();
                return Err(express.compress(encoding));
            }
            
            // let test = ctx.clone();
            // context request
            // Build a ContentRequest with the requested files
            let conreq: ContentRequest = ContentRequest {
                encoding,
                // cache: cache_lock.inner(),
                route: page,
                start,
                // context: ctx.clone(),
                // context: &test,
            };
            Ok(conreq)
            
        } else {
            // let template = hbs_template(...); // Content does not exist
            let template = hbs_template(TemplateBody::General(alert_danger("The requested content could not be found.")), None, Some("Content not found.".to_string()), String::from("/error404"), admin, user, None, Some(start.0));
            let express: Express = template.into();
            Err(express.compress(encoding))
        }
        
    } else {
        // let template = hbs_template(...); // Content does not exist
        let template = hbs_template(TemplateBody::General(alert_danger("An error occurred attempting to access content.")), None, Some("Content not available.".to_string()), String::from("/error404"), admin, user, None, Some(start.0));
        let express: Express = template.into();
        Err(express.compress(encoding))
    }
        
    
    
}

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

#[get("/download/<uri..>")]
pub fn code_download(start: GenTimer, 
                    uri: PathBuf, 
                    admin: Option<AdministratorCookie>, 
                    user: Option<UserCookie>, 
                    encoding: AcceptCompression, 
                    uhits: UniqueHits, 
                    context: State<ContentContext>, 
                    // cache_lock: State<ContentCacheLock>
                   ) -> Express {
    // If the requested URI cannot be found in the static page cache
    //   maybe try looking in the uploads folder
    
    let page = uri.to_string_lossy().into_owned();
    
    if let Ok(ctx_reader) = context.pages.read() {
            
        // if let Some(ctx) = context.pages.get(&page) {
        if let Some(ctx) = ctx_reader.get(&page) {
            // Permissions check
            if (ctx.admin && admin.is_none()) || (ctx.user && user.is_none()) {
                let template = hbs_template(TemplateBody::General(alert_danger("You do not have sufficient privileges to view this content.")), None, Some("Insufficient Privileges".to_string()), String::from("/error403"), admin, user, None, Some(start.0));
                let express: Express = template.into();
                return express.compress(encoding);
            }
            
            let express: Express = ctx.body.clone().into();
            
            // let mut headers = Headers::new();
            // headers.set(ContentDisposition {
            //     disposition: DispositionType::Attachment,
            //     parameters: vec![DispositionParam::Filename(
            //       Charset::Iso_8859_1, // The character set for the bytes of the filename
            //       None, // The optional language tag (see `language-tag` crate)
            //       b"\xa9 Copyright 1989.txt".to_vec() // the actual bytes of the filename
            //     )]
            // });
            
            let attachment = ContentDisposition {
                disposition: DispositionType::Attachment,
                parameters: vec![DispositionParam::Filename(
                  Charset::Iso_8859_1, // The character set for the bytes of the filename
                  None, // The optional language tag (see `language-tag` crate)
                  ctx.uri.clone().into_bytes()
                  // b"".to_vec() // the actual bytes of the filename
                )]
            };
            express
            // Disable cache headers; IE breaks if downloading a file over HTTPS with cache-control headers
            .set_ttl(-2)
            .add_header(attachment)
            // express
        } else {
            // let error_logs = vec![/*"page_stats", "unique_stats",*/ "page_stats.json", "unique_stats.json"];
            // for log in error_logs {
            for log in DOWNLOADABLE_LOGS {
                // if error_logs.iter().any(|&log| page == log) {
                if &page == log {
                    if admin.is_some() {
                        
                        // if let Ok(mut f) = NamedFile::open(Path::new("logs/").join(log)) {
                        let err_path = err_file_name(log);
                        println!("Attempting to open {}", err_path.display());
                        if !err_path.exists() {
                            let template = hbs_template(TemplateBody::General(alert_danger("Error log could not be found.")), None, Some("Content not Found".to_string()), String::from("/error404"), admin, user, None, Some(start.0));
                            let express: Express = template.into();
                            return express.compress(encoding);
                        }
                        if let Ok(mut f) = File::open( err_path ) {
                            let mut buffer = Vec::new();
                            f.read_to_end(&mut buffer);
                            let express: Express = buffer.into();
                            
                            let attachment = ContentDisposition {
                                disposition: DispositionType::Attachment,
                                parameters: vec![DispositionParam::Filename(
                                  Charset::Iso_8859_1, // The character set for the bytes of the filename
                                  None, // The optional language tag (see `language-tag` crate)
                                  // log.clone().into_bytes()
                                  log.to_string().into_bytes()
                                  // b"".to_vec() // the actual bytes of the filename
                                )]
                            };
                            
                            return express
                            .set_ttl(-2)
                            .add_header(attachment);
                        } else {
                            let template = hbs_template(TemplateBody::General(alert_danger("Error log could not be found.")), None, Some("Content not Found".to_string()), String::from("/error404"), admin, user, None, Some(start.0));
                            let express: Express = template.into();
                            return express.compress(encoding);
                        }
                    } else {
                        let template = hbs_template(TemplateBody::General(alert_danger("You do not have sufficient privileges to view this content.")), None, Some("Insufficient Privileges".to_string()), String::from("/error403"), admin, user, None, Some(start.0));
                        let express: Express = template.into();
                        return express.compress(encoding);
                    }
                }
            }
            // let template = hbs_template(...); // Content does not exist
            let template = hbs_template(TemplateBody::General(alert_danger("The requested download could not be found.")), None, Some("Content not found.".to_string()), String::from("/error404"), admin, user, None, Some(start.0));
            let express: Express = template.into();
            express.compress(encoding)
        }
    } else {
        // let template = hbs_template(...); // Content does not exist
        let template = hbs_template(TemplateBody::General(alert_danger("An error occurred attempting to access content.")), None, Some("Content not available.".to_string()), String::from("/error404"), admin, user, None, Some(start.0));
        let express: Express = template.into();
        express.compress(encoding)
    }
    
}









#[get("/admin-test")]
pub fn hbs_admin_test(start: GenTimer, user: Option<UserCookie>, admin: Option<AdministratorCookie>, encoding: AcceptCompression) -> Express {
    
    let output: Template;
    if let Some(a) = admin {
        output = hbs_template(TemplateBody::General(alert_success("You are logged in.")), None, Some("Admin Test".to_string()), String::from("/admin-test"), Some(a), user, None, Some(start.0));
    } else {
        let mut loginmsg = String::with_capacity(300);
        loginmsg.push_str("You are not logged in, please <a href=\"");
        loginmsg.push_str(BLOG_URL);
        loginmsg.push_str("admin");
        loginmsg.push_str("\">Login</a>");
        
        output = hbs_template(TemplateBody::General(alert_danger(&loginmsg)), None, Some("Admin Test".to_string()), String::from("/admin-test"), admin, user, None, Some(start.0));
    }
    
    let express: Express = output.into();
    express.compress( encoding )
    
}
// #[get("/admin-test", rank = 2)]
// pub fn hbs_admin_test_unauthorized(start: GenTimer, user: Option<UserCookie>, encoding: AcceptCompression, location: Location) -> Redirect {
//     // Redirect::to("/admin?referrer=")
//     admin_login(location)
// }






#[get("/admin", rank = 1)]
pub fn hbs_dashboard_admin_authorized(start: GenTimer, pagination: Page<Pagination>, conn: DbConn, user: Option<UserCookie>, admin: AdministratorCookie, flash_msg_opt: Option<FlashMessage>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
    // let start = Instant::now();
    // let flash = if let Some(flash) = flash_msg_opt {
    //     Some( alert_warning(flash.msg()) )
    // } else {
    //     None
    // };
    
    // let output: Template = hbs_template(TemplateBody::General(format!("Welcome Administrator {user}.  You are viewing the administrator dashboard page.", user=admin.username), flash), Some("Dashboard".to_string()), String::from("/admin"), Some(admin), user, None, Some(start.0));
    
    // let end = start.0.elapsed();
    // println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    // let express: Express = output.into();
    // express.compress(encoding)
    
    hbs_manage_full(start, "".to_string(), "".to_string(), pagination, conn, admin, user, flash_msg_opt, encoding, uhits)
    
}

// No longer needed - hbs_dashboard_admin_authorized takes care of flash messages
// #[get("/admin", rank = 2)]
#[get("/admin", rank = 7)]
pub fn hbs_dashboard_admin_flash(start: GenTimer, conn: DbConn, user: Option<UserCookie>, flash_msg_opt: Option<FlashMessage>, encoding: AcceptCompression, referrer: Referrer) -> Express {
    // let start = Instant::now();
    let output: Template;
    
    let mut fields: HashMap<String, String> = HashMap::new();
    
    if let Referrer(Some(refer)) = referrer {
        // println!("Referrer: {}", &refer);
        fields.insert("referrer".to_string(), refer);
    } else {
        // println!("No referrer");
    }
    
    if let Some(flash_msg) = flash_msg_opt {
        let flash = Some( alert_danger(flash_msg.msg()) );
        output = hbs_template(TemplateBody::LoginData(ADMIN_LOGIN_URL.to_string(), None, fields), flash, Some("Administrator Login".to_string()), String::from("/admin"), None, user, Some("set_login_focus();".to_string()), Some(start.0));
    } else {
        output = hbs_template(TemplateBody::LoginData(ADMIN_LOGIN_URL.to_string(), None, fields), None, Some("Administrator Login".to_string()), String::from("/admin"), None, user, Some("set_login_focus();".to_string()), Some(start.0));
    }
    
    let end = start.0.elapsed();
    // println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}

// No longer needed.  Was getting errors because the dashboard_admin_retry_user() route
// named the qrystr parameter user which already has a variable binding, renamed and fixed it

// #[get("/admin/<userqry>")]
// pub fn dashboard_admin_retry_route(conn: DbConn, user: Option<UserCookie>, mut userqry: String, flash_msg_opt: Option<FlashMessage>, encoding: AcceptCompression) -> Express {
//     unimplemented!()
// }


// #[get("/admin?<userqry>", rank=3)]
#[get("/admin?<userqry>", rank=4)]
pub fn hbs_dashboard_admin_retry_user(start: GenTimer, conn: DbConn, user: Option<UserCookie>, mut userqry: QueryUser, flash_opt: Option<FlashMessage>, encoding: AcceptCompression) -> Express {
    // let start = Instant::now();
    // let userqry: QueryUser = userqry_form.get();
    
    let flash = process_flash(flash_opt);
    
    // let mut fields: HashMap<String, String> = HashMap::new();
    
    // if let Referrer(Some(refer)) = referrer {
    //     println!("Referrer: {}", &refer);
    //     fields.insert("referrer".to_string(), refer);
    // }
    // // user = login::sanitization::sanitize(&user);
    
    let username = if &userqry.user != "" { Some(userqry.user.clone() ) } else { None };
    // let flash = if let Some(f) = flash_msg_opt { Some(alert_danger(f.msg())) } else { None };
    // let output = hbs_template(TemplateBody::LoginData(ADMIN_LOGIN_URL.to_string(), username, fields), flash, Some("Administrator Login".to_string()), String::from("/admin"), None, user, Some("set_login_focus();".to_string()), Some(start.0));
    let output = hbs_template(TemplateBody::Login(ADMIN_LOGIN_URL.to_string(), username), flash, Some("Administrator Login".to_string()), String::from("/admin"), None, user, Some("set_login_focus();".to_string()), Some(start.0));
    
    let end = start.0.elapsed();
    // println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}

// #[get("/admin?<rediruser>")]
#[get("/admin?<rediruser>", rank = 2)]
pub fn hbs_dashboard_admin_retry_redir(start: GenTimer, conn: DbConn, user: Option<UserCookie>, mut rediruser: QueryUserRedir, flash_opt: Option<FlashMessage>, encoding: AcceptCompression) -> Express {
    // let start = Instant::now();
    // let userqry: QueryUser = userqry_form.get();
    
    let flash = process_flash(flash_opt);
    
    let mut fields: HashMap<String, String> = HashMap::new();
    
    if &rediruser.referrer != "" && &rediruser.referrer != "noredirect" {
        // println!("Adding referrer {}", &rediruser.referrer);
        fields.insert("referrer".to_string(), rediruser.referrer.clone());
    } else {
        // println!("No referring page\n{:?}", rediruser);
    }
    // if let Referrer(Some(refer)) = referrer {
    //     println!("Referrer: {}", &refer);
    //     fields.insert("referrer".to_string(), refer);
    // }
    // // user = login::sanitization::sanitize(&user);
    
    let username = if &rediruser.user != "" { Some(rediruser.user.clone() ) } else { None };
    // let flash = if let Some(f) = flash_msg_opt { Some(alert_danger(f.msg())) } else { None };
    let output = hbs_template(TemplateBody::LoginData(ADMIN_LOGIN_URL.to_string(), username, fields), flash, Some("Administrator Login".to_string()), String::from("/admin"), None, user, Some("set_login_focus();".to_string()), Some(start.0));
    
    let end = start.0.elapsed();
    // println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}

// #[get("/admin?<rediruser>")]
#[get("/admin?<rediruser>", rank = 3)]
pub fn hbs_dashboard_admin_retry_redir_only(start: GenTimer, conn: DbConn, user: Option<UserCookie>, mut rediruser: QueryRedir, flash_opt: Option<FlashMessage>, encoding: AcceptCompression) -> Express {
    // let start = Instant::now();
    // let userqry: QueryUser = userqry_form.get();
    
    let flash = process_flash(flash_opt);
    
    let mut fields: HashMap<String, String> = HashMap::new();
    
    if &rediruser.referrer != "" && &rediruser.referrer != "noredirect" {
        // println!("Adding referrer {}", &rediruser.referrer);
        fields.insert("referrer".to_string(), rediruser.referrer.clone());
    } else {
        // println!("No referring page\n{:?}", rediruser);
    }
    // if let Referrer(Some(refer)) = referrer {
    //     println!("Referrer: {}", &refer);
    //     fields.insert("referrer".to_string(), refer);
    // }
    // // user = login::sanitization::sanitize(&user);
    
    // let username = if &rediruser.user != "" { Some(rediruser.user.clone() ) } else { None };
    let username = None;
    // let flash = if let Some(f) = flash_msg_opt { Some(alert_danger(f.msg())) } else { None };
    let output = hbs_template(TemplateBody::LoginData(ADMIN_LOGIN_URL.to_string(), username, fields), flash, Some("Administrator Login".to_string()), String::from("/admin"), None, user, Some("set_login_focus();".to_string()), Some(start.0));
    
    let end = start.0.elapsed();
    // println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}

#[allow(unused_mut)]
// #[post("/admin", data = "<form>")]
#[post("/admin", data = "<form>")]
// pub fn hbs_process_admin_login(start: GenTimer, form: Form<LoginCont<AdministratorForm>>, user: Option<UserCookie>, mut cookies: Cookies) -> Result<Redirect, Flash<Redirect>> {
pub fn hbs_process_admin_login(start: GenTimer, form: Form<LoginCont<AdministratorForm>>, user: Option<UserCookie>, mut cookies: Cookies) -> Result<Redirect, Flash<Redirect>> {
    // let start = Instant::now();
    
    let login: AdministratorForm = form.get().form();
    // let login: AdministratorForm = form.into_inner().form;
    
    let mut err_temp: String;
    let ok_addy: &str;
    let err_addy: &str;
    if &login.referrer != "" && &login.referrer != "noredierct" {
        // println!("Processing referrer: {}", &login.referrer);
        let referring = if login.referrer.starts_with(BLOG_URL) {
            &login.referrer[BLOG_URL.len()-1..]
        } else {
            &login.referrer
        };
        ok_addy = &referring;
        err_addy = {
            err_temp = String::with_capacity(referring.len() + 20);
            err_temp.push_str("/admin?redir=");
            err_temp.push_str(referring);
            &err_temp
        };
    } else {
        ok_addy = "/admin";
        err_addy = "/admin";
    }
    // let ok_addy: &str = if &login.referrer != "" {
    //     &login.referrer
    // } else {
    //     "/admin"
    // };
    // println!("Forwaring to {} or {}", ok_addy, err_addy);
    
    // let mut output = login.flash_redirect("/admin", "/admin", &mut cookies);
    let mut output = login.flash_redirect(ok_addy, err_addy, &mut cookies);
    
    if output.is_ok() {
        // println!("Login success, forwarding to {}", ok_addy);
        if let Some(user_cookie) = user {
            if &user_cookie.username != &login.username {
                if let Ok(redir) = output {
                    let flash_message: Flash<Redirect> = Flash::error( 
                        redir, 
                        &format!("The regular user {} has been logged out.  You cannot log in with two separate user accounts at once.", 
                            &user_cookie.username
                        )
                    );
                    // Log the regular user out
                    // would use UserCookie::delete_cookie(cookies) but cookies already gets sent elsewhere
                    cookies.remove_private( Cookie::named( UserCookie::cookie_id() ) );
                    
                    // the Err will still allow the cookies to get set to log the user in but will allow a message to be passed
                    output = Err( flash_message );
                }
            }
        }
    }
    
    let end = start.0.elapsed();
    // println!("Processed in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    output
}

// #[get("/admin_logout")]
#[get("/admin_logout")]
pub fn hbs_logout_admin(admin: Option<AdministratorCookie>, mut cookies: Cookies) -> Result<Flash<Redirect>, Redirect> {
    if let Some(_) = admin {
        // cookies.remove_private(Cookie::named(AdministratorCookie::cookie_id()));
        AdministratorCookie::delete_cookie(&mut cookies);
        Ok(Flash::success(Redirect::to("/"), "Successfully logged out."))
    } else {
        Err(Redirect::to("/admin"))
    }
}











#[get("/user", rank = 1)]
pub fn hbs_dashboard_user_authorized(start: GenTimer, conn: DbConn, admin: Option<AdministratorCookie>, user: UserCookie, flash_msg_opt: Option<FlashMessage>, encoding: AcceptCompression) -> Express {
    // let start = Instant::now();
    let flash = if let Some(flash) = flash_msg_opt {
        Some( alert_warning(flash.msg()) )
    } else {
        None
    };
    
    let output: Template = hbs_template(TemplateBody::General(format!("Welcome User {user}.  You are viewing the User dashboard page.", user=user.username)), flash, Some("User Dashboard".to_string()), String::from("/user"), admin, Some(user), None, Some(start.0));
    
    let end = start.0.elapsed();
    // println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}

// No longer needed - hbs_dhasboard_user_authorized handles flash messages
#[get("/user", rank = 2)]
pub fn hbs_dashboard_user_flash(start: GenTimer, conn: DbConn, admin: Option<AdministratorCookie>, flash_msg_opt: Option<FlashMessage>, encoding: AcceptCompression) -> Express {
    // let start = Instant::now();
    let output: Template;
    
    if let Some(flash_msg) = flash_msg_opt {
        let flash = Some( alert_danger(flash_msg.msg()) );
        output = hbs_template(TemplateBody::Login(USER_LOGIN_URL.to_string(), None), flash, Some("User Login".to_string()), String::from("/user"), admin, None, Some("set_login_focus();".to_string()), Some(start.0));
    } else {
        output = hbs_template(TemplateBody::Login(USER_LOGIN_URL.to_string(), None), None, Some("User Login".to_string()), String::from("/user"), admin, None, Some("set_login_focus();".to_string()), Some(start.0));
    }
    
    let end = start.0.elapsed();
    // println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}


// #[get("/user", rank = 3)]
// pub fn dashboard_user_login(conn: DbConn, admin: Option<AdministratorCookie>, encoding: AcceptCompression) -> Express {
//     hbs_template(TemplateBody::Login(URL_LOGIN_USER.to_string(), None, None), Some("User Login".to_string()), String::from("/user"), admin, None, None, None)
// }

#[get("/user?<user>")]
pub fn hbs_dashboard_user_retry_user(start: GenTimer, conn: DbConn, admin: Option<AdministratorCookie>, mut user: QueryUser, flash_msg_opt: Option<FlashMessage>, encoding: AcceptCompression) -> Express {
    // let start = Instant::now();
    // user = login::sanitization::sanitize(&user);
    let username = if &user.user != "" { Some(user.user.clone() ) } else { None };
    let flash = if let Some(f) = flash_msg_opt { Some(alert_danger(f.msg())) } else { None };
    let output = hbs_template(TemplateBody::Login(USER_LOGIN_URL.to_string(), username), flash, Some("User Login".to_string()), String::from("/user"), admin, None, Some("set_login_focus();".to_string()), Some(start.0));
    
    let end = start.0.elapsed();
    // println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}

#[allow(unused_mut)]
#[post("/user", data = "<form>")]
pub fn hbs_process_user_login(start: GenTimer, form: Form<LoginCont<UserForm>>, admin: Option<AdministratorCookie>, mut cookies: Cookies) -> Result<Redirect, Flash<Redirect>> {
    // let start = Instant::now();
    
    let login: UserForm = form.get().form();
    // let login: AdministratorForm = form.into_inner().form;
    let mut output = login.flash_redirect("/user", "/user", &mut cookies);
    
    if output.is_ok() {
        if let Some(admin_cookie) = admin {
            if &admin_cookie.username != &login.username {
                if let Ok(redir) = output {
                    let flash_message: Flash<Redirect> = Flash::error( 
                        redir, 
                        &format!("The administrator user {} has been logged out.  You cannot log in with two separate user accounts at once.", 
                            &admin_cookie.username
                        )
                    );
                    // Log the regular user out
                    // would use UserCookie::delete_cookie(cookies) but cookies already gets sent elsewhere
                    cookies.remove_private( Cookie::named( AdministratorCookie::cookie_id() ) );
                    
                    // the Err will still allow the cookies to get set to log the user in but will allow a message to be passed
                    output = Err( flash_message );
                }
            }
        }
    }
    
    let end = start.0.elapsed();
    // println!("Processed in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/user_logout")]
pub fn hbs_logout_user(admin: Option<UserCookie>, mut cookies: Cookies) -> Result<Flash<Redirect>, Redirect> {
    if let Some(_) = admin {
        // cookies.remove_private(Cookie::named(UserCookie::cookie_id()));
        UserCookie::delete_cookie(&mut cookies);
        Ok(Flash::success(Redirect::to("/"), "Successfully logged out."))
    } else {
        Err(Redirect::to("/user"))
    }
}










// #[get("/view")]
// pub fn hbs_all_articles(start: GenTimer, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
//     // let start = Instant::now();
//     let output: Template;
//     let results = Article::retrieve_all(conn, 0, Some(300), None, None, None, None);
    
//     if results.len() != 0 {
//         output = hbs_template(TemplateBody::Articles(results, None), Some("Viewing All Articles".to_string()), String::from("/"), admin, user, None, Some(start.0));
//     } else {
//         if admin.is_some() {
//             output = hbs_template(TemplateBody::General("There are no articles<br>\n<a href =\"/insert\">Create Article</a>".to_string(), None), Some("Viewing All Articles".to_string()), String::from("/"), admin, user, None, Some(start.0));
//         } else {
//             output = hbs_template(TemplateBody::General("There are no articles.".to_string(), None), Some("Viewing All Articles".to_string()), String::from("/"), admin, user, None, Some(start.0));
//         }
//     }
    
//     let end = start.0.elapsed();
//     println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
//     let express: Express = output.into();
//     express.compress(encoding)
// }

// #[get("/view?<page>")]
// pub fn hbs_articles_page(start: GenTimer, page: ViewPage, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
//     // let start = Instant::now();
//     let results = Article::retrieve_all(conn, 0, Some(300), None, None, None, None);
    
//     // Todo: Change title to: Viewing Article Page x/z
//     let output: Template = hbs_template(TemplateBody::General("You are viewing paginated articles.".to_string(), None), Some("Viewing Articles".to_string()), String::from("/"), admin, user, None, Some(start.0));
    
//     let end = start.0.elapsed();
//     println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
//     let express: Express = output.into();
//     express.compress(encoding)
// }

// BLOCK 1
// ROUTES: /all_tags through /article

#[post("/create", data = "<form>")]
pub fn hbs_article_process(start: GenTimer, form: Form<ArticleForm>, conn: DbConn, admin: AdministratorCookie, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
    // let start = Instant::now();
    
    let output: Template;
    
    let username = if let Some(ref display) = admin.display { display.clone() } else { titlecase(&admin.username) };
    
    let cr_options = ComrakOptions { ext_header_ids: Some("section-".to_string()), .. COMRAK_OPTIONS };
    
    let mut article: ArticleForm = form.into_inner();
    
    if &article.body == "" && &article.markdown != "" {
        let html: String = markdown_to_html(&article.markdown, &cr_options);
        article.body = html;
    }
    
    
    let result = article.save(&conn, admin.userid, &username);
    match result {
        Ok(article) => {
            // let article = articlesrc.to_article();
            let title = article.title.clone();
            output = hbs_template(TemplateBody::Article(article), None, Some(title), String::from("/create"), Some(admin), user, None, Some(start.0));
        },
        Err(why) => {
            output = hbs_template(TemplateBody::General(alert_danger(&format!("Could not post the submitted article.  Reason: {}", why))), None, Some("Could not post article".to_string()), String::from("/create"), Some(admin), user, None, Some(start.0));
        },
    }
    
    let end = start.0.elapsed();
    // println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}
#[post("/create", rank=2)]
pub fn hbs_create_unauthorized(start: GenTimer, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
    // let start = Instant::now();
    // let output: Template = hbs_template(TemplateBody::General(alert_danger(UNAUTHORIZED_POST_MESSAGE)), None, Some("Not Authorized".to_string()), String::from("/create"), admin, user, None, Some(start.0));
    
    let mut loginmsg = String::with_capacity(300);
        loginmsg.push_str("You are not logged in, please <a href=\"");
        loginmsg.push_str(BLOG_URL);
        loginmsg.push_str("admin");
        loginmsg.push_str("\">Login</a>");
    
    let output = hbs_template(TemplateBody::General(alert_danger(&loginmsg)), None, Some("Create".to_string()), String::from("/create"), None, user, None, Some(start.0));
    
    let end = start.0.elapsed();
    // println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}

#[get("/create")]
pub fn hbs_create_form(start: GenTimer, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
    // let start = Instant::now();
    
    let output: Template;
    if admin.is_some() {
        output = hbs_template(TemplateBody::Create(CREATE_FORM_URL.to_string()), None, Some("Create New Article".to_string()), String::from("/create"), admin, user, None, Some(start.0));
    } else {
        output = hbs_template(TemplateBody::General(alert_danger(UNAUTHORIZED_POST_MESSAGE)), None, Some("Not Authorized".to_string()), String::from("/create"), admin, user, None, Some(start.0));
    }
    
    let end = start.0.elapsed();
    // println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}

// #[get("/logout")]
// pub fn hbs_logout(admin: Option<AdministratorCookie>, user: Option<UserCookie>, mut cookies: Cookies) -> Result<Flash<Redirect>, Redirect> {
//     use cookie_data::CookieId;
//     if admin.is_some() || user.is_some() {
//         if let Some(a) = admin {
//             cookies.remove_private(Cookie::named(AdministratorCookie::get_cid()));
//             // cookies.remove_private(Cookie::named("user_id"));
//         }
//         if let Some(u) = user {
//             cookies.remove_private(Cookie::named(UserCookie::get_cid()));
//         }
//         Ok(Flash::success(Redirect::to("/"), "Successfully logged out."))
//     } else {
//         Err(Redirect::to("/admin"))
//     }
// }


// Do a full-text search on the body and title fields
//   on the tag field match each word against a tag,
//   this will only match complete word matches
//     research using array_to_tsvector() in the future
// https://www.postgresql.org/docs/current/static/functions-textsearch.html

// // NOT IMPLEMENTED YET
// #[get("/search")]
// pub fn hbs_search_page(start: GenTimer, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
//     //show an advanced search form
    
//     let output: Template = hbs_template(TemplateBody::General("Search page not implemented yet.  Please use the search form in the top right corner of the page.".to_string(), None), Some("Search".to_string()), String::from("/search"), admin, user, None, Some(start.0));
//     let express: Express = output.into();
//     express.compress(encoding)
// }

#[get("/search?<search>")]
pub fn hbs_search_redirect(start: GenTimer, pagination: Page<Pagination>, search: Search, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression) -> Redirect {
    // Add min/max date later, its not implemented in the search page anyways
    // let min = if let Some(mi) = search.min {
    //     format!("{}", mi.0.format("%Y-%m-%d %H:%M:%S"))
    // } else {
    //     String::new()
    // };
    // let max = if let Some(mi) = search.min {
    //     format!("{}", mi.0.format("%Y-%m-%d %H:%M:%S"))
    // } else {
    //     String::new()
    // };
    if let Some(q) = search.q {
        Redirect::to( &format!( "/search/{}", q ) )
    } else {
        Redirect::to( "/search" )
    }
}

#[get("/search/<searchstr>")]
pub fn hbs_search_results(start: GenTimer, pagination: Page<Pagination>, searchstr: String, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
    
    let search = Search {
        limit: None,
        o: None,
        p: None,
        q: Some(searchstr),
        min: None,
        max: None,
    };
    
    /*
        SELECT  
            a.aid, 
            a.title, 
            a.posted,
            a.tag, 
            ts_rank(a.fulltxt, fqry, 32) AS rank, 
            ts_headline('pg_catalog.english', a.body, fqry, 'StartSel = "<mark>", StopSel = "</mark>"') AS body
        FROM 
            articles a, 
            plainto_tsquery('pg_catalog.english', 'handlebars or hello') fqry
        WHERE 
            fqry @@ a.fulltxt
                OR
            'cool' = ANY(a.tag)
                AND
            a.posted > '2017-01-01'
                AND
            a.posted < '2018-01-01'
        ORDER BY 
            rank DESC
        LIMIT 20
    */
    
    // full-text search: title, description, body
    // entirety match 'each word' = ANY(tag)
    
    // println!("Search parameters:\n{:?}", search);
    
    let mut countqry = String::with_capacity(750);
    let mut qrystr = String::with_capacity(750);
    // aid title posted body tag description userid display
    
    // New Query:
    qrystr.push_str(r#"
SELECT a.aid, a.title, a.posted, 
    ts_headline('pg_catalog.english', a.body, fqry, 'StartSel = "<mark>", StopSel = "</mark>"') AS body, 
    a.tag, a.description, u.userid, u.display, u.username, 
    ts_rank(a.fulltxt, fqry, 32) AS rank
FROM articles a JOIN users u ON (a.author = u.userid),
    plainto_tsquery('pg_catalog.english', '"#);
    
// Old Query
//     qrystr.push_str(r#"
// SELECT a.aid, a.title, a.posted, a.tag, ts_rank(a.fulltxt, fqry, 32) AS rank, ts_headline('pg_catalog.english', a.body, fqry, 'StartSel = "<mark>", StopSel = "</mark>"') AS body,
//     u.userid, u.display, u.username
// FROM articles a JOIN users u ON (a.author = u.userid),
// plainto_tsquery('pg_catalog.english', '"#);
    
    countqry.push_str(r##"SELECT COUNT(*) FROM articles a, plainto_tsquery('pg_catalog.english', '"##);
    // ts_headline([ config regconfig, ] document text, query tsquery [, options text ]) returns text
    // qrystr.push_str(r#"SELECT ts_headline('english', body) FROM articles"#);
    
    let mut wherestr = String::new();
    let original = search.clone();
    
    let mut tags: Option<String> = None;
    if let Some(mut q) = search.q {
        if &q != "" {
            // prevent attacks based on length and complexity of the sql query for full-text searches
            if q.len() > 200 {
                q = q[..200].to_string();
            }
            // WHERE to_tsvector('english', body) @@ to_tsquery('english', 'friend');
            let sanitized = &sanitize_sql(q);
            qrystr.push_str(sanitized);
            countqry.push_str(sanitized);
            // do a full-text search on title, description, and body fields
            // for each word add: 'word' = ANY(tag)
            let ts = sanitized.split(" ").map(|s| format!("'{}' = ANY(a.tag)", s)).collect::<Vec<_>>().join(" OR ");
            tags = if &ts != "" { Some(ts) } else { None };
            // wherestr.push_str(&tags);
        }
    }
    qrystr.push_str("') fqry WHERE fqry @@ a.fulltxt");
    countqry.push_str("') fqry WHERE fqry @@ a.fulltxt");
    
    if let Some(t) = tags {
        qrystr.push_str(" OR ");
        qrystr.push_str(&t);
        countqry.push_str(" OR ");
        countqry.push_str(&t);
    }
    if let Some(min) = search.min {
        // after min
        qrystr.push_str(" AND a.posted > '");
        qrystr.push_str(&format!("{}", min.0.format("%Y-%m-%d %H:%M:%S")));
        qrystr.push_str("'");
        
        countqry.push_str(" AND a.posted > '");
        countqry.push_str(&format!("{}", min.0.format("%Y-%m-%d %H:%M:%S")));
        countqry.push_str("'");
    }
    if let Some(max) = search.max {
        // before max
        qrystr.push_str(" AND a.posted < '");
        qrystr.push_str(&format!("{}", max.0.format("%Y-%m-%d %H:%M:%S")));
        qrystr.push_str("'");
        
        countqry.push_str(" AND a.posted < '");
        countqry.push_str(&format!("{}", max.0.format("%Y-%m-%d %H:%M:%S")));
        countqry.push_str("'");
    }
    
    // qrystr.push_str(" ORDER BY rank DESC");
    // if let Some(limit) = search.limit {
    //     // if str_is_numeric(limit) {
    //     if limit <= 50 {
    //         qrystr.push_str(&format!(" LIMIT {}", limit));
    //         countqry.push_str(&format!(" LIMIT {}", limit));
    //     } else {
    //         qrystr.push_str(" LIMIT 50");
    //         countqry.push_str(" LIMIT 50");
    //     }
    // } else {
    //     qrystr.push_str(" LIMIT 40");
    //     countqry.push_str(" LIMIT 40");
    // }
    
    // println!("Generated the following SQL Query:\nCount:\n{}\n\nSearch Query:\n{}", countqry, qrystr);
    // println!("Generated the following SQL Query:\n{}", qrystr);
    
    let total_query = countqry;
    let output: Template;
    if let Ok(rst) = conn.query(&total_query, &[]) {
        if !rst.is_empty() && rst.len() == 1 {
            let row = rst.get(0);
            let count: i64 = row.get(0);
            let total_items: u32 = count as u32;
            let (ipp, cur, num_pages) = pagination.page_data(total_items);
            // let sql = pagination.sql(&format!("SELECT a.aid, a.title, a.posted, description({}, a.body, a.description) as body, a.tag, a.description, u.userid, u.display, u.username FROM articles a JOIN users u ON (a.author = u.userid)", DESC_LIMIT), Some("posted DESC"));
            let sql = pagination.sql(&qrystr, Some("rank DESC"));
            // println!("Prepared paginated query:\n{}", sql);
            if let Some(results) = conn.articles(&sql) {
                if results.len() != 0 {
                    // let page_information = pagination.page_info(total_items);
                    let pinfo = pagination.page_info(total_items);
                    let welcome = r##"<h1>Search Results</h1>"##;
                    
                    let mut page_information = String::with_capacity(pinfo.len() + welcome.len() + 50);
                    page_information.push_str(welcome);
                    page_information.push_str(&pinfo);
                    
                    output = hbs_template(TemplateBody::ArticlesPages(results, pagination, total_items, Some(page_information)), None, Some("Search Results".to_string()), String::from("/search"), admin, user, None, Some(start.0));
                    let express: Express = output.into();
                    
                    let end = start.0.elapsed();
                    // println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
                    
                    return express.compress( encoding );
                }
            }
        }
    }
    
    output = hbs_template(TemplateBody::General(alert_danger("No articles to show.")), None, Some("Search Results".to_string()), String::from("/search"), admin, user, None, Some(start.0));
    let express: Express = output.into();
    
    let end = start.0.elapsed();
    // println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    
    express.compress( encoding )
    
    // let mut articles: Vec<Article> = Vec::new();
    // let qry = conn.query(&qrystr, &[]);
    // let output: Template;
    // if let Ok(result) = qry {
    //     for row in &result {
            
    //         let display: Option<String> = row.get(7);
    //         let username: String = if let Some(disp) = display { disp } else { row.get(8) };
            
    //         let a = Article {
    //             aid: row.get(0),
    //             title: row.get(1),
    //             posted: row.get(2),
    //             tags: row.get_opt(3).unwrap_or(Ok(Vec::<String>::new())).unwrap_or(Vec::<String>::new()),
    //             body: row.get(5),
    //             description: String::new(),
    //             userid: row.get(6),
    //             username: titlecase( &username ),
    //         };
    //         articles.push(a);
    //     }
    //     output = hbs_template(TemplateBody::Search(articles, Some(original), None), Some("Search Results".to_string()), String::from("/search"), admin, user, None, Some(start.0));
    // } else {
    //     println!("Query failed. Query: {}", qrystr);
    //     output = hbs_template(TemplateBody::General(alert_danger("No results were found."), None), Some("Search Results".to_string()), String::from("/search"), admin, user, None, Some(start.0));
    // }
    // let end = start.0.elapsed();
    // println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    // let express: Express = output.into();
    // express.compress(encoding)
}


// BLOCK 2
// ROUTES: /rss through /author/<authorid>

#[get("/about")]
pub fn hbs_about(start: GenTimer, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
    // don't forget to put the start Instant in the hbs_template() function
    let output = hbs_template(TemplateBody::General("This page is not implemented yet.  Soon it will tell a little about me.".to_string()), None, Some("About Me".to_string()), String::from("/about"), admin, user, None, Some(start.0));
    let express: Express = output.into();
    express.compress(encoding)
}


#[get("/edit", rank=2)]
pub fn hbs_edit_unauthorized(start: GenTimer, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
    let mut loginmsg = String::with_capacity(300);
        loginmsg.push_str("You are not logged in, please <a href=\"");
        loginmsg.push_str(BLOG_URL);
        loginmsg.push_str("admin");
        loginmsg.push_str("\">Login</a>");
    
    let output = hbs_template(TemplateBody::General(alert_danger(&loginmsg)), None, Some("Unauthorized".to_string()), String::from("/edit"), None, user, None, Some(start.0));
    let express: Express = output.into();
    express.compress( encoding )
}

#[get("/edit/<aid>")]
pub fn hbs_edit(start: GenTimer, aid: u32, conn: DbConn, admin: AdministratorCookie, user: Option<UserCookie>, flash_opt: Option<FlashMessage>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {

    // let options = ComrakOptions {
    //     hardbreaks: true,            // \n => <br>\n
    //     width: 120usize,             
    //     github_pre_lang: false,      
    //     ext_strikethrough: true,     // hello ~world~ person.
    //     ext_tagfilter: true,         // filters out certain html tags
    //     ext_table: true,             // | a | b |\n|---|---|\n| c | d |
    //     ext_autolink: true,          
    //     ext_tasklist: true,          // * [x] Done\n* [ ] Not Done
    //     ext_superscript: true,       // e = mc^2^
    //     ext_header_ids: Some("section-".to_string()),        // None / Some("some-id-prefix-".to_string())
    //     ext_footnotes: true,         // Hi[^x]\n\n[^x]: A footnote here\n
    // };
    
    // let html: String = markdown_to_html(text, &ComrakOptions::default());
    // let html: String = markdown_to_html(text, &COMRAK_OPTIONS);
    // html
    
    let flash = process_flash(flash_opt);
    
    let cr_options = ComrakOptions { ext_header_ids: Some("section-".to_string()), .. COMRAK_OPTIONS };
    
    let output: Template;
    let id = ArticleId::new(aid);
    if let Some(mut article) = id.retrieve_with_conn(conn) {
        // println!("Retrieved article info: {}", article.info());
        //
        let title = article.title.clone();
        
        // If the body is empty that means the javascript did not process the Markdown into HTML
        // so convert the Markdown into HTML using Rust and the Comrak crate 
        //      The Comrak crate is slower than the pulldown-cmark but more options
        if &article.body == "" && &article.markdown != "" {
            let html: String = markdown_to_html(&article.markdown, &cr_options);
            article.body = html;
        }
        
        output = hbs_template(TemplateBody::Edit(EDIT_FORM_URL.to_string(), article), flash, Some(format!("Editing '{}'", title)), String::from("/edit"), Some(admin), user, None, Some(start.0));
        let express: Express = output.into();
        return express.compress(encoding);
    }
    
    // let qrystr = format!("a.aid, a.title, a.posted, a.body, a.tag, a.description, u.userid, u.display, u.username FROM articles a JOIN users u ON (a.author = u.userid) WHERE a.aid = {}", aid);
    // if let Some(articles) = conn.articles(&qrystr) {
    //     if articles.len() == 1 {
    //         let article = &articles[0];
    //         output = hbs_template(TemplateBody::Edit(EDIT_FORM_URL.to_string(), *article, None), Some(format!("Editing '{}'", &article.title)), String::from("/edit"), admin, user, None, Some(start.0));
    //         let express: Express = output.into();
    //         return express.compress(encoding);
    //     }
    // }
    output = hbs_template(TemplateBody::General("The reuqested article could not be found.".to_string()), flash, Some("Edit".to_string()), String::from("/edit"), Some(admin), user, None, Some(start.0));
    
    let express: Express = output.into();
    express.compress(encoding)
}


#[post("/edit", data = "<form>")]
// pub fn hbs_edit_process(start: GenTimer, form: Form<Article>, conn: DbConn, admin: AdministratorCookie, user: Option<UserCookie>, encoding: AcceptCompression) -> Flash<Redirect> {
pub fn hbs_edit_process(start: GenTimer, form: Form<ArticleWrapper>, conn: DbConn, admin: AdministratorCookie, encoding: AcceptCompression) -> Flash<Redirect> {
    
    let cr_options = ComrakOptions { ext_header_ids: Some("section-".to_string()), .. COMRAK_OPTIONS };
    
    let wrapper: ArticleWrapper = form.into_inner();
    let mut article: Article = wrapper.to_article();
    
    if &article.body == "" && &article.markdown != "" {
        let html: String = markdown_to_html(&article.markdown, &cr_options);
        article.body = html;
    }
    
    // println!("Processing Article info: {}", article.info());
    let result = article.save(conn);
    match result {
        Ok(k) => {
            Flash::success(Redirect::to(&format!("/edit/{}", &article.aid)), &k)
        },
        Err(ref e) if e == "" => {
            Flash::success(Redirect::to(&format!("/edit/{}", &article.aid)), &e)
            // Flash::error(Redirect::to(""), "")
        },
        Err(e) => {
            Flash::success(Redirect::to(&format!("/edit/{}", &article.aid)), &e)
            // Flash::error(Redirect::to(""), "")
        }
    }
    
}

#[get("/manage", rank=2)]
pub fn hbs_manage_unauthorized(start: GenTimer, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
    let mut loginmsg = String::with_capacity(300);
        loginmsg.push_str("You are not logged in, please <a href=\"");
        loginmsg.push_str(BLOG_URL);
        loginmsg.push_str("admin");
        loginmsg.push_str("\">Login</a>");
    
    let output = hbs_template(TemplateBody::General(alert_danger(&loginmsg)), None, Some("Unauthorized".to_string()), String::from("/manage"), None, user, None, Some(start.0));
    let express: Express = output.into();
    express.compress( encoding )
}


#[get("/manage")]
pub fn hbs_manage_basic(start: GenTimer, pagination: Page<Pagination>, conn: DbConn, admin: AdministratorCookie, user: Option<UserCookie>, flash: Option<FlashMessage>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
    hbs_manage_full(start, "".to_string(), "".to_string(), pagination, conn, admin, user, flash, encoding, uhits)
}

#[get("/manage/<sortstr>/<orderstr>")]
pub fn hbs_manage_full(start: GenTimer, sortstr: String, orderstr: String, pagination: Page<Pagination>, conn: DbConn, admin: AdministratorCookie, user: Option<UserCookie>, flash: Option<FlashMessage>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
    
    let output: Template;
    
    let fmsg: Option<String>;
    if let Some(flashmsg) = flash {
        if flashmsg.name() == "error" {
            fmsg = Some(alert_danger( flashmsg.msg() ));
        } else if flashmsg.name() == "warning" {
            fmsg = Some(alert_warning( flashmsg.msg() ));
        } else if flashmsg.name() == "success" {
            fmsg = Some(alert_success( flashmsg.msg() ));
        } else {
            fmsg = Some(alert_info( flashmsg.msg() ));
        }
    }  else {
        fmsg = None;
    }
    
    let mut total_query = "SELECT COUNT(*) AS count FROM articles";
    
    // let mut querystr: String = "SELECT a.aid, a.title, a.posted, description({}, a.body, a.description), a.tag, a.description, u.userid, u.display, u.username FROM articles a JOIN users u ON (a.author = u.userid)".to_string();
    let mut page_query = "SELECT a.aid, a.title, a.posted, '' as body, a.tag, a.description, u.userid, u.display, u.username FROM articles a JOIN users u ON (a.author = u.userid)";
    
    // let order_str = sortstr.to_lowercase();
    let order = match sortstr.to_lowercase().as_ref() {
        "date" if &orderstr == "desc" => "posted DESC",
        "date" if &orderstr == "asc" => "posted",
        "date" => "posted DESC",
        "title" if &orderstr == "desc" => "title DESC",
        "title" if &orderstr == "asc" => "title",
        "title" => "title",
        _ if &orderstr == "desc" => "posted DESC",
        _ if &orderstr == "asc" => "posted",
        _ => "posted DESC",
    };
    
    let sort_options: Sort = match order {
        "posted DESC" => Sort::Date(true),
        "posted" => Sort::Date(false),
        "title" => Sort::Title(false),
        "title DESC" => Sort::Title(true),
        _ => Sort::Date(true),
    };
    
    if let Ok(rst) = conn.query(total_query, &[]) {
        if !rst.is_empty() && rst.len() == 1 {
            let countrow = rst.get(0);
            let count: i64 = countrow.get(0);
            let total_items: u32 = count as u32;
            let (ipp, cur, num_pages) = pagination.page_data(total_items);
            let pagesql = pagination.sql(page_query, Some(order));
            // println!("Manage paginated query: {}", pagesql);
            if let Some(results) = conn.articles(&pagesql) {
                if results.len() != 0 {
                    // let page_info = pagination.page_info(total_items);
                    
                    output = hbs_template(TemplateBody::Manage(results, pagination, total_items, sort_options), fmsg, Some(format!("Manage Articles - Page {} of {}", cur, num_pages)), String::from("/manage"), Some(admin), user, None, Some(start.0));
                    
                    let express: Express = output.into();
                    return express.compress(encoding);
                    
                }
            }
            
            
        }
    }
    
    output = hbs_template(TemplateBody::General(alert_danger("No articles found.")), fmsg, Some("Manage Articles".to_string()), String::from("/manage"), Some(admin), user, None, Some(start.0));
    
    let express: Express = output.into();
    express.compress(encoding)
    
}

// #[get("/hit")]
// pub fn hit_count(uhits: UniqueHits) -> String {
//     // let (page, count) = hits;
//     let page = hits.0;
//     let count = hits.1;
//     let views = hits.2;
//     format!("The page `{}` has {} page views.\nTotal views: {}", page, count, views)
// }


// #[get("/hit2")]
// pub fn hit_count2(uhits: UniqueHits) -> String {
//     // let (page, count) = hits;
//     let page = hits.0;
//     let count = hits.1;
//     let views = hits.2;
//     format!("The page `{}` has {} page views.\nTotal views: {}", page, count, views)
// }


// #[get("/hit3")]
// pub fn hit_count3(uhits: UniqueHits) -> String {
//     // let (page, count) = hits;
//     let page = hits.0;
//     let count = hits.1;
//     let views = hits.2;
//     format!("The page `{}` has {} page views.\nTotal views: {}", page, count, views)
// }


#[get("/delete", rank=2)]
pub fn hbs_delete_unauthorized(start: GenTimer, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
    let mut loginmsg = String::with_capacity(300);
        loginmsg.push_str("You are not logged in, please <a href=\"");
        loginmsg.push_str(BLOG_URL);
        loginmsg.push_str("admin");
        loginmsg.push_str("\">Login</a>");
    
    let output = hbs_template(TemplateBody::General(alert_danger(&loginmsg)), None, Some("Unauthorized".to_string()), String::from("/delete"), None, user, None, Some(start.0));
    let express: Express = output.into();
    express.compress( encoding )
}

#[get("/delete/<aid>")]
pub fn hbs_delete_confirm(start: GenTimer, aid: u32, conn: DbConn, admin: AdministratorCookie, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
    
    let confirm = alert_warning(&format!(r#"
        You are attempting to permanently delete an article, are you sure you want to continue?  
        This action cannot be undone.
        <form action="{}process_delete/{}" method="post" id="delete-form">
            <input type="hidden" value="{}" id="manage-page">
            <div class="v-centered-text">
                <button type="submit" id="delete-button" class="v-del-confirm btn btn-danger">Delete</button>
                <span class="v-spacer-del"></span>
                <button type="button" id="delete-cancel" class="v-del-cancel btn btn-warning">Cancel</button>
            </div>
        </form>
        "#, BLOG_URL, aid, MANAGE_URL));
    
    let output = hbs_template(TemplateBody::General(confirm), None, Some("Delete Article".to_string()), String::from("/delete"), Some(admin), user, None, Some(start.0));
    
    let express: Express = output.into();
    express.compress( encoding )
}

#[post("/process_delete/<aid>")]
pub fn hbs_process_delete(aid: u32, conn: DbConn, admin: AdministratorCookie, user: Option<UserCookie>) -> Result<Flash<Redirect>, Redirect> {
    let qrystr = format!("DELETE FROM articles WHERE aid = {}", aid);
    
    // println!("Delete query:\n{}\n", &qrystr);
    
    if let Ok(num) = conn.execute(&qrystr, &[]) {
        if num == 1 {
            // println!("Delete succeeded");
            Ok( Flash::success(Redirect::to("/manage"), &format!("Article {} successfully deleted.", aid)) )
        } else if num == 0 {
            println!("Delete failed - no articles deleted.");
            Ok( Flash::error(Redirect::to("/manage"), &format!("Article {} was not deleted.", aid)) )
        } else {
            println!("Delete failed - multiple articles deleted!");
            Ok( Flash::error(Redirect::to("/manage"), &format!("A mistake occurred. Multiple articles ({} articles) appear to have been deleted.", num)) )
        }
    } else {
        println!("Delete failed.");
        Err( Redirect::to("/manage") )
    }
}

#[get("/backup", rank=2)]
pub fn hbs_backup_unauthorized(start: GenTimer, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
    let mut loginmsg = String::with_capacity(300);
        loginmsg.push_str("You are not logged in, please <a href=\"");
        loginmsg.push_str(BLOG_URL);
        loginmsg.push_str("admin");
        loginmsg.push_str("\">Login</a>");
    
    let output = hbs_template(TemplateBody::General(alert_danger(&loginmsg)), None, Some("Unauthorized".to_string()), String::from("/backup"), None, user, None, Some(start.0));
    let express: Express = output.into();
    express.compress( encoding )
}

#[get("/backup")]
pub fn backup(start: GenTimer, admin: AdministratorCookie, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
    use std::process::Command;
    use rocket::http::hyper::header::{Headers, ContentDisposition, DispositionType, DispositionParam, Charset};
    
    // "C:\Program Files\PostgreSQL\10\bin\pg_dump.exe"
    /*
    pg_dump --file "db_backup-2.sql" --format=p --no-owner --create 
        --no-privileges --inserts --column-inserts 
        --dbname="postgres://vishus:Mutex7892@localhost/blog"
    */ /*
     "C:\Program Files\PostgreSQL\10\bin\pg_dump.exe" 
         --file "db_backup-testy.sql" --format=p --no-owner 
         --create --no-privileges --inserts --column-inserts 
         --dbname="postgres://postgres:andrew@localhost/blog"
    */ /*
        Content-Disposition: attachment; filename="MyFileName.ext"
        Content-Transfer-Encoding: binary
        Content-Length: 
    */
    
    let constr = format!("--dbname=\"{}\"", DATABASE_URL);
    
    #[cfg(not(production))]
    let output_rst = Command::new(DB_BACKUP_SCRIPT).output();
    #[cfg(production)]
    let output_rst = Command::new(DB_BACKUP_SCRIPT)
        .arg(DB_BACKUP_ARG)
        .output();
    
    if let Ok(output) = output_rst {
        let now = Local::now().naive_local();
        let today = now.date();
        
        let dl_name = now.format("db_blog_%Y-%m-%d.sql").to_string();
        
        let disposition = ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![DispositionParam::Filename(
              Charset::Iso_8859_1, // The character set for the bytes of the filename
              None, // The optional language tag (see `language-tag` crate)
                dl_name.into_bytes() // b"db_blog-".to_vec() // the actual bytes of the filename
            )]
        };
        
        let backup = String::from_utf8_lossy(&output.stdout).into_owned();
        let length = backup.len();
        // println!("Backup succeeded with a length of {} bytes", length);
        let express: Express = backup.into();
        express.set_content_type(ContentType::Binary)
                .add_header(disposition)
    } else {
        let output = hbs_template(TemplateBody::General(alert_danger("Backup failed.")), None, Some("Backup Failed".to_string()), String::from("/backup"), Some(admin), user, None, Some(start.0));
        
        let express: Express = output.into();
        express.compress(encoding)
    }
}

#[get("/pageviews", rank=2)]
pub fn hbs_pageviews_unauthorized(start: GenTimer, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
    let mut loginmsg = String::with_capacity(300);
        loginmsg.push_str("You are not logged in, please <a href=\"");
        loginmsg.push_str(BLOG_URL);
        loginmsg.push_str("admin");
        loginmsg.push_str("\">Login</a>");
    
    let output = hbs_template(TemplateBody::General(alert_danger(&loginmsg)), None, Some("Unauthorized".to_string()), String::from("/pageviews"), None, user, None, Some(start.0));
    let express: Express = output.into();
    express.compress( encoding )
}

#[get("/pagestats", rank=2)]
pub fn hbs_pagestats_unauthorized(start: GenTimer, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
    let mut loginmsg = String::with_capacity(300);
        loginmsg.push_str("You are not logged in, please <a href=\"");
        loginmsg.push_str(BLOG_URL);
        loginmsg.push_str("admin");
        loginmsg.push_str("\">Login</a>");
    
    let output = hbs_template(TemplateBody::General(alert_danger(&loginmsg)), None, Some("Unauthorized".to_string()), String::from("/pagestats"), None, user, None, Some(start.0));
    let express: Express = output.into();
    express.compress( encoding )
}

#[get("/pagestats")]
pub fn hbs_pagestats_no_errors(start: GenTimer, admin: AdministratorCookie, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits, counter_state: State<Counter>, unique_state: State<UStatsWrapper>) -> Redirect {
    Redirect::to("/pagestats/false")
}

#[get("/pagestats/<show_errors>")]
pub fn hbs_pagestats(start: GenTimer, show_errors: bool, admin: AdministratorCookie, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits, counter_state: State<Counter>, unique_state: State<UStatsWrapper>) -> Express {
    use urlencoding::decode;
    use htmlescape::*;
    
    let output: Template;
    let counter_mutex = counter_state.stats.lock();
    if let Ok(counter) = counter_mutex {
        let unique_rwlock = unique_state.0.read();
        if let Ok(unique) = unique_rwlock {
            let mut buffer = String::with_capacity((unique.stats.len() * 1000) + 1000);
            
            // buffer.push_str(r#"<div class="v-stats-container-totals container"><div class="v-stats v-stats-total row"><div class="v-stats-page col"><i class="fa fa-bar-chart" aria-hidden="true"></i> Total Hits</div><div class="v-stats-hits col-auto">"#);
            buffer.push_str(r#"<div class="v-stats-container-totals container">
                <div class="v-stats v-stats-total row">
                <div class="v-stats-page col-md">
                <i class="fa fa-bar-chart" aria-hidden="true"></i> Total Hits"#);
            buffer.push_str("<br>");
            if show_errors {
                buffer.push_str(r#"<a href=""#);
                buffer.push_str(&BLOG_URL);
                buffer.push_str(r#"pagestats/false">Show statistics for normal pages</a>"#);
            } else {
                buffer.push_str(r#"<a href=""#);
                buffer.push_str(&BLOG_URL);
                buffer.push_str(r#"pagestats/true">Show statistics for error pages</a>"#);
            }
            // buffer.push_str(r#"</div><div class="v-stats-hits col-auto">"#);
            buffer.push_str(r#"</div><div class="v-stats-hits col-md">"#);
            buffer.push_str( &((&uhits.0).2.to_string()) );
            buffer.push_str(&format!(r#"<br><a href="{url}download/page_stats.json">Download basic log</a><br><a href="{url}download/unique_stats.json">Download IP Logs</a>"#, url=BLOG_URL));
            // buffer.push_str(r#"</div></div></div><div class="v-stats-container container">"#);
            buffer.push_str(r#"</div></div></div><div class="v-stats-container container">"#);
            
            for (page, hits) in counter.map.iter() {
                // if page is an error page and show_errors is false, skip the page
                if page.starts_with("error") || page.starts_with("!error") {
                    if !show_errors { continue; }
                // if page is not an error page but show_errors is true, skip the entry
                } else if show_errors {
                    continue;
                }
                
                // if !show_errors && (page.starts_with("!error") || page.starts_with("error")) { continue; }
                
                if let Some(u_stats) = unique.stats.get(page) {
                    // Extended statistics are available
                    let total_ips: usize = u_stats.len();
                    let total_visits: usize = u_stats.values().sum();
                    let avg_visits: usize = total_visits / total_ips;
                    
                    buffer.push_str(r#"<div class="v-stats row v-stats-extended"><div class="v-stats-page col">"#);
                    // buffer.push_str(&page);
                    buffer.push_str( &encode_minimal( &decode(&page).unwrap_or("DECODE ERROR".to_owned()) ) );
                    buffer.push_str(r#"</div></div><div class="v-stats row"><div class="v-stats-hits col-auto"></div><div class="v-stats-hits col-lg-2" data-toggle="tooltip" data-html="false" title="Total hits">Hits: "#);
                    buffer.push_str(&hits.to_string());
                    // buffer.push_str(" - ");
                    // buffer.push_str(&total_visits.to_string());
                    buffer.push_str(r#"</div><div class="v-stats-hits col-lg-3" data-toggle="tooltip" data-html="false" title="Unique Visitors">Visitors: "#);
                    buffer.push_str(&total_ips.to_string());
                    buffer.push_str(r#"</div><div class="v-stats-hits col-lg-3" data-toggle="tooltip" data-html="false" title="Average Visits Per Unique Visitor">Avg Visits: "#);
                    buffer.push_str(&avg_visits.to_string());
                    buffer.push_str(r#"</div><div class="v-stats-hits col-lg-3" data-toggle="tooltip" data-html="false" title="Total Visits For Unique Visitors">Total Visits: "#);
                    buffer.push_str(&total_visits.to_string());
                    buffer.push_str(r#"</div></div>"#);
                } else {
                    buffer.push_str(r#"<div class="v-stats row v-stats-basic"><div class="v-stats-page col">"#);
                    // buffer.push_str(&page);
                    buffer.push_str( &encode_minimal( &decode(&page).unwrap_or("DECODE ERROR".to_owned()) ) );
                    buffer.push_str(r#"</div></div><div class="v-stats row"><div class="v-stats-hits col-auto"></div><div class="v-stats-hits col-11" data-toggle="tooltip" data-html="false" title="Total hits">Hits: "#);
                    buffer.push_str(&hits.to_string());
                    buffer.push_str(r#"</div></div>"#);
                }
                
            }
            
            buffer.push_str("</div>");
            if buffer.capacity() > ((unique.stats.len() * 1000) + 1000) {
                println!("Pagestats buffer increased, original size: {}\n\tNew size: {}, new capacity: {}", ((unique.stats.len() * 1000) + 1000), buffer.len(), buffer.capacity());
            }
            output = hbs_template(TemplateBody::General(buffer), None, Some("Page Statistics".to_string()), String::from("/pagestats"), Some(admin), user, None, Some(start.0));
        } else {
            output = hbs_template(TemplateBody::General(alert_danger("Could not retrieve page statistics.<br>Failed to acquire read lock.")), None, Some("Page Views".to_string()), String::from("/pagestats"), Some(admin), user, None, Some(start.0));
        }
        
    } else {
        output = hbs_template(TemplateBody::General(alert_danger("Could not retrieve page statistics.<br>Failed to acquire mutex lock.")), None, Some("Page Views".to_string()), String::from("/pagestats"), Some(admin), user, None, Some(start.0));
    }
    
    let end = start.0.elapsed();
    println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    
    let express: Express = output.into();
    express.compress(encoding)
}

#[get("/pagestats2")]
// pub fn hbs_pagestats(start: GenTimer, admin: AdministratorCookie, user: Option<UserCookie>, encoding: AcceptCompression/*, uhits: UniqueHits*/, counter_state: State<Counter>, unique_state: State<UniqueHits>) -> Express {
pub fn hbs_pagestats2(start: GenTimer, admin: AdministratorCookie, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits, counter_state: State<Counter>, unique_state: State<UStatsWrapper>) -> Express {
    use urlencoding::decode;
    use htmlescape::*;
    
    let output: Template;
    let stats_lock = counter_state.stats.lock();
    if let Ok(counter) = stats_lock {
        let unique_lock = unique_state.0.read();
        if let Ok(unique_unlocked) = unique_lock {
            let unique = unique_unlocked;
            // let mut buffer = String::with_capacity(unique.len() * 200);
            let mut buffer = String::with_capacity(unique.stats.len() * 500);
            
            // for (page, ips) in unique.stats.iter() {
            // let u_stats = unique.stats.get(page);
            
            for (page, ips) in unique.stats.iter() {
            // for (page, ips) in unique.stats.iter() {
                let total_ips: usize = ips.len();
                let mut total_visits: usize = ips.values().sum();
                let avg_visits: usize = total_visits / total_ips;
                
                // let hits: String = if let Some(h) = (*counter).map.get(page) {
                //     h.to_string()
                // } else {
                //     "-".to_owned()
                // };
                
                
                let hits: String = if let Some(h) = (*counter).map.get(page) {
                    h.to_string()
                } else {
                    "-".to_owned()
                };
                
                
                buffer.push_str(r#"<div class="v-stats row"><div class="v-stats-page col">"#);
                buffer.push_str(&page);
                buffer.push_str(r#"</div></div><div class="v-stats row"><div class="v-stats col-1"></div><div class="v-stats-hits col-3" data-toggle="tooltip" data-html="false" title="Total hitsr">Hits: "#);
                buffer.push_str(&hits);
                buffer.push_str(r#"</div><div class="v-stats-hits col-4" data-toggle="tooltip" data-html="false" title="Unique Visitors">Visitors: "#);
                buffer.push_str(&total_ips.to_string());
                buffer.push_str(r#"</div><div class="v-stats-hits col-4" data-toggle="tooltip" data-html="false" title="Average Visits Per Unique Visitor">Avg Visits: "#);
                buffer.push_str(&avg_visits.to_string());
                
                // buffer.push(r#"</div><div class="v-stats-hits col-4" data-toggle="tooltip" data-html="false" title="">"#);
                // buffer.push();
                buffer.push_str(r#"</div></div>"#);
            }
            
            // if buffer.capacity() > (unique.len() * 500) {
            //     println!("Pagestats buffer capacity increased to {}, using {} items", buffer.capacity(), buffer.len() ):
            // }
            
            let mut page = String::with_capacity(buffer.len() + 500);
            
            // <div class="v-stats-container-totals container"><div class="v-stats v-stats-total row"><div class="v-stats-page col"><i class="fa fa-bar-chart" aria-hidden="true"></i> Total Hits</div><div class="v-stats-hits col-auto">
            // </div></div></div><div class="v-stats-container container">
            
            
            page.push_str(r#"<div class="v-stats-container-totals container"><div class="v-stats v-stats-total row"><div class="v-stats-page col"><i class="fa fa-bar-chart" aria-hidden="true"></i> Total Hits</div><div class="v-stats-hits col-auto">"#);
            page.push_str( &((&uhits.0).2.to_string()) );
            page.push_str(r#"</div></div></div><div class="v-stats-container container">"#);
            page.push_str( &buffer );
            page.push_str("</div>");
            
            output = hbs_template(TemplateBody::General(page), None, Some("Page Statistics".to_string()), String::from("/pagestats"), Some(admin), user, None, Some(start.0));
            
        } else {
            output = hbs_template(TemplateBody::General(alert_danger("Could not retrieve page statistics.<br>Failed to acquire read lock.")), None, Some("Page Views".to_string()), String::from("/pagestats"), Some(admin), user, None, Some(start.0));
        }
    } else {
        output = hbs_template(TemplateBody::General(alert_danger("Could not retrieve page statistics.<br>Failed to acquire mutex lock.")), None, Some("Page Views".to_string()), String::from("/pagestats"), Some(admin), user, None, Some(start.0));
        
    }
    
    let end = start.0.elapsed();
    println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    
    let express: Express = output.into();
    express.compress(encoding)
}

#[get("/pageviews")]
pub fn hbs_pageviews(start: GenTimer, admin: AdministratorCookie, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits, stats: State<Counter>) -> Express {
    use urlencoding::decode;
    use htmlescape::*;
    
    let output: Template;
    let lockstats = stats.stats.lock();
    // if let 
    if let Ok(counter) = lockstats {
        
        let statistics: Vec<String> = counter.map.iter()
            .map(|(n, v)| 
                format!(r#"<div class="v-stats row"><div class="v-stats-page col">{}</div><div class="v-stats-hits col-auto">{}</div></div>"#, 
                    encode_minimal(&decode(n).unwrap_or(String::new()) ), v))
            .collect();
        
        
        let pages = statistics.join("\n");
        let mut page: String = String::with_capacity(pages.len() + 250);
        page.push_str(r#"<div class="v-stats-container-totals container"><div class="v-stats v-stats-total row"><div class="v-stats-page col"><i class="fa fa-bar-chart" aria-hidden="true"></i> Total Hits</div><div class="v-stats-hits col-auto">"#);
        // page.push_str(&hits.2.to_string());
        page.push_str(&((&uhits.0).2.to_string()));
        page.push_str(r#"</div></div></div><div class="v-stats-container container">"#);
        page.push_str(&pages);
        page.push_str(r#"</div>"#);
        
        output = hbs_template(TemplateBody::General(page), None, Some("Page Views".to_string()), String::from("/pageviews"), Some(admin), user, None, Some(start.0));
    } else {
        output = hbs_template(TemplateBody::General(alert_danger("Could not retrieve page statistics.<br>Failed to acquire mutex lock.")), None, Some("Page Views".to_string()), String::from("/pageviews"), Some(admin), user, None, Some(start.0));
    }
    
    let express: Express = output.into();
    express.compress(encoding)
    
}

// BLOCK 3
// ROUTES: /


