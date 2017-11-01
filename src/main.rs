
/* Todo:
  AUTHENTICATE
    Setup a database with a user table and query the username and password for authentication
  ARTICLES
    Setup a database table with an article table
  VIEW
    Add: articles in a category, articles with a tag, single article
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
  POLYMORPHIC AUTH
    Wrap the AdminAuth and UserAuth structs in another struct or trait
      -The trait should require the functions that the structs currently implement
          but the functions will be moved to the wrapper struct
          

*/


#![feature(custom_derive)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

// extern crate multipart;
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_simpleauth as auth;
extern crate serde;
#[macro_use] extern crate serde_derive;

extern crate chrono;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate time;

extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;

extern crate titlecase;

extern crate handlebars;
#[macro_use] extern crate serde_json;
extern crate htmlescape;
extern crate rss;


use handlebars::Handlebars;
use titlecase::titlecase;
use regex::Regex;
use std::time::Instant;

use std::{env, str, io};
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};

use rocket_contrib::Template;
use rocket::{Request, Data, Outcome, Response};
use rocket::response::{content, NamedFile, Redirect, Flash, Responder, Content};
use rocket::response::content::Html;
use rocket::data::FromData;
use rocket::request::{FlashMessage, Form};
use rocket::http::{Cookie, Cookies, MediaType, ContentType, Status};
use auth::userpass::UserPass;
use auth::status::{LoginStatus,LoginRedirect};
use auth::dummy::DummyAuthenticator;
use auth::authenticator::Authenticator;
// use rocket::request::FlashMessage;

// use chrono::prelude::*;
// use multipart::server::Multipart;
// #[macro_use] extern crate diesel;
// #[macro_use] extern crate diesel_codegen;
extern crate dotenv;
#[macro_use] extern crate log;
extern crate env_logger;

mod layout;
mod cookie_data;
mod admin_auth;
mod user_auth;
mod users;
mod login_form_status;
mod blog;
mod data;
mod templates;
mod pages;
mod sanitize;

// use layout::*;
// use cookie_data::*;
// use admin_auth::*;
// use user_auth::*;
// use users::*;
// use login_form_status::*;
// use login_form_status::LoginFormRedirect;
// use blog::*;
// use templates::*;
use data::*;
use pages::*;
use templates::TemplateMenu;

// BLOG_URL MUST HAVE A TRAILING FORWARD SLASH /
pub const BLOG_URL: &'static str = "http://localhost:8000/";
pub const USER_LOGIN_URL: &'static str = "http://localhost:8000/user";
pub const ADMIN_LOGIN_URL: &'static str = "http://localhost:8000/admin";
pub const CREATE_FORM_URL: &'static str = "http://localhost:8000/create";
pub const MAX_CREATE_TITLE: usize = 120;
pub const MAX_CREATE_DESCRIPTION: usize = 400;
pub const MAX_CREATE_TAGS: usize = 250;


#[get("/<file..>", rank=10)]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}


fn main() {
    // env_logger::init();
    // init_pg_pool();
    // if let Ok(pg_conn) = init_pg_pool().get() {
    //     pg_conn;
    //     // (*pg_conn).connect();
    // }
    
    // init_pg_pool().get();
    
    rocket::ignite()
        .manage(data::init_pg_pool())
        .attach(Template::fairing())
        .mount("/", routes![
            
            pages::hbs_admin_page,
            pages::hbs_admin_login,
            pages::hbs_admin_retry,
            pages::hbs_process_admin,
            pages::hbs_user_page,
            pages::hbs_user_login,
            pages::hbs_user_retry,
            pages::hbs_user_process,
            pages::hbs_all_articles,
            pages::hbs_articles_page,
            pages::hbs_tags_all,
            pages::hbs_articles_tag,
            pages::hbs_article_view,
            pages::hbs_article_not_found,
            pages::hbs_article_process,
            pages::hbs_create_unauthorized,
            pages::hbs_create_form,
            pages::hbs_logout,
            pages::hbs_search_page,
            pages::hbs_search_results,
            pages::hbs_about,
            pages::rss_page,
            pages::hbs_index,
            
            static_files
        ])
        .launch();
}



