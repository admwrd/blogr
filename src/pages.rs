
use std::time::Instant;
use std::{env, str, io};
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};

use rocket_contrib::Template;
use rocket::response::{content, NamedFile, Redirect, Flash};
use rocket::{Request, Data, Outcome};
use rocket::request::{FlashMessage, Form, FromForm};
use rocket::data::FromData;
use rocket::response::content::Html;
// use rocket::request::{Form, FlashMessage};
use rocket::http::{Cookie, Cookies, RawStr};
// use auth::userpass::UserPass;
// use auth::status::{LoginStatus,LoginRedirect};
// use auth::dummy::DummyAuthenticator;
// use auth::authenticator::Authenticator;
use regex::Regex;
use titlecase::titlecase;

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
use super::*;
use layout::*;
use blog::*;
use data::*;
use sanitize::*;
use rocket_auth_login::authorization::*;
use rocket_auth_login::sanitization::*;
use ral_administrator::*;
use ral_user::*;
use hbs_templates::*;
use xpress::*;
use accept::*;
// use ::templates::*;





// #[get("/admin")]
// pub fn hbs_admin_page(conn: DbConn, admin: AdministratorCookie, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
//     let start = Instant::now();
//     let username = admin.username.clone();
//     let output: Template = hbs_template(TemplateBody::General(format!("Welcome Administrator {user}.  You are viewing the administrator dashboard page.", user=username), None), Some("Administrator Dashboard".to_string()), String::from("/admin"), Some(admin), user, None, Some(start.0));
        
//     let end = start.0.elapsed();
//     println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
//     output
// }

// #[get("/admin", rank = 2)]
// pub fn hbs_admin_login(conn: DbConn, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
//     let start = Instant::now();
    
//     let output: Template = hbs_template(TemplateBody::Login(ADMIN_LOGIN_URL.to_string(), None, None), Some("Administrator Login".to_string()), String::from("/admin"), None, user, None, Some(start.0));
        
//     let end = start.0.elapsed();
//     println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
//     output
// }

// #[get("/admin?<fail>")]
// pub fn hbs_admin_retry(conn: DbConn, user: Option<UserCookie>, fail: AuthFailure, encoding: AcceptCompression) -> Express {
//     let start = Instant::now();
    
//     let clean_user = if fail.user != "" { Some(strict_sanitize(fail.user)) } else { None };
//     let clean_msg = if fail.msg != "" { Some(alert_danger(&input_sanitize(fail.msg))) } else { None };
//     let output: Template = hbs_template(TemplateBody::Login(ADMIN_LOGIN_URL.to_string(), clean_user, clean_msg), Some("Administrator Login".to_string()), String::from("/admin"), None, user, None, Some(start.0));
        
//     let end = start.0.elapsed();
//     println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
//     output
// }

// #[post("/admin", data = "<form>")]
// pub fn hbs_process_admin(form: Form<LoginFormStatus<AdminAuth>>, cookies: Cookies) -> LoginFormRedirect {
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
    
//     let end = start.0.elapsed();
//     println!("Processed in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    
//     inside.redirect("/admin", cookies).unwrap_or( LoginFormRedirect::new(Redirect::to(&failurl)) )
// }




// #[get("/user")]
// pub fn hbs_user_page(conn: DbConn, admin: Option<AdministratorCookie>, user: UserCookie, encoding: AcceptCompression) -> Express {
//     let start = Instant::now();
    
//     let username = user.username.clone();
//     let output: Template = hbs_template(TemplateBody::General(format!("Welcome {user}.  You are viewing your dashboard page.", user=username), None), Some("User Dashboard".to_string()), String::from("/user"), admin, Some(user), None, Some(start.0));
        
//     let end = start.0.elapsed();
//     println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
//     output
// }

// #[get("/user", rank = 2)]
// pub fn hbs_user_login(conn: DbConn, admin: Option<AdministratorCookie>, encoding: AcceptCompression) -> Express {
//     let start = Instant::now();
    
//     let output: Template = hbs_template(TemplateBody::Login(USER_LOGIN_URL.to_string(), None, None), Some("User Login".to_string()), String::from("/user"), admin, None, None, Some(start.0));
        
