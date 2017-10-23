
use rocket;
use ::rocket::request::{FromRequest, FromForm, FormItems};
use ::rocket::Request;
use ::rocket::outcome::Outcome;
use ::rocket::config::{Config, Environment};

use titlecase::titlecase;
use regex::Regex;
use chrono::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};

use postgres::{Connection};

use users::*;
use data::*;


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
}


#[derive(Debug, Clone)]
// #[derive(Debug, Clone, Insertable)]
// #[table_name="posts"]
pub struct ArticleForm {
    // pub userid: u32,
    pub title: String,
    pub body: String,
    pub tags: String,
}

pub struct Search {
    pub q: String,
}

#[derive(FromForm)]
pub struct ViewPage {
    pub page: u32,
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
}

impl ArticleForm {
    pub fn to_article(&self) -> Article {
        // get next aid
        let next_aid = 0;
        Article {
            aid: next_aid,
            title: sanitize_title(self.title.clone()),
            posted: Local::now().naive_local(), // This fn is only used when saving new articles
            body: sanitize_body(self.body.clone()),
            tags: split_tags(sanitize_tags(self.tags.clone())),
        }
    }
}


impl ArticleId {
    pub fn exists(&self) -> bool {
        unimplemented!()
    }
    pub fn retrieve(&self) -> Option<Article> {
        // unimplemented!()
        let pgconn = establish_connection();
        let rawqry = pgconn.query(&format!("SELECT aid, title, posted, body, tags FROM articles WHERE aid = {id}", id=self.aid), &[]);
        if let Ok(aqry) = rawqry {
            println!("Querying articles: found {} rows", aqry.len());
            if !aqry.is_empty() && aqry.len() == 1 {
                let row = aqry.get(0); // get first row
                Some( Article {
                    aid: row.get(0),
                    title: row.get(1), // todo: call sanitize title here
                    posted: row.get(2),
                    body: row.get(3), // Todo: call sanitize body here
                    tags: Article::split_tags(row.get(4)),
                })
            } else { None }
        } else { None }
    }
    pub fn retrieve_with_conn(&self, pgconn: DbConn) -> Option<Article> {
        // unimplemented!()
        // let pgconn = establish_connection();
        let rawqry = pgconn.query(&format!("SELECT aid, title, posted, body, tags FROM articles WHERE aid = {id}", id=self.aid), &[]);
        if let Ok(aqry) = rawqry {
            println!("Querying articles: found {} rows", aqry.len());
            if !aqry.is_empty() && aqry.len() == 1 {
                let row = aqry.get(0); // get first row
                Some( Article {
                    aid: row.get(0),
                    title: row.get(1), // todo: call sanitize title here
                    posted: row.get(2),
                    body: row.get(3), // Todo: call sanitize body here
                    tags: Article::split_tags(row.get(4)),
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

pub fn sanitize_body(string: String) -> String {
    // escape html entities/elements
    // unimplemented!()
    string
}

pub fn sanitize_title(string: String) -> String {
    // set max length to 120 characters
    string
    // unimplemented!()
}

pub fn sanitize_tags(string: String) -> String {
    string
    // unimplemented!()
}
pub fn split_tags(string: String) -> Vec<String> {
    let tags: Vec<String> = string.split(',').filter(|t| t != &"").map(|t| t.to_string()).collect();
    tags
}

pub fn get_len<T>(input: &Option<Vec<T>>) -> usize {
    if let &Some(ref inner) = input {
        inner.len()
    } else {
        0
    }
}

impl Article {
    pub fn split_tags(string: String) -> Vec<String> {
        // Todo: call sanitize tags before splitting:
        // let tags: Vec<String> = sanitize_tags(string).split(',')...
        let tags: Vec<String> = string.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| s.as_str() != "" && s.as_str() != " ")
            .collect();
        tags
    }
    pub fn retrieve(aid: u32) -> Option<Article> {
        // unimplemented!()
        let pgconn = establish_connection();
        let rawqry = pgconn.query(&format!("SELECT aid, title, posted, body, tags FROM articles WHERE aid = {id}", id=aid), &[]);
        if let Ok(aqry) = rawqry {
            // println!("Querying articles: found {} rows", aqry.len());
            if !aqry.is_empty() && aqry.len() == 1 {
                let row = aqry.get(0); // get first row
                Some( Article {
                    aid: row.get(0),
                    title: row.get(1), // todo: call sanitize title here
                    posted: row.get(2),
                    body: row.get(3), // Todo: call sanitize body here
                    tags: Article::split_tags(row.get(4)),
                })
            } else { None }
        } else { None }
    }
    pub fn info(&self) -> String {
        format!("Aid: {aid}, Title: {title}, Posted on: {posted}, Body:<br>\n{body}<br>\ntags: {tags:#?}", aid=self.aid, title=self.title, posted=self.posted, body=self.body, tags=self.tags)
    }
    
    pub fn retrieve_all(pgconn: DbConn, limit: u32, description: Option<u32>, min_date: Option<NaiveDate>, max_date: Option<NaiveDate>, tag: Option<Vec<String>>, search: Option<Vec<String>>) -> Vec<Article> {
        let mut qrystr: String = if let Some(summary) = description {
            format!("SELECT aid, title, posted, LEFT(body, {}) AS body, tags FROM articles", summary)
        } else {
            String::from("SELECT aid, title, posted, body, tags FROM articles")
        };
        // let mut qrystr: String = String::from("SELECT aid, title, posted, body, tags FROM articles");
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
                    tag_str.push_str( &format!(" tags LIKE '%{}%'", t) );
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
        
        if limit != 0 { qrystr.push_str(&format!(" LIMIT {}", limit)); }
        println!("Query: {}", qrystr);
        let qryrst = pgconn.query(&qrystr, &[]);
        // if let Ok(aqry) = qryrst {
        //     println!("Query found {} rows", aqry.len());
        //     if !aqry.is_empty() && aqry.len() != 0 {
                
        //     }
        // }
        if let Ok(result) = qryrst {
            let mut articles: Vec<Article> = Vec::new();
            for row in &result {
                let a = Article {
                    aid: row.get(0),
                    title: row.get(1),
                    posted: row.get(2),
                    body: row.get(3),
                    tags: split_tags(row.get(4)),
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
    pub fn new(title: String, body: String, tags: String) -> ArticleForm {
        ArticleForm {
            // userid: 0,
            title,
            body,
            tags,
        }
    }
    // Removed userid from ArticleForm and no userid exists in Article
    // pub fn author(self, userid: u32) -> ArticleForm {
    //     let new: ArticleForm = self.clone();
    //     ArticleForm {
    //         userid,
    //         .. new
    //     }
    // }
    pub fn save(&self, conn: &DbConn) -> Result<Article, String> {
        // unimplemented!()
        let now = Local::now().naive_local();
        // return both id and posted date
        // let qrystr = format!("INSERT INTO blog (aid, title, posted, body, tags) VALUES ('', '{title}', '{posted}', '{body}', {tags}) RETURNING aid, posted",
        let qrystr = format!("INSERT INTO articles (title, posted, body, tags) VALUES ('{title}', '{posted}', '{body}', '{tags}') RETURNING aid",
            title=self.title, posted=now, body=self.body, tags=self.tags);
        // let rawqry = conn.prepare(&qrystr).expect("Could not prepare query successfully");
        // println!("Insert query: {}", qrystr);
        let result = conn.query(&qrystr, &[]);
        match result {
            // Ok(ref qry) if qry.is_empty() => Err("Error inserting article, result is empty.".to_string()),
            // Ok(ref qry) if !qry.is_empty() && qry.len() > 1 => Err(format!("Error inserting article, returned {} rows.", qry.len())),
            Err(err) => Err(format!("Could not insert article. Error: {}", err)),
            // Ok(qry) if !qry.is_empty() && qry.len() == 1 => {
            Ok(qry)  => {
                if !qry.is_empty() && qry.len() == 1 {
                    let row = qry.get(0);
                    Ok( Article {
                        aid: row.get(0),
                        title: self.title.clone(),
                        // posted: row.get(1),
                        posted: now,
                        body: self.body.clone(),
                        tags: Article::split_tags(self.tags.clone()),
                    })
                } else if qry.is_empty() {
                    Err("Error inserting article, result is empty.".to_string())
                } else if qry.len() > 1 {
                    Err(format!("Error inserting article, returned {} rows.", qry.len()))
                } else {
                    Err("Unknown error inserting article.".to_string())
                }
            },
            // Ok(_) => Err("Unknown error inserting article.".to_string()),
        }
        // let id: i32 = stmt.query(&[])
            // .expect("Died for this reason").().iter().next().unwrap().get(0);
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
        
        for (field, value) in form_items {
            match field.as_str() {
                "title" => { title = sanitize_title(value.url_decode().expect("URL Decode failed")) },
                "body" => { body = sanitize_body(value.url_decode().expect("URL Decode failed")) },
                "tags" => { tags = sanitize_tags(value.url_decode().expect("URL Decode failed")) },
                _ => {},
            }
        }
        if title == "" || body == "" {
            Err("Missing a required field.")
        } else {
            Ok( ArticleForm::new(title, body, tags) )
        }
    }
}

