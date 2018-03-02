use rocket::data::FromData;
use rocket::Data;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Cookie, Cookies, MediaType, ContentType, Method, Status};
use rocket::Outcome;
use rocket::request::{FlashMessage, Form, FromForm, FormItems, FromRequest, Request};
use rocket::response::content::Html;
use rocket::response::{self, Response, content, NamedFile, Redirect, Flash, Responder, Content};
use rocket::State;
use rocket_contrib::Template;
use rocket::response::content::Html;

use regex::Regex;
use std::io::prelude::*;

// BLOG_URL must have a trailing slash
const BLOG_UR: &'static str = "https://your_domain.com/";

// Indicates whether the app is running in production mode (assumes HTTPS) or not
const PRODUCTION: bool = false;

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
    let url = T::link(page, cur_page);
    let mut link = String::with_capacity(url.len() + text.len() + 70 + 10);
    // Define the html for each page item
    link.push_str(" <li class=\"page-item\"><a class=\"page-link\" href=\"");
    link.push_str(&url);
    link.push_str("\">");
    link.push_str(text);
    link.push_str("</a></li> ");
    link
}
fn link_active<T: Collate>(page: &Page<T>, cur_page: u32, text: &str) -> String {
    let url = T::link(page, cur_page);
    let mut link = String::with_capacity(url.len() + text.len() + 30 + 20);
    // Define html for each page item that is active
    let mut link = String::with_capacity(url.len() + text.len() + 120 + 20);
    link.push_str(" <li class=\"page-item active\"><a class=\"page-link\" href=\"");
    link.push_str(&url);
    link.push_str("\">");
    link.push_str(text);
    link.push_str("<span class=\"sr-only\">(current)</span></a></li> ");
    link
}


impl<T: Collate> Page<T> {
    /// Returns the index number of the first item on the page.
    /// If there are 5 items per page and the current page is 3
    /// then the start() would return 10
    pub fn start(&self) -> u32 {
        (self.cur_page - 1) * (self.settings.ipp() as u32)
    }
    
