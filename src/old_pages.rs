
use std::{thread, time};
use std::time::Instant;
use std::time::Duration;
use std::{env, str, io};
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs::{self, File};

use rocket_contrib::Template;
use rocket::response::{content, NamedFile, Redirect, Flash};
use rocket::{Request, Data, Outcome};
use rocket::request::{FlashMessage, Form, FromForm};
use rocket::data::FromData;
use rocket::response::content::Html;
use rocket::State;
// use rocket::request::{Form, FlashMessage};
use rocket::http::{Cookie, Cookies, RawStr};
// use auth::userpass::UserPass;
// use auth::status::{LoginStatus,LoginRedirect};
// use auth::dummy::DummyAuthenticator;
// use auth::authenticator::Authenticator;
use regex::Regex;
use titlecase::titlecase;

use chrono::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Mutex, Arc, RwLock};

use rocket::http::hyper::header::{Headers, ContentDisposition, DispositionType, DispositionParam, Charset};

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
// use counter::*;
use super::*;
// use routes::*;
// use routes::pages::*;
use cache::*;
// use content::*;
use content::{destruct_cache, destruct_context};
use cache::body::*;
use cache::pages::*;
use counter::*;
use location::*;
use referrer::*;
use collate::*;
use layout::*;
use blog::*;
use data::*;
use sanitize::*;
use rocket_auth_login::authorization::*;
use rocket_auth_login::sanitization::*;
use ral_administrator::*;
use ral_user::*;
use templates::*;
use xpress::*;
use accept::*;
// use ::templates::*;


use comrak::{markdown_to_html, ComrakOptions};


