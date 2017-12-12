use rocket::Data;
use rocket::data::FromData;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Cookie, Cookies, MediaType, ContentType, Method, Status};
use rocket::Outcome;
use rocket::request::{FlashMessage, Form, FromForm, FormItems, FromRequest, Request};
use rocket::response::{self, Response, content, NamedFile, Redirect, Flash, Responder, Content};
use rocket::response::content::Html;
use rocket::State;

use std::mem;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::prelude::*;

use std::sync::{Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::net::Ipv4Addr;


/* Todo:
    Convert the PageCount HashMap from HashMap<String, usize> to HashMap<&str, usize>
        then in the from_request() delete the pagestr variable (which is also cloned, double bad)
*/



pub struct ViewsTotal(pub AtomicUsize);

impl ViewsTotal {
    pub fn new() -> ViewsTotal {
        ViewsTotal(AtomicUsize::new(0))
    }
}

// impl<'a, 'r> FromRequest<'a, 'r> for ViewsTotal {
//     type Error = ();
    
//     // fn from_request(req: &'a Request<'r>) -> ::rocket::request::Outcome<PageCount,Self::Error>{
//     fn from_request(req: &'a Request<'r>) -> ::rocket::request::Outcome<ViewsTotal,Self::Error>{
        
//     }
// }



pub struct PageCount {
    pub count: Mutex<HashMap<String, usize>>,
}

impl PageCount {
    pub fn new() -> PageCount {
        PageCount {
            count: Mutex::new(HashMap::new()),
        }
    }
}


// current page/route, page views, total site hits/views
pub struct Hits(pub String, pub usize, pub usize);



// https://rocket.rs/guide/state/#within-guards
// https://api.rocket.rs/rocket/http/uri/struct.URI.html
// impl<'a, 'r> FromRequest<'a, 'r> for PageCount {
impl<'a, 'r> FromRequest<'a, 'r> for Hits {
    type Error = ();
    
    // fn from_request(req: &'a Request<'r>) -> ::rocket::request::Outcome<PageCount,Self::Error>{
    fn from_request(req: &'a Request<'r>) -> ::rocket::request::Outcome<Hits,Self::Error>{
            let uri = req.uri();
            let route = uri.path();
            
            let page: &str;
            let pagestr: String;
            if let Some(pos) = route[1..].find("/") {
                let (p, _) = route[1..].split_at(pos);
                println!("Found route `{}`, splitting at {} to get `{}`", route, pos, p);
                page = p;
                pagestr = p.to_string();
            } else {
                // page = route.to_string();
                println!("Found route: {}", route);
                page = route;
                pagestr = route.to_string();
            }
            
            // let hit_count_state = req.guard::<State<PageCount>>()?;
            
            // https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html
            let counter = req.guard::<State<PageCount>>()?;
            let mut pages = counter.count.lock().unwrap();
            
            let views_state = req.guard::<State<ViewsTotal>>()?;
            let mut views = views_state.0.load(Ordering::Relaxed);
            views_state.0.store(views+1, Ordering::Relaxed);
            views += 1;
            
            // // Method 1 - and_modify() - Nightly Only
            // pages.entry(page)
            //    .and_modify(|p| { *p += 1 })
            //    .or_insert(1);
            
            // // Method 2
            // *pages.entry(page).or_insert(1) += 1;
            
            // Method 3
            let mut hits = pages.entry(pagestr.clone()).or_insert(0);
            *hits += 1;
            
            
            
            Outcome::Success( Hits(pagestr, *hits, views) )
    }
}


// pub struct UniqueCount {
//     pub unique: HashMap<>,
// }




