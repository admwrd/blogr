
use super::{BLOG_URL, COMRAK_OPTIONS, BASE, DEFAULT_PAGE_TEMPLATE, PAGE_TEMPLATES};
use accept::*;
// use static_pages::*;
use templates::TemplateMenu;
use xpress::*;


use std::fmt::Display;
use std::{env, str, thread};
use std::fs::{self, File, DirEntry};
use std::io::prelude::*;
use std::io::{self, Cursor, Read};
use std::path::{Path, PathBuf};
use std::time::{self, Instant, Duration};
use std::prelude::*;
use std::ffi::OsStr;
use std::collections::HashMap;
use std::sync::{Mutex, Arc, RwLock};
use std::sync::atomic::AtomicUsize;
// use rocket;
use rocket::http::Status;
use rocket::State;
use rocket_contrib::Template;
use rocket::response::{self, Response, Responder};
use rocket::request::{FromRequest, Request};
use rocket::Outcome;
use rocket::Outcome::Success;
use rocket::response::NamedFile;
use rocket::http::{ContentType, Header, HeaderMap};
// use ::rocket::request::{FromRequest, FromForm, FormItems, FromFormValue, FromParam};
// use ::rocket::outcome::Outcome;
// use rocket::http::RawStr;
// use rocket::response::{content, NamedFile, Redirect, Flash};

use comrak::{markdown_to_html, ComrakOptions};
use twoway;
use brotli;
use libflate::gzip;
use libflate::deflate;

use ::serde::{Deserialize, Serialize};
use serde_json::{Value, Error};



// pub const DEFAULT_TEMPLATE: &'static str = "static-default.html.hbs";

pub const SEPARATOR: &[u8] = b"
-----";


// How to load all the pages and get the contents of the Templates on app start?
//   it requires a Request either through a FromRequest or a Responder
//   - maybe pass the PageContext and other info to a Responder
//      Responder:
//          encoding
//          &ContentCacheLock (don't need &mut as the AtomicUsize and RwLock have interior mutability)
//          
//          
//          
//      maybe add a Template in the ContentContext because the Template can be used without a Responder directly
//      then use the cache structure to store the bytes and compressed bytes generated by the respond_to
//      only store the bytes or compressed versions when actually used
//          or maybe: only generate a ContentCached when used but generate all compressed versions at same time
//      
//      
//      

pub struct ContentContext {
    // pub pages: RwLock<HashMap<String, ContentCached>>,
    pub pages: HashMap<String, PageContext>,
    pub size: AtomicUsize,
}

pub struct ContentCacheLock {
    pub pages: RwLock<HashMap<String, ContentCached>>,
    pub size: AtomicUsize,
}

// pub struct ContentRequest<'c, 'u> {
// pub struct ContentRequest<'c, 'p, 'u> {
//     pub encoding: AcceptCompression,
//     pub cache: &'c ContentCacheLock,
//     // pub contexts: &'c ContentContext,
//     pub route: &'u str,
//     pub context: &'p PageContext,
// }
pub struct ContentRequest {
    pub encoding: AcceptCompression,
    // pub cache: ContentCacheLock,
    pub route: String,
    // pub start: GenTimer,
    // pub context: PageContext,
    // pub contexts: &'c ContentContext,
}
// pub struct ContentCacheMap {
//     pub pages: HashMap<String, ContentCached>,
//     // pub size: AtomicUsize,
//     // pub size: u64,
// }

#[derive(Debug, Clone)]
pub struct ContentCached {
    // uses: u64,
    // pub size: u64, // size of uncomprsesed
    // pub total: u64, // total size of uncompressed plus all compressed versions
    // pub gzip: Option<Vec<u8>>,
    // pub br: Option<Vec<u8>>,
    // pub deflate: Option<Vec<u8>>,
    pub page: Vec<u8>,
    pub gzip: Vec<u8>,
    pub br: Vec<u8>,
    pub deflate: Vec<u8>,
    pub size: usize,
}

// pub struct PageCached
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