//     let end = start.0.elapsed();
//     println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
//     output
// }

// #[get("/user?<fail>")]
// pub fn hbs_user_retry(conn: DbConn, admin: Option<AdministratorCookie>, fail: AuthFailure, encoding: AcceptCompression) -> Express {
//     let start = Instant::now();
    
//     let clean_user = if fail.user != "" { Some(strict_sanitize(fail.user)) } else { None };
//     let clean_msg = if fail.msg != "" { Some(alert_danger(&input_sanitize(fail.msg))) } else { None };
//     let output: Template = hbs_template(TemplateBody::Login(USER_LOGIN_URL.to_string(), clean_user, clean_msg), Some("User Login".to_string()), String::from("/user"), admin, None, None, Some(start.0));
        
//     let end = start.0.elapsed();
//     println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
//     output
// }

// #[post("/user", data = "<form>")]
// pub fn hbs_user_process(form: Form<LoginFormStatus<UserAuth>>, cookies: Cookies) -> LoginFormRedirect {
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
    
//     let end = start.0.elapsed();
//     println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    
//     inside.redirect("/user", cookies).unwrap_or( LoginFormRedirect::new(Redirect::to(&failurl)) )
// }






// output\.into\(\)\.compress\(encoding\)
// let express: Express = output.into();
//   express.compress(encoding)



#[get("/admin", rank = 1)]
pub fn hbs_dashboard_admin_authorized(start: GenTimer, conn: DbConn, user: Option<UserCookie>, admin: AdministratorCookie, encoding: AcceptCompression) -> Express {
    // let start = Instant::now();
    
    let output: Template = hbs_template(TemplateBody::General(format!("Welcome Administrator {user}.  You are viewing the administrator dashboard page.", user=admin.username), None), Some("Dashboard".to_string()), String::from("/admin"), Some(admin), user, None, Some(start.0));
    
    let end = start.0.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}

#[get("/admin", rank = 2)]
pub fn hbs_dashboard_admin_flash(start: GenTimer, conn: DbConn, user: Option<UserCookie>, flash_msg_opt: Option<FlashMessage>, encoding: AcceptCompression) -> Express {
    // let start = Instant::now();
    let output: Template;
    
    if let Some(flash_msg) = flash_msg_opt {
        let flash = Some( alert_danger(flash_msg.msg()) );
        output = hbs_template(TemplateBody::Login(URL_LOGIN_ADMIN.to_string(), None, flash), Some("Administrator Login".to_string()), String::from("/admin"), None, user, None, Some(start.0));
    } else {
        output = hbs_template(TemplateBody::Login(URL_LOGIN_ADMIN.to_string(), None, None), Some("Administrator Login".to_string()), String::from("/admin"), None, user, None, Some(start.0));
    }
    
    let end = start.0.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}

// No longer needed.  Was getting errors because the dashboard_admin_retry_user() route
// named the qrystr parameter user which already has a variable binding, renamed and fixed it
// 
// #[get("/admin/<userqry>")]
// pub fn dashboard_admin_retry_route(conn: DbConn, user: Option<UserCookie>, mut userqry: String, flash_msg_opt: Option<FlashMessage>, encoding: AcceptCompression) -> Express {
//     unimplemented!()
// }


#[get("/admin?<userqry>")]
pub fn hbs_dashboard_admin_retry_user(start: GenTimer, conn: DbConn, user: Option<UserCookie>, mut userqry: QueryUser, flash_msg_opt: Option<FlashMessage>, encoding: AcceptCompression) -> Express {
    // let start = Instant::now();
    // let userqry: QueryUser = userqry_form.get();
    
    // user = login::sanitization::sanitize(&user);
    let username = if &userqry.user != "" { Some(userqry.user.clone() ) } else { None };
    let flash = if let Some(f) = flash_msg_opt { Some(alert_danger(f.msg())) } else { None };
    let output = hbs_template(TemplateBody::Login(URL_LOGIN_ADMIN.to_string(), username, flash), Some("Administrator Login".to_string()), String::from("/admin"), None, user, None, Some(start.0));
    
    let end = start.0.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}

