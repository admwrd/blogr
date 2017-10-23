
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
extern crate rocket_simpleauth as auth;

extern crate chrono;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate time;

extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;

extern crate titlecase;

use titlecase::titlecase;

use regex::Regex;
// use chrono::prelude::*;
// use multipart::server::Multipart;

use std::time::Instant;

use std::{env, str, io};
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};

use rocket::response::{content, NamedFile, Redirect};
use rocket::{Request, Data, Outcome};
use rocket::data::FromData;
use rocket::response::content::Html;
use rocket::request::Form;
use rocket::http::Cookies;

use auth::userpass::UserPass;
use auth::status::{LoginStatus,LoginRedirect};
use auth::dummy::DummyAuthenticator;
use auth::authenticator::Authenticator;

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

use layout::*;
use cookie_data::*;
use admin_auth::*;
use user_auth::*;
use users::*;
use login_form_status::*;
use login_form_status::LoginFormRedirect;
use blog::*;
use data::*;

pub const BLOG_URL: &'static str = "http://localhost:8000/";
pub const USER_LOGIN_URL: &'static str = "http://localhost:8000/user";
pub const ADMIN_LOGIN_URL: &'static str = "http://localhost:8000/admin";


#[get("/admin")]
fn admin_page(data: AdminCookie) -> Html<String> {
    let start = Instant::now();

    let body = format!("Welcome {user}! You have reach the administrator page.", user = data.username);
    let output = template(&body);
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/admin", rank = 2)]
fn admin_login() -> Html<String> {
    let start = Instant::now();

    // let output = Html(template_login_admin());
    let output = template(&login_form(ADMIN_LOGIN_URL));
        
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/admin?<fail>")]
fn admin_retry(fail: AuthFailure) -> Html<String> {
    // template( &template_admin_login_fail(&fail.user, &fail.msg) )
    template(&login_form_fail(ADMIN_LOGIN_URL, &fail.user, &fail.msg))
}

#[post("/admin", data = "<form>")]
fn admin_process(form: Form<LoginFormStatus<AdminAuth>>, cookies: Cookies) -> LoginFormRedirect {
    let start = Instant::now();
    
    let inside = form.into_inner();
    let failuser = inside.user_str();
    let failmsg = inside.fail_str();
    let mut failurl = ADMIN_LOGIN_URL.to_string();
    if failmsg != "" && failmsg != " " {
        failurl.push_str("?user=");
        failurl.push_str(&failuser);
        failurl.push_str("&msg=");
        failurl.push_str(&failmsg);
    }
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    
    inside.redirect("/admin", cookies).unwrap_or( LoginFormRedirect::new(Redirect::to(&failurl)) )
}




#[get("/user")]
fn user_page(data: UserCookie) -> Html<String> {
    let body = format!("Welcome {user}! You are at the user page.", user = data.username);
    template(&body)
}

#[get("/user", rank = 2)]
fn user_login() -> Html<String> {
    template( &login_form(USER_LOGIN_URL) )
    // Html(template_login_user())
}

#[get("/user?<fail>")]
// #[get("/user/?<user>&<msg>")]
// #[get("/user/<user>/<msg>")]
// fn user_retry(user: String, msg: String) -> Html<String> {
fn user_retry(fail: AuthFailure) -> Html<String> {
    // template( &template_user_login_fail(&fail.user, &fail.msg) )
    template(&login_form_fail(USER_LOGIN_URL, &fail.user, &fail.msg))
}

#[post("/user", data = "<form>")]
fn user_process(form: Form<LoginFormStatus<UserAuth>>, cookies: Cookies) -> LoginFormRedirect {
    let start = Instant::now();
    
    let inside = form.into_inner();
    let failuser = inside.user_str();
    let failmsg = inside.fail_str();
    let mut failurl = USER_LOGIN_URL.to_string();
    if failmsg != "" && failmsg != " " {
        failurl.push_str("?user=");
        failurl.push_str(&failuser);
        failurl.push_str("&msg=");
        failurl.push_str(&failmsg);
    }
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    
    inside.redirect("/user", cookies).unwrap_or( LoginFormRedirect::new(Redirect::to(&failurl)) )
}




#[get("/view")]
fn all_articles(conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Html<String> {
    let start = Instant::now();
    // let mut content = String::from("You are viewing all of the articles.");
    let output: Html<String>;
    let results = Article::retrieve_all(conn, 20, Some(300), None, None, None, None);
    if results.len() != 0 {
        let is_admin = if admin.is_some() { true } else { false };
        let is_user = if user.is_some() { true } else { false };
        let username: Option<String> = if let Some(a_user) = admin { Some(a_user.username) } else if let Some(u_user) = user { Some(u_user.username) } else { None };
        output = template_articles(results, is_admin, is_user, username);
    } else {
        if admin.is_some() {
            output = template(r##"
            <div class="alert alert-danger" role="alert">
                There are no articles.<br>
                <a href="/insert">Create Article</a>
            </div>"##);
        } else {
            output = template(r##"
            <div class="alert alert-danger" role="alert">
                There are no articles.
            </div>"##);
        }
    }
    
    // template(&content)
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/view?<page>")]
fn view_page(page: ViewPage, conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Html<String> {
    // unimplemented!()
    template("You are viewing paginated articles.")
}

#[get("/tag?<tag>", rank = 2)]
fn view_tag(tag: Tag, conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Html<String> {
    // find articles where tag LIKE '%<tag.tag>%'
    let start = Instant::now();
    
    let output: Html<String>;
    // limit, # body chars, min date, max date, tags, strings
    let tags = Some(split_tags(tag.tag));
    let results = Article::retrieve_all(conn, 0, None, None, None, tags, None);
    if results.len() != 0 {
        let is_admin = if admin.is_some() { true } else { false };
        let is_user = if user.is_some() { true } else { false };
        let username: Option<String> = if let Some(a_user) = admin { Some(a_user.username) } else if let Some(u_user) = user { Some(u_user.username) } else { None };
        output = template_articles(results, is_admin, is_user, username);
        
    } else {
        output = template(r##"
            <div class="alert alert-danger" role="alert">
                Could not find any articles with the specified tag.
            </div>"##);
    }
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    // template(&content)
    output
}

#[get("/article?<aid>")]
// #[get("/article?<aid>")]
// fn view_article(aid: ArticleId) -> Html<String> {
fn view_article(aid: ArticleId, conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Html<String> {
    let start = Instant::now();
    // let article: Article = aid.retrieve();
    // let mut content = String::new();
    // content.push_str("You have reached the article page.<br>\n");
    // let rst = aid.retrieve(); // retrieve result
    let rst = aid.retrieve_with_conn(conn); // retrieve result
    let mut output: Html<String>; 
    if let Some(article) = rst {
        // admin, user, username
        let is_admin = if admin.is_some() { true } else { false };
        let is_user = if user.is_some() { true } else { false };
        let username: Option<String> = if let Some(a_user) = admin { Some(a_user.username) } else if let Some(u_user) = user { Some(u_user.username) } else { None };
        output = full_template_article(&article, is_admin, is_user, username)
        // content.push_str(&format!("You are viewing article #{id}.<br>\nInfo:<br>\n", id=aid.aid));
        // content.push_str(&article.info());
    } else {
        output =  template(&format!("Article #{id} could not be retrieved.", id=aid.aid))
    }
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
    // template(&content)
}

#[get("/article")]
fn article_not_found() -> Html<String> {
    let start = Instant::now();
    let mut content = String::from("The article you have specified does not exist.");
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    template(&content)
}

#[post("/article", data = "<form>")]

// fn post_article(user: Option<UserCookie>, admin: Option<AdminCookie>, form: Form<ArticleForm>, conn: DbConn) -> Html<String> {
fn post_article(admin: AdminCookie, form: Form<ArticleForm>, conn: DbConn) -> Html<String> {
    let start = Instant::now();
    
    let mut content = String::new();
    let result = form.into_inner().save(&conn);
    match result {
        Ok(article) => {
            // article, admin, user, username
            // let is_admin = if admin.is_some() { true } else { false };
            // let is_user = if user.is_some() { true } else { false };
            // let username: Option<String> = if is_user { Some(user.unwrap().username) } else if is_admin { Some(admin.unwrap().username) } else { None };
            // content.push_str(&template_article( &article, is_admin, is_user, username) );
            content.push_str(&template_article( &article, true, true, Some(admin.username) ));
        },
        Err(why) => {
            content.push_str(&format!("Could not post the blog article.  Reason: {}", why))
        },
    }
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    template(&content)
}
#[post("/article", rank=2)]
fn unauthorized_post() -> Html<String> {
    
    template(UNAUTHORIZED_POST_MESSAGE)
}

#[get("/insert")]
fn insert_form(user: Option<AdminCookie>) -> Html<String> {
    let start = Instant::now();
    
    let content;
    if let Some(admin) = user {
        // let content = format!("Insert an article.");
        content = r##"
        <form method="post" action="http://localhost:8000/article" name="insert_form">
            <input type="text" name="title" placeholder="Title"><br>
            <textarea class="form-control" name="body" id="insert_body" rows="3"></textarea><br>
            <input type="text" name="tags" placeholder="Comma, Separated, Tags"><br>
            <button type="submit" class="btn btn-primary">Submit</button>
        </form>
        <script>
        StartText();
        </script>
        "##;
    } else {
        content = UNAUTHORIZED_POST_MESSAGE;
    }
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    template(&content)
}

#[get("/search")]
fn search_results() -> Html<String> {
    unimplemented!()
}

#[get("/")]
fn index() -> Html<String> {
    let body = r#"Hello! This is a blog.<br><a href="/user">User page</a><br><a href="/admin">Go to admin page</a>"#;
    template(&body)
}

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
    init_pg_pool().get();
    
    rocket::ignite()
        .manage(data::init_pg_pool())
        .mount("/", routes![
            admin_page,
            admin_login,
            admin_retry,
            admin_process,
            
            user_page,
            user_retry,
            user_login,
            user_process,
            
            all_articles,
            // view_category,
            view_tag,
            view_page,
            view_article,
            article_not_found,
            post_article,
            insert_form,
            unauthorized_post,
            
            index,
            static_files
        ])
        // .manage(data::pg_conn_pool())
        .launch();
}






