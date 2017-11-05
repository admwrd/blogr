
use std::fmt::Display;

use rocket;
use ::rocket::Request;
use ::rocket::request::{FromRequest, FromForm, FormItems, FromFormValue, FromParam};
use ::rocket::outcome::Outcome;
use ::rocket::config::{Config, Environment};
use rocket::http::RawStr;
// use rocket::request::{FromFormValue, FromParam};

use titlecase::titlecase;
use regex::Regex;
use chrono::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};
use htmlescape::*;

use postgres::{Connection};

use super::{MAX_CREATE_TITLE, MAX_CREATE_DESCRIPTION, MAX_CREATE_TAGS};
use users::*;
use data::*;
use sanitize::*;

// type ArticleId = u32;
#[derive(Debug, Clone)]
pub struct ArticleId {
    pub aid: u32,
}

// used for retrieving a GET url tag
#[derive(Debug, Clone)]
pub struct Tag {
    pub tag: String,
}

#[derive(Debug, Clone)]
pub struct Article {
    pub aid: u32,
    pub title: String,
    pub posted: NaiveDateTime,
    pub body: String,
    pub tags: Vec<String>,
    pub description: String,
    // pub author_id: u32,
    // pub author_name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ArticleDisplay {
    pub aid: u32,
    pub title: String,
    pub posted_machine: String,
    pub posted_human: String,
    pub body: String,
    pub tags: Vec<String>,
    pub description: String,
    // pub author_id: u32,
    // pub author_name: String,
}

#[derive(Debug, Clone)]
pub struct ArticleForm {
    // pub userid: u32,
    pub title: String,
    pub body: String,
    pub tags: String,
    pub description: String,
    // pub author_id: u32,
    // pub author_name: String,
}

#[derive(Debug, Clone)]
pub struct Search {
    pub limit: Option<u16>, // use u16 as limit as u16 does not implement FromSql
    pub o: Option<String>, // opposite / negated
    pub p: Option<String>, // possible words, or'd
    pub q: Option<String>, // query, and'd together
    pub min: Option<NaiveDateTimeWrapper>, // min
    pub max: Option<NaiveDateTimeWrapper>, // min
}

#[derive(Serialize)]
pub struct SearchDisplay {
    pub limit: u16,
    pub q: String,
    pub min: String,
    pub max: String,
}

impl SearchDisplay {
    pub fn default() -> SearchDisplay {
        SearchDisplay {
            limit: 0,
            q: String::new(),
            min: String::new(),
            max: String::new(),
        }
    }
}

impl Search {
    pub fn to_display(&self) -> SearchDisplay {
        SearchDisplay {
            limit: if let Some(limit) = self.limit { limit } else { 0 },
            q: if let Some(ref q) = self.q { q.to_string() } else { String::new() },
            min: if let Some(ref min) = self.min { format!("{}", min.0.format("%Y-%m-%d %H:%M:%S")) } else { String::new() },
            max: if let Some(ref max) = self.max { format!("{}", max.0.format("%Y-%m-%d %H:%M:%S")) } else { String::new() },
        }
    }
    pub fn default() -> Search {
        Search {
            limit: None,
            o: None,
            p: None,
            q: None,
            min: None,
            max: None,
        }
    }
}



#[derive(FromForm)]
pub struct ViewPage {
    pub page: u32,
    // Articles Per Page
    pub app: Option<u8>,
}

