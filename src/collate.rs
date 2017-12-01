
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

#[derive(Debug, Clone)]
pub struct Page<T: Collate> {
    pub cur_page: u32,
    pub route: String,
    pub settings: T,
}

// Can also make a custom structure based on this that
// implements custom methods for the Collate trait
// in order to ovveride defaults with custom values
#[derive(Debug, Clone)]
pub struct Pagination {
    pub ipp: u8
}

fn link<T: Collate>(page: &Page<T>, cur_page: u32, text: &str) -> String {
    // let url = T::link(page, cur_page-1);
    let url = T::link(page, cur_page);
    // <a href="" class="active"></a>
    // <a href=""></a>
    let mut link = String::with_capacity(url.len() + text.len() + 15 + 10);
    link.push_str("<a href=\"");
    link.push_str(&url);
    link.push_str("\">");
    link.push_str(text);
    link.push_str("</a>");
    link
}
fn link_active<T: Collate>(page: &Page<T>, cur_page: u32, text: &str) -> String {
    let url = T::link(page, cur_page-1);
    // <a href="" class="active"></a>
    // <a href=""></a>
    let mut link = String::with_capacity(url.len() + text.len() + 30 + 20);
    link.push_str(" <a href=\"");
    link.push_str(&url);
    link.push_str("\" class=\"active\">");
    link.push_str(text);
    link.push_str("</a> ");
    link
}


impl<T: Collate> Page<T> {
    
    
    /* 0 1 2 3 4    5 6 7 8 9    10 11 12 13 14 */
    /// Returns the index number of the first item on the page.
    /// If there are 5 items per page and the current page is 3
    /// then the start() would return 10
    pub fn start(&self) -> u32 {
        // (cur_page - 1) * ipp
        (self.cur_page - 1) * (self.settings.ipp() as u32)
    }
    
    /// Returns the index number of the last item on the page.
    /// If there are 5 items per page and the current page is 3
    /// then the start() would return 14
    pub fn end(&self) -> u32 {
        // (cur_page * ipp) - 1
        (self.cur_page * (self.settings.ipp() as u32)) - 1
    }
    
    
    pub fn num_pages(&self, total_items: u32) -> u32 {
        let ipp = self.settings.ipp() as u32;
        if total_items % ipp != 0 {
            (total_items / ipp) + 1
        } else {
            total_items / ipp
        }
    }
    
    pub fn sql(&self, query: &str, orderby: Option<&str>) -> String {
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
            // qrystr.push_str(&( self.cur_page * (self.settings.ipp()-1) as u32 ).to_string());
            qrystr.push_str(&( self.start() ).to_string());
        } else {
            qrystr = String::with_capacity(query.len() + 20 + 20);
            qrystr.push_str(query);
            qrystr.push_str(" LIMIT ");
            qrystr.push_str( &self.settings.ipp().to_string() );
            qrystr.push_str(" OFFSET ");
            // qrystr.push_str(&( self.cur_page * (self.settings.ipp()-1) as u32 ).to_string());
            qrystr.push_str(&( self.start() ).to_string());
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
            // return T::link(&self, 1);
            return link(&self, 1, "1");
        }
        
        
        // 4       8
        // 0100 -> 1000
        //
        
        // let abs = T::abs_links();
        // let rel = T::rel_links();
        // let links_min = abs as u32 + abs as u32 + rel as u32 + rel as u32 + 1u32;
        let abs = T::abs_links() as u32;
        let rel = T::rel_links() as u32;
        let links_min = abs + rel;
        
        // let links_total = (abs << 2) + (rel << 2) + 1;
        // let links_total_original = abs + abs + rel + rel + 1;
        // assert_eq!(links_total, links_total_original);
        
        let cur = if self.cur_page > num_pages { 
            num_pages 
        } else if self.cur_page == 0 {
            1
        } else { 
            self.cur_page
        };
        
        // abs ... rel cur rel ... abs
        // add padding
        
        // rel left items
        let mut rel_left = rel;
        let mut rel_right = rel;
        let mut abs_left = abs;
        let mut abs_right = abs;
        
        let mut max = if abs > rel { abs } else { rel };
        
        // 1 2 3     4 5 6 7 8  9  10 11 12 13 14 15 16 17 18 19 20
        // 1 2 3 ...     6 7 8 [9] 10 11 12            ... 18 19 20
        
        // 1 2 3 4 5 6 7
        // 1 2 3 4 5 6 7
        let first_page = link(&self, 1, "First Page");
        
        let mut pages_left: Vec<u32> = Vec::new();
        let mut pages_right: Vec<u32> = Vec::new();
        let mut front: (Vec<u32>, Vec<u32>) = (Vec::new(), Vec::new());
        let mut back: (Vec<u32>, Vec<u32>) = (Vec::new(), Vec::new());
        
        
        // print left side
        if cur <= links_min || num_pages <= links_min {
            if links_min > num_pages {
                // prints everything but last page, which is taken care of in the middle
                // pages_left = (1..num_pages).map(|p| p ).collect();
                pages_left = (1..num_pages).collect();
            } else {
                // pages_left = (1..cur).map(|p| p).collect();
                
                pages_left = (1..cur).collect();
            }
        } else {
            // 1.. 3[4]    7.. 9[10]
            // 1 2 3 4 5 6 7 8 9 [10] 11 12 13 14 15 16 17 18 19 20
            // pages_left = (1..abs+1)
            //     // .map(|p| p)
            //     .chain((cur-rel)..cur)
            //     .collect();
            front = ( (1..abs+1).collect(), ((cur-rel)..cur).collect() );
        }
        
