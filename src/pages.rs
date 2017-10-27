
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
use sanitize::*;

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




#[get("/admin")]
pub fn hbs_admin_page(conn: DbConn, admin: AdminCookie, user: Option<UserCookie>) -> Template {
    let start = Instant::now();
    let username = admin.username.clone();
    let output: Template = hbs_template(TemplateBody::General(format!("Welcome Administrator {user}.  You are viewing the administrator dashboard page.", user=username), None), Some("Administrator Dashboard".to_string()), String::from("/admin"), Some(admin), user, None, Some(start));
        
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/admin", rank = 2)]
pub fn hbs_admin_login(conn: DbConn, user: Option<UserCookie>) -> Template {
    let start = Instant::now();
    
    let output: Template = hbs_template(TemplateBody::Login(ADMIN_LOGIN_URL.to_string(), None, None), Some("Administrator Login".to_string()), String::from("/admin"), None, user, None, Some(start));
        
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/admin?<fail>")]
pub fn hbs_admin_retry(conn: DbConn, user: Option<UserCookie>, fail: AuthFailure) -> Template {
    let start = Instant::now();
    
    let clean_user = if fail.user != "" { Some(strict_sanitize(fail.user)) } else { None };
    let clean_msg = if fail.msg != "" { Some(alert_danger(&input_sanitize(fail.msg))) } else { None };
    let output: Template = hbs_template(TemplateBody::Login(ADMIN_LOGIN_URL.to_string(), clean_user, clean_msg), Some("Administrator Login".to_string()), String::from("/admin"), None, user, None, Some(start));
        
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
    
    let username = user.username.clone();
    let output: Template = hbs_template(TemplateBody::General(format!("Welcome {user}.  You are viewing your dashboard page.", user=username), None), Some("User Dashboard".to_string()), String::from("/user"), admin, Some(user), None, Some(start));
        
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/user", rank = 2)]
pub fn hbs_user_login(conn: DbConn, admin: Option<AdminCookie>) -> Template {
    let start = Instant::now();
    
    let output: Template = hbs_template(TemplateBody::Login(USER_LOGIN_URL.to_string(), None, None), Some("User Login".to_string()), String::from("/user"), admin, None, None, Some(start));
        
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/user?<fail>")]
pub fn hbs_user_retry(conn: DbConn, admin: Option<AdminCookie>, fail: AuthFailure) -> Template {
    let start = Instant::now();
    
    let clean_user = if fail.user != "" { Some(strict_sanitize(fail.user)) } else { None };
    let clean_msg = if fail.msg != "" { Some(alert_danger(&input_sanitize(fail.msg))) } else { None };
    let output: Template = hbs_template(TemplateBody::Login(USER_LOGIN_URL.to_string(), clean_user, clean_msg), Some("User Login".to_string()), String::from("/user"), admin, None, None, Some(start));
        
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[post("/user", data = "<form>")]
pub fn hbs_user_process(form: Form<LoginFormStatus<UserAuth>>, cookies: Cookies) -> LoginFormRedirect {
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
pub fn hbs_all_articles(conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
    let start = Instant::now();
    let output: Template;
    let results = Article::retrieve_all(conn, 0, Some(300), None, None, None, None);
    
    if results.len() != 0 {
        output = hbs_template(TemplateBody::Articles(results, None), Some("Viewing All Articles".to_string()), String::from("/"), admin, user, None, Some(start));
    } else {
        if admin.is_some() {
            output = hbs_template(TemplateBody::General("There are no articles<br>\n<a href =\"/insert\">Create Article</a>".to_string(), None), Some("Viewing All Articles".to_string()), String::from("/"), admin, user, None, Some(start));
        } else {
            output = hbs_template(TemplateBody::General("There are no articles.".to_string(), None), Some("Viewing All Articles".to_string()), String::from("/"), admin, user, None, Some(start));
        }
    }
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/view?<page>")]
pub fn hbs_articles_page(page: ViewPage, conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
    let start = Instant::now();
    let results = Article::retrieve_all(conn, 0, Some(300), None, None, None, None);
    
    // Todo: Change title to: Viewing Article Page x/z
    let output: Template = hbs_template(TemplateBody::General("You are viewing paginated articles.".to_string(), None), Some("Viewing Articles".to_string()), String::from("/"), admin, user, None, Some(start));
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}


#[get("/all_tags")]
pub fn hbs_tags_all(conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
    let start = Instant::now();
    
    // let qrystr = "SELECT COUNT(*) as cnt, unnest(tag) as untag FROM articles GROUP BY untag;";
    let qrystr = "SELECT COUNT(*) as cnt, unnest(tag) as untag FROM articles GROUP BY untag ORDER BY cnt DESC;";
    let qry = conn.query(qrystr, &[]);
    // let tags: Vec<TagCount> = if !qry.is_empty() && qry.len() != 0 {
    let mut tags: Vec<TagCount> = Vec::new();
    if let Ok(result) = qry {
        let mut sizes: Vec<u16> = Vec::new();
        for row in &result {
            let c: i64 = row.get(0);
            let c2: u32 = c as u32;
            sizes.push(c2 as u16);
            let t: String = row.get(1);
            let t2: String = t.trim_matches('\'').to_string();
            let tagcount = TagCount { 
                // tag: titlecase(t.trim_matches('\'')), 
                url: t2.clone(), 
                tag: titlecase(&t2), 
                count: c2,
                size: 0,
            };
            tags.push(tagcount);
        }
        if tags.len() > 4 {
            // let top = if tags.len() > 7 { 6 } else { 3 };
            // sizes.sort();
            // println!("Top: {:?}", top);
            // {
                // sizes.sort_by(|a, b| a.cmp(b).reverse());
            // }
            // println!("Sorting tag sizes: {:?}", top);
            if tags.len() > 7 {
                // for (i, mut v) in &mut tags[0..6].iter().enumerate() {
                let mut i = 0u16;
                for mut v in &mut tags[0..6] {
                    v.size = 6-i;
                    i += 1;
                }
                
            } else {
                // for mut i in &mut tags[0..3].iter().enumerate() {
                let mut i = 0u16;
                for mut v in &mut tags[0..3] {
                    v.size = (3-i)*2;
                }
            }
            tags.sort_by(|a, b| a.tag.cmp(&b.tag));
            
            // let topx: Vec<u32> = sizes.iter().position(|n| tags[..top].contains(tag.count)).expect("Iterator failed.").collect();
            // for t in &topx {
                
            // }
            // for mut tag in &mut tags {
            //     // let tpos = sizes.iter().position(|n| tags[..top].contains(tag.count));
                
            //     if sizes[..top].contains(&(tag.count as u16)) {
            //         let mut s = 0;
            //         // for (i, cnt) in sizes[(sizes.len()-top)..].iter().enumerate() {
            //         for (i, cnt) in sizes[..top].iter().enumerate() {
            //             if tag.count == i as u32 {
            //                 // double the size if there is fewer tags
            //                 tag.size = if top == 3 { ((top-i)*2) as u16 } else { (top-i) as u16 };
            //                 break;
            //             }
            //         }
            //     }
            // }
        }
        
    }
    
    // let output: Template = hbs_template(TemplateBody::General("The all tags page is not implemented yet.".to_string(), None), Some("Viewing All Tags".to_string()), String::from("/all_tags"), admin, user, None, Some(start));
    let output: Template = hbs_template(TemplateBody::Tags(tags, None), Some("Viewing All Tags".to_string()), String::from("/all_tags"), admin, user, None, Some(start));
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}


#[get("/tag?<tag>", rank = 2)]
pub fn hbs_articles_tag(tag: Tag, conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
    let start = Instant::now();
    
    let output: Template;
    let tags = Some(split_tags(medium_sanitize(tag.tag.clone())));
    // limit, # body chars, min date, max date, tags, strings
    let results = Article::retrieve_all(conn, 0, Some(-1), None, None, tags, None);
    if results.len() != 0 {
        output = hbs_template(TemplateBody::Articles(results, None), Some(format!("Viewing Articles with Tags: {}", tag.tag)), String::from("/all_tags"), admin, user, None, Some(start));
    } else {
        output = hbs_template(TemplateBody::General(alert_danger("Could not find any articles with the specified tag."), None), Some(format!("Could not find any articles with the tag(s): {}", medium_sanitize(tag.tag) )), String::from("/tag"), admin, user, None, Some(start));
    }
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/article?<aid>")]
pub fn hbs_article_view(aid: ArticleId, conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
    let start = Instant::now();
    let rst = aid.retrieve_with_conn(conn); // retrieve result
    let mut output: Template; 
    if let Some(article) = rst {
        let title = article.title.clone();
        output = hbs_template(TemplateBody::Article(article, None), Some(title), String::from("/article"), admin, user, None, Some(start));
    } else {
        output = hbs_template(TemplateBody::General(alert_danger(&format!("Article {} not found.", aid.aid)), None), Some("Article Not Found".to_string()), String::from("/article"), admin, user, None, Some(start));
    }
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/article")]
pub fn hbs_article_not_found(conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
    let start = Instant::now();
    let output: Template = hbs_template(TemplateBody::General(alert_danger("Article not found"), None), Some("Article not found".to_string()), String::from("/article"), admin, user, None, Some(start));
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[post("/create", data = "<form>")]
pub fn hbs_article_process(form: Form<ArticleForm>, conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
// pub fn hbs_post_article(admin: AdminCookie, form: Form<ArticleForm>, conn: DbConn) -> Html<String> {
    let start = Instant::now();
    
    let output: Template;
    let result = form.into_inner().save(&conn);
    match result {
        Ok(article) => {
            let title = article.title.clone();
            output = hbs_template(TemplateBody::Article(article, None), Some(title), String::from("/create"), admin, user, None, Some(start));
        },
        Err(why) => {
            output = hbs_template(TemplateBody::General(alert_danger(&format!("Could not post the submitted article.  Reason: {}", why)), None), Some("Could not post article".to_string()), String::from("/create"), admin, user, None, Some(start));
        },
    }
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}
#[post("/create", rank=2)]
pub fn hbs_create_unauthorized(conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
    let start = Instant::now();
    
    let output: Template = hbs_template(TemplateBody::General(alert_danger(UNAUTHORIZED_POST_MESSAGE), None), Some("Not Authorized".to_string()), String::from("/create"), admin, user, None, Some(start));
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/create")]
pub fn hbs_create_form(conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
    let start = Instant::now();
    
    let output: Template;
    if admin.is_some() {
        output = hbs_template(TemplateBody::Create(CREATE_FORM_URL.to_string(), None), Some("Create New Article".to_string()), String::from("/create"), admin, user, None, Some(start));
    } else {
        output = hbs_template(TemplateBody::General(alert_danger(UNAUTHORIZED_POST_MESSAGE), None), Some("Not Authorized".to_string()), String::from("/create"), admin, user, None, Some(start));
    }
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/logout")]
pub fn hbs_logout(admin: Option<AdminCookie>, user: Option<UserCookie>, mut cookies: Cookies) -> Result<Flash<Redirect>, Redirect> {
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
pub fn hbs_search_page(conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
    // unimplemented!()
    // don't forget to put the start Instant in the hbs_template() function
    hbs_template(TemplateBody::General("Search page not implemented yet".to_string(), None), Some("Search".to_string()), String::from("/search"), admin, user, None, None)
}

#[get("/search?<search>")]
pub fn hbs_search_results(search: Search, conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
    // unimplemented!()
    // don't forget to put the start Instant in the hbs_template() function
    hbs_template(TemplateBody::General("Search results page not implemented yet.".to_string(), None), Some("Search Results".to_string()), String::from("/search"), admin, user, None, None)
}

#[get("/about")]
pub fn hbs_about(conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>) -> Template {
    // don't forget to put the start Instant in the hbs_template() function
    hbs_template(TemplateBody::General("This page is not implemented yet.  Soon it will tell a little about me.".to_string(), None), Some("About Me".to_string()), String::from("/about"), admin, user, None, None)
}

#[get("/")]
pub fn hbs_index(conn: DbConn, admin: Option<AdminCookie>, user: Option<UserCookie>, flash: Option<FlashMessage>) -> Template {
    // let body = r#"Hello! This is a blog.<br><a href="/user">User page</a><br><a href="/admin">Go to admin page</a>"#;
    // template(body)
    let start = Instant::now();
    // let mut output: Html<String> = Html(String::new());
    let output: Template;
    let fmsg: Option<String>;
    if let Some(flashmsg) = flash {
        fmsg = Some(alert_info( flashmsg.msg() ));
    } else {
        fmsg = None;
    }
    let results = Article::retrieve_all(conn, 0, Some(300), None, None, None, None);
    if results.len() != 0 {
        output = hbs_template(TemplateBody::Articles(results, fmsg), None, String::from("/"), admin, user, None, Some(start));
    } else if admin.is_some() {
        output = hbs_template(TemplateBody::General("There are no articles.<br>\n<a href =\"/insert\">Create Article</a>".to_string(), None), None, String::from("/"), admin, user, None, Some(start));
    } else {
        output = hbs_template(TemplateBody::General("There are no articles.".to_string(), None), None, String::from("/"), admin, user, None, Some(start));
    }
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}


