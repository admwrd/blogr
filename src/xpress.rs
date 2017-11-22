

use rocket::response::{self, Response, Responder};
use rocket::Data;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Method, Status};
use rocket::response::{content, NamedFile, Content};
use rocket_contrib::Template;
use rocket::http::ContentType;

use std::mem;
use std::ffi::OsStr;
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::prelude::*;
use std::option;
// use std::io::BufReader;
// use std::option::Option;
// use std::sync::atomic::{AtomicUsize, Ordering};

use zopfli;
use brotli;
use rocket::request::{FromRequest, Request};
use rocket::Outcome;

use accept::*;

const DEFAULT_TTL: usize = 3600;  // 1*60*60 = 1 hour, 43200=1/2 day, 86400=1 day


// Do not add content type to ExData, allow user to set explicitly
// with express.set_content_type() method

pub trait ExpressData {
    fn content_type(&self) -> ContentType;
    fn contents(self, &Request) -> Vec<u8>;
}

#[derive(Debug, Clone)]
// pub struct ExData<T: ExpressData>(T);
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
        }
    }
    fn contents(self, req: &Request) -> Vec<u8> {
        match self {
            ExData::Bytes(data) => data.contents(req),
            ExData::File(data) => data.contents(req),
            ExData::Named(data) => data.contents(req),
            ExData::String(data) => data.contents(req),
            ExData::Template(data) => data.contents(req),
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
       // // Crashes
       //  let new: NamedFile;
       //  unsafe {
       //      new = mem::transmute_copy( &self.0 );
       //  }
       // DataNamed(new)
       
       // let named = NamedFile { 0: self.0.path().to_path_buf(), 1: *self.0.file() };
       let named = NamedFile::open(self.0.path()).expect("Cloning DataNamed(NamedFile) failed, ensure to check that the file exists before creating an Express structure.");
       
       DataNamed(named)
    }
}

impl Clone for DataTemplate {
    fn clone(&self) -> DataTemplate {
        // // let new: Template;
        // let size = mem::size_of_val(&self.0);
        // // let new_bytes: Vec<u8> = vec![0u8; size*4];
        // let mut new_bytes: Vec<u8> = Vec::with_capacity(size*4);
        // new_bytes = vec![0u8; size*4];
        // // let new_bytes: [u8; size] = [0u8; size]; 
        // let mut new: Template;
        // unsafe {
        //     println!("Cloning DataTemplate.  Size of source: {}, Destination: {}", mem::size_of_val(&self.0), mem::size_of_val(&new));
        //     new_bytes = mem::transmute_copy( &self.0 );
        //     new = mem::transmute(new_bytes);
        // }
        
        // DataTemplate(new)
        
        
        // Bleh nothing works.  I hate how the Rocket Templates are organized/written.
        unimplemented!()
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


#[derive(Debug,Clone)]
pub struct Express {
    data: ExData,
    method: CompressionEncoding,
    content_type: ContentType,
    ttl: usize,
    streamed: bool,
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
    pub fn set_ttl(mut self, ttl: usize) -> Self {
        self.ttl = ttl;
        self
    }
    /// Set the data to a byte vector
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
    pub fn new(data: ExData) -> Express {
        Express {
            content_type: (&data).content_type(),
            data,
            method: CompressionEncoding::Uncompressed,
            ttl: DEFAULT_TTL,
            streamed: true,
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
        Express::new( ExData::Named( DataNamed(original) ) )
    }
}

impl From<Vec<u8>> for Express {
    fn from(original: Vec<u8>) -> Express {
        Express::new( ExData::Bytes( DataBytes(original) ) )
    }
}

impl From<PathBuf> for Express {
    fn from(original: PathBuf) -> Express {
        Express::new( ExData::File( DataFile(original) ) )
    }
}

impl From<Template> for Express {
    fn from(original: Template) -> Express {
        Express::new( ExData::Template( DataTemplate(original) ) )
    }
}

impl<'a> Responder<'a> for Express {
    fn respond_to(self, req: &Request) -> response::Result<'a> {
        let mut response = Response::build();
        
        response.header(self.content_type);
        response.raw_header("Cache-Control", format!("max-age={}, must-revalidate", self.ttl));
        
        let mut data = self.data.contents(req);
        
        match self.method {
            CompressionEncoding::Brotli => {
                response.raw_header("Content-Encoding", "br");
                
                let mut output = Vec::with_capacity(data.len()+200);
                let mut compressor = ::brotli::CompressorReader::new(Cursor::new(data), 10*1024, 9, 22);
                let _ = compressor.read_to_end(&mut output);
                
                data = output;
            },
            CompressionEncoding::Gzip =>{
                response.raw_header("Content-Encoding", "gzip");
                
                let mut output = Vec::with_capacity(data.len()+200);
                zopfli::compress(&zopfli::Options::default(), &zopfli::Format::Gzip, &data, &mut output).expect("Gzip compression failed.");
                
                data = output;
            },
            CompressionEncoding::Deflate => {
                response.raw_header("Content-Encoding", "deflate");
                
                let mut output = Vec::with_capacity(data.len()+200);
                zopfli::compress(&zopfli::Options::default(), &zopfli::Format::Deflate, &data, &mut output).expect("Deflate compression failed.");
                
                data = output;
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