impl ContentContext {
    pub fn load(dir: &str) -> ContentContext {
        // unimplemented!()
        
        let dir_iter = fs::read_dir(dir);
        if let Ok(dir) = dir_iter {
            let mut size = 0;
            let mut pages: HashMap<String, PageContext> = HashMap::new();
            
            for file_rst in dir {
                if let Ok(file) = file_rst {
                    if let Ok(file_type) = file.file_type() {
                        if !file_type.is_file() {
                            continue;
                        }
                    } else {
                        // if no file type can be found skip the file
                        continue;
                    }
                    let name = file.file_name().to_string_lossy().into_owned();
                    if !name.ends_with(".page") {
                        continue;
                    }
                    
                    let path = file.path();
                    
                    // let loaded = ::static_pages::PageContext::load(&path);
                    let loaded = PageContext::load(&path);
                    if let Ok(ctx) = loaded {
                        size += ctx.body.len();
                        pages.insert(ctx.uri.clone(), ctx);
                    } else if let Err(err_msg) = loaded {
                        println!("Error loading page {}: {}", name, err_msg);
                    } else {
                        println!("Unknown error");
                    }
                }
            }
            
            ContentContext {
                pages,
                size: AtomicUsize::new(size),
            }
            
        } else {
            ContentContext {
                pages: HashMap::new(),
                size: AtomicUsize::new(0),
            }
        }
        
    }
    
    // The retrieve() method does not appear to be needed
    // pub fn retrieve(&self, uri: &str) -> Option<&PageContext> {
    //     unimplemented!()
    // }
}







impl ContentCacheLock {
    // Must start with an empty cache and fill it in as the pages are requested
    //   this is because the data inside a Template is hard to get to and 
    //   in this case uses a Responder to extract the contents
    pub fn new() -> ContentCacheLock {
        ContentCacheLock {
            pages: RwLock::new( HashMap::new() ),
            size: AtomicUsize::new( 0 ),
        }
    }
    
    // pub fn retrieve() -> 
    
    
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
        // let mut template = String::new();
        let mut template = DEFAULT_PAGE_TEMPLATE.to_owned();
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
        
        
        
