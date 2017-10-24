
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

mod layout;
mod cookie_data;
mod admin_auth;
mod user_auth;
mod users;
mod login_form_status;
mod blog;

use layout::*;
use cookie_data::*;
use admin_auth::*;
use user_auth::*;
use users::*;
use login_form_status::*;
use login_form_status::LoginFormRedirect;
use blog::*;

#[get("/admin")]
fn admin_page(data: AdminCookie) -> Html<String> {
    let body = format!("Welcome {user}! You have reach the administrator page.", user = data.username);
    template(&body)
}

#[get("/admin", rank = 2)]
fn admin_login() -> Html<&'static str> {
    Html(template_login_admin())
}

#[get("/admin?<fail>")]
// #[get("/user/?<user>&<msg>")]
// #[get("/user/<user>/<msg>")]
// fn user_retry(user: String, msg: String) -> Html<String> {
fn admin_retry(fail: AuthFailure) -> Html<String> {
    template( &template_admin_login_fail(&fail.user, &fail.msg) )
}

#[post("/admin", data = "<form>")]
fn admin_process(form: Form<LoginFormStatus<AdminAuth>>, cookies: Cookies) -> LoginFormRedirect {
    let inside = form.into_inner();
    let failuser = inside.user_str();
    let failmsg = inside.fail_str();
    let mut failurl = "http://localhost:8000/admin".to_string();
    if failmsg != "" && failmsg != " " {
        failurl.push_str("?user=");
        failurl.push_str(&failuser);
        failurl.push_str("&msg=");
        failurl.push_str(&failmsg);
    }
    inside.redirect("/admin", cookies).unwrap_or( LoginFormRedirect::new(Redirect::to(&failurl)) )
}

#[get("/user")]
fn user_page(data: UserCookie) -> Html<String> {
    let body = format!("Welcome {user}! You are at the user page.", user = data.username);
    template(&body)
}

#[get("/user", rank = 2)]
fn user_login() -> Html<&'static str> {
    Html(template_login_user())
}

#[get("/user?<fail>")]
// #[get("/user/?<user>&<msg>")]
// #[get("/user/<user>/<msg>")]
// fn user_retry(user: String, msg: String) -> Html<String> {
fn user_retry(fail: AuthFailure) -> Html<String> {
    template( &template_user_login_fail(&fail.user, &fail.msg) )
}

#[post("/user", data = "<form>")]
fn user_process(form: Form<LoginFormStatus<UserAuth>>, cookies: Cookies) -> LoginFormRedirect {
    let inside = form.into_inner();
    let failuser = inside.user_str();
    let failmsg = inside.fail_str();
    let mut failurl = "http://localhost:8000/user".to_string();
    if failmsg != "" && failmsg != " " {
        failurl.push_str("?user=");
        failurl.push_str(&failuser);
        failurl.push_str("&msg=");
        failurl.push_str(&failmsg);
    }
    inside.redirect("/user", cookies).unwrap_or( LoginFormRedirect::new(Redirect::to(&failurl)) )
}

#[get("/view")]
fn all_articles() -> Html<String> {
    let mut content = String::new();
    
    template(&content)
}


#[get("/view?<category>")]
fn view_category(category: Category) -> Html<String> {
    let mut content = String::new();
    
    template(&content)
}


#[get("/view?<tag>", rank = 2)]
fn view_tag(tag: Tag) -> Html<String> {
    let mut content = String::new();
    
    template(&content)
}

#[get("/article?<aid>")]
fn view_article(aid: ArticleId) -> Html<String> {
    let article = aid.retrieve();
    let mut content = String::new();
    
    template(&content)
}

#[get("/article")]
fn article_not_found() -> Html<String> {
    let mut content = String::new();
    
    template(&content)
}

#[post("/article", data = "<form>")]

fn post_article(user: Option<UserCookie>, admin: Option<AdminCookie>, form: Form<ArticleForm>) -> Html<String> {
    let result = form.into_inner().save();
    let mut content = String::new();
    
    match result {
        Ok(article) => {
            // article, admin, user, username
            let is_admin = if admin.is_some() { true } else { false };
            let is_user = if user.is_some() { true } else { false };
            let username: Option<String> = if is_user { Some(user.unwrap().username) } else if is_admin { Some(admin.unwrap().username) } else { None };
            content.push_str(template_article( article, is_admin, is_user, username) );
        },
        Err(why) => {
            content.push_str(&format!("Could not post the blog article.  Reason: {}", why))
        },
    }
    
    template(&content)
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
    rocket::ignite().mount("/", routes![
        admin_page,
        admin_login,
        admin_retry,
        admin_process,
        
        user_page,
        user_retry,
        user_login,
        user_process,
        
        all_articles,
        view_category,
        view_tag,
        view_article,
        article_not_found,
        post_article,
        
        index,
        static_files
        ]).launch();
}






