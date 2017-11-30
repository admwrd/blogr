
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
}

pub trait Collate {
    fn ipp(&self) -> u8;
    fn default_ipp() -> u8 { 20 }
    fn rel_links() -> u8 { 5 }
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
}


// impl Collate for Pagination {
    
// }



// impl<'a, 'r> FromRequest<'a, 'r> for Paginate {
//     type Error = ();
    
//     fn from_request(request: &'a Request<'r>) -> ::rocket::request::Outcome<Paginate,Self::Error>{
//             let uri = request.uri();
//             
//             // Outcome::Success( page )
//             // Outcome::Forward(())
//     }
// }
