#[allow(unused_mut)]
#[post("/admin", data = "<form>")]
pub fn hbs_process_admin_login(start: GenTimer, form: Form<LoginCont<AdministratorForm>>, mut cookies: Cookies) -> Result<Redirect, Flash<Redirect>> {
    // let start = Instant::now();
    
    let login: AdministratorForm = form.get().form();
    // let login: AdministratorForm = form.into_inner().form;
    let output = login.flash_redirect("/admin", "/admin", cookies);
    
    let end = start.0.elapsed();
    println!("Processed in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/admin_logout")]
pub fn hbs_logout_admin(admin: Option<AdministratorCookie>, mut cookies: Cookies) -> Result<Flash<Redirect>, Redirect> {
    if let Some(_) = admin {
        // cookies.remove_private(Cookie::named(AdministratorCookie::cookie_id()));
        AdministratorCookie::delete_cookie(cookies);
        Ok(Flash::success(Redirect::to("/"), "Successfully logged out."))
    } else {
        Err(Redirect::to("/admin"))
    }
}











#[get("/user", rank = 1)]
pub fn hbs_dashboard_user_authorized(start: GenTimer, conn: DbConn, admin: Option<AdministratorCookie>, user: UserCookie, encoding: AcceptCompression) -> Express {
    // let start = Instant::now();
    
    let output: Template = hbs_template(TemplateBody::General(format!("Welcome User {user}.  You are viewing the User dashboard page.", user=user.username), None), Some("User Dashboard".to_string()), String::from("/user"), admin, Some(user), None, Some(start.0));
    
    let end = start.0.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}

#[get("/user", rank = 2)]
pub fn hbs_dashboard_user_flash(start: GenTimer, conn: DbConn, admin: Option<AdministratorCookie>, flash_msg_opt: Option<FlashMessage>, encoding: AcceptCompression) -> Express {
    // let start = Instant::now();
    let output: Template;
    
    if let Some(flash_msg) = flash_msg_opt {
        let flash = Some( alert_danger(flash_msg.msg()) );
        output = hbs_template(TemplateBody::Login(URL_LOGIN_USER.to_string(), None, flash), Some("User Login".to_string()), String::from("/user"), admin, None, None, Some(start.0));
    } else {
        output = hbs_template(TemplateBody::Login(URL_LOGIN_USER.to_string(), None, None), Some("User Login".to_string()), String::from("/user"), admin, None, None, Some(start.0));
    }
    
    let end = start.0.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
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
    let output = hbs_template(TemplateBody::Login(URL_LOGIN_USER.to_string(), username, flash), Some("User Login".to_string()), String::from("/user"), admin, None, None, Some(start.0));
    
    let end = start.0.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}

#[allow(unused_mut)]
#[post("/user", data = "<form>")]
pub fn hbs_process_user_login(start: GenTimer, form: Form<LoginCont<UserForm>>, mut cookies: Cookies) -> Result<Redirect, Flash<Redirect>> {
    // let start = Instant::now();
    
    let login: UserForm = form.get().form();
    // let login: AdministratorForm = form.into_inner().form;
    let output = login.flash_redirect("/user", "/user", cookies);
    
    let end = start.0.elapsed();
    println!("Processed in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/user_logout")]
pub fn hbs_logout_user(admin: Option<UserCookie>, mut cookies: Cookies) -> Result<Flash<Redirect>, Redirect> {
    if let Some(_) = admin {
        // cookies.remove_private(Cookie::named(UserCookie::cookie_id()));
        UserCookie::delete_cookie(cookies);
        Ok(Flash::success(Redirect::to("/"), "Successfully logged out."))
    } else {
        Err(Redirect::to("/user"))
    }
}










#[get("/view")]
pub fn hbs_all_articles(start: GenTimer, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
    // let start = Instant::now();
    let output: Template;
    let results = Article::retrieve_all(conn, 0, Some(300), None, None, None, None);
    
    if results.len() != 0 {
        output = hbs_template(TemplateBody::Articles(results, None), Some("Viewing All Articles".to_string()), String::from("/"), admin, user, None, Some(start.0));
    } else {
        if admin.is_some() {
            output = hbs_template(TemplateBody::General("There are no articles<br>\n<a href =\"/insert\">Create Article</a>".to_string(), None), Some("Viewing All Articles".to_string()), String::from("/"), admin, user, None, Some(start.0));
        } else {
            output = hbs_template(TemplateBody::General("There are no articles.".to_string(), None), Some("Viewing All Articles".to_string()), String::from("/"), admin, user, None, Some(start.0));
        }
    }
    
    let end = start.0.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}

