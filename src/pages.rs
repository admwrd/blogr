
use std::time::Instant;
use std::{env, str, io};
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};

use rocket_contrib::Template;
use rocket::response::{content, NamedFile, Redirect, Flash};
use rocket::{Request, Data, Outcome};
use rocket::request::FlashMessage;
use rocket::data::FromData;
use rocket::response::content::Html;
use rocket::request::Form;
use rocket::http::{Cookie, Cookies};
use auth::userpass::UserPass;
use auth::status::{LoginStatus,LoginRedirect};
use auth::dummy::DummyAuthenticator;
use auth::authenticator::Authenticator;
use regex::Regex;
use titlecase::titlecase;

use super::{BLOG_URL, ADMIN_LOGIN_URL, USER_LOGIN_URL, CREATE_FORM_URL};
use layout::*;
use cookie_data::*;
use admin_auth::*;
use user_auth::*;
use users::*;
use login_form_status::*;
use login_form_status::LoginFormRedirect;
use blog::*;
use data::*;
use templates::*;

/*

pub fn hbs_(conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {


hbs_template(TemplateBody::General("".to_string()), Some("".to_string()), admin, user, None, Some(start));


something(conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
    let start = Instant::now();
    let output: Template;
    let results = Article::retrieve_all(conn, 0, Some(300), None, None, None, None);
    
    // Todo: Change title to: Viewing Article Page x/z
    output = hbs_template(TemplateBody::General("You are viewing paginated articles."), Some("Viewing Articles".to_string()), admin, user, None, Some(start));
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output

}



*/

// escapes html tags and special characters
pub fn input_sanitize(string: String) -> String {
    string
}
// removes non-word characters
pub fn strict_sanitize(string: String) -> String {
    // use lazy_static! to make a regexp to remove everything but word characters
    string
}
// leaves spaces, commas, hyphens, and underscores but removes all other non-word characters
pub fn medium_sanitize(string: String) -> String {
    
}



