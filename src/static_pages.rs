
#![feature(test)]
// #![feature(plugin)]
// #![feature(custom_derive)]
// #![plugin(rocket_codegen)]
// extern crate rocket;
// extern crate rocket_contrib;
// extern crate serde;
// #[macro_use] extern crate serde_json;
// // #[macro_use] extern crate serde_json;
// #[macro_use] extern crate serde_derive;
// extern crate serde_yaml;
// // extern crate rmp_serde as rmps;
// // #[macro_use] extern crate lazy_static;
// // extern crate regex;
// // extern crate comrak;
// extern crate libflate;
// extern crate brotli;
// extern crate twoway;
// extern crate comrak;

// extern crate test;
// extern crate zopfli;
// extern crate urlencoding;
// extern crate titlecase;
// extern crate htmlescape;

// use rocket::{Request, Data, Outcome, Response};
// use rocket::response::{content, NamedFile, Redirect, Flash, Responder, Content};
// use rocket::response::content::Html;
// use rocket::data::FromData;
// use rocket::request::{FlashMessage, Form, FromForm, FormItems, FromRequest};
// use rocket::http::{Cookie, Cookies, MediaType, ContentType, Status};
// use rocket::State;



// mod accept;
use twoway;

use super::{COMRAK_OPTIONS, BASE};
use accept::*;
use templates::TemplateMenu;

use std::fmt::Display;
use std::{env, str, thread};
use std::fs::{self, File, DirEntry};
use std::io::{self, Cursor, Read};
use std::path::{Path, PathBuf};
use std::time::{self, Instant, Duration};
use std::prelude::*;
use std::ffi::OsStr;
use std::collections::HashMap;
use std::sync::{Mutex, Arc};

// use rocket;
use rocket_contrib::Template;
use ::rocket::Request;
// use ::rocket::request::{FromRequest, FromForm, FormItems, FromFormValue, FromParam};
// use ::rocket::outcome::Outcome;
// use rocket::http::RawStr;
// use rocket_contrib::Template;
// use rocket::response::{content, NamedFile, Redirect, Flash};

use comrak::{markdown_to_html, ComrakOptions};

use ::serde::{Deserialize, Serialize};
use serde_json::{Value, Error};
// use ::rmps::{Deserializer, Serializer};
// use serde_json::Error;
// use regex::Regex;

// pub const COMRAK_OPTIONS: ComrakOptions = ComrakOptions {
//     hardbreaks: true,            // \n => <br>\n
//     width: 120usize,             
//     github_pre_lang: false,      
//     ext_strikethrough: true,     // hello ~world~ person.
//     ext_tagfilter: true,         // filters out certain html tags
//     ext_table: true,             // | a | b |\n|---|---|\n| c | d |
//     ext_autolink: true,          
//     ext_tasklist: true,          // * [x] Done\n* [ ] Not Done
//     ext_superscript: true,       // e = mc^2^
//     ext_header_ids: None,        // None / Some("some-id-prefix-".to_string())
//     ext_footnotes: true,         // Hi[^x]\n\n[^x]: A footnote here\n
// };




// pub const BLOG_URL: &'static str = "http://localhost:8000/";
// pub const BASE: &'static str = "http://localhost:8000";


pub const DEFAULT_TEMPLATE: &'static str = "static-default.html.hbs";

pub const SEPARATOR: &[u8] = b"
-----";





// impl<'a, 'r> FromRequest<'a, 'r> for StaticPage {
//     
// }

/*

my_route(..., StaticPage, ...)
    would need to 
        find the req.route()
        get the State
        find encoding


USE A RESPONDER TO GET A &Request

#[get("/content/<req_page>")]
my_route(..., req_page: String, state: State<PagesMutex>) {
    let mut pages = state.lock();
    
    if let Some(page) = pages.retrieve() {
        page.prepare(&mut pages, encoding)
        // prepare() produces an express instance
    } else {
        // Page not found
    }
    OR
    if let Some(page) = pages.requested(page, &mut pages, encoding) {
        page
    } else {
        // Page not found
    }
    Won't work because responder needs to be just a type no methods
    could be something like:
    // could use either StaticPage or StaticRequest
    let requested = StaticPage::new(page, encoding, &mut pages);
        StaticPage {
            page: page,
            encoding: encoding,
            pages: &mut pages,
        }
    requested
    
    the responder would then look for the requested page
    make sure to check for the page's existence before it gets to the responder
    
    NO THE FOLLOWING WILL **NOT** WORK:
        could have the route's return type Result<StaticPage, Redirect>
        then if the responder fails (the page requested doesn't exist)
        it can be forwarded to another page or something
        the responder could use the return type: Result<...>
    
    
}



*/