#[get("/view?<page>")]
pub fn hbs_articles_page(start: GenTimer, page: ViewPage, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
    // let start = Instant::now();
    let results = Article::retrieve_all(conn, 0, Some(300), None, None, None, None);
    
    // Todo: Change title to: Viewing Article Page x/z
    let output: Template = hbs_template(TemplateBody::General("You are viewing paginated articles.".to_string(), None), Some("Viewing Articles".to_string()), String::from("/"), admin, user, None, Some(start.0));
    
    let end = start.0.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}


#[get("/all_tags")]
pub fn hbs_tags_all(start: GenTimer, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
    // let start = Instant::now();
    
    let qrystr = "SELECT COUNT(*) as cnt, unnest(tag) as untag FROM articles GROUP BY untag ORDER BY cnt DESC;";
    let qry = conn.query(qrystr, &[]);
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
            if tags.len() > 7 {
                let mut i = 0u16;
                for mut v in &mut tags[0..6] {
                    v.size = 6-i;
                    i += 1;
                }
                
            } else {
                let mut i = 0u16;
                for mut v in &mut tags[0..3] {
                    v.size = (3-i)*2;
                }
            }
            tags.sort_by(|a, b| a.tag.cmp(&b.tag));
        }
        
    }
    
    let output: Template = hbs_template(TemplateBody::Tags(tags, None), Some("Viewing All Tags".to_string()), String::from("/all_tags"), admin, user, None, Some(start.0));
    
    let end = start.0.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}


#[get("/tag?<tag>", rank = 2)]
pub fn hbs_articles_tag(start: GenTimer, tag: Tag, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
    // let start = Instant::now();
    
    let output: Template;
    let tags = Some(split_tags(medium_sanitize(tag.tag.clone())));
    // limit, # body chars, min date, max date, tags, strings
    let results = Article::retrieve_all(conn, 0, Some(-1), None, None, tags, None);
    if results.len() != 0 {
        output = hbs_template(TemplateBody::Articles(results, None), Some(format!("Viewing Articles with Tags: {}", tag.tag)), String::from("/all_tags"), admin, user, None, Some(start.0));
    } else {
        output = hbs_template(TemplateBody::General(alert_danger("Could not find any articles with the specified tag."), None), Some(format!("Could not find any articles with the tag(s): {}", medium_sanitize(tag.tag) )), String::from("/tag"), admin, user, None, Some(start.0));
    }
    
    let end = start.0.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}

#[get("/article/<aid>/<title>")]

pub fn hbs_article_title(start: GenTimer, aid: ArticleId, title: Option<&RawStr>, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
    hbs_article_view(start, aid, conn, admin, user, encoding)
}

#[get("/article/<aid>")]
pub fn hbs_article_id(start: GenTimer, aid: ArticleId, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
    hbs_article_view(start, aid, conn, admin, user, encoding)
}

#[get("/article?<aid>")]
pub fn hbs_article_view(start: GenTimer, aid: ArticleId, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
    // let start = Instant::now();
    let rst = aid.retrieve_with_conn(conn); // retrieve result
    let mut output: Template; 
    if let Some(article) = rst {
        let title = article.title.clone();
        output = hbs_template(TemplateBody::Article(article, None), Some(title), String::from("/article"), admin, user, None, Some(start.0));
    } else {
        output = hbs_template(TemplateBody::General(alert_danger(&format!("Article {} not found.", aid.aid)), None), Some("Article Not Found".to_string()), String::from("/article"), admin, user, None, Some(start.0));
    }
    let end = start.0.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}

#[get("/article")]
pub fn hbs_article_not_found(start: GenTimer, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
    // let start = Instant::now();
    let output: Template = hbs_template(TemplateBody::General(alert_danger("Article not found"), None), Some("Article not found".to_string()), String::from("/article"), admin, user, None, Some(start.0));
    let end = start.0.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}