pub struct ArticleSearch {
    // pub min_date: NaiveDate,
    // pub max_date: NaiveDate,
    pub keywords: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct User {
    pub userid: u32,
    pub username: String,
    pub display: Option<String>,
    pub email: Option<String>,
    pub password: String,
    pub is_admin: bool,
    pub is_public: bool,
}

pub const DESC_LIMIT: usize = 300;

pub fn opt_col<T>(rst: Option<Result<T, T>>) -> T where T: Display + Default {
    match rst {
        Some(Ok(d)) => d,
        Some(Err(e)) => { println!("Encountered an error retrieving the description. Error: {}", e); T::default() },
        None => T::default(),
    }
}

impl ArticleId {
    pub fn exists(&self) -> bool {
        unimplemented!()
    }
    // Retrieve with a new connection - not a pooled connection
    //   Do not use unless you have to - unless you have no db connection
    pub fn retrieve(&self) -> Option<Article> {
        let pgconn = establish_connection();
        // let rawqry = pgconn.query(&format!("SELECT aid, title, posted, body, tag, description FROM articles WHERE aid = {id}", id=self.aid), &[]);
        let rawqry = pgconn.query(&format!("SELECT a.aid, a.title, a.posted, a.body, a.tag, a.description, u.userid, u.display, u.username FROM articles a JOIN users u ON (a.author = u.userid) WHERE a.aid = {id}", id=self.aid), &[]);
        if let Ok(aqry) = rawqry {
            println!("Querying articles: found {} rows", aqry.len());
            if !aqry.is_empty() && aqry.len() == 1 {
                let row = aqry.get(0); // get first row
                Some( Article {
                    aid: row.get(0),
                    title: row.get(1), // todo: call sanitize title here
                    posted: row.get(2),
                    body: row.get(3), // Todo: call sanitize body here
                    tags: row.get_opt(4).unwrap_or(Ok(Vec::<String>::new())).unwrap_or(Vec::<String>::new()).into_iter().map(|s| s.trim().trim_matches('\'').to_string()).collect(),
                    // description: opt_col(row.get_opt(5)),
                    description: row.get_opt(5).unwrap_or(Ok(String::new())).unwrap_or(String::new()),
                    // author_id: row.get(6),
                    // author_name: row.get_opt(7).unwrap_or(Ok(row.get(8))).unwrap_or(String::new()), 
                })
            } else { None }
        } else { None }
    }
    // Prefer to use this over retrieve()
    pub fn retrieve_with_conn(&self, pgconn: DbConn) -> Option<Article> {
        // let rawqry = pgconn.query(&format!("SELECT aid, title, posted, body, tag, description FROM articles WHERE aid = {id}", id=self.aid), &[]);
        let rawqry = pgconn.query(&format!("SELECT a.aid, a.title, a.posted, a.body, a.tag, a.description, u.userid, u.display, u.username FROM articles a JOIN users u ON (a.author = u.userid) WHERE a.aid = {id}", id=self.aid), &[]);
        if let Ok(aqry) = rawqry {
            println!("Querying articles: found {} rows", aqry.len());
            if !aqry.is_empty() && aqry.len() == 1 {
                let row = aqry.get(0); // get first row
                
                Some( Article {
                    aid: row.get(0),
                    title: row.get(1), // todo: call sanitize title here
                    posted: row.get(2),
                    body: row.get(3), // Todo: call sanitize body here
                    tags: row.get_opt(4).unwrap_or(Ok(Vec::<String>::new())).unwrap_or(Vec::<String>::new()).into_iter().map(|s| s.trim().trim_matches('\'').to_string()).collect(),
                    description: row.get_opt(5).unwrap_or(Ok(String::new())).unwrap_or(String::new()),
                    // author_id: row.get(6),
                    // author_name: row.get_opt(7).unwrap_or(Ok(row.get(8))).unwrap_or(String::new()), 
                })
            } else { None }
        } else { None }
    }
    pub fn last_id() -> u32 {
        unimplemented!()
    }
    pub fn next_id() -> u32 {
        unimplemented!()
    }
    
}


impl Search {
    pub fn search() -> Vec<Article> {
        unimplemented!()
    }
}

pub fn get_len<T>(input: &Option<Vec<T>>) -> usize {
    if let &Some(ref inner) = input {
        inner.len()
    } else {
        0
    }
}

impl Article {
    pub fn to_display(&self) -> ArticleDisplay {
        ArticleDisplay {
            aid: self.aid.clone(),
            title: self.title.clone(),
            posted_machine: self.posted.format("%Y-%m-%dT%H:%M:%S").to_string(),
            posted_human: self.posted.format("%Y-%m-%d @ %I:%M%P").to_string(),
            body: self.body.clone(),
            tags: self.tags.clone(),
            description: self.description.clone(),
            // author_id: self.author_id,
            // author_name: self.author_name.clone(),
        }
    }
    pub fn split_tags(string: String) -> Vec<String> {
        // Todo: call sanitize tags before splitting:
        let tags: Vec<String> = string.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| s.as_str() != "" && s.as_str() != " ")
            .collect();
        tags
    }
    pub fn retrieve(aid: u32) -> Option<Article> {
        let pgconn = establish_connection();
        let rawqry = pgconn.query(&format!("SELECT aid, title, posted, body, tag, description FROM articles WHERE aid = {id}", id=aid), &[]);
        if let Ok(aqry) = rawqry {
            // println!("Querying articles: found {} rows", aqry.len());
            if !aqry.is_empty() && aqry.len() == 1 {
                let row = aqry.get(0); // get first row
                Some( Article {
                    aid: row.get(0),
                    title: row.get(1), // todo: call sanitize title here
                    posted: row.get(2),
                    body: row.get(3), // Todo: call sanitize body here
                    tags: row.get_opt(4).unwrap_or(Ok(Vec::<String>::new())).unwrap_or(Vec::<String>::new()).into_iter().map(|s| s.trim().trim_matches('\'').to_string()).collect(),
                    description: row.get_opt(5).unwrap_or(Ok(String::new())).unwrap_or(String::new()),
                })
            } else { None }
        } else { None }
    }
    pub fn info(&self) -> String {
        format!("Aid: {aid}, Title: {title}, Posted on: {posted}, Description:<br>\n{desc}<br>\nBody:<br>\n{body}<br>\ntags: {tags:#?}", aid=self.aid, title=self.title, posted=self.posted, body=self.body, tags=self.tags, desc=self.description)
    }
    
    
    /// Description: Some(50) displays 50 characters of body text
    ///              Some(-1) displays the description field as the body
    ///              None displays all of the body text
    pub fn retrieve_all(pgconn: DbConn, limit: u32, description: Option<i32>, min_date: Option<NaiveDate>, max_date: Option<NaiveDate>, tag: Option<Vec<String>>, search: Option<Vec<String>>) -> Vec<Article> {
        let mut show_desc = false;
        let mut qrystr: String = if let Some(summary) = description {
            if summary < 1 {
                show_desc = true;
                format!("SELECT aid, title, posted, LEFT(body, {}) as body, tag, description FROM articles", DESC_LIMIT)
            } else {
                format!("SELECT aid, title, posted, LEFT(body, {}) AS body, tag, description FROM articles", summary)
            }
        } else {
            String::from("SELECT aid, title, posted, body, tag, description FROM articles")
        };
        if min_date.is_some() || max_date.is_some() || (tag.is_some() && get_len(&tag) != 0) || (search.is_some() && get_len(&search) != 0) {
            qrystr.push_str(" WHERE");
            let mut where_str = String::from("");
            if let Some(date_min) = min_date {
                where_str.push_str( &format!(" posted >= '{}'", date_min.format("%Y-%m-%d %H:%M:%S")) );
            }
            if let Some(date_max) = max_date {
                if &where_str != "" { where_str.push_str(" AND "); }
                where_str.push_str( &format!(" posted <= '{}'", date_max.format("%Y-%m-%d %H:%M:%S")) );
                
            }
            if let Some(v) = tag {
                if &where_str != "" { where_str.push_str(" AND "); }
                let mut tag_str = String::new();
                let mut first: bool = true;
                for t in v {
                    if first { first = false; } else { tag_str.push_str(" AND "); }
                    // tag_str.push_str( &format!(" tags LIKE '%{}%'", t) );
                    tag_str.push_str( &format!(" '{}' = ANY(tag)", t) );
                }
                if &tag_str != "" { where_str.push_str(&tag_str); }
            }
            if let Some(strings) = search {
                if &where_str != "" { where_str.push_str(" AND "); }
                let mut search_str = String::new();
                let mut first: bool = true;
                for string in strings {
                    if first { first = false; } else { search_str.push_str(" AND ") }
                    search_str.push_str( &format!(" (title LIKE '%{s}%' OR body LIKE '%{s}%')", s=string) );
                }
                if &search_str != "" { where_str.push_str(&search_str); }
            }
            qrystr.push_str(&where_str);
        }
        qrystr.push_str(" ORDER BY posted DESC");
        if limit != 0 { qrystr.push_str(&format!(" LIMIT {}", limit)); }
        println!("Query: {}", qrystr);
        let qryrst = pgconn.query(&qrystr, &[]);
        if let Ok(result) = qryrst {
            let mut articles: Vec<Article> = Vec::new();
            for row in &result {
                let a = Article {
                    aid: row.get(0),
                    title: row.get(1),
                    posted: row.get(2),
                    body: if show_desc {  // show the truncated body if there is no description when show_desc is true
                            let d = row.get_opt(5).unwrap_or(Ok(String::new())).unwrap_or(String::new());
                            if &d == "" { row.get(3) }
                            else { d }
                        } else { row.get(3) },
                    tags: row.get_opt(4).unwrap_or(Ok(Vec::<String>::new())).unwrap_or(Vec::<String>::new()).into_iter().map(|s| s.trim().trim_matches('\'').to_string()).collect(),
                    // description: if show_desc { String::new() } else { String::new() },
                    // show_desc moves the description to the body
                    description: if show_desc { 
                            String::new() 
                        } else { 
                            row.get_opt(5).unwrap_or(Ok(String::new())).unwrap_or(String::new()) 
                        },
                };
                articles.push(a);
            }
            println!("Found {} articles with the specified query.", articles.len());
            articles
        } else {
            println!("Query failed.");
            Vec::<Article>::new()
        }
    }
}