// #[get("/admin")]
// fn admin_page(data: AdminCookie) -> Html<String> {
//     let start = Instant::now();

//     let body = format!("Welcome {user}! You have reach the administrator page.", user = data.username);
//     let output = template(&body);
    
//     let end = start.elapsed();
//     println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
//     output
// }

// #[get("/admin", rank = 2)]
// fn admin_login() -> Html<String> {
//     let start = Instant::now();

//     let output = template(&login_form(ADMIN_LOGIN_URL));
        
//     let end = start.elapsed();
//     println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
//     output
// }

// #[get("/admin?<fail>")]
// fn admin_retry(fail: AuthFailure) -> Html<String> {
//     template(&login_form_fail(ADMIN_LOGIN_URL, &fail.user, &fail.msg))
// }

// #[post("/admin", data = "<form>")]
// fn admin_process(form: Form<LoginFormStatus<AdminAuth>>, cookies: Cookies) -> LoginFormRedirect {
//     let start = Instant::now();
    
//     let inside = form.into_inner();
//     let failuser = inside.user_str();
//     let failmsg = inside.fail_str();
//     let mut failurl = ADMIN_LOGIN_URL.to_string();
//     if failmsg != "" && failmsg != " " {
//         failurl.push_str("?user=");
//         failurl.push_str(&failuser);
//         failurl.push_str("&msg=");
//         failurl.push_str(&failmsg);
//     }
    
//     let end = start.elapsed();
//     println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    
//     inside.redirect("/admin", cookies).unwrap_or( LoginFormRedirect::new(Redirect::to(&failurl)) )
// }




// #[get("/user")]
// fn user_page(data: UserCookie) -> Html<String> {
//     let body = format!("Welcome {user}! You are at the user page.", user = data.username);
//     template(&body)
// }

// #[get("/user", rank = 2)]
// fn user_login() -> Html<String> {
//     template( &login_form(USER_LOGIN_URL) )
// }

// #[get("/user?<fail>")]
// fn user_retry(fail: AuthFailure) -> Html<String> {
//     template(&login_form_fail(USER_LOGIN_URL, &fail.user, &fail.msg))
// }

// #[post("/user", data = "<form>")]
// fn user_process(form: Form<LoginFormStatus<UserAuth>>, cookies: Cookies) -> LoginFormRedirect {
//     let start = Instant::now();
    
//     let inside = form.into_inner();
//     let failuser = inside.user_str();
//     let failmsg = inside.fail_str();
//     let mut failurl = USER_LOGIN_URL.to_string();
//     if failmsg != "" && failmsg != " " {
//         failurl.push_str("?user=");
//         failurl.push_str(&failuser);
//         failurl.push_str("&msg=");
//         failurl.push_str(&failmsg);
//     }
    
//     let end = start.elapsed();
//     println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    
//     inside.redirect("/user", cookies).unwrap_or( LoginFormRedirect::new(Redirect::to(&failurl)) )
// }



// #[get("/view")]
// fn all_articles(conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Html<String> {
//     let start = Instant::now();
//     let output: Html<String>;
//     let results = Article::retrieve_all(conn, 0, Some(300), None, None, None, None);
//     if results.len() != 0 {
//         let is_admin = if admin.is_some() { true } else { false };
//         let is_user = if user.is_some() { true } else { false };
//         let username: Option<String> = if let Some(a_user) = admin { Some(a_user.username) } else if let Some(u_user) = user { Some(u_user.username) } else { None };
//         output = template_articles(results, is_admin, is_user, username);
//     } else {
//         if admin.is_some() {
//             output = template( &alert_danger("There are no articles<br>\n<a href =\"/insert\">Create Article</a>") );
//         } else {
//             output = template( &alert_danger("There are no articles.") );
//         }
//     }
    
//     let end = start.elapsed();
//     println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
//     output
// }

// #[get("/view?<page>")]
// fn view_page(page: ViewPage, conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Html<String> {
//     // unimplemented!()
//     template("You are viewing paginated articles.")
// }


