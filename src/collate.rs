
// use rocket::request::FromRequest;

use rocket::data::FromData;
use rocket::Data;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Cookie, Cookies, MediaType, ContentType, Method, Status};
use rocket::Outcome;
use rocket::request::{FlashMessage, Form, FromForm, FormItems, FromRequest, Request};
use rocket::response::content::Html;
use rocket::response::{self, Response, content, NamedFile, Redirect, Flash, Responder, Content};
use rocket::State;
// use rocket::{Request, Data, Outcome, Response};
use rocket_contrib::Template;

use regex::Regex;
use std::io::prelude::*;

use super::BLOG_URL;

/* SQL
    
    Pg Specific:
    SELECT * FROM articles ORDER BY posted DESC LIMIT 10 OFFSET 0
    
    SQL Standard:
    SELECT * FROM articles ORDER BY posted DESC OFFSET 5 ROWS FETCH NEXT 2 ROWS ONLY
    SELECT * FROM articles ORDER BY posted DESC FETCH FIRST 2 ROWS ONLY
    
*/

pub struct Page<T: Collate> {
    pub cur_page: u32,
    pub route: String,
    pub settings: T,
}

pub struct Pagination {
    pub ipp: u8
}

impl<T: Collate> Page<T> {

    pub fn sql(&self, query: &str, orderby: Option<&str>) -> String {
        // unimplemented!()
        let mut qrystr: String;
        if let Some(order) = orderby {
            // orderby text, plus offset/limit is 20 characters, plus 20 character extra buffer
            // better to have larger capacity than too little to avoid unnecessary allocations
            qrystr = String::with_capacity(query.len() + order.len() + 10 + 20 + 20);
            qrystr.push_str(query);
            qrystr.push_str(" ORDER BY ");
            qrystr.push_str(order);
            qrystr.push_str(" LIMIT ");
            qrystr.push_str( &self.settings.ipp().to_string() );
            qrystr.push_str(" OFFSET ");
            qrystr.push_str(&( self.cur_page * (self.settings.ipp()-1) as u32 ).to_string());
        } else {
            qrystr = String::with_capacity(query.len() + 20 + 20);
            qrystr.push_str(query);
            qrystr.push_str(" LIMIT ");
            qrystr.push_str( &self.settings.ipp().to_string() );
            qrystr.push_str(" OFFSET ");
            qrystr.push_str(&( self.cur_page * (self.settings.ipp()-1) as u32 ).to_string());
        }
        // let mut qrystr = String::with_capacity(qrystr.len() +  );
        
        qrystr
    }
    
    pub fn navigation(&self, total_items: u32) -> String {
        // self.settings
        // <a href="{base}{route}[?[page=x][[&]ipp=y]]">{page}</a>
        // let something = T::default_ipp();
        
        // 50/20 = 2
        let ipp = self.settings.ipp() as u32;
        // integer division rounds towards zero, so if it does not evenly divide add 1
        let num_pages = if total_items % ipp != 0 {
            (total_items / ipp) + 1
        } else {
            total_items / ipp
        };
        
        if num_pages == 1 {
            return T::link(&self, 1);
        }
        
        // abs ... 
        if num_pages <= (T::abs_links() + T::abs_links() + T::rel_links() + T::rel_links() + 1) as u32 {
            
        }
        
        
        
        // if self.cur_page < self.settings.abs_links() {
        //     if 
        // }
        
        String::new()
    }
}

pub trait Collate {
    fn new(u8) -> Self;
    fn ipp(&self) -> u8;
    fn default_ipp() -> u8 { 20 }
    // relative pages on each side of page
    fn rel_links() -> u8 { 3 }
    // number of pages to show from first and last page
    fn abs_links() -> u8 { 3 }
    fn link_base() -> &'static str { BLOG_URL }
    fn min_ipp() -> u8 { 5 }
    fn max_ipp() -> u8 { 50 }
    fn check_ipp(ipp: u8) -> u8 {
        if ipp < Self::min_ipp() {
            Self::min_ipp()
        } else if ipp > Self::max_ipp() {
            Self::max_ipp()
        } else {
            ipp
        }
    }
    fn link<T: Collate>(page: &Page<T>, page_num: u32) -> String {
        let mut link = String::new();
        link.push_str( &page.route );
        
        let mut has_qrystr = false;
        if page.cur_page > 1 {
            link.push_str("?page=");
            link.push_str( &page.cur_page.to_string() );
            has_qrystr = true;
        }
        
        if page.settings.ipp() != T::default_ipp() {
            if has_qrystr { link.push_str("&ipp="); }
            else          { link.push_str("?ipp="); }
            
            link.push_str( &page.settings.ipp().to_string() )
        }
        
        link
    }
    fn parse_uri(qrystr: &str, route: String) -> Page<Self> where Self: ::std::marker::Sized {
        
        let mut cur_page: u32 = 1;
        let mut ipp: u8 = Self::default_ipp();
        
        lazy_static! {
            static ref PARSE_QUERYSTRING: Regex = Regex::new(r#"^(?page=(?P<page>\d+))?&?(?:ipp=(?P<ipp>\d+))$"#).unwrap();
        }
        
        for cap in PARSE_QUERYSTRING.captures(qrystr) {
            if let Some(pg) = cap.name("page") {
                cur_page = pg.as_str().parse().unwrap_or(1);
            } else if let Some(ip) = cap.name("ipp") {
                ipp = ip.as_str().parse().unwrap_or(Self::default_ipp());
            }
        }
        
        Page {
            cur_page,
            route,
            settings: Self::new(ipp),
        }
    }
    
}


impl Collate for Pagination {
    fn new(ipp: u8) -> Self { Pagination { ipp } }
    fn ipp(&self) -> u8 { self.ipp }
}



impl<'a, 'r, T: Collate> FromRequest<'a, 'r> for Page<T> {
    type Error = ();
    
    fn from_request(request: &'a Request<'r>) -> ::rocket::request::Outcome<Page<T>,Self::Error>{
            let uri = request.uri();
            let route = uri.path();
            let query: &str;
            if let Some(qrystr) = uri.query() {
                Outcome::Success(T::parse_uri(qrystr, route.to_string()))
            } else {
                Outcome::Success(Page {
                    cur_page: 1,
                    route: route.to_string(),
                    settings: T::new(T::default_ipp()),
                })
            }
            // Outcome::Success( page )
            // Outcome::Forward(())
    }
}
