impl ArticleForm {
    pub fn new(title: String, body: String, tags: String, description: String) -> ArticleForm {
        ArticleForm {
            title,
            body,
            tags,
            description,
        }
    }
    pub fn to_article(&self) -> Article {
        // get next aid
        let next_aid = 0;
        Article {
            aid: next_aid,
            title: sanitize_title(self.title.clone()),
            posted: Local::now().naive_local(), // This fn is only used when saving new articles
            body: sanitize_body(self.body.clone()),
            tags: split_tags(sanitize_tags(self.tags.clone())),
            description: sanitize_body(self.description.clone()),
        }
    }
    pub fn save(&self, conn: &DbConn) -> Result<Article, String> {
        let now = Local::now().naive_local();
        
        // take blah, blah2, blah3 and convert into {'blah', 'blah2', 'blah3'}
        let tagstr = format!( 
            "{{{}}}", self.tags.clone()
            .split(',')
            .map(
                |s| format!("\"{}\"", s.trim().to_lowercase())
            ).collect::<Vec<_>>()
            .join(",")
            // .replace(",''")
        );
            
        //  can return both id and posted date
        // let qrystr = format!("INSERT INTO blog (aid, title, posted, body, tags) VALUES ('', '{title}', '{posted}', '{body}', {tags}) RETURNING aid, posted",
        let qrystr = format!("INSERT INTO articles (title, posted, body, tag, description) VALUES ('{title}', '{posted}', '{body}', '{tags}', '{desc}') RETURNING aid",
            title=self.title, posted=now, body=self.body, tags=tagstr, desc=self.description);
        println!("Insert query: {}", qrystr);
        let result = conn.query(&qrystr, &[]);
        match result {
            Err(err) => Err(format!("Could not insert article. Error: {}", err)),
            Ok(qry)  => {
                if !qry.is_empty() && qry.len() == 1 {
                    let row = qry.get(0);
                    Ok( Article {
                        aid: row.get(0),
                        title: self.title.clone(),
                        posted: now,
                        body: self.body.clone(),
                        tags: Article::split_tags(self.tags.clone()),
                        description: self.description.clone(),
                    })
                } else if qry.is_empty() {
                    Err("Error inserting article, result is empty.".to_string())
                } else if qry.len() > 1 {
                    Err(format!("Error inserting article, returned {} rows.", qry.len()))
                } else {
                    Err("Unknown error inserting article.".to_string())
                }
            },
        }
    }
}