// <block n=1>
#[get("/all_tags")]
pub fn hbs_tags_all(start: GenTimer, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
    // let start = Instant::now();
    
    let qrystr = "SELECT COUNT(*) as cnt, unnest(tag) as untag FROM articles GROUP BY untag ORDER BY cnt DESC;";
    let qry = conn.query(qrystr, &[]);
    let mut tags: Vec<TagCount> = Vec::new();
    if let Ok(result) = qry {
        // let mut sizes: Vec<u16> = Vec::new();
        for row in &result {
            let c: i64 = row.get(0);
            let c2: u32 = c as u32;
            // sizes.push(c2 as u16);
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
    
    let output: Template = hbs_template(TemplateBody::Tags(tags), None, Some("Viewing All Tags".to_string()), String::from("/all_tags"), admin, user, None, Some(start.0));
    
    let end = start.0.elapsed();
    // println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}

// // NOT USED ANYMORE?
// // View paginated articles - pretty much just a test route
// #[get("/view_articles")]
// pub fn hbs_view_articles(start: GenTimer, pagination: Page<Pagination>, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
//     
//     let total_query = "SELECT COUNT(*) as count FROM articles";
//     let output: Template;
//     if let Ok(rst) = conn.query(total_query, &[]) {
//         if !rst.is_empty() && rst.len() == 1 {
//             let row = rst.get(0);
//             let count: i64 = row.get(0);
//             let total_items: u32 = count as u32;
//             let (ipp, cur, num_pages) = pagination.page_data(total_items);
//             // let sql = pagination.sql("SELECT a.aid, a.title, a.posted, a.body, a.tag, a.description, u.userid, u.display, u.username FROM articles a JOIN users u ON (a.author = u.userid)", Some("posted DESC"));
//             let sql = pagination.sql(&format!("SELECT a.aid, a.title, a.posted, description({}, a.body, a.description) as body, a.tag, a.description, u.userid, u.display, u.username FROM articles a JOIN users u ON (a.author = u.userid)", DESC_LIMIT), Some("posted DESC"));
//             println!("Prepared paginated query:\n{}", sql);
//             if let Some(results) = conn.articles(&sql) {
//                 // let results: Vec<Article> = conn.articles(&sql);
//                 if results.len() != 0 {
//                     let page_information = pagination.page_info(total_items);
//                     output = hbs_template(TemplateBody::ArticlesPages(results, pagination, total_items, Some(page_information), None), Some(format!("Viewing All Articles - Page {} of {}", cur, num_pages)), String::from("/view_articles"), admin, user, None, Some(start.0));
//                     let express: Express = output.into();
//                     return express.compress( encoding );
//                 }
//             }
//             // if let Ok(qry) = conn.query(sql, &[]) {
//             //     if !qry.is_empty() && rst.len() != 0 {
//                       
//             //     }
//             // }
//         }
//     }
//    
//     output = hbs_template(TemplateBody::General(alert_danger("Database query failed."), None), Some("Viewing All Articles".to_string()), String::from("/view_articles"), admin, user, None, Some(start.0));
//     let express: Express = output.into();
//     express.compress( encoding )
// }

#[get("/tag?<tag>")]
pub fn hbs_articles_tag_redirect(tag: Tag) -> Redirect {
    Redirect::to(&format!("/tag/{}", tag.tag))
}

#[get("/tag/<tag>")]
pub fn hbs_articles_tag(start: GenTimer, tag: String, pagination: Page<Pagination>, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
    
    let output: Template;
    
    // let tag = 
    
    // vtags - Vector of Tags - Vector<Tags>
    let vtags = split_tags(tag.clone());
    if vtags.len() == 0 {
            output = hbs_template(TemplateBody::General(alert_danger("No tag specified.")), None, Some("No Tag Specified".to_string()), String::from("/tag"), admin, user, None, Some(start.0));
    } else {
        let sql: String = if vtags.len() == 1 {
            format!(" WHERE '{}' = ANY(a.tag)", sanitize_tag(&vtags[0]))
        } else {
            let mut tmp = String::with_capacity((vtags.len()*35) + 50);
            // tmp.push_str(" WHERE ");
            tmp.push_str(" WHERE '");
            tmp.push_str(&sanitize_tag(&vtags[0]));
            tmp.push_str("' = ANY(a.tag)");
            // tmp.push_str("");
            
            for t in &vtags[1..] {
                tmp.push_str(" AND '");
                tmp.push_str(&sanitize_tag(t));
                tmp.push_str("' = ANY(a.tag)");
                // tmp.push_str("");
            }
            tmp
        };
        
        let mut countqrystr = String::with_capacity(sql.len() + 60);
        countqrystr.push_str("SELECT COUNT(*) as count FROM articles a");
        countqrystr.push_str(&sql);
        
        let mut qrystr = String::with_capacity(sql.len() + 60);
        qrystr.push_str(&format!("SELECT a.aid, a.title, a.posted, description({}, a.body, a.description) as body, a.tag, a.description, u.userid, u.display, u.username FROM articles a JOIN users u ON (a.author = u.userid)", DESC_LIMIT));
        qrystr.push_str(&sql);
        
        // println!("\nTag count query: {}\nTag articles query: {}\n", countqrystr, qrystr);
        
        if let Ok(rst) = conn.query(&countqrystr, &[]) {
            if !rst.is_empty() && rst.len() == 1 {
                let countrow = rst.get(0);
                let count: i64 = countrow.get(0);
                let total_items: u32 = count as u32;
                let (ipp, cur, num_pages) = pagination.page_data(total_items);
                let pagesql = pagination.sql(&qrystr, Some("posted DESC"));
                // println!("Tag pagination query:\n{}", pagesql);
                if let Some(results) = conn.articles(&pagesql) {
                    if results.len() != 0 {
                        let page_information = pagination.page_info(total_items);
                        output = hbs_template(TemplateBody::ArticlesPages(results, pagination, total_items, Some(page_information)), None, Some(format!("Viewing Tag {} - Page {} of {}", tag, cur, num_pages)), String::from("/tag"), admin, user, None, Some(start.0));
                    } else {
                        output = hbs_template(TemplateBody::General(alert_danger("No articles found with the specified tag.")), None, Some("Tag".to_string()), String::from("/tag"), admin, user, None, Some(start.0));
                    }
                } else {
                        output = hbs_template(TemplateBody::General(alert_danger("No articles found with the specified tag.")), None, Some("Tag".to_string()), String::from("/tag"), admin, user, None, Some(start.0));
                }
            } else {
                output = hbs_template(TemplateBody::General(alert_danger("No articles found with the specified tag.")), None, Some("Tag".to_string()), String::from("/tag"), admin, user, None, Some(start.0));
            }
        } else {
                output = hbs_template(TemplateBody::General(alert_danger("No articles found with the specified tag.")), None, Some("Tag".to_string()), String::from("/tag"), admin, user, None, Some(start.0));
        }
    }
    let express: Express = output.into();
    express.compress( encoding )
    
    
    
    
    
    // let total_query = "SELECT COUNT(*) as count FROM articles";
    // let output: Template;
    // if let Ok(rst) = conn.query(total_query, &[]) {
    //     if !rst.is_empty() && rst.len() == 1 {
    //         let row = rst.get(0);
    //         let count: i64 = row.get(0);
    //         let total_items: u32 = count as u32;
    //         let (ipp, cur, num_pages) = pagination.page_data(total_items);
    //         // let sql = pagination.sql("SELECT a.aid, a.title, a.posted, a.body, a.tag, a.description, u.userid, u.display, u.username FROM articles a JOIN users u ON (a.author = u.userid)", Some("posted DESC"));
    //         let sql = pagination.sql(&format!("SELECT a.aid, a.title, a.posted, description({}, a.body, a.description) as body, a.tag, a.description, u.userid, u.display, u.username FROM articles a JOIN users u ON (a.author = u.userid)", DESC_LIMIT), Some("posted DESC"));
    //         println!("Prepared paginated query:\n{}", sql);
    //         if let Some(results) = conn.articles(&sql) {
    //             // let results: Vec<Article> = conn.articles(&sql);
    //             if results.len() != 0 {
    //                 let page_information = pagination.page_info(total_items);
    //                 output = hbs_template(TemplateBody::ArticlesPages(results, pagination, total_items, Some(page_information), None), Some(format!("Viewing All Articles - Page {} of {}", cur, num_pages)), String::from("/view_articles"), admin, user, None, Some(start.0));
    //                 let express: Express = output.into();
    //                 return express.compress( encoding );
    //             }
    //         }
    //         // if let Ok(qry) = conn.query(sql, &[]) {
    //         //     if !qry.is_empty() && rst.len() != 0 {
                    
    //         //     }
    //         // }
    //     }
    // }
    
    // output = hbs_template(TemplateBody::General(alert_danger("Database query failed."), None), Some("Viewing All Articles".to_string()), String::from("/view_articles"), admin, user, None, Some(start.0));
    // let express: Express = output.into();
    // express.compress( encoding )
    
    
    // let output: Template;
    // let tags = Some(split_tags(medium_sanitize(tag.tag.clone())));
    // // limit, # body chars, min date, max date, tags, strings
    // let results = Article::retrieve_all(conn, 0, Some(-1), None, None, tags, None);
    // if results.len() != 0 {
    //     output = hbs_template(TemplateBody::Articles(results, None), Some(format!("Viewing Articles with Tags: {}", tag.tag)), String::from("/all_tags"), admin, user, None, Some(start.0));
    // } else {
    //     output = hbs_template(TemplateBody::General(alert_danger("Could not find any articles with the specified tag."), None), Some(format!("Could not find any articles with the tag(s): {}", medium_sanitize(tag.tag) )), String::from("/tag"), admin, user, None, Some(start.0));
    // }
    
    // let end = start.0.elapsed();
    // println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    // let express: Express = output.into();
    // express.compress(encoding)
}

#[get("/article/<aid>/<title>")]

pub fn hbs_article_title(start: GenTimer, aid: ArticleId, title: Option<&RawStr>, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
    hbs_article_view(start, aid, conn, admin, user, encoding, uhits)
}

#[get("/article/<aid>")]
pub fn hbs_article_id(start: GenTimer, aid: ArticleId, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
    hbs_article_view(start, aid, conn, admin, user, encoding, uhits)
}

#[get("/article?<aid>")]
pub fn hbs_article_view(start: GenTimer, aid: ArticleId, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
    // let start = Instant::now();
    let rst = aid.retrieve_with_conn(conn); // retrieve result
    let mut output: Template; 
    if let Some(article) = rst {
        let title = article.title.clone();
        output = hbs_template(TemplateBody::Article(article), None, Some(title), String::from("/article"), admin, user, Some("enable_toc(true);".to_owned()), Some(start.0));
    } else {
        output = hbs_template(TemplateBody::General(alert_danger(&format!("Article {} not found.", aid.aid))), None, Some("Article Not Found".to_string()), String::from("/article"), admin, user, None, Some(start.0));
    }
    let end = start.0.elapsed();
    // println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}

#[get("/article")]
pub fn hbs_article_not_found(start: GenTimer, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
    // let start = Instant::now();
    let output: Template = hbs_template(TemplateBody::General(alert_danger("Article not found")), None, Some("Article not found".to_string()), String::from("/article"), admin, user, None, Some(start.0));
    let end = start.0.elapsed();
    // println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}

// </block n=1>

// <block n=2

// application/rss+xml
#[get("/rss.xml")]
// EXPRESS X - pub fn rss_page(conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>) -> String {
pub fn rss_page(start: GenTimer, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
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
    // println!("RSS Generated in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    rss
}

// #[get("/author/<authorid>/<authorname>")]
// pub fn hbs_author_display(start: GenTimer, authorid: u32, authorname: Option<String>, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
//     unimplemented!()
    
// }

#[get("/author/<authorid>/<authorname>")]
pub fn hbs_author_display(start: GenTimer, authorid: u32, pagination: Page<Pagination>, authorname: Option<&RawStr>, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
    hbs_author(start, authorid, pagination, conn, admin, user, encoding, uhits)
}

#[get("/author/<authorid>")]
pub fn hbs_author(start: GenTimer, authorid: u32, pagination: Page<Pagination>, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
    // unimplemented!()
    let output: Template;
    
    // let results = conn.articles( &format!("SELECT a.aid, a.title, a.posted, description({}, a.body, a.description), a.tag, a.description, u.userid, u.display, u.username FROM articles a JOIN users u ON (a.author = u.userid) WHERE userid = {}", DESC_LIMIT, authorid) );
    
    // if let Some(articles) = results {
    //     output = hbs_template(TemplateBody::Articles(articles, None), Some("Articles by Author".to_string()), String::from("/author"), admin, user, None, Some(start.0));
    // } else {
    //     output = hbs_template(TemplateBody::General("There are no articles by the specified user.".to_string(), None), Some("Articles by Author".to_string()), String::from("/author"), admin, user, None, Some(start.0));
    // }
    
    let total_query = format!("SELECT COUNT(*) AS count FROM articles WHERE author = {}", authorid);
    if let Ok(rst) = conn.query(&total_query, &[]) {
        if !rst.is_empty() && rst.len() == 1 {
            let row = rst.get(0);
            let count: i64 = row.get(0);
            let total_items: u32 = count as u32;
            let (ipp, cur, num_pages) = pagination.page_data(total_items);
            let sql = pagination.sql(&format!("SELECT a.aid, a.title, a.posted, description({}, a.body, a.description) as body, a.tag, a.description, u.userid, u.display, u.username FROM articles a JOIN users u ON (a.author = u.userid) WHERE a.author = {}", DESC_LIMIT, authorid), Some("posted DESC"));
            // println!("Prepared paginated query:\n{}", sql);
            if let Some(results) = conn.articles(&sql) {
                if results.len() != 0 {
                    let page_information = pagination.page_info(total_items);
                    
                    output = hbs_template(TemplateBody::ArticlesPages(results, pagination, total_items, Some(page_information)), None, Some("Articles by Author".to_string()), String::from("/author"), admin, user, None, Some(start.0));
                } else {
                    output = hbs_template(TemplateBody::General("There are no articles by the specified user.".to_string()), None, Some("Articles by Author".to_string()), String::from("/author"), admin, user, None, Some(start.0));
                }
            } else {
                    output = hbs_template(TemplateBody::General("There are no articles by the specified user.".to_string()), None, Some("Articles by Author".to_string()), String::from("/author"), admin, user, None, Some(start.0));
            }
        } else {
            output = hbs_template(TemplateBody::General("There are no articles by the specified user.".to_string()), None, Some("Articles by Author".to_string()), String::from("/author"), admin, user, None, Some(start.0));
        }
    } else {
        output = hbs_template(TemplateBody::General("There are no articles by the specified user.".to_string()), None, Some("Articles by Author".to_string()), String::from("/author"), admin, user, None, Some(start.0));
    }
    
    let end = start.0.elapsed();
    // println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    let express: Express = output.into();
    express.compress(encoding)
}

// </block n=2>

// <block n=3>
#[get("/")]
pub fn hbs_index(start: GenTimer, pagination: Page<Pagination>, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, flash: Option<FlashMessage>, encoding: AcceptCompression, uhits: UniqueHits) -> Express {
    
    // println!("Unique hits:\n\tRoute: {}\n\tIP Address: {}\n\tVisits: {}\n\tUnique Visitors: {}\n", &uhits.0, &uhits.1, &uhits.2, &uhits.3);
    
    let fmsg: Option<String>;
    if let Some(flashmsg) = flash {
        fmsg = Some(alert_info( flashmsg.msg() ));
    } else {
        fmsg = None;
    }
    
    
    
    let total_query = "SELECT COUNT(*) AS count FROM articles";
    let output: Template;
    if let Ok(rst) = conn.query(total_query, &[]) {
        if !rst.is_empty() && rst.len() == 1 {
            let row = rst.get(0);
            let count: i64 = row.get(0);
            let total_items: u32 = count as u32;
            let (ipp, cur, num_pages) = pagination.page_data(total_items);
            let sql = pagination.sql(&format!("SELECT a.aid, a.title, a.posted, description({}, a.body, a.description) as body, a.tag, a.description, u.userid, u.display, u.username, a.image, a.markdown, a.modified FROM articles a JOIN users u ON (a.author = u.userid)", DESC_LIMIT), Some("posted DESC"));
            // println!("Prepared paginated query:\n{}", sql);
            if let Some(results) = conn.articles(&sql) {
                if results.len() != 0 {
                    // let page_information = pagination.page_info(total_items);
                    let mut page_information: String;
                    let pinfo = pagination.page_info(total_items);
                    if cur == 1 {
                        // r##"background: rgba(0, 0, 0, 0) url("http://localhost:8000/assets/welcome.png") no-repeat scroll center center;"##
                        /*let welcome = format!(r##"<h1 style="text-align: center; background: #212529 url('{}assets/welcome.png') no-repeat scroll center center;">Welcome</h1>
                            <p>This is the personal blog of Andrew Prindle.  My recent topics of interest include:
                             the Rust programming language, web development, javascript, databases, cryptology, security, and compression.  
                             Feel free to contact me at the email address at the bottom of the page.</p>
                             <hr>
                             "##, BLOG_URL);*/
                        let welcome = r##"<h1 style="text-align: center;">Welcome</h1>
                            <p>This is the personal blog of Andrew Prindle.  My recent topics of interest include:
                             the Rust programming language, web development, javascript, databases, cryptology, security, and compression.  
                             Feel free to contact me at the email address at the bottom of the page.</p>
                             <hr>
                             "##;
                             // <h3>All Articles By Date</h3>
                             // </div></div><div class="v-content"><div class="v-pageinfo">
                        page_information = String::with_capacity(pinfo.len() + welcome.len() + 50);
                        page_information.push_str( welcome );
                        page_information.push_str( &pinfo );
                    } else {
                        // page_information = pagination.page_info(total_items);
                        // page_information = String::with_capacity(pinfo.len() + 50);
                        let welcome = r##"<h1 style="text-align: center;">Articles By Date</h1>
                        "##;
                        /*let welcome = format!(r##"</*h1*/ style="text-align: center; background: #212529 url('{}assets/welcome.png') no-repeat scroll center center;">Articles By Date</h1>
                        "##, BLOG_URL);*/
                        page_information = String::with_capacity(pinfo.len() + welcome.len() + 50);
                        page_information.push_str( welcome );
                        page_information.push_str( &pinfo );
                    }
                    
                    output = hbs_template(TemplateBody::ArticlesPages(results, pagination, total_items, Some(page_information)), fmsg, None, String::from("/"), admin, user, None, Some(start.0));
                    let express: Express = output.into();
                    
                    let end = start.0.elapsed();
                    // println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
                    
                    return express.compress( encoding );
                }
            }
        }
    }
    
    output = hbs_template(TemplateBody::General(alert_danger("No articles to show.")), fmsg, None, String::from("/"), admin, user, None, Some(start.0));
    let express: Express = output.into();
    
    let end = start.0.elapsed();
    // println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    
    express.compress( encoding )
    
    // let output: Template;
    // let fmsg: Option<String>;
    // if let Some(flashmsg) = flash {
    //     fmsg = Some(alert_info( flashmsg.msg() ));
    // } else {
    //     fmsg = None;
    // }
    // let results = Article::retrieve_all(conn, 0, Some(300), None, None, None, None);
    // if results.len() != 0 {
    //     output = hbs_template(TemplateBody::Articles(results, fmsg), None, String::from("/"), admin, user, None, Some(start.0));
    // } else if admin.is_some() {
    //     output = hbs_template(TemplateBody::General("There are no articles.<br>\n<a href =\"/insert\">Create Article</a>".to_string(), None), None, String::from("/"), admin, user, None, Some(start.0));
    // } else {
    //     output = hbs_template(TemplateBody::General("There are no articles.".to_string(), None), None, String::from("/"), admin, user, None, Some(start.0));
    // }
    
    // // let end = start.0.elapsed();
    // let end = start.0.elapsed();
    // println!("Served in {}.{:09} seconds", end.as_secs(), end.subsec_nanos());
    // let express: Express = output.into();
    // express.compress(encoding)
}

// </block n=3>
