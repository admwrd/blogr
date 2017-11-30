
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

/* Pagination
    Use Paginate to retrieve pagination parameters from a URI/query string
    Specify a zero-sized type that returns a PaginateSettings structure
        The ZST will define the default settings for the pagination
    Paginate<MyPaginateSettings>
    
    SQL - Selecting a subset of rows
    
    Pg Specific:
    SELECT * FROM articles ORDER BY posted DESC LIMIT 10 OFFSET 0
    SQL Standard:
    SELECT * FROM articles ORDER BY posted DESC OFFSET 5 ROWS FETCH NEXT 2 ROWS ONLY
    SELECT * FROM articles ORDER BY posted DESC FETCH FIRST 2 ROWS ONLY
    
*/




pub struct Paginate<T: PaginateSettings> {
    /// Page indicates which page is currently being viewed.
    /// Starts at 1 not 0.
    pub page: u32,
    pub route: String,
    pub settings: T,
}

// Switched names from PaginateSettings for the structure to using it for the trait name
//      intead of using PaginateDefaults for the trait name
//
// This structure is returned from a Request Guard and specifies the number of items
// per page.  The default ipp is used if the ipp is not found in the URI/query string
pub struct Pagination {
    /// Items per page
    pub ipp: u8,
    /// number of links to show before and after
    
    // the following are not retrieved in a query string or URI path so
    // rely on the trait methods to retrieve these values.
    //
    // pub nav_relative: u8, 
    // /// number of links to show from first and last page.  
    // /// Example: 3 would give  1 2 3  ... 8 9 10
    // pub nav_absolute: u8, 
    // /// Maximum number of items that can be on one page
    // pub max_ipp: u8,
    // /// Minimum number of items on a page
    // pub min_ipp: u8,
}


/// Specifies methods that should be available to paginate data structures.
pub trait Collate {
    
    // These should be overridden to return self.0.{var} in cases
    //     where the structure is a tuple-struct with a Pagination
    //     structure being the first/only item
    fn ipp(&self) -> u8 { Self::default_ipp() };
    // rel abs and base were moved to below (below were renamed as above)
    // fn rel(&self) -> u8 { Self::default_rel() };
    // fn abs(&self) -> u8 { Self::default_abs() };
    // fn base(&self) -> &'static str { BLOG_URL };
    fn min_ipp() -> u8 { 5 };
    fn max_ipp() -> u8 { 50 };
    /// Checks if the current items per page is within the max and min
    /// specified by the `Collate` trait implementation
    fn ipp_valid(&self) -> Option<u8> {
        if self.ipp() < Self::min_ipp() {
            Some( Self::min_ipp() )
        } else if self.ipp() > Self::max_ipp() {
            Some( Self::max_ipp() )
        } else {
            None
        }
    }
    
    // Default values
    /// Default items per page.
    /// The default implementation defaults to 20 items.
    fn default_ipp() -> u8 { 20 };
    /// Default number of relative page links.
    /// The default implementation defaults to 5 links.
    /// Example: rel_links = 2: 1 ... 5 6 7 8 9 ... 20
    fn rel() -> u8 { 5 };
    /// Default number of absolute page links, from the beginning and end.
    /// The default implementation defaults to 3 links.
    /// Example abs_links = 3: 1 2 3 ... 25 ... 48 49 50
    fn abs() -> u8 { 3 };
    
    /// Default link base, everything before the page.
    /// Example: localhost:8000/
    fn base() -> &'static str { BLOG_URL }
    
    /// Generate a link for the pagination navigation.
    /// The default implementation generates a link in the following form:
    /// {link_base}{route}?page={page}[&ipp=20]
    /// the ipp can be left out if it is the default value (ipp==default_ipp)
    fn gen_link(paginate: &Paginate, page: u32) -> String {
        let mut link = String::new();
        link.push_str( &paginate.route );
        
        let mut has_qrystr = false;
        if paginate.page != 1 {
            link.push_str("?page=");
            link.push_str( &paginate.page.to_string() );
            has_qrystr = true;
        }
        
        if paginate.settings.ipp() != paginate.settings.default_ipp() {
            if has_qrystr { link.push_str("&ipp="); }
            else { link.push_str("?ipp="); }
            link.push_str( &paginate.settings.ipp.to_string() );
        }
        
        link
    }
    
    fn gen_query(&self, qrystr: &str, orderby: Option<&str>) -> String {
        // add limit/offset or offset/fetch
        unimplemented!()
    }
    
    // Retrieves the Paginate<Pagination20> which holds the specified
    // PaginateSettings struct
    // fn req_guard() -> Option<Paginate<>> {
    //     
    // }
}

pub struct Pagination20(Pagination);

// impl PaginateSettings for Pagination20 {
//     fn ipp() -> u8 { 20 }
//     fn rel_links() -> u8 { 5 }
//     fn abs_links() -> u8 { 3 }
// }

impl Paginate {
    // page number
    pub fn get_page(&self) -> u32 {
        self.page
    }
    // page*ipp
    pub fn get_offset(&self) -> u32 {
        self.page * self.settings.
    }
    // items per page
    pub fn get_ipp(&self) -> u32 {
        
    }
    // change/set the page route for the nav links
    pub fn set_route(&mut self, route: &str) -> Self {
        self.route = route.to_string();
        self
    }
    
    // generate the navigation links html
    pub fn navigation(&self, total_items: u32) -> String {
        
    }
}


impl Pagination20 {
    // pub fn new()
}


// impl<'a, 'r> FromRequest<'a, 'r> for Paginate {
//     type Error = ();
    
//     fn from_request(request: &'a Request<'r>) -> ::rocket::request::Outcome<Paginate,Self::Error>{
//             let uri = request.uri();
//             
//             // Outcome::Success( page )
//             // Outcome::Forward(())
//     }
// }

