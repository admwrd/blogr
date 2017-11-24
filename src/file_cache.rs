

use rocket_contrib::Template;
use rocket::{Request, Data, Outcome, Response};
use rocket::response::{content, NamedFile, Redirect, Flash, Responder, Content};
use rocket::http::{Cookie, Cookies, MediaType, ContentType, Status};

use std::{env, str, io};
use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

pub struct FileCache {
    pub path: &Path,
    
    
}

lazy_static {
    let file_cache = Mutex<HashMap<PathBuf, Vec<u8>>>
    
}


