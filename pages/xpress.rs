use rocket::response::{self, Response, Responder};
use rocket::Data;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Method, Status};
use rocket::response::{content, NamedFile, Content};
use rocket_contrib::Template;
use rocket::http::ContentType;
use rocket::http::Header;
use rocket::http::HeaderMap;
use rocket::request::{FromRequest, Request};
use rocket::Outcome;

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::mem;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::prelude::*;
use std::option;
use serde_json::{Value, to_value};
use std::borrow::Cow;
use std::io;

use brotli;
use libflate::gzip;
use libflate::deflate;

use accept::*;

const DEFAULT_TTL: isize = 3600;

pub trait ExpressData {
    fn content_type(&self) -> ContentType;
    fn contents(self, &Request) -> Vec<u8>;
}

// Do not derive clone since Templates cannot be cloned
#[derive(Debug)]
pub enum ExData {
    Bytes(DataBytes),
    File(DataFile),
    Named(DataNamed),
    String(DataString),
    Template(DataTemplate),
}

impl ExpressData for ExData {
    fn content_type(&self) -> ContentType {
        // self.0.content_type()
        match self {
            &ExData::Bytes(ref data) => data.content_type(),
            &ExData::File(ref data) => data.content_type(),
            &ExData::Named(ref data) => data.content_type(),
            &ExData::String(ref data) => data.content_type(),
            &ExData::Template(ref data) => data.content_type(),
            // &ExData::Cache(ref data) => data.content_type(),
        }
    }
    fn contents(self, req: &Request) -> Vec<u8> {
        match self {
            ExData::Bytes(data) => data.contents(req),
            ExData::File(data) => data.contents(req),
            ExData::Named(data) => data.contents(req),
            ExData::String(data) => data.contents(req),
            ExData::Template(data) => data.contents(req),
            // ExData::Cache(data) => data.contents(req),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DataBytes(Vec<u8>);
#[derive(Debug, Clone)]
pub struct DataFile(PathBuf);
#[derive(Debug)]
pub struct DataNamed(NamedFile);
#[derive(Debug, Clone)]
pub struct DataString(String);
#[derive(Debug)]
pub struct DataTemplate(Template);

impl Clone for DataNamed {
    fn clone(&self) -> DataNamed {
       let named = NamedFile::open(self.0.path()).expect("Cloning DataNamed(NamedFile) failed, ensure to check that the file exists before creating an Express structure.");
       
       DataNamed(named)
    }
}

impl ExpressData for DataBytes {
    fn content_type(&self) -> ContentType {
        ContentType::HTML
    }
    fn contents(self, _: &Request) -> Vec<u8> {
        self.0
    }
}

impl ExpressData for DataFile {
    fn content_type(&self) -> ContentType {
        ContentType::from_extension(self.0.extension().unwrap_or(OsStr::new("")).to_str().unwrap_or("")).unwrap_or(ContentType::Plain)
    }
    fn contents(self, _: &Request) -> Vec<u8> {
        let file_rst = File::open(self.0);
        let mut data: Vec<u8> = Vec::new();
        if let Ok(mut file) = file_rst {
            file.read_to_end(&mut data);
        }
        data
    }
}

impl ExpressData for DataNamed {
    fn content_type(&self) -> ContentType {
        ContentType::from_extension(self.0.path().extension().unwrap_or(OsStr::new("")).to_str().unwrap_or("")).unwrap_or(ContentType::Plain)
    }
    fn contents(self, _: &Request) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        self.0.file().read_to_end(&mut data);
        data
    }
}

impl ExpressData for DataString {
    fn content_type(&self) -> ContentType {
        ContentType::HTML
    }
    fn contents(self, _: &Request) -> Vec<u8> {
        self.0.bytes().collect::<Vec<u8>>()
    }
}

impl ExpressData for DataTemplate {
    fn content_type(&self) -> ContentType {
        ContentType::HTML
    }
    fn contents(self, req: &Request) -> Vec<u8> {
        let mut response = self.0.respond_to(req).unwrap_or_default();
        if let Some(body) = response.body_bytes() {
            body
        } else {
            println!("Could not retrieve template contents using respond_to()");
            Vec::new()
        }
    }
}


#[derive(Debug)]
pub struct Express {
    data: ExData,
    method: CompressionEncoding,
    content_type: ContentType,
    ttl: isize,
    streamed: bool,
    extras: HashMap<String, String>,
}


impl ExpressData for Express {
    fn content_type(&self) -> ContentType {
        self.data.content_type()
    }
    fn contents(self, req: &Request) -> Vec<u8> {
        self.data.contents(req)
    }
}

impl Express {
    
    /// Alias for add_comrpession
    #[inline(always)]
    pub fn compress(mut self, encoding: AcceptCompression) -> Self {
        self.add_compression(encoding)
    }
    /// Add the preferred compression method for the AcceptCompression.
    /// To use a specific compression algorithm use AcceptCompression.checked_gzip(AcceptCompression).
    /// This will check for a given compression method in the accept encoding and only
    /// return that specific algorithm for use, if it is not in the accept encoding
    /// no compression is used.
    pub fn add_compression(mut self, encoding: AcceptCompression) -> Self {
        self.method = encoding.preferred();
        self
    }
    /// Set ttl to the number of seconds the content should be cached
    pub fn set_ttl(mut self, ttl: isize) -> Self {
        self.ttl = ttl;
        self
    }
    /// Set the contents / data to a byte vector
    pub fn set_data<T: Into<ExData>>(mut self, data: T) -> Self {
        self.data = data.into();
        self
    }
    /// Resets the compression method to None
    pub fn reset_compression(mut self) -> Self {
        self.method = CompressionEncoding::Uncompressed;
        self
    }
    /// Sets the content type to the specified Rocket ContentType
    pub fn set_content_type(mut self, cont_type: ContentType) -> Self {
        self.content_type = cont_type;
        self
    }
    pub fn set_streamed(mut self) -> Self {
        self.streamed = true;
        self
    }
    pub fn unset_streamed(mut self) -> Self {
        self.streamed = false;
        self
    }
    pub fn add_extra(mut self, key: String, value: String) -> Self {
        self.extras.insert(key, value);
        self
    }
    pub fn add_header<'p, H: Into<Header<'p>>>(mut self, header: H) -> Self {
        // Not the greatest solution; ideally the headers would be stored in a Vec<Header>
        // or in a HeaderMap but since those both require lifetime parameters it would
        // require everything to be rewritten.
        let h: Header = header.into();
        
        self.extras.insert(h.name().to_string(), h.value().to_string());
        self
    }
    pub fn new(data: ExData) -> Express {
        Express {
            content_type: (&data).content_type(),
            data,
            method: CompressionEncoding::Uncompressed,
            ttl: -1,
            streamed: true,
            extras: HashMap::new(),
        }
    }
}

impl From<String> for Express {
    fn from(original: String) -> Express {
        Express::new( ExData::String( DataString(original) ) )
    }
}

impl From<NamedFile> for Express {
    fn from(original: NamedFile) -> Express {
        Express::new( ExData::Named( DataNamed(original) ) ).set_ttl(DEFAULT_TTL)
    }
}

impl From<Vec<u8>> for Express {
    fn from(original: Vec<u8>) -> Express {
        Express::new( ExData::Bytes( DataBytes(original) ) )
    }
}

impl From<PathBuf> for Express {
    fn from(original: PathBuf) -> Express {
        Express::new( ExData::File( DataFile(original) ) ).set_ttl(DEFAULT_TTL)
    }
}

impl From<Template> for Express {
    fn from(original: Template) -> Express {
        Express::new( ExData::Template( DataTemplate(original) ) )
    }
}

pub fn express_gzip(data: Vec<u8>) -> Vec<u8> {
    let mut output = Vec::with_capacity(data.len()+200);
    let mut encoder = gzip::Encoder::new(output).unwrap();
    encoder.write_all(&data).expect("Gzip compression failed.");
    encoder.finish().into_result().unwrap()
}

pub fn express_brotli(data: Vec<u8>) -> Vec<u8> {
    let length = data.len()+200;
    let mut output = Vec::with_capacity(length);
    let mut compressor = ::brotli::CompressorReader::new(Cursor::new(data), length, 9, 22);
    let _ = compressor.read_to_end(&mut output);
    output
}
pub fn express_deflate(data: Vec<u8>) -> Vec<u8> {
    let mut output = Vec::with_capacity(data.len()+200);
    let mut encoder = deflate::Encoder::new(output);
    encoder.write_all(&data).expect("Deflate compression failed.");
    encoder.finish().into_result().unwrap()
    
}

impl<'a> Responder<'a> for Express {
    fn respond_to(self, req: &Request) -> response::Result<'a> {
        let mut response = Response::build();
        let extras = self.extras;
        
        if extras.len() != 0 {
            for (key, value) in extras {
                response.raw_header(key, value);
            }
        }
        
        response.header(self.content_type);
        
        if self.ttl == -1 {
            response.raw_header("Pragma", "no-cache");
            response.raw_header("Cache-Control", "no-store");
            response.raw_header_adjoin("Cache-Control", "no-cache, no-store, must-revalidate");
        } else {
            response.raw_header("Cache-Control", format!("max-age={}", self.ttl));
        }
        
        let mut data = self.data.contents(req);
        
        match self.method {
            CompressionEncoding::Brotli => {
                response.raw_header("Content-Encoding", "br");
                
                data = express_brotli(data);
            },
            CompressionEncoding::Gzip =>{
                response.raw_header("Content-Encoding", "gzip");
                
                data = express_gzip(data);
            },
            CompressionEncoding::Deflate => {
                response.raw_header("Content-Encoding", "deflate");
                
                data = express_deflate(data);
            },
            CompressionEncoding::Uncompressed => {},
        }
        if self.streamed {
            response.streamed_body(Cursor::new(data));
        } else {
            response.sized_body(Cursor::new(data));
        }
        
        response.ok()
    }
}



