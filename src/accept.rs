
// Copied from
// https://mmstick.tk/post/jmP
// and andded the gzip(), deflate(), brotli(), and the check_*() methods

use rocket::request::{FromRequest, Request};
use rocket::http::Status;
use rocket::Outcome;
use std::collections::HashMap;

pub const GZIP:    u8 = 1;
pub const DEFLATE: u8 = 2;
pub const BROTLI:  u8 = 4;

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum CompressionEncoding { Brotli, Gzip, Deflate, Uncompressed }

#[derive(Copy, Clone, Debug)]
pub struct AcceptCompression {
    supported: u8
}

impl AcceptCompression {
    pub fn new(supported: u8) -> AcceptCompression { AcceptCompression { supported } }
    pub fn contains_gzip(self)    -> bool { self.supported & GZIP != 0 }
    pub fn contains_deflate(self) -> bool { self.supported & DEFLATE != 0 }
    pub fn contains_brotli(self)  -> bool { self.supported & BROTLI != 0 }
    pub fn is_uncompressed(self)  -> bool { self.supported == 0 }
    
    // Consider maybe using &self instead of self??
    pub fn preferred(self) -> CompressionEncoding {
        if self.supported & BROTLI != 0 {
            CompressionEncoding::Brotli
        } else if self.supported & GZIP != 0 {
            CompressionEncoding::Gzip
        // Disable Deflate due to problems with IE6 
        // gzip and brotli are so much better anways, not really worth using anyways
        // } else if self.supported & DEFLATE != 0 {
            // CompressionEncoding::Deflate
        } else {
            CompressionEncoding::Uncompressed
        }
    }
    /// Returns a new AcceptCompression that specifies the gzip method
    #[inline(always)]
    pub fn gzip() -> Self { AcceptCompression { supported: GZIP } }
    /// Returns a new AcceptCompression that specifies the deflate method
    #[inline(always)]
    pub fn deflate() -> Self { AcceptCompression { supported: DEFLATE } }
    /// Returns a new AcceptCompression that specifies the brotli method
    #[inline(always)]
    pub fn brotli() -> Self { AcceptCompression { supported: BROTLI } }
    
    /// Returns a new AcceptCompression that uses no compression
    #[inline(always)]
    pub fn no_compression() -> Self { AcceptCompression { supported: 0 } }
    
    /// Returns a new AcceptCompression that specifies the gzip method
    pub fn checked_gzip(&self) -> Self { 
        if self.contains_gzip() { 
            AcceptCompression { 
                supported: GZIP 
            } 
        } else { 
            AcceptCompression::no_compression() 
        } 
    }
    /// Returns a new AcceptCompression that specifies the deflate method
    pub fn checked_deflate(&self) -> Self {
        if self.contains_deflate() { 
            AcceptCompression { 
                supported: DEFLATE 
            } 
        } else { 
            AcceptCompression::no_compression() 
        } 
    }
    /// Returns a new AcceptCompression that specifies the brotli method
    pub fn checked_brotli(&self) -> Self { 
        if self.contains_brotli() { 
            AcceptCompression { 
                supported: BROTLI 
            } 
        } else { 
            AcceptCompression::no_compression() 
        } 
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AcceptCompression {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> Outcome<AcceptCompression, (Status, ()), ()> {
        let mut supported = 0u8;
        let headers = request.headers();
        
        // Could also use request.accept() instead of request.headers().get("...").next()
        // Actually doesn't work as expected so no not using this
        // println!("Headers: {:?}, Accept: {:?}", 
        //     request.headers().get("Accept-Encoding").next() , 
        //     request.accept() 
        // );
        
        // Referrer Code
        // let referer = headers.get("Referer").next();
        // if let Some(refer) = referer {
        //     println!("Referer: {}", refer);
        // }
        
        // let all_headers: HashMap<String, String> = request.headers();
        
        // let all_headers: HashMap<String, String> = headers
        //                                             .iter()
        //                                             .map(|h| ( h.name().to_string(), h.value().to_string() ) )
        //                                             .collect();
        // println!("All headers: {:#?}", all_headers);
        
        // if let Some(encoding) = request.headers().get("Accept-Encoding").next() {
        // if let Some(encoding) = headers.get("Accept-Encoding").next() {
        if let Some(encoding) = headers.get("Accept-Encoding").next() {
            if encoding.contains("gzip") { supported |= GZIP; }
            if encoding.contains("deflate") { supported |= DEFLATE; }
            if encoding.contains("br") { supported |= BROTLI; }
        }
        // println!("GZIP: {}  BR: {}  DEFLATE: {}", supported & GZIP, supported & BROTLI, supported & DEFLATE);
        Outcome::Success(AcceptCompression { supported })
    }
}