#[get("/admin")]
pub fn hbs_admin_page(conn: DbConn, admin: AdminCookie, user: Option<UserCookie>) -> Template {
    let start = Instant::now();
    
    let output: Template = hbs_template(TemplateBody::General(format!("Welcome Administrator {user}.  You are viewing the administrator dashboard page.")), Some("Administrator Dashboard".to_string()), Some(admin), user, None, Some(start));
        
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/admin", rank = 2)]
pub fn hbs_admin_login(conn: DbConn, user: Option<UserCookie>) -> Template {
    let start = Instant::now();
    
    let output: Template = hbs_template(TemplateBody::Login(ADMIN_LOGIN_URL.to_string(), None, None), Some("Administrator Login".to_string()), None, user, None, Some(start));
        
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/admin?<fail>")]
pub fn hbs_admin_retry(conn: DbConn, user: Option<UserCookie>, fail: AuthFailure) -> Template {
    let start = Instant::now();
    
    let output: Template = hbs_template(TemplateBody::Login(ADMIN_LOGIN_URL.to_string(), strict_sanitize(fail.user), input_sanitize(fail.msg)), Some("Administrator Login".to_string()), None, user, None, Some(start));
        
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[post("/admin", data = "<form>")]
pub fn hbs_process_admin(form: Form<LoginFormStatus<AdminAuth>>, cookies: Cookies) -> LoginFormRedirect {
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
    println!("Processed in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    
    inside.redirect("/admin", cookies).unwrap_or( LoginFormRedirect::new(Redirect::to(&failurl)) )
}




#[get("/user")]
pub fn hbs_user_page(conn: DbConn, admin: Option<AdminCookie>, user: UserCookie) -> Template {
    let start = Instant::now();
    
    let output: Template = hbs_template(TemplateBody::General(format!("Welcome {user}.  You are viewing your dashboard page.")), Some("User Dashboard".to_string()), admin, Some(user), None, Some(start));
        
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/user", rank = 2)]
pub fn hbs_user_login(conn: DbConn, admin: Option<AdminCookie>) -> Template {
    let start = Instant::now();
    
    let output: Template = hbs_template(TemplateBody::Login(USER_LOGIN_URL.to_string(), None, None), Some("User Login".to_string()), admin, None, None, Some(start));
        
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/user?<fail>")]
pub fn hbs_user_retry(conn: DbConn, admin: Option<AdminCookie>, fail: AuthFailure) -> Template {
    let start = Instant::now();
    
    let output: Template = hbs_template(TemplateBody::Login(USER_LOGIN_URL.to_string(), strict_sanitize(fail.user), input_sanitize(fail.msg)), Some("Administrator Login".to_string()), admin, None, None, Some(start));
        
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
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
fn all_articles(conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
    let start = Instant::now();
    let output: Template;
    let results = Article::retrieve_all(conn, 0, Some(300), None, None, None, None);
    
    if results.len() != 0 {
        output = hbs_template(TemplateBody::Articles(results), Some("Viewing All Articles".to_string()), admin, user, None, Some(start));
    } else {
        if admin.is_some() {
            output = hbs_template(TemplateBody::General("There are no articles<br>\n<a href =\"/insert\">Create Article</a>".to_string()), Some("Viewing All Articles".to_string()), admin, user, None, Some(start));
        } else {
            output = hbs_template(TemplateBody::General("There are no articles.".to_string()), Some("Viewing All Articles".to_string()), admin, user, None, Some(start));
        }
    }
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/view?<page>")]
fn articles_page(page: ViewPage, conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
    let start = Instant::now();
    let results = Article::retrieve_all(conn, 0, Some(300), None, None, None, None);
    
    // Todo: Change title to: Viewing Article Page x/z
    let output: Template = hbs_template(TemplateBody::General("You are viewing paginated articles."), Some("Viewing Articles".to_string()), admin, user, None, Some(start));
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}


#[get("/all_tags")]
fn tags_all(conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
    let start = Instant::now();
    
    let output: Template = hbs_template(TemplateBody::General("The all tags page is not implemented yet.".to_string()), Some("Viewing All Tags".to_string()), admin, user, None, Some(start));
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}


#[get("/tag?<tag>", rank = 2)]
fn articles_tag(tag: Tag, conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
    let start = Instant::now();
    
    let output: Template;
    let tags = Some(split_tags(medium_sanitize(tag.tag)));
    // limit, # body chars, min date, max date, tags, strings
    let results = Article::retrieve_all(conn, 0, Some(-1), None, None, tags, None);
    if results.len() != 0 {
        hbs_template(TemplateBody::Articles(results), Some(format!("Viewing Articles with Tags: {}", tag.tag)), admin, user, None, Some(start));
    } else {
        output = template( &alert_danger("Could not find any articles with the specified tag.") );
    }
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/article?<aid>")]
fn article_view(aid: ArticleId, conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
    let start = Instant::now();
    let rst = aid.retrieve_with_conn(conn); // retrieve result
    let mut output: Template; 
    if let Some(article) = rst {
        let title = article.title.clone();
        output = hbs_template(TemplateBody::Article(article), Some(title), admin, user, None, Some(start));
    } else {
        output = hbs_template(TemplateBody::General(alert_danger(&format!("Article {} not found.", aid.aid))), Some("Article Not Found".to_string()), admin, user, None, Some(start));
    }
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/article")]
fn article_not_found() -> Template {
    let start = Instant::now();
    let output: Template = hbs_template(TemplateBody::General(alert_danger("Article not found")), Some("Article not found".to_string()), admin, user, None, Some(start));
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[post("/article", data = "<form>")]
fn article_process(conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
// fn post_article(admin: AdminCookie, form: Form<ArticleForm>, conn: DbConn) -> Html<String> {
    let start = Instant::now();
    
    let output: Template;
    let result = form.into_inner().save(&conn);
    match result {
        Ok(article) => {
            output = full_template_article(&article, true, true, Some(admin.username));
            let title = article.title.clone();
            output = hbs_template(TemplateBody::Article(article), Some(article), admin, user, None, Some(start));
        },
        Err(why) => {
            output = hbs_template(TemplateBody::General(alert_danger(&format!("Could not post the submitted article.  Reason: {}", why))), Some("Could not post article".to_string()), admin, user, None, Some(start));
        },
    }
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}
#[post("/article", rank=2)]
fn create_unauthorized(conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
    let start = Instant::now();
    
    let output: Template = hbs_template(TemplateBody::General(alert_danger(UNAUTHORIZED_POST_MESSAGE)), Some("Not Authorized".to_string()), admin, user, None, Some(start));
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/insert")]
fn create_form(conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
    let start = Instant::now();
    
    let output: Template;
    if let Some(admin) = user {
        output = hbs_template(TemplateBody::Create(CREATE_FORM_URL.to_string()), Some("Create New Article".to_string()), admin, user, None, Some(start));
    } else {
        output = hbs_template(TemplateBody::General(alert_danger(UNAUTHORIZED_POST_MESSAGE)), Some("Not Authorized".to_string()), admin, user, None, Some(start));
    }
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/logout")]
fn logout(admin: Option<AdminCookie>, user: Option<UserCookie>, mut cookies: Cookies) -> Result<Flash<Redirect>, Redirect> {
    if admin.is_some() || user.is_some() {
        if let Some(a) = admin {
            cookies.remove_private(Cookie::named(AdminCookie::get_cid()));
            // cookies.remove_private(Cookie::named("user_id"));
        }
        if let Some(u) = user {
            cookies.remove_private(Cookie::named(UserCookie::get_cid()));
        }
        Ok(Flash::success(Redirect::to("/"), "Successfully logged out."))
    } else {
        Err(Redirect::to("/admin"))
    }
}

#[get("/search")]
fn search_page(conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
    // unimplemented!()
    // don't forget to put the start Instant in the hbs_template() function
    hbs_template(TemplateBody::General("Search page not implemented yet".to_string()), Some("Search".to_string()), admin, user, None, None)
}

#[get("/search?<search>")]
fn search_results(search: Search, conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
    // unimplemented!()
    // don't forget to put the start Instant in the hbs_template() function
    hbs_template(TemplateBody::General("Search results page not implemented yet.".to_string()), Some("Search Results".to_string()), admin, user, None, None)
}

#[get("/about")]
fn about(conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
    hbs_template(TemplateBody::General("This page is not implemented yet.  Soon it will tell a little about me.".to_string()), Some("About Me".to_string()), admin, user, None, Some(start))
}

#[get("/")]
fn index(conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>, flash: Option<FlashMessage>) -> Html<String> {
    // let body = r#"Hello! This is a blog.<br><a href="/user">User page</a><br><a href="/admin">Go to admin page</a>"#;
    // template(body)
    let start = Instant::now();
    let mut output: Html<String> = Html(String::new());
    if let Some(flashmsg) = flash {
        // let output: Html<String>;
        let results = Article::retrieve_all(conn, 0, Some(300), None, None, None, None);
        if results.len() != 0 {
            let is_admin = if admin.is_some() { true } else { false };
            let is_user = if user.is_some() { true } else { false };
            let username: Option<String> = if let Some(a_user) = admin { Some(a_user.username) } else if let Some(u_user) = user { Some(u_user.username) } else { None };
            
            output = template_articles_msg(&alert_success("You have been logged out."), false, results, is_admin, is_user, username);
        } else {
            if admin.is_some() {
                output = template( &alert_danger("There are no articles<br>\n<a href =\"/insert\">Create Article</a>") );
            } else {
                output = template( &alert_danger("There are no articles.") );
            }
        }
    } else {
        output = all_articles(conn, admin, user);
    }
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}


