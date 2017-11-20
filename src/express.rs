
use rocket::response::{self, Response, Responder};

use std::io::{Cursor, Read};
// use std::sync::atomic::{AtomicUsize, Ordering};

use rocket::Data;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Method, Status};
use rocket::response::{content, NamedFile, Content};
use rocket_contrib::Template;
use std::path::{Path, PathBuf};
use std::fs::File;
// use std::io::BufReader;
use std::io::prelude::*;
use rocket::http::ContentType;

use zopfli;
use brotli;
use rocket::request::{FromRequest, Request};
use rocket::Outcome;



const DEFAULT_TTL: usize = 3600;  // 1*60*60 = 1 hour, 43200=1/2 day, 86400=1 day



/* Process Flow
    Enter a route with an AcceptEncoding parameter
    The AcceptEncoding FromRequest is triggered and finds out which algorithms are supported
    Once the route is finished and has generated a template use the following line of code:
        Express::From(template).compress(encoding)
        1. The template is converted to an Express structure
        2. The compress function compresses the content with an available compression function
        3. The Express (possibly modified) Express structured is returned from compress
        4. The Express function's Responder is called by Rocket and sets the appropriate
            content type, content encoding, ttl, and related headers
    
*/


/* Use like:
    fn some_route(AcceptEncoding) -> Express {
        Express::From(template).compress(encoding)
    }
*/

// My implementation
// express - EXPiration + comPRESS

// AcceptEncoding
// https://mmstick.tk/post/jmP

// Compression and CMS tutorial
// https://mmstick.tk/post/q42

const GZIP:    u8 = 1;
const DEFLATE: u8 = 2;
const BROTLI:  u8 = 4;

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum PreferredEncoding { Brotli, Gzip, Deflate, Uncompressed }

#[derive(Copy, Clone, Debug)]
pub struct AcceptEncoding {
    supported: u8
}