pub struct PagesMutex(pub RwLock<PagesContextMap>);
pub struct PagesCache(pub RwLock<PagesCacheMap>);


// ACTUALLY YES... you call respond_to() with a Request not Responder
//
// NO!  This should not be a FromRequest, it should be a Responder
// NO AGAIN!
//     NO feed the following info into it and get a context
//     NO then add a responder for the context to convert to Template/PageCached
//     

#[derive(Debug)]
pub struct StaticRequest<'a> {
    pub route: &'a str,
    pub encoding: AcceptCompression,
    // pub context: &PageContext,
    
}

#[derive(Debug)]
pub struct PagesContextMap {
    // pub size: u64, 
    pub pages: HashMap<String, PageContext>,
}

#[derive(Debug)]
pub struct PagesCacheMap {
    pub pages: HashMap<String, PageCached>,
}

#[derive(Debug)]
pub struct PageCached {
    // uses: u64,
    // pub size: u64,
    pub page: Template,
    pub gzip: Option<Vec<u8>>,
    pub br: Option<Vec<u8>>,
    pub deflate: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PageContext {
    pub uri: String,
    pub title: String,
    pub body: String,
    pub template: String,
    pub js: Option<String>,
    pub description: Option<String>,
    pub gentime: String,
    pub base_url: String,
    pub admin: bool,
    pub user: bool,
    pub menu: Option<Vec<TemplateMenu>>,
    pub menu_dropdown: Option<Vec<TemplateMenu>>,
    // pub info: TemplatePageInfo,
}

/// Used to retrieve html and metadata from the page
pub struct PageFormat {
    yaml: Vec<u8>,
    html: Vec<u8>,
}

// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// pub struct TemplateMenu {
//     #[serde(default)]
//     pub separator: bool,
//     #[serde(default)]
//     pub name: String,
//     #[serde(default)]
//     pub url: String,
//     #[serde(default)]
//     pub classes: String,
// }

// Used for the yaml deserialization method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageInfo {
    pub uri: String,
    pub title: String,
    pub template: String,
    #[serde(default)]
    pub markdown: bool,
    #[serde(default)]
    pub js: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

// #[derive(Debug, Clone, Serialize)]
// pub struct TemplatePageInfo {
//     pub title: String,
//     pub js: String,
//     pub gentime: String,
//     pub base_url: &'static str,
// }


impl PagesContextMap {
    pub fn new() -> Self {
        PagesContextMap {
            pages: HashMap::new(),
        }
    }
    pub fn load_all(dir: &str) -> Self {
        // Load all static pages in a directory
        // Iterate the directory looking for all .page files
        // Call PageContext::load(Path) on each file
        unimplemented!()
        
        
    }
    pub fn retrieve(&mut self, uri: &str, compression: Option<AcceptCompression>) -> Option<PageContext> {
        // if no item is found in the cache look in the file system
        // and add the new item to the cache
        unimplemented!()
        
        // let encoding = if let Some(enc) = compression {
        //     enc.preffered()
        // } else {
        //     CompressionEncoding::Uncompressed
        // };
        
        // let result = self.pages.get(uri);
        
        // if let Some(page) = result {
            
        //     match encoding {
        //         CompressionEncoding::Uncompressed => { result.page },
        //         CompressionEncoding:: => {},
                
        //     }
            
        // } else {
        //     // do not try to load a specific file from disk
        //     // no good way to convert uri to filename correctly, they could be named very differently
        //     // also could be a vulnerability to let any file be accessed unless secured
        //     // maybe call the load_all again?
        //     None
        // }
        
        
        
    }
}

impl PageContext {
    pub fn load(path: &Path) -> Result<Self, String> {
        // call PageFormat::get_file()
        // then PageFormat::get_parts()
        // then PageFormat::parse_metadata()
        
        let file_opt = PageFormat::get_file(path);
        if let Some(file) = file_opt {
                
            // let file: Vec<u8> = PageFormat::get_file(path);
            // let mut file: Vec<u8> = Vec::with_capacity(contents.len()+10);
            // file.extend_from_slice(contents);
            
            let parts_opt = PageFormat::get_parts(file);
            if let Some(parts) = parts_opt {
                // if print {
                    // println!("Yaml:\n`{:?}`\n\nHtml:\n`{:?}`", parts.yaml, parts.html);
                // }
                // Some(parts)
                // let context = parts.parse_metadata();
                // context
                if let Some(meta) = parts.parse_metadata() {
                    Ok(meta)
                } else {
                    Err(format!("Could not load metadata for: {}", path.display()))
                }
            } else {
                Err(format!("Failed to load parts of: {}.", path.display()))
            }
        } else {
            Err(format!("Could not load file: {} ", path.display()))
        }
    }
    // Not sure what render() was supposed to do really...
    // pub fn render(&self) -> PageContext {
    //     unimplemented!()
    // }
}

impl PageCached {
    pub fn send(&self) -> Template {
        unimplemented!()
    }
    
}




impl PageFormat {
    /// Reads a file into a byte vector
    pub fn get_file(path: &Path) -> Option<Vec<u8>> {
        if let Ok(mut file) = File::open(path) {
            if let Ok(metadata) = file.metadata() {
                let mut buffer: Vec<u8> = Vec::with_capacity((metadata.len() + 50) as usize);
                file.read_to_end(&mut buffer);
                Some(buffer)
            } else {
                None
            }
        } else {
            None
        }
    }
    