// #[get("/all_tags")]
// fn all_tags() -> Html<String> {
//     template("All tags page is not yet implemented.")
// }


// #[get("/tag?<tag>", rank = 2)]
// fn view_tag(tag: Tag, conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Html<String> {
//     let start = Instant::now();
    
//     let output: Html<String>;
//     // limit, # body chars, min date, max date, tags, strings
//     let tags = Some(split_tags(tag.tag));
//     let results = Article::retrieve_all(conn, 0, Some(-1), None, None, tags, None);
//     if results.len() != 0 {
//         let is_admin = if admin.is_some() { true } else { false };
//         let is_user = if user.is_some() { true } else { false };
//         let username: Option<String> = if let Some(a_user) = admin { Some(a_user.username) } else if let Some(u_user) = user { Some(u_user.username) } else { None };
//         output = template_articles(results, is_admin, is_user, username);
        
//     } else {
//         output = template( &alert_danger("Could not find any articles with the specified tag.") );
//     }
    
//     let end = start.elapsed();
//     println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
//     output
// }

// #[get("/article?<aid>")]
// fn view_article(aid: ArticleId, conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Html<String> {
//     let start = Instant::now();
//     let rst = aid.retrieve_with_conn(conn); // retrieve result
//     let mut output: Html<String>; 
//     if let Some(article) = rst {
//         let is_admin = if admin.is_some() { true } else { false };
//         let is_user = if user.is_some() { true } else { false };
//         let username: Option<String> = if let Some(a_user) = admin { Some(a_user.username) } else if let Some(u_user) = user { Some(u_user.username) } else { None };
//         output = full_template_article(&article, is_admin, is_user, username)
//     } else {
//         // output =  template(&format!("Article #{id} could not be retrieved.", id=aid.aid))
//         output =  template(&alert_danger(&format!("Article {id} could not be found.", id=aid.aid)))
//     }
//     let end = start.elapsed();
//     println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
//     output
// }

// #[get("/article")]
// fn article_not_found() -> Html<String> {
//     let start = Instant::now();
//     let mut content = String::from("The article you have specified does not exist.");
    
//     let output = template(&content);
//     let end = start.elapsed();
//     println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
//     output
// }

// #[post("/article", data = "<form>")]
// fn post_article(admin: AdminCookie, form: Form<ArticleForm>, conn: DbConn) -> Html<String> {
//     let start = Instant::now();
    
//     let result = form.into_inner().save(&conn);
//     let output: Html<String>;
//     match result { 
//         Ok(article) => {
//             output = full_template_article(&article, true, true, Some(admin.username));
//         },
//         Err(why) => {
//             output = template(&format!("Could not post the blog article.  Reason: {}", why));
//         },
//     }
    
//     let end = start.elapsed();
//     println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
//     output
// }
// #[post("/article", rank=2)]
// fn unauthorized_post() -> Html<String> {
    
//     template(UNAUTHORIZED_POST_MESSAGE)
// }

// #[get("/insert")]
// fn insert_form(user: Option<AdminCookie>) -> Html<String> {
//     let start = Instant::now();
    
//     let content;
//     if let Some(admin) = user {
//         content = template_create_article(BLOG_URL);
//     } else {
//         content = UNAUTHORIZED_POST_MESSAGE.to_string();
//     }
//     let output = template(&content);
//     let end = start.elapsed();
//     println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
//     output
// }

// #[get("/logout")]
// fn logout(admin: Option<AdminCookie>, user: Option<UserCookie>, mut cookies: Cookies) -> Result<Flash<Redirect>, Redirect> {
//     if admin.is_some() || user.is_some() {
//         if let Some(a) = admin {
//             cookies.remove_private(Cookie::named(AdminCookie::get_cid()));
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

// #[get("/search")]
// fn search_page() -> Html<String> {
//     // unimplemented!()
//     template("The search page has not been implemented yet.")
// }

// #[get("/search?<search>")]
// fn search_results(search: Search, conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Html<String> {
//     // unimplemented!()
//     template("The search results page has not been implemented yet.")
// }