// A request guard to ensure that an article exists for a given ArticleId or aid
impl<'a, 'r> FromRequest<'a, 'r> for ArticleId {
    type Error = ();
    
    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<ArticleId, Self::Error> {
        
        
        // match Article::retrieve() {
        //     Some(cookie) => Outcome::Success(UserCookie::new(cookie.value().to_string())),
        //     None => Outcome::Forward(()),
        // }
        
        unimplemented!()
    }
}
impl<'f> FromForm<'f> for ArticleId {
    type Error = &'static str;
    fn from_form(form_items: &mut FormItems<'f>, _strict: bool) -> Result<Self, Self::Error> {
        let mut aid: u32 = 0;
        for (field, value) in form_items {
            if field.as_str() == "aid" {
                aid = value.to_string().parse::<u32>().unwrap_or(0u32);
            }
            // match field.as_str() {
            //     "aid" => { aid = value.to_string().parse::<u32>().unwrap_or(0u32) },
            //     _ => {},
            // }
        }
        if aid == 0  {
            Err("Invalid user id specified")
        } else {
            Ok( ArticleId { aid} )
        }
    }
}
// A request guard to ensure that an article exists for a given ArticleId or aid
impl<'a, 'r> FromRequest<'a, 'r> for Tag {
    type Error = ();
    