    /// Take a byte vector and convert it to metadata and html parts.
    pub fn get_parts(buffer: Vec<u8>) -> Option<Self> {
        let sep_pos = twoway::find_bytes(&buffer, SEPARATOR);
        
        if let Some(pos) = sep_pos {
            // println!("DEBUG: found separator at index: {}", pos);
            let start_at = pos + (SEPARATOR.len());
            
            // println!("DEBUG: starting search for html at index: {}", start_at);
            
            let html_start = twoway::find_bytes(&buffer[start_at..], b"
");
            if let Some(mut html_pos) = html_start {
                html_pos += start_at+1;
                
                // println!("DEBUG: found html at index: {}", html_pos);
                
                let parts = PageFormat {
                    yaml: buffer[..pos].to_vec(),
                    html: buffer[html_pos..].to_vec(),
                };
                
                Some(parts)
            } else {
                None
            }
        } else {
            None
        }
    }
    
    /// Deserializes a yaml byte vector into a PageContext
    pub fn parse_yaml(self) -> Option<PageContext> {
        let yaml_des: Result<PageInfo, _> = ::serde_yaml::from_slice(&self.yaml);
        
        if let Ok(info) = yaml_des {
            Some(info.to_context(self.html))
        } else if let Err(err) = yaml_des {
            println!("Error occurred converting yaml to PageContext:\n{}", err);
            None
        } else {
            None
        }
    }
    
    pub fn parse_metadata(self) -> Option<PageContext> {
        let mut pos = 0usize;
        let colon = b":";
        let newline = b"\n";
        
        let mut uri = String::new();
        let mut title = String::new();
        let mut template = String::new();
        let mut js = None;
        let mut description = None;
        let mut admin = false;
        let mut user = false;
        let mut menu: Option<Vec<TemplateMenu>> = None;
        let mut menu_dropdown: Option<Vec<TemplateMenu>> = None;
        let mut markdown = false;
        
        while let Some(end) = next_field(&self.yaml, pos) {
            // let end = e + pos;
            // println!("Searching for field separator @ {}..{}", pos, end);
            
            // field separator
            if let Some(f) = twoway::find_bytes(&self.yaml[pos..end], colon) {
                let fs = f + pos;
                // println!("Found field separator @ {fs}.  {pos} .. {fs}: .. {end}", fs=fs, pos=pos, end=end);
                
                let k = String::from_utf8_lossy(&self.yaml[pos..fs]).into_owned();
                let key = k.trim();
                // println!("Found key: `{}`, val: `{}`", key, String::from_utf8_lossy(&self.yaml[fs+1..end]));
                
                let val_range: &[u8] = &self.yaml[fs+1..end];
                
                match key {
                    "uri" => { uri = String::from_utf8_lossy(&self.yaml[fs+1..end]).into_owned().trim().to_owned(); },
                    "title" => { title = String::from_utf8_lossy(&self.yaml[fs+1..end]).into_owned().trim().to_owned(); },
                    "template" => { template = String::from_utf8_lossy(&self.yaml[fs+1..end]).into_owned().trim().to_owned(); },
                    "js" => { js = Some(String::from_utf8_lossy(&self.yaml[fs+1..end]).into_owned().trim().to_owned()); },
                    "description" => { description = Some(String::from_utf8_lossy(&self.yaml[fs+1..end]).into_owned().trim().to_owned()); },
                    "admin" | "administrator" => { admin = bytes_are_true(&self.yaml[fs+1..end], false); },
                    "user" | "logged_in" | "logged-in" => { user = bytes_are_true(&self.yaml[fs+1..end], false); },
                    "menu" => { menu = json_menu(&self.yaml[fs+1..end]); },
                    "menu-dropdown" | "dropdown-menu" | "dropdown" => { menu_dropdown = json_menu(&self.yaml[fs+1..end]); },
                    "markdown" => { markdown = bytes_are_true(val_range, false) },
                    _ => {},
                }
                
            } else {
                // println!("No field separator found.");
                // break;
            }
            pos = end+1;
            if pos >= self.yaml.len() {
                // println!("Reached end, breaking...");
                break;
            }
        }
        
        if &uri != ""
        && &title != "" {
            Some(PageContext {
                uri,
                title,
                template: if &template != "" { template } else { DEFAULT_TEMPLATE.to_owned() },
                js,
                description,
                body: {
                    // String::from_utf8_lossy(&self.html).into_owned().trim().to_owned()
                    let body = String::from_utf8_lossy(&self.html).into_owned();
                    if markdown {
                        // let cr_options = ComrakOptions { ext_header_ids: Some("section-".to_string()), .. COMRAK_OPTIONS };
                         let html: String = markdown_to_html(&body, &COMRAK_OPTIONS);
                         html
                    } else {
                        body
                    }
                },
                gentime: String::new(),
                base_url: String::new(),
                admin,
                user,
                menu,
                menu_dropdown,
            })
        } else {
            // println!("Required fields missing for PageContext:\nuri: `{}`\ntitle: `{}`", &uri, &title);
            None
        }
    }
}

/// Takes a byte vector and converts to a TemplateMenu vector.
pub fn json_menu(json: &[u8]) -> Option<Vec<TemplateMenu>> {
    // Some(String::from_utf8_lossy(&self.yaml[fs+1..end]).into_owned().trim().to_owned());
    let des: Result<Vec<TemplateMenu>, _> = ::serde_json::from_slice(json);
    
    if let Ok(d) = des {
        Some(d)
    } else if let Err(e) = des {
        println!("Error deserializing the json menu:\n{:?}", e);
        None
    } else {
        println!("Error :(");
        None
    }
}

pub fn bytes_are_true(bytes: &[u8], default: bool) -> bool {
    let mut pos: usize = 0;
    for b in bytes.iter() {
        // look for first non-space (32 in ascii decimal)
        if *b != 32u8 {
            break;
        }
        pos += 1;
    }
    if &bytes[pos..pos+4] == b"true" 
    || &bytes[pos..pos+3] == b"yes" 
    || &bytes[pos..pos+4] == b"Yes" 
    || &bytes[pos..pos+4] == b"True" 
    || &bytes[pos..pos+2] == b"on" 
    || &bytes[pos..pos+2] == b"On" {
        !default
    } else { 
        default
    }
}

/// Find next newline character until the end is reached.
/// The final line of data will not contain a linebreak but 
/// must still be processed.
pub fn next_field(yaml: &Vec<u8>, pos: usize) -> Option<usize> {
    let newline = b"
";
    if let Some(end) = twoway::find_bytes(&yaml[pos..yaml.len()], newline) {
        Some(end + pos)
    } else {
        if pos < yaml.len() {
            Some(yaml.len())
        } else {
            None
        }
    }
}

impl PageInfo {
    /// Takes metadata and body contents and creates a context for the Template.
    pub fn to_context(self, html: Vec<u8>) -> PageContext {
        let context = PageContext {
            uri: self.uri,
            title: self.title,
            body: {
                let body = String::from_utf8_lossy(&html).into_owned();
                if self.markdown {
                    let cr_options = ComrakOptions { ext_header_ids: Some("section-".to_string()), .. COMRAK_OPTIONS };
                     let html: String = markdown_to_html(&body, &cr_options);
                     html
                } else {
                    body
                }
            },
            template: self.template,
            js: self.js,
            description: self.description,
            admin: false,           // Default values, this is only used for the yaml deserialization and not used with any actual menus
            user: false,            // Default values, this is only used for the yaml deserialization and not used with any actual menus
            menu: None,             // Default values, this is only used for the yaml deserialization and not used with any actual menus
            menu_dropdown: None,    // Default values, this is only used for the yaml deserialization and not used with any actual menus
            gentime: String::with_capacity(200),
            base_url: String::with_capacity(200),
        };
        context
    }
}















pub fn test_parts(print: bool) -> Option<PageFormat> {
        let contents = b"title: This is the title
uri: testing123
template: rust-code
description: This page is a test of the emergency warning system.  Beeeeeeep.
-----
<h1>Hello!</h1>
<p>Hi friends this is a test</p>
<p>This is super interesting</p>
<p>So keep reading because this just gets more interesting</p>
<p>You wouldn't want to miss any awesome dummy text would you?</p>";
    
    let mut file: Vec<u8> = Vec::with_capacity(contents.len()+10);
    file.extend_from_slice(contents);
    
    let parts_opt = PageFormat::get_parts(file);
    if let Some(parts) = parts_opt {
        if print {
            println!("Yaml:\n`{:?}`\n\nHtml:\n`{:?}`", parts.yaml, parts.html);
        }
        Some(parts)
    } else {
        if print {
            println!("Failed.");
        }
        None
    }
}


// menu: [{separator: false, name: \"home\", url: \"/\", classes: \"\"}, {separator: true, name: \"\", url: \"\", classes: \"\"}, {separator: false, name: \"about\", url: \"/about\", classes: \"\"}]
// menu: [{separator: false, name: "home", url: "/", classes: ""}, {separator: true, name: "", url: "", classes: ""}, {separator: false, name: "about", url: "/about", classes: ""}]
// menu: [{separator: false, name: "home", url: "/", classes: ""}]


// menu: [{"name": "home", "url": "/", "separator": true, "classes": ""}, {"name": "", "url": "", "separator": true, "classes": ""}, {"name": "about", "url": "/about", "separator": true, "classes": ""}]


pub fn test_parts2(print: bool) -> Option<PageFormat> {
        let contents = br#"title: This is the title
uri: testing123
template: rust-code
description: This page is a test of the emergency warning system.  Beeeeeeep.
menu: [{"name": "home", "url": "/", "separator": false}, {"separator": true}, {"name": "about", "url": "/about"}]
dropdown: []
user: true
admin: true
-----
<h1>Hello!</h1>
<p>Hi friends this is a test</p>
<p>This is super interesting</p>
<p>So keep reading because this just gets more interesting</p>
<p>You wouldn't want to miss any awesome dummy text would you?</p>"#;
    
    let mut file: Vec<u8> = Vec::with_capacity(contents.len()+10);
    file.extend_from_slice(contents);
    
    let parts_opt = PageFormat::get_parts(file);
    if let Some(parts) = parts_opt {
        if print {
            println!("Yaml:\n`{:?}`\n\nHtml:\n`{:?}`", parts.yaml, parts.html);
        }
        Some(parts)
    } else {
        if print {
            println!("Failed.");
        }
        None
    }
}

pub fn test_context_yaml(print: bool) -> Option<PageContext> {
    let parts_opt = test_parts(print);
    if let Some(parts) = parts_opt {
        let context = parts.parse_yaml();
        if print {
            println!("Context:\n{:?}", context);
        }
        context
    } else {
        if print {
            println!("Could not parse yaml into PageContext.");
        }
        None
    }
}


pub fn test_context_twoway(print: bool) -> Option<PageContext> {
    let parts_opt = test_parts(print);
    if let Some(parts) = parts_opt {
        let context = parts.parse_metadata();
        if print {
            println!("Context:\n{:?}", context);
        }
        context
    } else {
        if print {
            println!("Could not parse yaml into PageContext.");
        }
        None
    }
}

pub fn test_context_twoway2(print: bool) -> Option<PageContext> {
    let parts_opt = test_parts2(print);
    if let Some(parts) = parts_opt {
        let context = parts.parse_metadata();
        if print {
            println!("Context:\n{:?}", context);
        }
        context
    } else {
        if print {
            println!("Could not parse yaml into PageContext.");
        }
        None
    }
}

fn main() {
    println!("End Result:\n`{:?}`", test_context_twoway2(false));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    
    fn correct_context(rst: Option<PageContext>, v2: bool) -> bool {
        let html = "<h1>Hello!</h1>
<p>Hi friends this is a test</p>
<p>This is super interesting</p>
<p>So keep reading because this just gets more interesting</p>
<p>You wouldn't want to miss any awesome dummy text would you?</p>";
        
        let title = "This is the title";
        let uri = "testing123";
        let template = "rust-code";
        let description = Some("This page is a test of the emergency warning system.  Beeeeeeep.".to_owned());
        let body = html.to_owned();
        let js = None;
        let gentime = String::new();
        let base_url = String::new();
        let menu: Option<Vec<TemplateMenu>>;
        let menu_dropdown: Option<Vec<TemplateMenu>>;
        let admin: bool;
        let user: bool;
        if v2 == true {
            menu = Some(vec![
                TemplateMenu {
                    separator: false,
                    name: "home".to_owned(),
                    url: "/".to_owned(),
                    classes: "".to_owned(),
                },
                TemplateMenu {
                    separator: true,
                    name: "".to_owned(),
                    url: "".to_owned(),
                    classes: "".to_owned(),
                },
                TemplateMenu {
                    separator: false,
                    name: "about".to_owned(),
                    url: "/about".to_owned(),
                    classes: "".to_owned(),
                },
            ]);
            menu_dropdown = Some(Vec::new());
            admin = true;
            user = true;
        } else {
            menu = None;
            menu_dropdown = None;
            admin = true;
            user = true;
        };
        
        if let Some(result) = rst {
            assert_eq!(title,       result.title);
            assert_eq!(uri,         result.uri);
            assert_eq!(body,        result.body);
            assert_eq!(template,    result.template);
            assert_eq!(js,          result.js);
            assert_eq!(description, result.description);
            assert_eq!(menu,        result.menu);
            assert_eq!(menu_dropdown,result.menu_dropdown);
            assert_eq!("",          &result.gentime);
            assert_eq!("",          &result.base_url);
            true
        } else {
            // panic!("No result.");
            false
        }
    }
    
    #[test]
    fn test_yaml_context() {
        assert!(
            correct_context( test_context_yaml(false), false )
        );
    }
    
    
    #[test]
    fn test_twoway_context() {
        assert!(
            correct_context( test_context_twoway(false), false )
        );
    }
    
     #[test]
    fn test_twoway_context2() {
        assert!(
            correct_context( test_context_twoway2(false), true )
        );
    }
    
    
    
    
    
    
    
    
    #[bench]
    fn bench_html_yaml_parts(b: &mut Bencher) {
        b.iter(|| test_parts(false))
    }
    
    
    #[bench]
    fn bench_context_yaml(b: &mut Bencher) {
        b.iter(|| test_context_yaml(false))
    }
    
    #[bench]
    fn bench_context_twoway(b: &mut Bencher) {
        b.iter(|| test_context_twoway(false))
    }
    
        #[bench]
    fn bench_context_twoway2(b: &mut Bencher) {
        b.iter(|| test_context_twoway2(false))
    }
    
    
    
}