#[post("/create", data = "<form>")]
pub fn hbs_article_process(start: GenTimer, form: Form<ArticleForm>, conn: DbConn, admin: AdministratorCookie, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
    // let start = Instant::now();
    
    let output: Template;
    
    let username = if let Some(ref display) = admin.display { display.clone() } else { titlecase(&admin.username) };
    
    let result = form.into_inner().save(&conn, admin.userid, &username);
    match result {
        Ok(article) => {
            let title = article.title.clone();
            output = hbs_template(TemplateBody::Article(article, None), Some(title), String::from("/create"), Some(admin), user, None, Some(start.0));
        },
        Err(why) => {
            output = hbs_template(TemplateBody::General(alert_danger(&format!("Could not post the submitted article.  Reason: {}", why)), None), Some("Could not post article".to_string()), String::from("/create"), Some(admin), user, None, Some(start.0));
        },
    }
    
    let end = start.0.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}
#[post("/create", rank=2)]
pub fn hbs_create_unauthorized(start: GenTimer, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
    // let start = Instant::now();
    
    let output: Template = hbs_template(TemplateBody::General(alert_danger(UNAUTHORIZED_POST_MESSAGE), None), Some("Not Authorized".to_string()), String::from("/create"), admin, user, None, Some(start.0));
    
    let end = start.0.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}

