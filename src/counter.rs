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
use std::env;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::io::prelude::*;
use std::io::{Cursor, Read, Write};
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use ::serde::{Deserialize, Serialize};

use std::sync::{Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::net::Ipv4Addr;


/* Todo:
    Convert the PageCount HashMap from HashMap<String, usize> to HashMap<&str, usize>
        then in the from_request() delete the pagestr variable (which is also cloned, double bad)
*/




pub fn cur_dir_file(name: &str) -> PathBuf {
    if let Ok(mut dir) = env::current_exe() {
        dir.pop();
        println!("Climbing directory tree into: {}", &dir.display());
        dir.pop();
        println!("Loading into directory: {}", &dir.display());
        dir.set_file_name(name);
        println!("Load file is: {}", &dir.display());
        dir
    } else {
        PathBuf::from(name)
    }
}



#[derive(Debug)]
pub struct ViewsTotal(pub AtomicUsize);

impl ViewsTotal {
    pub fn new() -> ViewsTotal {
        ViewsTotal(AtomicUsize::new(0))
    }
    pub fn load() -> ViewsTotal {
        let filename = cur_dir_file("views_total.json");
        // let mut f = File::open(filename).expect("Could not load ViewsTotal file.");
        let mut f_rst = File::open(&filename);
        if let Ok(mut f) = f_rst {
            let mut buf: String = String::with_capacity(50);
            f.read_to_string(&mut buf);
            let des: usize = ::serde_json::from_str(&mut buf).unwrap_or(0);
            
            ViewsTotal( AtomicUsize::new(des) )
        } else {
            if let Ok(mut f) = File::create(&filename) {
                let bytes = f.write(0.to_string().as_bytes());
                ViewsTotal( AtomicUsize::new(0) )
            } else {
                ViewsTotal( AtomicUsize::new(0) )
            }
        }
    }
    pub fn save(views: usize) {
        let filename = cur_dir_file("views_total.json");
        let mut f = File::create(&filename).expect("Could not create ViewsTotal file.");
        
        let ser: String = ::serde_json::to_string_pretty(&views).expect("Could not serialize ViewsTotal.");
        println!("Saving ViewsTotal to: {}.  Data: {}", filename.display(), ser);
        let bytes = f.write(ser.as_bytes());
    }
}



#[derive(Debug, Serialize, Deserialize)]
pub struct PageCount {
    pub count: Mutex<HashMap<String, usize>>,
}

impl PageCount {
    pub fn new() -> PageCount {
        PageCount {
            count: Mutex::new(HashMap::new()),
        }
    }
    pub fn load() -> PageCount {
        let filename = cur_dir_file("page_count.json");
        // let mut f = File::open(filename).expect("Could not load ViewsTotal file.");
        let mut f_rst = File::open(&filename);
        if let Ok(mut f) = f_rst {
            let mut buf: String = String::with_capacity(1500);
            f.read_to_string(&mut buf);
            let des: PageCount = ::serde_json::from_str(&mut buf).unwrap_or( PageCount::new() );
            des
            
            // ViewsTotal( AtomicUsize::new(des) )
        } else {
            if let Ok(mut f) = File::create(&filename) {
                let new = PageCount::new();
                new.save();
                new
                // let mut buf: String = String::with_capacity(50);
                // les des: PageCount = serde_json::from_str(&mut)
                // let bytes = f.write("".to_string().as_bytes());
            } else {
                PageCount::new()
            }
        }
    }
    pub fn save(&self) {
        let filename = cur_dir_file("page_count.json");
        let mut f = File::create(&filename).expect("Could not create PageCount file.");
        
        let ser: String = ::serde_json::to_string_pretty(self).expect("Could not serialize PageCount.");
        println!("Saving PageCount to: {}.  Data: {}", filename.display(), ser);
        let bytes = f.write(ser.as_bytes());
    }
}


// current page/route, page views, total site hits/views
#[derive(Debug, Clone, Serialize)]
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
            // views += 1;
            views.wrapping_add(1);
            
            // // Method 1 - and_modify() - Nightly Only
            // pages.entry(page)
            //    .and_modify(|p| { *p += 1 })
            //    .or_insert(1);
            
            // // Method 2
            // *pages.entry(page).or_insert(1) += 1;
            
            // Method 3
            let mut hits = pages.entry(pagestr.clone()).or_insert(0);
            // *hits += 1;
            (*hits).wrapping_add(1);
            
            // let hit = *hits;
            // Every 100 page views save the stats
            if (*hits) % 100usize == 0 {
                
            }
            
            
            Outcome::Success( Hits(pagestr, *hits, views) )
    }
}


// pub struct UniqueCount {
//     pub unique: HashMap<>,
// }