    /// Returns the index number of the last item on the page.
    /// If there are 5 items per page and the current page is 3
    /// then the start() would return 14
    pub fn end(&self) -> u32 {
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
            qrystr = String::with_capacity(query.len() + order.len() + 10 + 20 + 20);
            qrystr.push_str(query);
            qrystr.push_str(" ORDER BY ");
            qrystr.push_str(order);
            qrystr.push_str(" LIMIT ");
            qrystr.push_str( &self.settings.ipp().to_string() );
            qrystr.push_str(" OFFSET ");
            qrystr.push_str(&( self.start() ).to_string());
        } else {
            qrystr = String::with_capacity(query.len() + 20 + 20);
            qrystr.push_str(query);
            qrystr.push_str(" LIMIT ");
            qrystr.push_str( &self.settings.ipp().to_string() );
            qrystr.push_str(" OFFSET ");
            qrystr.push_str(&( self.start() ).to_string());
        }
        qrystr
    }
    pub fn page_info(&self, total_items: u32) -> String {
        let (ipp, cur, pages) = self.page_data(total_items);
        format!("Showing page {cur} of {pages} &nbsp; - &nbsp; {total} items found.", 
            cur=cur, pages=pages, total=total_items)
        
    }
    /// Returns the items per page, page number, and number of pages
    pub fn page_data(&self, total_items: u32) -> (u8, u32, u32) {
        let ipp8 = self.settings.ipp();
        let ipp = ipp8 as u32;
        let num_pages = if total_items % ipp != 0 {
            (total_items / ipp) + 1
        } else {
            total_items / ipp
        };
        
        let cur = if self.cur_page > num_pages { 
            num_pages 
        } else if self.cur_page == 0 {
            1
        } else { 
            self.cur_page
        };
        (ipp8, cur, num_pages)
    }
    
    pub fn navigation(&self, total_items: u32) -> String {
        let ipp = self.settings.ipp() as u32;
        // integer division rounds towards zero, so if it does not evenly divide add 1
        let num_pages = if total_items % ipp != 0 {
            (total_items / ipp) + 1
        } else {
            total_items / ipp
        };
        
        let abs = T::abs_links() as u32;
        let rel = T::rel_links() as u32;
        let links_min = abs + rel;
        
        let cur = if self.cur_page > num_pages { 
            num_pages 
        } else if self.cur_page == 0 {
            1
        } else { 
            self.cur_page
        };
        
        let mut pages_left: Vec<u32> = Vec::new();
        let mut pages_right: Vec<u32> = Vec::new();
        let mut front: (Vec<u32>, Vec<u32>) = (Vec::new(), Vec::new());
        let mut back: (Vec<u32>, Vec<u32>) = (Vec::new(), Vec::new());
        
        // print left side
        if cur <= links_min || num_pages <= links_min {
            pages_left = (1..cur).collect();
        } else {
            front = ( (1..abs+1).collect(), ((cur-rel)..cur).collect() );
        }
        
        // print right side
        let right = num_pages - cur;
        if right <= links_min {
            pages_right = (cur+1..num_pages+1).collect();
        } else {
            back = ( (cur+1..cur+rel+1).collect(), ((num_pages-abs+1)..num_pages+1).collect() );
        }
        
        // count links
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
        
        let html_capacity = (count_left + count_right) * 70 + 150 +70 + 100;
        let mut html: String = String::with_capacity(html_capacity);
        
        html.push_str(r#"<div class="row"><div class="v-pagnav-before col"></div><div class="v-pagnav-nav col"><nav><ul class="pagination">"#);
        
        if cur != 1 {
            html.push_str( &link(&self, cur-1, "Previous") );
        } else {
            html.push_str(r#"<li class="page-item disabled"><span class="page-link">Previous</span></li>"#);
        }
        if pages_left.len() != 0 {
            for page in pages_left {
                html.push_str( &link(&self, page, &page.to_string()) );
            }
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
        if html.capacity() != html_capacity { println!("0 Capacity has changed from {} to: {}", html_capacity, html.capacity()); }
        
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
        if cur != num_pages {
            html.push_str( &link(&self, cur+1, "Next") );
        } else {
            html.push_str(r#"<li class="page-item disabled"><span class="page-link">Next</span></li>"#);
        }
        
        // Define the Items Per Page selection box
        html.push_str( &format!(r##"</ul></nav></div>
<div class="v-pagnav-after col"> 
<form action="{}{}" method="GET" class="ipp-form">
<input type="hidden" class="ipp-total-items" value="{}">
<input type="hidden" class="ipp-curpage" name="page" value="{}"># Articles 
<select name="ipp" class="pagination-ipp" id="pagination-ipp" size="1">"##, &BLOG_URL[..BLOG_URL.len()-1], &self.route, total_items, self.cur_page));
        if T::min_ipp() <= 5 && T::max_ipp() >= 5 { if self.settings.ipp() == 5 { html.push_str(r#"<option value="5" SELECTED>5</option>"#); } else { html.push_str(r#"<option value="5">5</option>"#); } }
        if T::min_ipp() <= 6 && T::max_ipp() >= 6 { if self.settings.ipp() == 6 { html.push_str(r#"<option value="6" SELECTED>6</option>"#); } else { html.push_str(r#"<option value="6">6</option>"#); } }
        if T::min_ipp() <= 8 && T::max_ipp() >= 8 { if self.settings.ipp() == 8 { html.push_str(r#"<option value="8" SELECTED>8</option>"#); } else { html.push_str(r#"<option value="8">8</option>"#); } }
        if T::min_ipp() <= 10 && T::max_ipp() >= 10 { if self.settings.ipp() == 10 { html.push_str(r#"<option value="10" SELECTED>10</option>"#); } else { html.push_str(r#"<option value="10">10</option>"#); } }
        if T::min_ipp() <= 15 && T::max_ipp() >= 15 { if self.settings.ipp() == 15 { html.push_str(r#"<option value="15" SELECTED>15</option>"#); } else { html.push_str(r#"<option value="15">15</option>"#); } }
        if T::min_ipp() <= 20 && T::max_ipp() >= 20 { if self.settings.ipp() == 20 { html.push_str(r#"<option value="20" SELECTED>20</option>"#); } else { html.push_str(r#"<option value="20">20</option>"#); } }
        if T::min_ipp() <= 25 && T::max_ipp() >= 25 { if self.settings.ipp() == 25 { html.push_str(r#"<option value="25" SELECTED>25</option>"#); } else { html.push_str(r#"<option value="25">25</option>"#); } }
        if T::min_ipp() <= 30 && T::max_ipp() >= 30 { if self.settings.ipp() == 30 { html.push_str(r#"<option value="30" SELECTED>30</option>"#); } else { html.push_str(r#"<option value="30">30</option>"#); } }
        if T::min_ipp() <= 35 && T::max_ipp() >= 35 { if self.settings.ipp() == 35 { html.push_str(r#"<option value="35" SELECTED>35</option>"#); } else { html.push_str(r#"<option value="35">35</option>"#); } }
        if T::min_ipp() <= 40 && T::max_ipp() >= 40 { if self.settings.ipp() == 40 { html.push_str(r#"<option value="40" SELECTED>40</option>"#); } else { html.push_str(r#"<option value="40">40</option>"#); } }
        if T::min_ipp() <= 50 && T::max_ipp() >= 50 { if self.settings.ipp() == 50 { html.push_str(r#"<option value="50" SELECTED>50</option>"#); } else { html.push_str(r#"<option value="50">50</option>"#); } }
        html.push_str(r##"
</select>
</form>
</div>
</div>"##);
        
        // Just a debug notice to let you know you didn't allocate enough space for the string initially
        // This is just a performance thing, not really even that important
        if PRODUCTION && html.capacity() != html_capacity { println!("1 Capacity has changed from {} to: {}", html_capacity, html.capacity()); }
        
        html.shrink_to_fit();
        html
    }
}

pub trait Collate {
    fn new(u8) -> Self;
    fn ipp(&self) -> u8;
    fn default_ipp() -> u8 { 10 }
    /// relative pages on each side of page
    fn rel_links() -> u8 { 3 }
    /// number of pages to show from first and last page
    fn abs_links() -> u8 { 3 }
    /// max number of pages above the min page links before not all pages are shown
    fn links_padding() -> u8 { 4 }
    fn link_base() -> &'static str { BLOG_URL }
    fn min_ipp() -> u8 { 5 }
    fn max_ipp() -> u8 { 50 }
    fn current_link() -> &'static str { "v-collate-cur" } // active is bootstrap's default
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
        link.push_str( &BLOG_URL[..BLOG_URL.len()-1] );
        link.push_str( &page.route );
        
        let mut has_qrystr = false;
        if page_num > 1 {
            link.push_str("?page=");
            link.push_str( &page_num.to_string() );
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
        
        for pair in qrystr.split('&') {
            let chunks: Vec<&str> = pair.splitn(2, '=').collect();
            if chunks.len() != 2 { continue; }
            let key = chunks[0];
            let value = chunks[1];
                match key {
                    "page" => { cur_page = value.parse().unwrap_or(1); },
                    "ipp" => { ipp = value.parse().unwrap_or(Self::default_ipp()); },
                    _ => {},
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
    }
}

