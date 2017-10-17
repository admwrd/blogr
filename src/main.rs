

#![feature(custom_derive)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

// extern crate multipart;
extern crate rocket;
extern crate rocket_simpleauth as auth;

// extern crate chrono;
// #[macro_use] extern crate lazy_static;
// extern crate regex;
extern crate time;

// use regex::Regex;
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
use layout::*;

// mod cookies;
// use cookies::*;

mod cookie_data;
use cookie_data::*;

mod admin_auth;
use admin_auth::*;

mod user_auth;
use user_auth::*;

mod user_data;
use user_data::*;

mod login_form_status;
use login_form_status::*;
use login_form_status::LoginFormRedirect;

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
        index,
        static_files
        ]).launch();
}






