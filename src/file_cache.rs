

use rocket_contrib::Template;
use rocket::{Request, Data, Outcome, Response};
use rocket::response::{content, NamedFile, Redirect, Flash, Responder, Content};
use rocket::http::{Cookie, Cookies, MediaType, ContentType, Status};

// use concurrent_hashmap::*;

use std::collections::HashMap;
use std::{env, str, io};
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, Instant};

pub struct FileCache {
    pub entries: ,
    pub total_limit: u64,
    pub file_limit: u64,
    
    
}

pub struct CacheEntry {
    pub path: &Path,
    pub last_used: Instant,
    pub added: Instant,
    pub data: Vec<u8>,
    pub access_count: usize,
    
}

lazy_static {
    static ref fcache: Mutex<HashMap<PathBuf, FileCache>> = Mutex::new( HashMap::new() );
    static ref fcache_size: Mutex<u64> = Mutex::new(0);
}

pub fn cache_lookup(path: &Path) -> Option<Vec<u8>> {
    unimplemented!()
}

