

use rocket::response::{NamedFile, Redirect, Flash};
use rocket::response::content::Html;
use rocket::request::{FlashMessage, Form};
use rocket::http::{Cookie, Cookies};
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use postgres::Connection;
use std::sync::Mutex;
use std::path::{Path, PathBuf};

use rocket_auth_login::authorization::*;
use rocket_auth_login::sanitization::*;
use ral_administrator::*;

use ::templates::*;
use ::blog::*;
use ::data::*;
use ::layout::*;

use super::*;
pub const URL_LOGIN_ADMIN: &'static str = "http://localhost:8000/admin_login";

#[derive(Debug, Clone, FromForm)]
pub struct QueryUser {
    pub user: String,
}

/* Todo:
    Add a struct that implements the Responder trait
        Use this for adding an expiration header
    Add another struct that implements the Responder trait
        Use this for compressing the output using brotli/gzip/deflate
    Add structs that implement the Responder trait
        that will combine the expiration and compression responders
    Maybe even add a struct that will handle static file caching
        Database queries are cached by postgresql
            Look into how postgresql caches recent queries
            and look up how to increase how many queries are cached
            and when they are cached.  Try to get them cached sooner.
*/

#[get("/dashboard", rank = 1)]
pub fn dashboard_authorized(admin: AdministratorCookie, conn: DbConn) -> Template {
    let start = Instant::now();
    
    let output: Template = hbs_template(TemplateBody::General(format!("Welcome Administrator {user}.  You are viewing the administrator dashboard page.", user=admin.username), None), Some("Dashboard".to_string()), String::from("/dashboard"), None, None, None, Some(start));
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/admin_login", rank = 1)]
pub fn dashboard_login() -> Template {
    hbs_template(TemplateBody::Login(URL_LOGIN_ADMIN.to_string(), None, None), Some("Administrator Login".to_string()), String::from("/admin_login"), None, None, None, None)
}

#[get("/admin_login?<user>")]
// fn dashboard_retry_user(user: UserQuery, flash_msg_opt: Option<FlashMessage>) -> Template {
// fn dashboard_retry_user(mut user: String, flash_msg_opt: Option<FlashMessage>) -> Template {
pub fn dashboard_retry_user(mut user: QueryUser, flash_msg_opt: Option<FlashMessage>) -> Template {
    let start = Instant::now();
    // user = login::sanitization::sanitize(&user);
    let username = if &user.user != "" { Some(user.user.clone() ) } else { None };
    let flash = if let Some(f) = flash_msg_opt { Some(alert_danger(f.msg())) } else { None };
    let output = hbs_template(TemplateBody::Login(URL_LOGIN_ADMIN.to_string(), username, flash), Some("Administrator Login".to_string()), String::from("/admin_login"), None, None, None, Some(start));
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/admin_login", rank = 2)]
pub fn dashboard_retry_flash(flash_msg: FlashMessage) -> Template {
    let start = Instant::now();
    
    let flash = Some( alert_danger(flash_msg.msg()) );
    let output = hbs_template(TemplateBody::Login(URL_LOGIN_ADMIN.to_string(), None, flash), Some("Administrator Login".to_string()), String::from("/admin_login"), None, None, None, Some(start));
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[allow(unused_mut)]
#[post("/admin_login", data = "<form>")]
// fn process_admin_login(form: Form<LoginCont<AdminLogin>>, mut cookies: Cookies) -> Result<Redirect, Flash<Redirect>> {
// fn process_admin_login(form: Form<LoginCont<AdministratorForm>>, mut cookies: Cookies) -> Result<Redirect, Flash<Redirect>> {
// fn process_admin_login(form: Form<AdministratorForm>, mut cookies: Cookies) -> Result<Redirect, Flash<Redirect>> {
pub fn process_admin_login(form: Form<LoginCont<AdministratorForm>>, mut cookies: Cookies) -> Result<Redirect, Flash<Redirect>> {
    let start = Instant::now();
    
    // let inner = form.into_inner();
    // let inner = &form;
    // let login = inner.form;
    let login: AdministratorForm = form.into_inner().form;
    let output = login.flash_redirect("/dashboard", "/admin_login", cookies);
    
    let end = start.elapsed();
    println!("Processed in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/admin_logout")]
pub fn logout(admin: Option<AdministratorCookie>, mut cookies: Cookies) -> Result<Flash<Redirect>, Redirect> {
    if let Some(_) = admin {
        cookies.remove_private(Cookie::named(AdministratorCookie::cookie_id()));
        // AdministratorCookie::delete_cookie(&mut cookies);
        Ok(Flash::success(Redirect::to("/"), "Successfully logged out."))
    } else {
        Err(Redirect::to("/admin_login"))
    }
}
