use rocket::Data;
use rocket::data::FromData;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Cookie, Cookies, MediaType, ContentType, Method, Status};
use rocket::Outcome;
use rocket::Outcome::Success;
use rocket::request::{FlashMessage, Form, FromForm, FormItems, FromRequest, Request};
use rocket::response::{self, Response, content, NamedFile, Redirect, Flash, Responder, Content};
use rocket::response::content::Html;
use rocket::State;
use rocket;

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

use htmlescape::*;

// specify how often the htis should be saved to disk (save every x hits)
const Hits_SAVE_INTERVAL: usize = 20;

// define which pages can have sub-pages that should be counted separately
const MULTI_SEGMENT_PATHS: &[&'static str] = &["article", "search", "tag"];

// Get the base directory of the program
pub fn cur_dir_file(name: &str) -> PathBuf {
    if let Ok(mut dir) = env::current_exe() {
        dir.pop();
        dir.pop();
        dir.set_file_name(name);
        dir
    } else {
        PathBuf::from(name)
    }
}

// Tracks the total number of hits the website recieves
#[derive(Debug)]
pub struct TotalHits {
    pub total: AtomicUsize,
}

// A serializable non-multithreaded version of TotalHits to allow the total number of hits the website has recieved to be stored on disk
#[derive(Debug, Serialize, Deserialize)]
pub struct TotalHitsSerde {
    pub total: usize,
}

// Tracks the number of hits each page gets
#[derive(Debug, Serialize, Deserialize)]
pub struct PageStats {
    pub map: HashMap<String, usize>,
}

// Wraps PageStats in a Mutex to allow multithreaded access to the individual page hits
#[derive(Debug, Serialize, Deserialize)]
pub struct Counter {
    pub stats: Mutex<PageStats>,
}

// Implements a Request Guard to pull data into a route
// current page/route, page views, total site hits/views
#[derive(Debug, Clone, Serialize)]
pub struct Hits(pub String, pub usize, pub usize);

// Use this for error pages to track errors
#[derive(Debug, Clone, Serialize)]
pub struct ErrorHits(pub String, pub usize, pub usize);

impl TotalHits {
    pub fn new() -> TotalHits {
        TotalHits { total: AtomicUsize::new(0) }
    }
    pub fn save(&self) {
        let filename = cur_dir_file("logs/total_views.json");
        
        let mut f = File::create(&filename)
            .expect("Could not create file for TotalHits.");
        
        let serdes = TotalHitsSerde { total: self.total.load(Ordering::Relaxed) };
        
        let ser: String = ::serde_json::to_string_pretty( &serdes )
            .expect("Could not serialize TotalHits.");
        
        let bytes = f.write( ser.as_bytes() );
    }
    pub fn load() -> Self {
        let filename = cur_dir_file("logs/total_views.json");
        let mut f_rst = File::open(&filename);
        if let Ok(mut f) = f_rst {
            let mut buffer: String = String::with_capacity(100);
            f.read_to_string(&mut buffer);
            
            let des: TotalHitsSerde = ::serde_json::from_str(&mut buffer)
                .expect("Could not deserialize TotalHits from file.");
            
            let out: TotalHits = TotalHits { total: AtomicUsize::new( des.total ) };
            
            out
        } else {
            let new = TotalHits::new();
            new.save();
            new
        }
    }
}

impl Counter {
    pub fn new() -> Counter {
        Counter { stats: Mutex::new( PageStats::new() ) }
    }
    pub fn save(buffer: &str) {
        let filename = cur_dir_file("logs/page_stats.json");
        
        let mut f = File::create(&filename)
            .expect("Could not create file for Counter.");
        
        let bytes = f.write( buffer.as_bytes() );
    }
    pub fn load() -> Counter {
        let filename = cur_dir_file("logs/page_stats.json");
        let mut f_rst = File::open(&filename);
        if let Ok(mut f) = f_rst {
            let mut buffer: String = String::with_capacity(1000);
            f.read_to_string(&mut buffer);
            
            let page_stats = PageStats::des(buffer);
            
            Counter {
                stats: Mutex::new( page_stats ),
            }
        } else {
            let new = PageStats::new();
            let buffer = new.ser();
            Counter::save(&buffer);
            Counter {
                stats: Mutex::new( new )
            }
        }
    }
}

impl PageStats {
    pub fn new() -> PageStats {
        PageStats { map: HashMap::new() }
    }
    pub fn ser(&self) -> String {
        let ser: String = ::serde_json::to_string_pretty(self)
            .expect("Could not serialize PageStats");
        ser
    }
    pub fn des(mut buffer: String) -> Self {
        let des_rst = ::serde_json::from_str(&mut buffer);
        if let Ok(des) = des_rst {
            des
        } else {
            println!("Deserialization failed for PageStats.");
            PageStats::new()
        }
    }
}

fn route<'a>(req: &Request) -> String {
    let uri = req.uri();
    let route = uri.path();
    let mut page: &str;
    
    if route == "/" {
        page = "/";
    } else if let Some(pos) = route[1..].find("/") {
        let (p, _) = route[1..].split_at(pos);
        if MULTI_SEGMENT_PATHS.contains(&p) {
            page = if &route[0..1]== "/" { &route[1..] } else { route };
        } else {
            page = p;
        }
    } else {
        page = if &route[0..1]== "/" { &route[1..] } else { route };
    }
    if page != "" { page.to_string() } else { route.to_string() }
}