    fn from_request(request: &'a Request<'r>) -> rocket::request::Outcome<Tag, Self::Error> {
        
        
        // match Article::retrieve() {
        //     Some(cookie) => Outcome::Success(UserCookie::new(cookie.value().to_string())),
        //     None => Outcome::Forward(()),
        // }
        
        unimplemented!()
    }
}


impl<'f> FromForm<'f> for Search {
    type Error = &'static str;
    fn from_form(form_items: &mut FormItems<'f>, _strict: bool) -> Result<Self, Self::Error> {
        let mut tag: String = String::new();
        let mut limit: Option<u16> = None;
        let mut o: Option<String> = None;
        let mut p: Option<String> = None;
        let mut q: Option<String> = None;
        let mut min: Option<NaiveDateTimeWrapper> = None;
        let mut max: Option<NaiveDateTimeWrapper> = None;
        for (field, value) in form_items {
            match field.as_str() {
                "limit" => { limit = Some(value.parse::<u16>().unwrap_or(0)) },
                "o" => { o = Some(encode_attribute(&value.url_decode().unwrap_or(String::new())) ) },
                "p" => { p = Some(encode_attribute(&value.url_decode().unwrap_or(String::new())) ) },
                "q" => { q = Some(encode_attribute(&value.url_decode().unwrap_or(String::new())) ) },
                "min" => { min = if let Ok(m) = value.as_str().parse::<NaiveDateTime>() { Some(NaiveDateTimeWrapper(m)) } else { None } },
                "max" => { max = if let Ok(m) = value.as_str().parse::<NaiveDateTime>() { Some(NaiveDateTimeWrapper(m)) } else { None } },
                _ => {},
            }
        }
        Ok(Search {
            limit,
            o,
            p,
            q,
            min,
            max,
        })
        // if &tag == ""  {
            // Err("No tag specified")
        // } else {
            // Ok( Tag{ tag } )
        // }
    }
}



