

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

#[post("/admin", data = "<form>")]
fn admin_process(form: Form<LoginFormStatus<AdminAuth>>, cookies: Cookies) -> LoginFormRedirect {
    let inside = form.into_inner();
    let failurl = inside.fail_str();
    // form.into_inner().redirect("/admin", cookies).unwrap_or( LoginFormRedirect(Redirect::to(failurl)) )
    inside.redirect("/admin", cookies).unwrap_or( LoginFormRedirect::new(Redirect::to(failurl)) )
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

#[post("/user", data = "<form>")]
fn user_process(form: Form<LoginFormStatus<UserAuth>>, cookies: Cookies) -> LoginFormRedirect {
    let inside = form.into_inner();
    let failurl = inside.fail_str();
    inside.redirect("/user", cookies).unwrap_or( LoginFormRedirect::new(Redirect::to(failurl)) )
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
        admin_process,
        user_page,
        user_login,
        user_process,
        index,
        static_files
        ]).launch();
}