#[get("/create")]
pub fn hbs_create_form(start: GenTimer, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
    // let start = Instant::now();
    
    let output: Template;
    if admin.is_some() {
        output = hbs_template(TemplateBody::Create(CREATE_FORM_URL.to_string(), None), Some("Create New Article".to_string()), String::from("/create"), admin, user, None, Some(start.0));
    } else {
        output = hbs_template(TemplateBody::General(alert_danger(UNAUTHORIZED_POST_MESSAGE), None), Some("Not Authorized".to_string()), String::from("/create"), admin, user, None, Some(start.0));
    }
    
    let end = start.0.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
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

#[get("/search")]
pub fn hbs_search_page(start: GenTimer, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
    // unimplemented!()
    
    //show an advanced search form
     
    
    // don't forget to put the start Instant in the hbs_template() function
    let output: Template = hbs_template(TemplateBody::General("Search page not implemented yet".to_string(), None), Some("Search".to_string()), String::from("/search"), admin, user, None, Some(start.0));
    let express: Express = output.into();
    express.compress(encoding)
}

#[get("/search?<search>")]
pub fn hbs_search_results(start: GenTimer, search: Search, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
    // let start = Instant::now();
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
    
    println!("Search parameters:\n{:?}", search);
    
    let mut qrystr = String::with_capacity(750);
    qrystr.push_str(r#"
SELECT a.aid, a.title, a.posted, a.tag, ts_rank(a.fulltxt, fqry, 32) AS rank, ts_headline('pg_catalog.english', a.body, fqry, 'StartSel = "<mark>", StopSel = "</mark>"') AS body,
    u.userid, u.display, u.username
FROM articles a JOIN users u ON (a.author = u.userid),
plainto_tsquery('pg_catalog.english', '"#);
    
    // ts_headline([ config regconfig, ] document text, query tsquery [, options text ]) returns text
    // qrystr.push_str(r#"SELECT ts_headline('english', body) FROM articles"#);
    
    let mut wherestr = String::new();
    let original = search.clone();
    
    let mut tags: Option<String> = None;
    if let Some(mut q) = search.q {
        if &q != "" {
            // prevent most attacks on length and complexity of the sql query
            if q.len() > 200 {
                q = q[..200].to_string();
            }
            // WHERE to_tsvector('english', body) @@ to_tsquery('english', 'friend');
            let sanitized = &sanitize_sql(q);
            qrystr.push_str(sanitized);
            // do a full-text search on title, description, and body fields
            // for each word add: 'word' = ANY(tag)
            let ts = sanitized.split(" ").map(|s| format!("'{}' = ANY(a.tag)", s)).collect::<Vec<_>>().join(" OR ");
            tags = if &ts != "" { Some(ts) } else { None };
            // wherestr.push_str(&tags);
        }
    }
    qrystr.push_str("') fqry WHERE fqry @@ a.fulltxt");
    if let Some(t) = tags {
        qrystr.push_str(" OR ");
        qrystr.push_str(&t);
    }
    if let Some(min) = search.min {
        // after min
        qrystr.push_str(" AND a.posted > '");
        qrystr.push_str(&format!("{}", min.0.format("%Y-%m-%d %H:%M:%S")));
        qrystr.push_str("'");
    }
    if let Some(max) = search.max {
        // before max
        qrystr.push_str(" AND a.posted < '");
        qrystr.push_str(&format!("{}", max.0.format("%Y-%m-%d %H:%M:%S")));
        qrystr.push_str("'");
    }
    qrystr.push_str(" ORDER BY rank DESC");
    if let Some(limit) = search.limit {
        // if str_is_numeric(limit) {
        if limit <= 50 {
        qrystr.push_str(&format!(" LIMIT {}", limit));
        } else {
            qrystr.push_str(" LIMIT 50");
        }
    } else {
        qrystr.push_str(" LIMIT 40");
    }
    println!("Generated the following SQL Query:\n{}", qrystr);
    
    let mut articles: Vec<Article> = Vec::new();
    let qry = conn.query(&qrystr, &[]);
    let output: Template;
    if let Ok(result) = qry {
        for row in &result {
            
            let display: Option<String> = row.get(7);
            let username: String = if let Some(disp) = display { disp } else { row.get(8) };
            
            let a = Article {
                aid: row.get(0),
                title: row.get(1),
                posted: row.get(2),
                tags: row.get_opt(3).unwrap_or(Ok(Vec::<String>::new())).unwrap_or(Vec::<String>::new()),
                body: row.get(5),
                description: String::new(),
                userid: row.get(6),
                username: titlecase( &username ),
            };
            articles.push(a);
        }
        output = hbs_template(TemplateBody::Search(articles, Some(original), None), Some("Search Results".to_string()), String::from("/search"), admin, user, None, Some(start.0));
    } else {
        println!("Query failed. Query: {}", qrystr);
        output = hbs_template(TemplateBody::General(alert_danger("No results were found."), None), Some("Search Results".to_string()), String::from("/search"), admin, user, None, Some(start.0));
    }
    let end = start.0.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}

// application/rss+xml
#[get("/rss.xml")]
// EXPRESS X - pub fn rss_page(conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>) -> String {
pub fn rss_page(start: GenTimer, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
    use rss::{Channel, ChannelBuilder, Guid, GuidBuilder, Item, ItemBuilder, Category, CategoryBuilder, TextInput, TextInputBuilder, extension};
    use chrono::{DateTime, TimeZone, NaiveDateTime, Utc};
    use urlencoding::encode;
    
    let rss: Express;
    
    let result = conn.articles("");
    if let Some(articles) = result {
        let mut article_items: Vec<Item> = Vec::new();
        for article in &articles {
            let mut link = String::with_capacity(BLOG_URL.len()+20);
            link.push_str(BLOG_URL);
            // link.push_str("article?aid=");
            link.push_str("article/");
            link.push_str(&article.aid.to_string());
            link.push_str("/");
            // let encoded = encode("This string will be URL encoded.");
            link.push_str( &encode(&article.title) );
            
            let desc: &str = if &article.description != "" {
                &article.description
            } else {
                if article.body.len() > DESC_LIMIT {
                    &article.body[..200]
                } else {
                    &article.body[..]
                }
            };
            
            let guid = GuidBuilder::default()
                .value(link.clone())
                .build()
                .expect("Could not create article guid.");
            
            let date_posted = DateTime::<Utc>::from_utc(article.posted, Utc).to_rfc2822();
            
            let item =ItemBuilder::default()
                .title(article.title.clone())
                .link(link)
                .description(desc.to_string())
                // .author("Andrew Prindle".to_string())
                .author(article.username.clone())
                // .categories()
                .guid(guid)
                .pub_date(date_posted)
                .build();
                
            match item {
                Ok(i) => article_items.push(i),
                Err(e) => println!("Could not create rss article {}.  Error: {}", article.aid, e),
            }
        }
        // Items:
        // title    link    description author  categories  guid    pub_date
        // Channels:
        // title    link    description categories  language    copyright   rating  ttl
        let mut search_link = String::with_capacity(BLOG_URL.len()+10);
        search_link.push_str(BLOG_URL);
        search_link.push_str("search");
        
        let searchbox = TextInputBuilder::default()
            .title("Search")
            .name("q")
            .description("Search articles")
            .link(search_link)
            .build()
            .expect("Could not create text input item in RSS channel.");
        
        let channel = ChannelBuilder::default()
            .title("Vishus Blog")
            .link(BLOG_URL)
            .description("A programming and development blog about Rust, Javascript, and Web Development.")
            .language("en-us".to_string())
            .copyright("2017 Andrew Prindle".to_string())
            .ttl(720.to_string()) // half a day, 1440 minutes in a day
            .items(article_items)
            .text_input(searchbox)
            .build()
            .expect("Could not create RSS channel.");
        
        let rss_output = channel.to_string();
        let mut output = String::with_capacity(rss_output.len() + 30);
        output.push_str(r#"<?xml version="1.0"?>"#);
        output.push_str(&rss_output);
        
        let express: Express = output.into();
        rss = express.set_content_type(ContentType::XML).compress(encoding);
            // set_ttl(-1)
            // .add_extra("Content-Type".to_string(), "application/rss+xml".to_string())
    } else {
        let output = String::from("Could not create RSS feed.");
        let express: Express = output.into();
        // Do not need to compress output with such a small string.
        // express.compress(encoding).set_content_type(ContentType::XML
        rss = express;
    }
    
    let end = start.0.elapsed();
    println!("RSS Generated in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    rss
}

// #[get("/author/<authorid>/<authorname>")]
// pub fn hbs_author_display(start: GenTimer, authorid: u32, authorname: Option<String>, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
//     unimplemented!()
    
// }

#[get("/author/<authorid>/<authorname>")]
pub fn hbs_author_display(start: GenTimer, authorid: u32, authorname: Option<&RawStr>, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
    hbs_author(start, authorid, conn, admin, user, encoding)
}

#[get("/author/<authorid>")]
pub fn hbs_author(start: GenTimer, authorid: u32, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
    // unimplemented!()
    let output: Template;
    let results = conn.articles( &format!("SELECT a.aid, a.title, a.posted, a.body, a.tag, a.description, u.userid, u.display, u.username FROM articles a JOIN users u ON (a.author = u.userid) WHERE userid = {}", authorid) );
    
    if let Some(articles) = results {
        output = hbs_template(TemplateBody::Articles(articles, None), Some("Articles by Author".to_string()), String::from("/author"), admin, user, None, Some(start.0));
    } else {
        output = hbs_template(TemplateBody::General("There are no articles by the specified user.".to_string(), None), Some("Articles by Author".to_string()), String::from("/author"), admin, user, None, Some(start.0));
    }
    
    let end = start.0.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}

#[get("/about")]
pub fn hbs_about(start: GenTimer, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
    // don't forget to put the start Instant in the hbs_template() function
    let output = hbs_template(TemplateBody::General("This page is not implemented yet.  Soon it will tell a little about me.".to_string(), None), Some("About Me".to_string()), String::from("/about"), admin, user, None, Some(start.0));
    let express: Express = output.into();
    express.compress(encoding)
}

#[get("/")]
pub fn hbs_index(start: GenTimer, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, flash: Option<FlashMessage>, encoding: AcceptCompression) -> Express {
    // let body = r#"Hello! This is a blog.<br><a href="/user">User page</a><br><a href="/admin">Go to admin page</a>"#;
    // template(body)
    // let start = Instant::now();
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
        output = hbs_template(TemplateBody::Articles(results, fmsg), None, String::from("/"), admin, user, None, Some(start.0));
    } else if admin.is_some() {
        output = hbs_template(TemplateBody::General("There are no articles.<br>\n<a href =\"/insert\">Create Article</a>".to_string(), None), None, String::from("/"), admin, user, None, Some(start.0));
    } else {
        output = hbs_template(TemplateBody::General("There are no articles.".to_string(), None), None, String::from("/"), admin, user, None, Some(start.0));
    }
    
    // let end = start.0.elapsed();
    let end = start.0.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}