fn req_guard(req: &Request, pagestr: String) -> ::rocket::request::Outcome<Hits,()> {
    let total_state = req.guard::<State<TotalHits>>()?;
    let mut total = total_state.total.load(Ordering::Relaxed);
    if total < usize::max_value() {
        total += 1;
    }
    total_state.total.store( total, Ordering::Relaxed );
    let page_views: usize;
    let ser_stats: String;
    {
        let counter = req.guard::<State<Counter>>()?;
        let mut stats = counter.stats.lock().expect("Could not unlock Counter stats mutex.");
        {
            let mut hits = stats.map.entry(pagestr.clone()).or_insert(0);
            if *hits < usize::max_value() {
                *hits += 1;
            }
            page_views = *hits;
        }
        ser_stats = stats.ser();
    }
    if total % 10 == 0 || &pagestr == "save-hits" {
        Counter::save(&ser_stats);
        total_state.save();
    }
    
    Outcome::Success( Hits(pagestr, page_views, total) )
}

// https://rocket.rs/guide/state/#within-guards
// https://api.rocket.rs/rocket/http/uri/struct.URI.html
// impl<'a, 'r> FromRequest<'a, 'r> for PageCount {
impl<'a, 'r> FromRequest<'a, 'r> for Hits {
    type Error = ();
    
    fn from_request(req: &'a Request<'r>) -> ::rocket::request::Outcome<Hits,Self::Error> {
        req_guard(req, route(req))
    }
}


impl ErrorHits {
    pub fn error404(req: &Request) -> Hits {
        let route = req.uri().path();
        let prepend = "error404";
        
        let mut uri: String = String::with_capacity(route.len() + prepend.len() + 8);
        uri.push_str(prepend);
        uri.push_str(route);
        
        let hits = req_guard(req, uri);
        if let Success(hitcount) = hits {
            hitcount
        } else {
            Hits(String::from("uError"), 0, 0)
        }
    }
    pub fn error500(req: &Request) -> Hits {
        let route = req.uri().path();
        let prepend = "error500";
        
        let mut uri: String = String::with_capacity(route.len() + prepend.len() + 8);
        uri.push_str(prepend);
        uri.push_str(route);
        
        let hits = req_guard(req, uri);
        if let Success(hitcount) = hits {
            hitcount
        } else {
            Hits(String::from("uError"), 0, 0)
        }
    }
    pub fn error(req: &Request) -> Hits {
                let route = req.uri().path();
        let prepend = "error";
        
        let mut uri: String = String::with_capacity(route.len() + prepend.len() + 8);
        uri.push_str(prepend);
        uri.push_str(route);
        
        let hits = req_guard(req, uri);
        if let Success(hitcount) = hits {
            hitcount
        } else {
            Hits(String::from("uError"), 0, 0)
        }
    }
}