impl AcceptEncoding {
    pub fn contains_gzip(self)    -> bool { self.supported & GZIP != 0 }
    pub fn contains_deflate(self) -> bool { self.supported & DEFLATE != 0 }
    pub fn contains_brotli(self)  -> bool { self.supported & BROTLI != 0 }
    pub fn is_uncompressed(self)  -> bool { self.supported == 0 }
    pub fn preferred(self) -> PreferredEncoding {
        if self.supported & GZIP != 0 {
            PreferredEncoding::Gzip
        } else if self.supported & DEFLATE != 0 {
            PreferredEncoding::Deflate
        } else if self.supported & BROTLI != 0 {
            PreferredEncoding::Brotli
        } else {
            PreferredEncoding::Uncompressed
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AcceptEncoding {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> Outcome<AcceptEncoding, (Status, ()), ()> {
        let mut supported = 0u8;
        if let Some(encoding) = request.headers().get("Accept-Encoding").next() {
            if encoding.contains("gzip") { supported |= GZIP; }
            if encoding.contains("deflate") { supported |= DEFLATE; }
            if encoding.contains("br") { supported |= BROTLI; }
        }
        Outcome::Success(AcceptEncoding { supported: supported })
    }
}

#[derive(Debug,Clone)]
pub struct Express {
    pub data: Vec<u8>,
    pub content_type: ContentType,
    pub ttl: usize,
    pub compress: Option<PreferredEncoding>,
}

impl Express {
    pub fn from_string(template: String) -> Self {
        Express {
            data: template.bytes().collect::<Vec<u8>>(), // Todo: maybe change this to into_bytes() so the String is consumed and changed instead of copied as bytes
            content_type: ContentType::HTML, // assume all template are HTML files.  If your templates are not all html files change this
            ttl: 0, // Do not cache the regular html files as they may change immediately
            compress: None,
    }
    }
    pub fn set_ttl(mut self, ttl: usize) -> Self {
        self.ttl = ttl;
        self
    }
    pub fn set_data(mut self, data: Vec<u8>) -> Self {
        self.data = data;
        self
    }
    pub fn set_compression(mut self, encoding: Option<PreferredEncoding>) -> Self {
        self.compress = encoding;
        self
    }
    pub fn set_content_type(mut self, cont_type: ContentType) -> Self {
        self.content_type = cont_type;
        self
    }
    pub fn compress(mut self, encoding: AcceptEncoding) -> Self {
        println!("Attempting to set compression, support: {:?}", encoding);
        match encoding.preferred() {
            PreferredEncoding::Brotli => {
                println!("Encoding with Brotli");
                let mut output = Vec::with_capacity(10*1024);
                
                // use std::io::Cursor;
                // let mut buff = Cursor::new(vec![0; 15]);
                // write_ten_bytes_at_end(&mut buff).unwrap();
                
                
                // let mut compressor = ::brotli::BrotliReader::new(Cursor::new(self.data), 10*1024, 9, 22);
                // let mut compressor = ::brotli::Compressor::new(Cursor::new(self.data), 10*1024, 9, 22);
                
                // let mut compressor = ::brotli::CompressorReader::new(Cursor::new(self.data), 10*1024, 9, 22);
                let mut compressor = ::brotli::CompressorReader::new(Cursor::new(self.data), 10*1024, 2, 22);
                let _ = compressor.read_to_end(&mut output);
                // Some(ContentEncoding::Brotli)
                self.data = output;
                self.compress = Some(PreferredEncoding::Brotli);
            },
            PreferredEncoding::Gzip => {
                println!("Encoding with Gzip");
                let mut output = Vec::with_capacity(10*1024);
                zopfli::compress(&zopfli::Options::default(), &zopfli::Format::Gzip, &self.data, &mut output).expect("Gzip compression failed.");
                self.data = output;
                self.compress = Some(PreferredEncoding::Gzip);
            },
            PreferredEncoding::Deflate => {
                println!("Encoding with Deflate");
                let mut output = Vec::with_capacity(10*1024);
                zopfli::compress(&zopfli::Options::default(), &zopfli::Format::Deflate, &self.data, &mut output).expect("Deflate compression failed.");
                self.data = output;
                self.compress = Some(PreferredEncoding::Deflate);
                
            },
            _ => {
                println!("No compression methods available.");
            },
        }
        
        // if encoding.contains_brotli() {
        //     println!("Encoding with Brotli");
        //     let mut output = Vec::with_capacity(10*1024);
            
        //     // use std::io::Cursor;
        //     // let mut buff = Cursor::new(vec![0; 15]);
        //     // write_ten_bytes_at_end(&mut buff).unwrap();
            
            
        //     // let mut compressor = ::brotli::BrotliReader::new(Cursor::new(self.data), 10*1024, 9, 22);
        //     // let mut compressor = ::brotli::Compressor::new(Cursor::new(self.data), 10*1024, 9, 22);
        //     let mut compressor = ::brotli::CompressorReader::new(Cursor::new(self.data), 10*1024, 9, 22);
        //     let _ = compressor.read_to_end(&mut output);
        //     // Some(ContentEncoding::Brotli)
        //     self.data = output;
        //     self.compress = Some(PreferredEncoding::Brotli);
        // } else if encoding.contains_gzip() {
        //     println!("Encoding with Gzip");
        //     let mut output = Vec::with_capacity(10*1024);
        //     zopfli::compress(&zopfli::Options::default(), &zopfli::Format::Gzip, &self.data, &mut output).expect("Gzip compression failed.");
        //     self.data = output;
        //     self.compress = Some(PreferredEncoding::Gzip);
            
        // } else if encoding.contains_deflate() {
        //     println!("Encoding with Deflate");
        //     let mut output = Vec::with_capacity(10*1024);
        //     zopfli::compress(&zopfli::Options::default(), &zopfli::Format::Deflate, &self.data, &mut output).expect("Deflate compression failed.");
        //     self.data = output;
        //     self.compress = Some(PreferredEncoding::Deflate);
        // } else {
        //     println!("No compression methods available.");
        // }
        
        self
    }
}

impl From<String> for Express {
    fn from(template: String) -> Express {
        // let mut output = Vec::with_capacity(30*1024);
        // zopfli::compress(&Options::default(), &Format::Gzip, template.to_string().as_bytes(), &mut output).unwrap();
        
        // let data = template.to_string().as_bytes();
        // let (contents) = template.finalize();
        // let data = template.show();
        
        Express {
            data: template.bytes().collect::<Vec<u8>>(), // Todo: maybe change this to into_bytes() so the String is consumed and changed instead of copied as bytes
            content_type: ContentType::HTML, // assume all template are HTML files.  If your templates are not all html files change this
            ttl: 0, // Do not cache the regular html files as they may change immediately
            compress: None,
        }
    }
}

// NamedFiles passed the static files route
impl From<NamedFile> for Express {
    fn from(named: NamedFile) -> Express {
        let path = named.path();
        let content_type = ContentType::from_extension(path.extension().expect("No extension found").to_str().expect("Path to string conversion failed")).unwrap_or(ContentType::Plain);
        let mut data: Vec<u8> = Vec::new();
        let result = named.file().read_to_end(&mut data);
        Express {
            data,
            content_type,
            ttl: DEFAULT_TTL,
            compress: None,
        }
    }
}



impl<'a> Responder<'a> for Express {
    // fn respond(self) -> response::Result<'a> {
    fn respond_to(self, _: &Request) -> response::Result<'a> {
        let mut resp = Response::build();
        // println!("Setting headers to:\n{:?}", self);
        resp.sized_body(Cursor::new(self.data));
        resp.raw_header("Cache-Control", format!("max-age={}, must-revalidate", self.ttl));
        if let Some(enc) = self.compress {
            match enc {
                PreferredEncoding::Brotli => { resp.raw_header("Content-Encoding", "br"); },
                PreferredEncoding::Gzip => { resp.raw_header("Content-Encoding", "gzip"); },
                PreferredEncoding::Deflate => { resp.raw_header("Content-Encoding", "deflate"); },
                _ => {},
            }
        }
        resp.header(self.content_type);
        resp.ok()
    }
}














// #[derive(Debug,Clone)]
// pub struct Express2 {
//     pub data: Vec<u8>,
//     pub content_type: ContentType,
//     pub ttl: usize,
//     pub compress: bool,
// }

// impl fairing::Fairing for Express2 {
//     fn info(&self) -> fairing::Info {
//         fairing::Info {
//             name: "Gzip Compression",
//             kind: fairing::Kind::Response,
//         }
//     }

//     fn on_response(&self, request: &Request, response: &mut Response) {
//         // use flate2::{Compression, FlateReadExt};
//         // use std::io::{Cursor, Read};
//         let headers = request.headers();
//         if headers
//             .get("Accept-Encoding")
//             .any(|e| e.to_lowercase().contains("gzip"))
//         {
//             response.body_bytes().and_then(|body| {
//                 let mut enc = body.gz_encode(Compression::Default);
//                 let mut buf = Vec::with_capacity(body.len());
//                 enc.read_to_end(&mut buf)
//                     .map(|_| {
//                         response.set_sized_body(Cursor::new(buf));
//                         response.set_raw_header("Content-Encoding", "gzip");
//                         response.raw_header("Cache-Control", format!("max-age={}, must-revalidate", self.ttl))
//                     })
//                     .map_err(|e| eprintln!("{}", e)).ok()
//             });
//         }
//     }
// }
















// https://github.com/SergioBenitez/Rocket/issues/195
// https://mmstick.tk/post/q42

// impl From<Template> for RequestedContent {
//     fn from(template: Template) -> RequestedContent {
//         let mut output = Vec::with_capacity(8*1024);
//         zopfli::compress(&Options::default(), &Format::Gzip, template.to_string().as_bytes(), &mut output).unwrap();
//         RequestedContent {
//             data: output,
//             content_type: ContentType::HTML,
//             ttl: 600
//         }
//     }
// }

// #[derive(Debug,Clone)]
// pub struct RequestedContent {
//     pub data: Vec<u8>,
//     pub content_type: ContentType,
//     pub ttl: usize,
// }

// impl<'a> Responder<'a> for RequestedContent {
//     fn respond(self) -> response::Result<'a> {
//         Response::build()
//             .sized_body(Cursor::new(self.data))
//             .raw_header("Cache-Control", format!("max-age={}, must-revalidate", self.ttl))
//             .raw_header("Content-Encoding", "gzip")
//             .header(self.content_type)
//             .ok()
//     }
// }





// struct Expire {
//     age: 
// }

// impl<'r> Responder<'r> for Person {
//     fn respond_to(self, _: &Request) -> response::Result<'r> {
//         Response::build()
//             .sized_body(Cursor::new(format!("{}:{}", self.name, self.age)))
//             .raw_header("X-Person-Name", self.name)
//             .raw_header("X-Person-Age", self.age.to_string())
//             .header(ContentType::new("application", "x-person"))
//             .ok()
//     }
// }

// pub struct Gzip {
//     ttl: usize;
// };

// impl fairing::Fairing for Gzip {
//     fn info(&self) -> fairing::Info {
//         fairing::Info {
//             name: "Gzip Compression",
//             kind: fairing::Kind::Response,
//         }
//     }

//     fn on_response(&self, request: &Request, response: &mut Response) {
//         use flate2::{Compression, FlateReadExt};
//         use std::io::{Cursor, Read};
//         let headers = request.headers();
//         if headers
//             .get("Accept-Encoding")
//             .any(|e| e.to_lowercase().contains("gzip"))
//         {
//             response.body_bytes().and_then(|body| {
//                 let mut enc = body.gz_encode(Compression::Default);
//                 let mut buf = Vec::with_capacity(body.len());
//                 enc.read_to_end(&mut buf)
//                     .map(|_| {
//                         response.set_sized_body(Cursor::new(buf));
//                         response.set_raw_header("Content-Encoding", "gzip");
//                         response.raw_header("Cache-Control", format!("max-age={}, must-revalidate", self.ttl))
//                     })
//                     .map_err(|e| eprintln!("{}", e)).ok()
//             });
//         }
//     }
// }