        // 1 2 3 4 5 6 7 8 9 [10] 11 12 13 14 15 16 17 18 19 20
        // print right side
        // if cur >= (num_pages-)
        let right = num_pages - cur;
        if right <= links_min {
            pages_right = (cur+1..num_pages+1).collect();
        } else {
            // pages_right = (cur+1..rel+1)
            //     .chain((num_pages-abs+1)..num_pages+1)
            back = ( (cur+1..rel+1).collect(), ((num_pages-abs+1)..num_pages+1).collect() );
        }
        
        let count_left = if pages_left.len() != 0 {
            pages_left.len()
        } else {
            front.0.len() + front.1.len()
        };
        let count_right = if pages_right.len() != 0 {
            pages_right.len()
        } else {
            back.0.len() + back.1.len()
        };
        
        // <a href="">10</a> + http://localhost:8000/pagination?page=10&ipp=20
        // 65 characters roughly for each link
        // Add 70 chars per link, plus 150 for previous/next links, plus 70 for the 
        // current link plus buffer of 100 characters to avoid extra allocations
        let mut html: String = String::with_capacity((count_left + count_right) * 70 + 150 +70 + 100);
        
        println!(r"Pagination Debug:
            cur: {}, ipp: {}, num_pages: {},
            abs: {}, rel: {}, links_min: {},
            pages_left: {:?},
            front: {:?},
            pages_right: {:?}
            back: {:?}
            ", 
            cur, ipp, num_pages, 
            abs, rel, links_min,
            pages_left,
            front,
            pages_right,
            back
            );
        
        if pages_left.len() != 0 {
            for page in pages_left {
                html.push_str( &link(&self, page, &page.to_string()) );
            }
            // print previous page link
            // print link to all pages in the vector
        } else if front.0.len() !=0 || front.1.len() != 0 {
            for page in front.0 {
                html.push_str( &link(&self, page, &page.to_string()) );
            }
            html.push_str(" ... ");
            for page in front.1 {
                html.push_str( &link(&self, page, &page.to_string()) );
            }
        }
        
        html.push_str( &link_active(&self, cur, &cur.to_string()) );
        
        if pages_right.len() != 0 {
            // print next page link
            // print link to all pages in the vector
            for page in pages_right {
                html.push_str( &link(&self, page, &page.to_string()) );
            }
        } else if back.0.len() != 0 || back.1.len() != 0 {
            for page in back.0 {
                html.push_str( &link(&self, page, &page.to_string()) );
            }
            html.push_str(" ... ");
            for page in back.1 {
                html.push_str( &link(&self, page, &page.to_string()) );
            }
        }
        
        html.shrink_to_fit();
        html
        
        
        
        
        // if cur_page != 1 {
        //     // display first link
        //     let url = T::link(self, cur_page-1);
        //     // <a href="">Previous</a>
        //     let mut link = String::with_capacity(url.len() + 23 + 10);
            
        // }
        
        
        // only add ... if there is a gap of more than {padding} pages
        // keep the +1 ???
        // if cur <= rel+1 {
        // if cur <= rel {
        //     let start_front = cur - 1;  // TODO - fix this equation
        //     let start_back = rel - cur; // TODO - fix this equation
        //     // let end = num_pages - rel + 1; // Removed - forgetting absolutes for now
            
            
        // } else if cur >= (num_pages-rel) {
        //     let end_front = ;
        //     let end_back = ;
        //     // let start = ; // Removed - forgetting absolutes for now
            
            
            
        // } else {
        //     // first page ... rels page 
        //     // 
            
        // }
        
        // if cur_page != num_pages && num_pages > 1 {
        //     // display next link
        // }
        
        
        
        
        
        
        
        
        
        
        
        
        
        
        // if cur < max {
        //     rel_front = 
        //     rel_left = max - cur;
        //     abs_left = 0;
        // } else {
        //     if cur > abs {
        //         rel_left = rel;
                
        //     } else {
                
        //     }
        // }
        
        
        
        
        
        
        // let padding = T::links_padding();
        // // add lenience to num_pages so that if there is ONE (or so) more page
        // // more than the links_min it still shows all the pages.
        // if num_pages + padding <= links_min {
        //     if cur == 1 {
                
        //     } else if cur < abs {
                 
                
                
        //     } else if cur == num_pages {
                
        //     } else if cur > (num_pages - max) {
                
        //     }
            
        //     // show all links between start and end
        //     // highlight current link
        // } else if self.cur_page < abs {
        //     // print first abs and last abs
        //     // current page should fall in the first abs, highlight it
            
        // } else if self.cur_page >= num_pages - abs {
        //     // print first abs and last abs
        //     // current page should fall in the last abs, highlight it
            
        // } else if  {
        //     // print first abs pages
        //     // print rel links before and after
        //     // print last abs pages
        // }
        
        
        
        // if self.cur_page < self.settings.abs_links() {
        //     if 
        // }
        
        // String::new()
    }
}

pub trait Collate {
    fn new(u8) -> Self;
    fn ipp(&self) -> u8;
    fn default_ipp() -> u8 { 20 }
    /// relative pages on each side of page
    fn rel_links() -> u8 { 3 }
    /// number of pages to show from first and last page
    fn abs_links() -> u8 { 3 }
    /// max number of pages above the min page links before not all pages are shown
    fn links_padding() -> u8 { 4 }
    fn link_base() -> &'static str { BLOG_URL }
    fn min_ipp() -> u8 { 5 }
    fn max_ipp() -> u8 { 50 }
    fn current_link() -> &'static str { "active" }
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
        if page_num > 1 {
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
