// #[get("/about")]
// fn about() -> Html<String> {
//     template("This is the about page.")
// }

// #[get("/template")]
// fn template_testing(conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
//     let output: Template;
//     let start = Instant::now();
    
//     output = hbs_template(TemplateBody::General("This is some content.".to_string()), Some("Article Page".to_string()), admin, user, None, Some(start));
    
//     let end = start.elapsed();
//     println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
//     output
// }

// #[get("/template2")]
// fn template_testing2(conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
//     let start = Instant::now();
//     let output: Template;
//     let aid = ArticleId { aid: 9 };
//     let result = aid.retrieve_with_conn(conn);
//     if let Some(article) = result {
//         let a_title = article.title.clone();
//         output = hbs_template(TemplateBody::Article(article), Some(format!("VishusBlog::{}", a_title)), admin, user, None, Some(start));
//     } else {
//         output = hbs_template(TemplateBody::General("Could not find article 9".to_string()), Some("VishusBlog::Invalid Article".to_string()), admin, user, None, Some(start));
//     }
    
//     let end = start.elapsed();
//     println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
//     output
// }

// #[get("/template3")]
// fn template_testing3(conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
//     let start = Instant::now();
//     let output: Template;
//     let results = Article::retrieve_all(conn, 0, Some(300), None, None, None, None);
//     if results.len() != 0 {
//         output = hbs_template(TemplateBody::Articles(results), Some("VishusBlog::Viewing All Articles".to_string()), admin, user, None, Some(start));
//     } else {
//         output = hbs_template(TemplateBody::General("Could not find any articles.".to_string()), Some("VishusBlog::Invalid Articles".to_string()), admin, user, None, Some(start));
//     }
    
//     let end = start.elapsed();
//     println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
//     output
// }

// #[get("/login")]
// fn login_admin(conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
//     let output: Template;
//     let start = Instant::now();
    
//     output = hbs_template(TemplateBody::Login(ADMIN_LOGIN_URL.to_string(), None, None), Some("Administrator Login".to_string()), admin, user, None, Some(start));
    
//     let end = start.elapsed();
//     println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
//     output
// }

// #[get("/create")]
// fn create_post(conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
//     let start = Instant::now();
//     let output: Template;
//     if admin.is_some() {
//         output = hbs_template(TemplateBody::Create(CREATE_FORM_URL.to_string()), Some("New Post".to_string()), admin, user, None, Some(start));
//     } else {
//         output = hbs_template(TemplateBody::General(UNAUTHORIZED_POST_MESSAGE.to_string()), Some("VishusBlog::Not Authorized".to_string()), admin, user, None, Some(start));
//     }
    
//     let end = start.elapsed();
//     println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
//     output
// }

// #[get("/")]
// fn index(conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>, flash: Option<FlashMessage>) -> Html<String> {
//     // let body = r#"Hello! This is a blog.<br><a href="/user">User page</a><br><a href="/admin">Go to admin page</a>"#;
//     // template(body)
//     let start = Instant::now();
//     let mut output: Html<String> = Html(String::new());
//     if let Some(flashmsg) = flash {
//         // let output: Html<String>;
//         let results = Article::retrieve_all(conn, 0, Some(300), None, None, None, None);
//         if results.len() != 0 {
//             let is_admin = if admin.is_some() { true } else { false };
//             let is_user = if user.is_some() { true } else { false };
//             let username: Option<String> = if let Some(a_user) = admin { Some(a_user.username) } else if let Some(u_user) = user { Some(u_user.username) } else { None };
            
//             output = template_articles_msg(&alert_success("You have been logged out."), false, results, is_admin, is_user, username);
//         } else {
//             if admin.is_some() {
//                 output = template( &alert_danger("There are no articles<br>\n<a href =\"/insert\">Create Article</a>") );
//             } else {
//                 output = template( &alert_danger("There are no articles.") );
//             }
//         }
//     } else {
//         output = all_articles(conn, admin, user);
//     }
//     let end = start.elapsed();
//     println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
//     output
// }