        let mut title_ok = false;
        for temp in PAGE_TEMPLATES {
            if &title == temp {
                title_ok = true;
            }
        }
        
        
        if &uri != ""
        && &title != "" {
            Some(PageContext {
                uri,
                title: if title_ok { title } else { DEFAULT_PAGE_TEMPLATE.to_owned() },
                template: if &template != "" { template } else { DEFAULT_PAGE_TEMPLATE.to_owned() },
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
                base_url: BLOG_URL.to_owned(),
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







// impl<'a, 'c, 'u> Responder<'a> for ContentRequest<'c, 'u> {
impl<'a> Responder<'a> for ContentRequest 
{
    fn respond_to(self, req: &Request) -> response::Result<'a> 
    {
        // get body contents then build an Express instance
        
        // look for the uri entry in self.cache
        // if exists use self.cache.pages.read() to read from the RwLock in the ContentCache
        //   and pull the compression method/original (specified by encoding.preferred()) from the cache
        // if not then create the new cache entry
        
        // let cache: Result<_, _>;
        
        //     cache = self.cache.pages.read();
        //     if let Ok(cache) = self.cache.pages.read() {
        //         // cache_entry = cache.get(self.route);
        //         // cache_entry = cache.get(self.route).map(|r| *r);
        //         cache_entry = cache.get(self.route);
        //     } else {
        //         cache_entry = None;
        //     }
        // }
        
        // Replacing self.cache and self.context
        
        // DEBUG PRINT - println!("Responding to static page: {}", &self.route);
        
        let context_state = req.guard::<State<ContentContext>>().unwrap();
        let cache_state = req.guard::<State<ContentCacheLock>>().unwrap();
        
        
        
        /*  1. Check for existence of uri in cache map (the content page route already checks for existence of page context for the given uri)
            2. If uri is not in cache map look for it in the context map (has to be in there but double check - don't use unwrap())
                   Add combined size of ContentCached fields to ContentCacheLock's size field
                       Use a checked add so the size never overflows, it just reaches a max value
            
        */
        
        
        // ----- ----- ----- ----- ----- ----- ----- ----- ----- ----- ----- -----
        // To improve performance: instead of cloning the byte vector:
        //   instead of body_bytes make the match'd contents a reference
        //   make a String::new() that is converted .into() an Express instance
        //   then do xresp.streamed_body( Cursor::new(body_bytes_reference) )
        // ----- ----- ----- ----- ----- ----- ----- ----- ----- ----- ----- -----
        
        // let cache_map = content_cache.cache.pages.read().unwrap();
        {
            let cache_map = cache_state.pages.read().unwrap();
            let cache_uri_opt = cache_map.get(&self.route);
            // let mut output_contents: Vec<u8> = Vec::new();
            
            if let Some(cache_uri) = cache_uri_opt {
                // DEBUG PRINT - 
                println!("Page exists in cache");
                
                let mut body_bytes = match self.encoding.preferred() {
                    Uncompressed => { cache_uri.page.clone() },
                    Brotli => { cache_uri.br.clone() },
                    Gzip => { cache_uri.gzip.clone() },
                    Deflate => { cache_uri.deflate.clone() },
                };
                let express: Express = body_bytes.into();
                return express.respond_to(req)
            }
        }
            // DEBUG PRINT - 
            println!("Page not found in cache, generating cache for page");
            
            if let Some(ctx) = context_state.pages.get(&self.route) {
                
                // DEBUG PRINT - println!("Retrieved context");
                
                let template: Template = Template::render( (&ctx.template).to_owned(), &ctx );
                let express: Express = template.into();
                let mut resp = express.respond_to(req).unwrap_or_default();
                let mut output_contents: Vec<u8> = Vec::new();
                let mut new_cache = ContentCached {
                    page: Vec::new(),
                    gzip: Vec::new(),
                    br: Vec::new(),
                    deflate: Vec::new(),
                    size: 0usize,
                };
                if let Some(body) = resp.body_bytes() {
                    output_contents = body;
                    
                    // DEBUG PRINT - print!("Generating compressed versions.. ");
                    
                    let gzip: Vec<u8>;
                    {
                        let mut buffer = Vec::with_capacity(output_contents.len() + 200);
                        let mut gzip_encoder = gzip::Encoder::new(buffer).unwrap();
                        gzip_encoder.write_all(&output_contents).expect("hi gzip"); // .expect("Gzip compression failed.");
                        gzip = gzip_encoder.finish().into_result().unwrap_or(Vec::new());
                    }
                    
                    let br: Vec<u8>;
                    {
                        let length = output_contents.len()+200;
                        let mut buffer = Vec::with_capacity(length);
                        // let mut compressor = ::brotli::CompressorReader::new(Cursor::new(data), 10*1024, 9, 22);
                        let mut compressor = ::brotli::CompressorReader::new(Cursor::new(&output_contents), length, 9, 22);
                        let _ = compressor.read_to_end(&mut buffer);
                        br = buffer;
                    }
                    
                    let deflate: Vec<u8>;
                    {
                        let mut buffer = Vec::with_capacity(output_contents.len()+200);
                        let mut encoder = deflate::Encoder::new(buffer);
                        encoder.write_all(&output_contents); //.expect("Deflate compression failed.");
                        deflate = encoder.finish().into_result().unwrap_or(Vec::new());
                        
                    }
                    
                    // DEBUG PRINT - print!(" Finished! Compressed versions of page have been generated.\n");
                    
                    // resp.set_streamed_body(  Cursor::new( output_contents )  );
                    
                    // Find the best compression algorithm for the client
                    let mut supported = 0u8;
                    let headers = req.headers();
                    if let Some(encoding) = headers.get("Accept-Encoding").next() {
                        if encoding.contains("gzip") { supported |= ::accept::GZIP; }
                        if encoding.contains("deflate") { supported |= ::accept::DEFLATE; }
                        if encoding.contains("br") { supported |= ::accept::BROTLI; }
                    }
                    // let accepted = AcceptCompression { supported };
                    let accepted = AcceptCompression::new(supported);
                    let compression = accepted.preferred();
                    // Set the correct version of the contents based on best supported compression algorithm
                    let bytes = match compression {
                        CompressionEncoding::Brotli => { 
                            resp.set_raw_header("Content-Encoding", "br");
                            br.clone() 
                        },
                        CompressionEncoding::Gzip => { 
                            resp.set_raw_header("Content-Encoding", "gzip");
                            gzip.clone() 
                        },
                        CompressionEncoding::Deflate => { 
                            resp.set_raw_header("Content-Encoding", "deflate");
                            deflate.clone() 
                        },
                        CompressionEncoding::Uncompressed => output_contents.clone(),
                    };
                    
                    // DEBUG PRINT - print!("Setting body content..  ");
                    
                    resp.set_streamed_body(
                        Cursor::new( bytes )
                    );
                    
                    // DEBUG PRINT - print!("Finished setting body content.\n");
                    
                    let total_size = output_contents.len() + gzip.len() + br.len() + deflate.len();
                    new_cache = ContentCached {
                        page: output_contents,
                        gzip,
                        br,
                        deflate,
                        size: total_size,
                    };
                    
                    // DEBUG PRINT - println!("Created cache object");
                    
                    println!("Attempting to write cache object to hashmap");
                    // remember to put the body from body_bytes back into the resp, body_bytes() consumes the bytes
                    // insert new_cache into cache map, make sure to unlock it for write access
                    {
                        // let mut wcache = cache_state.pages.write().unwrap();
                        // wcache.insert(self.route.clone(), new_cache);
                        let mut wcache = cache_state.pages.write().unwrap();
                        wcache.insert(self.route.clone(), new_cache.clone());
                        
                    }
                    
                    // DEBUG PRINT - println!("Successfully inserted cache object");
                    
                    // DEBUG PRINT - println!("Responder finished, returning response..");
                    
                    // Outcome::Success( resp )
                    Ok( resp )
                    
                } else {
                    // Outcome::Failure("Responder failed to extract response body.")
                    // fail - uri not found in context map
                    // Err("Responder failed to extract response body")
                    println!("Responder failed to extract response body");
                    Err(Status::ImATeapot)
                }
                
                
            } else {
                println!("Responder failed to find uri `{}` in the context map.", &self.route);
                // Outcome::Failure("Responder failed to find uri in context map")
                // Err("Responder failed to find uri in context map")
                Err(Status::NotFound)
            }
        // }
        
        
        /*
        // let content_context_map = req.guard::<ContentContext>();
        // let content_cache = req.guard::<ContentCacheLock>();
        let content_context_map_rst =  req.guard::<State<ContentContext>>();
        let content_cache_rst =  req.guard::<State<ContentCacheLock>>();
        // if content_context_map_rst == 0usize {}
        if let Success(content_context) = content_context_map_rst {
            if let Success(content_cache2) = content_cache_rst {
                let content_cache = content_cache2.inner();
                // let content_context_opt = content_context_map.get(&self.route);
                
                // cache_entry = self.cache.pages.read().unwrap().get(self.route);
                // if let Some(content_context) = content_context_opt {
                    if let Ok(cache) = content_cache.cache.pages.read() 
                    {
                    // if let Ok(cache) = self.cache.pages.read() {
                        let cache_entry: Option<&ContentCached>;
                        cache_entry = cache.get(&self.route);
                        // if let Some(cache_entry) = cache.get(&self.route) {
                        // output_contents is used as a variable to reference in output_bytes
                        //   when there is no existing cache entry for the uri
                        let mut output_contents: Vec<u8> = Vec::new();
                        
                        if let Some(entry) = cache_entry {
                            let entry = cache_entry.unwrap(); // ok because this is guaranteed to be something
                            let mut output_bytes: &Vec<u8> = &Vec::new();
                            match self.encoding.preferred() 
                            {
                                CompressionEncoding::Uncompressed => { output_bytes = &entry.page; },
                                CompressionEncoding::Brotli => { output_bytes = &entry.br; },
                                CompressionEncoding::Gzip => { output_bytes = &entry.gzip; },
                                CompressionEncoding::Deflate => { output_bytes = &entry.deflate; },
                            }
                            
                            // ----- ----- ----- ----- ----- ----- ----- ----- ----- ----- ----- -----
                            // To improve performance: instead of cloning the vector:
                            //   make a String::new() that is converted .into() an Express instance
                            //   then do xresp.streamed_body( Cursor::new(output_bytes) )
                            // ----- ----- ----- ----- ----- ----- ----- ----- ----- ----- ----- -----
                            
                            output_contents = output_bytes.clone();
                            let express: Express = output_contents.into();
                            express.respond_to(req) // do not use express.compress(encoding) as the contents are already compressed!!!
                            
                            // let mut xresp = express.respond_to(req).unwrap_or_default();
                            // xresp.streamed_body(Cursor::new( output_bytes ))
                                
                        }
                         else 
                        {
                            
                            
                            // let template: Template = Template::render(self.context.template.clone(), &self.context);
                            let template: Template = Template::render(content_context.template.clone(), &content_context);
                            
                            // let bytes: Vec<u8> = Vec::new();
                            let express: Express = template.into();
                            
                            let mut xresp = express.respond_to(req).unwrap_or_default();
                            // let mut tresp = template.respond_to(req).unwrap_or_default();
                            // xresp.set_streamed_body(Cursor::new( output_bytes )); // need to use set_streamed_body() when not using a response builder
                            
                            
                            // Performance:
                            // make the if let Some(body) for the xresp.body_bytes() not assign to a variable
                            // instead move all of the compression and the entry: ContentCached struct initialization
                            //   into the if statement, and make an else statement that creates a BLANK entry (otherwise it will have to create the blank entry over and over again)
                            
                            // output_contents = if let Some(body) = xresp.body_bytes() {
                            //     body
                            // } else {
                            //     Vec::new()
                            // };
                            let mut output_contents: Vec<u8> = Vec::new();
                            let entry: ContentCached;
                            if let Some(body) = xresp.body_bytes() 
                            {
                                output_contents = body;
                                let gzip: Vec<u8>;
                                {
                                    let mut buffer = Vec::with_capacity(output_contents.len() + 200);
                                    let mut gzip_encoder = gzip::Encoder::new(buffer).unwrap();
                                    gzip_encoder.write_all(&output_contents).expect("hi gzip"); // .expect("Gzip compression failed.");
                                    gzip = gzip_encoder.finish().into_result().unwrap_or(Vec::new());
                                }
                                
                                let br: Vec<u8>;
                                {
                                    let length = output_contents.len()+200;
                                    let mut buffer = Vec::with_capacity(length);
                                    // let mut compressor = ::brotli::CompressorReader::new(Cursor::new(data), 10*1024, 9, 22);
                                    let mut compressor = ::brotli::CompressorReader::new(Cursor::new(&output_contents), length, 9, 22);
                                    let _ = compressor.read_to_end(&mut buffer);
                                    br = buffer;
                                }
                                
                                let deflate: Vec<u8>;
                                {
                                    let mut buffer = Vec::with_capacity(output_contents.len()+200);
                                    let mut encoder = deflate::Encoder::new(buffer);
                                    encoder.write_all(&output_contents); //.expect("Deflate compression failed.");
                                    deflate = encoder.finish().into_result().unwrap_or(Vec::new());
                                    
                                }
                                
                                let output_length = output_contents.len();
                                let gzip_length = gzip.len();
                                let br_length = br.len();
                                let deflate_length = deflate.len();
                                
                                entry = ContentCached 
                                {
                                    page: output_contents,
                                    gzip,
                                    br,
                                    deflate,
                                    // size: output_contents.len() + gzip.len() + br.len() + deflate.len(),
                                    size: output_length + gzip_length + br_length + deflate_length,
                                };
                            } 
                            else 
                            {
                                let output_length = output_contents.len();
                                output_contents = Vec::new();
                                entry = ContentCached 
                                {
                                    page: output_contents,
                                    gzip: Vec::new(),
                                    br: Vec::new(),
                                    deflate: Vec::new(),
                                    size: output_length,
                                };
                            }
                            {
                                // let map_rst = self.cache.pages.write();
                                let map_rst = content_cache.pages.write();
                                if let Ok(mut map) = map_rst 
                                {
                                    map.insert(self.route.to_owned(), entry.clone());
                                }
                            }
                            
                            // Add entry
                            // Add total size to the cache.size:
                            //   br.len() + gzip.len() + deflate.len() + output_contents.len()
                            
                            // need to set the body because the body_bytes() method consumes it
                            // which is ok because it gets 
                            
                            let output = match self.encoding.preferred() 
                            {
                                CompressionEncoding::Uncompressed => { entry.page },
                                CompressionEncoding::Brotli => { entry.br },
                                CompressionEncoding::Gzip => { entry.gzip },
                                CompressionEncoding::Deflate => { entry.deflate },
                            };
                            
                            xresp.set_streamed_body(  Cursor::new( output )  );
                            Ok(xresp)
                        }
                    } else {
                        Err(Status::BadRequest)
                    }
                // } else {
                //     Err(Status::BadRequest)
                // }
            } else {
                Err(Status::ImATeapot)
            }
        } else {
            Err(Status::ImATeapot)
        }
        */
    }
}

















