
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


#![feature(custom_derive)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

// extern crate multipart;
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_simpleauth as auth;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate rmp_serde as rmps;

extern crate chrono;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate time;

extern crate postgres;
extern crate r2d2;
extern crate r2d2_postgres;

extern crate brotli;
extern crate zopfli;
extern crate titlecase;

extern crate handlebars;
#[allow(unused_imports)]
#[macro_use] extern crate serde_json;
extern crate htmlescape;
extern crate rss;
extern crate dotenv;
#[macro_use] extern crate log;
extern crate env_logger;
// #[macro_use] extern crate diesel_codegen;
// #[macro_use] extern crate diesel;

extern crate rocket_auth_login;

mod responder;
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
mod ral_administrator;
mod ral_user;
mod pages_administrator;

use responder::*;
use ral_administrator::*;
use pages_administrator::*;
use data::*;
use pages::*;
use templates::TemplateMenu;
use rocket_auth_login::authorization::*;
use rocket_auth_login::*;


use handlebars::Handlebars;
use titlecase::titlecase;
use regex::Regex;
use std::time::Instant;

use std::{env, str, io};
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use rocket_contrib::Template;
use rocket::{Request, Data, Outcome, Response};
use rocket::response::{content, NamedFile, Redirect, Flash, Responder, Content};
use rocket::response::content::Html;
use rocket::data::FromData;
use rocket::request::{FlashMessage, Form, FromForm, FormItems};
use rocket::http::{Cookie, Cookies, MediaType, ContentType, Status};


use auth::userpass::UserPass;
use auth::status::{LoginStatus,LoginRedirect};
use auth::dummy::DummyAuthenticator;
use auth::authenticator::Authenticator;
// use rocket::request::FlashMessage;



// use chrono::prelude::*;
// use multipart::server::Multipart;



// BLOG_URL MUST HAVE A TRAILING FORWARD SLASH /
pub const BLOG_URL: &'static str = "http://localhost:8000/";
pub const USER_LOGIN_URL: &'static str = "http://localhost:8000/user";
pub const ADMIN_LOGIN_URL: &'static str = "http://localhost:8000/admin";
pub const TEST_LOGIN_URL: &'static str = "http://localhost:8000/login";
pub const CREATE_FORM_URL: &'static str = "http://localhost:8000/create";
pub const MAX_CREATE_TITLE: usize = 120;
pub const MAX_CREATE_DESCRIPTION: usize = 400;
pub const MAX_CREATE_TAGS: usize = 250;


#[get("/<file..>", rank=10)]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}


// TEST ROUTE 1
// #[allow(unused_mut)]
// #[post("/admin_login", data = "<form>")]
// // fn process_admin_login(form: Form<LoginCont<AdminLogin>>, mut cookies: Cookies) -> Result<Redirect, Flash<Redirect>> {
// // fn process_admin_login(form: Form<LoginCont<AdministratorForm>>, mut cookies: Cookies) -> Result<Redirect, Flash<Redirect>> {
// // fn process_admin_login(form: Form<AdministratorForm>, mut cookies: Cookies) -> Result<Redirect, Flash<Redirect>> {
// pub fn process_admin_login(form: Form<LoginCont<AdministratorForm>>, mut cookies: Cookies) -> Result<Redirect, Flash<Redirect>> {
//     let start = Instant::now();
    
//     // let inner = form.into_inner();
//     // let inner = &form;
//     // let login = inner.form;
//     let login = form.into_inner();
//     let output = login.flash_redirect("/dashboard", "/admin_login", cookies);
    
//     let end = start.elapsed();
//     println!("Processed in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
//     output
// }

// TEST ROUTE 2
// #[post("/admin_login", data = "<form>")]
// fn process_login(form: Form<AdministratorForm>, mut cookies: Cookies) -> Result<Redirect, Flash<Redirect>> {
//     // let inner = form.into_inner();
//     // let login = inner.form;
//     let login = form.into_inner();
//     login.flash_reidrect("/dashboard", "/login", cookies)
// }

lazy_static! {
    static ref PGCONN: Mutex<DbConn> = Mutex::new( DbConn(init_pg_pool().get().expect("Could not connect to database.")) );
}

// #[get("/testroute")]
// // fn test_route(mut cookies: rocket::http::cookies::Cookies) -> Result<rocket::response::redirect::Redirect, rocket::response::flash::Flash<rocket::response::redirect::Redirect>> {
// fn test_route(mut cookies: Cookies) -> Result<Redirect, Flash<Redirect>> {
//     let testform = AdministratorForm {
//         username: String::from("andrew"),
//         password: String::from("password"),
//     };
//     // let testresult = testform.from_form();
//     // let testresult = testform.flash_redirect("/dashboard", "/login", cookies as rocket::http::cookies::Cookies);
//     let testresult = testform.flash_redirect("/dashboard", "/login", cookies);
//     testresult
// }

fn main() {
    // use login::authorization::LoginCont;
    // use ral_administrator::AdministratorForm;
    
    // env_logger::init();
    // init_pg_pool();
    // if let Ok(pg_conn) = init_pg_pool().get() {
    //     pg_conn;
    //     // (*pg_conn).connect();
    // }
    
    init_pg_pool().get().unwrap();
    
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
            pages::hbs_article_title,
            pages::hbs_article_id,
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
            
            pages_administrator::dashboard_admin_authorized,
            pages_administrator::dashboard_admin_unauthorized,
            pages_administrator::dashboard_admin_login,
            pages_administrator::dashboard_admin_retry_user,
            pages_administrator::dashboard_admin_retry_flash,
            pages_administrator::process_admin_login,
            pages_administrator::logout_admin,
            
            pages_administrator::dashboard_user_authorized,
            pages_administrator::dashboard_user_unauthorized,
            pages_administrator::dashboard_user_login,
            pages_administrator::dashboard_user_retry_user,
            pages_administrator::dashboard_user_retry_flash,
            pages_administrator::process_user_login,
            pages_administrator::logout_user,
            
            static_files
        ])
        .launch();
}


