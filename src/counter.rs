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

pub const HITS_SAVE_INTERVAL: usize = 5;


pub fn cur_dir_file(name: &str) -> PathBuf {
    if let Ok(mut dir) = env::current_exe() {
        dir.pop();
        // println!("Climbing directory tree into: {}", &dir.display());
        dir.pop();
        // println!("Loading into directory: {}", &dir.display());
        dir.set_file_name(name);
        // println!("Load file is: {}", &dir.display());
        dir
    } else {
        PathBuf::from(name)
    }
}



#[derive(Debug)]
pub struct ViewsTotal(pub AtomicUsize);


#[derive(Debug, Serialize, Deserialize)]
pub struct PageCount {
    // pub count: Mutex<HashMap<String, usize>>,
    pub count: Mutex<PageStats>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PageGhost {
    pub count: HashMap<String, usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PageStats (pub HashMap<String, usize>);


// Implements a Request Guard to pull data into a route
// current page/route, page views, total site hits/views
#[derive(Debug, Clone, Serialize)]
pub struct Hits(pub String, pub usize, pub usize);



impl PageStats {
    pub fn serialize(&self) -> String {
        let ser: String = ::serde_json::to_string_pretty(self).expect("Could not serialize PageStats");
        ser
    }
    pub fn deserialize(mut buffer: String) -> Self {
        let des_rst: Self = ::serde_json::from_str(&mut buffer);
        if let Ok(des) = des_rst {
            des
        } else {
            println!("Deserialization failed for PageStats.");
            PageStats::new()
        }
    }
}


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
            // println!("\nSuccessfully loaded ViewsTotal.  Data:\n{}\n", &buf);
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
        // println!("\nSaving ViewsTotal.  Data:\n{}\n", ser);
        // println!("Saving ViewsTotal to: {}.  Data: {}", filename.display(), ser);
        let bytes = f.write(ser.as_bytes());
    }
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
            // println!("\nSuccessfully loaded PageCount data:\n{}\n", &buf);
            des
            
            // ViewsTotal( AtomicUsize::new(des) )
        } else {
            if let Ok(mut f) = File::create(&filename) {
                let new = PageCount::new();
                // println!("Saving blank PageCount");
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
        
        // let ser: String = ::serde_json::to_string_pretty(&self.count.lock().expect("Could not unlock count hashmap to serialize")).expect("Could not serialize PageCount.");
        // let ser: String = ::serde_json::to_string_pretty( &self.count.get_mut().expect("Could not unlock PageCount mutex to serialize") ).expect("Could not serialize PageCount.");
        // let ser: String = ::serde_json::to_string_pretty( &*(self.count.lock().expect("Could not unlock PageCount mutex to serialize")) ).expect("Could not serialize PageCount.");
        // let ser: String = ::serde_json::to_string_pretty(&*(self).count.lock().expect("unlock PageCount mutex serialization failed")).expect("Could not serialize PageCount.");
        // let ser: String = ::serde_json::to_string_pretty(&*(self).count.lock().expect("unlock PageCount mutex serialization failed")).expect("Could not serialize PageCount.");
        
        // let ser: String = ::serde_json::to_string_pretty(& PageGhost { count: &*(self).count.lock().expect("unlock PageCount mutex serialization failed") }  ).expect("Could not serialize PageCount.");
        
        // let ser: String = ::serde_json::to_string_pretty(& PageGhost { count: &*(self).count.lock().expect("unlock PageCount mutex serialization failed") }  ).expect("Could not serialize PageCount.");
        
        let ghostvalue = self.count.lock().expect("Could not unlock PageCount for serialization");
        let ghost = PageGhost{ count: ghostvalue.clone() };
        let ser: String = ::serde_json::to_string_pretty(&ghost).expect("Could not serialize PageCount");
        
        
        // println!("\nSaving PageCount.  Data:\n{}\n", ser);
        // println!("Saving PageCount to: {}.  Data: {}", filename.display(), ser);
        let bytes = f.write(ser.as_bytes());
    }
}

// Specialized Page Count
// fn reroute(page: &str, part: &str) -> String {
//     let mut output: String = String::with_capacity(page.len() + part.len() + 20);
//     output.push_str(page);
//     match page {
//         "article" | "author" | "search" | "tag" => {
//             output.push_str(part);
//         },
//         _ => {},
//     }
// }

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
            // // Specialized Page Count
            // if let Some(pos) = route[1..].find("/") {
            //     let (p, _) = route[1..].split_at(pos);
            //     // println!("Found route `{}`, splitting at {} to get `{}`", route, pos, p);
            //     // page = p;
            //     // pagestr = p.to_string();
            //     pagestr = reroute(p);
            //     page = &pagestr;
            // } else {
            //     // page = route.to_string();
            //     // println!("Found route: {}", route);
            //     page = route;
            //     pagestr = route.to_string();
            //     // pagestr = reroute(route);
            //     // page = &pagestr;
            // }
            
            // Non-specialized Page Count
            page = route;
            pagestr = route.to_string();
            
            // let hit_count_state = req.guard::<State<PageCount>>()?;
            
            // https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html
            let counter = req.guard::<State<PageCount>>()?;
            
            // Retrieve and increment ViewsTotal
            let views_state = req.guard::<State<ViewsTotal>>()?;
            let mut views = views_state.0.load(Ordering::Relaxed);
            views_state.0.store(views+1, Ordering::Relaxed);
            // views += 1;
            views.wrapping_add(1);
            
            // Retrieve hit count for the current route and increment
            // let mut hits = pages.entry(pagestr.clone()).or_insert(0);
            let hit: usize;
            // {
                let mut pages = counter.count.lock().unwrap();
                let mut hits;
                hits = pages.entry(pagestr.clone()).or_insert(0);
                (*hits).wrapping_add(1);
                hit = (*hits);
                // *hits += 1;
                
            // }
            
            
            // Every 100 page views save the stats
            // if (*hits) > 5 && (*hits) % HITS_SAVE_INTERVAL == 0 {
                // println!("Save interval reached, saving.");
                
                // ViewsTotal::save(*hits);
                // '''''
                // ViewsTotal::save(hit);
                // counter.save();
                // '''''
                // println!("Saving finished.");
            // }
            
            // Outcome::Success( Hits(pagestr, *hits, views) )
            Outcome::Success( Hits(pagestr, hit, views) )
    }
}


// pub struct UniqueCount {
//     pub unique: HashMap<>,
// }