impl<'f> FromForm<'f> for Tag {
    type Error = &'static str;
    fn from_form(form_items: &mut FormItems<'f>, _strict: bool) -> Result<Self, Self::Error> {
        let mut tag: String = String::new();
        for (field, value) in form_items {
            if field.as_str() == "tag" {
                tag = value.url_decode().expect("URL Decode fail."); 
            }
            // match field.as_str() {
            //     "aid" => { aid = value.to_string().parse::<u32>().unwrap_or(0u32) },
            //     _ => {},
            // }
        }
        if &tag == ""  {
            Err("No tag specified")
        } else {
            Ok( Tag{ tag } )
        }
    }
}

impl<'f> FromForm<'f> for ArticleForm {
    type Error = &'static str;
    
    fn from_form(form_items: &mut FormItems<'f>, _strict: bool) -> Result<Self, Self::Error> {
        
        // Author should be blank string here, when saving the author can be identified from cookies
        // this prevents user from altering the userid in submitted form data when using a hidden field to save the userid
        
        let mut title: String = String::new();
        let mut body: String = String::new();
        let mut tags: String = String::new();
        let mut description: String = String::new();
        
        for (field, value) in form_items {
            match field.as_str() {
                "title" => { title = sanitize_title(value.url_decode().expect("URL Decode failed")) },
                "body" => { body = sanitize_body(value.url_decode().expect("URL Decode failed")) },
                "tags" => { tags = sanitize_tags(value.url_decode().expect("URL Decode failed")) },
                "description" => { description = sanitize_body(value.url_decode().expect("URL Decode failed")) },
                _ => {},
            }
        }
        if title.len() > MAX_CREATE_TITLE {
            title = title[..MAX_CREATE_TITLE].to_string();
        }
        if tags.len() > MAX_CREATE_TAGS {
            tags = tags[..MAX_CREATE_TAGS].to_string();
        }
        if description.len() > MAX_CREATE_DESCRIPTION {
            description = description[..MAX_CREATE_DESCRIPTION].to_string();
        }
        if title == "" || body == "" {
            Err("Missing a required field.")
        } else {
            Ok( ArticleForm::new(title, body, tags, description) )
        }
    }
}

#[derive(Debug, Clone)]
pub struct NaiveDateTimeWrapper(pub NaiveDateTime);

impl<'v> FromFormValue<'v> for NaiveDateTimeWrapper {
    type Error = ();

    // fn from_form_value(form_value: &'v RawStr) -> Result<NaiveDateTime, &'v RawStr> {
    fn from_form_value(form_value: &'v RawStr) -> Result<NaiveDateTimeWrapper, ()> {
        let val = form_value.as_str();
        // match NaiveDateTime::from_str(val) {
        match val.parse::<NaiveDateTime>() {
            Ok(date) => Ok(NaiveDateTimeWrapper(date)),
            Err(e) => Err(()),
        }
    }
}




impl<'r> FromParam<'r> for ArticleId {
    type Error = &'r str;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        // https://api.rocket.rs/rocket/request/trait.FromParam.html
        // let (key, val_str) = match param.find(':') {
        //     Some(i) if i > 0 => (&param[..i], &param[(i + 1)..]),
        //     _ => return Err(param)
        // };

        // if !key.chars().all(|c| (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z')) {
        //     return Err(param);
        // }
        // if !key.chars().all(|c| (c >= '0' && c <= '9') {
        //     return Err(param);
        // }

        // val_str.parse().map(|value| {
        //     ArticleId {
        //         key: key,
        //     }
        // }).map_err(|_| param)
        
        match param.parse::<u32>() {
            Ok(i) => Ok(ArticleId { aid: i }),
            Err(e) => Err("Could not convert id.")
        }
    }
}


