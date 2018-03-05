
use super::{BLOG_URL, COMRAK_OPTIONS, BASE, DEFAULT_PAGE_TEMPLATE, PAGE_TEMPLATES, STATIC_PAGES_DIR};
use content::PageContext;

// use std::fmt::Display;
// use std::{env, str, thread};
// use std::fs::{self, File, DirEntry};
// use std::io::prelude::*;
// use std::io::{self, Cursor, Read};
// use std::path::{Path, PathBuf};
// use std::time::{self, Instant, Duration};
// use std::prelude::*;
// use std::ffi::OsStr;
// use std::collections::HashMap;
// use std::sync::{Mutex, Arc, RwLock};
// use std::sync::atomic::AtomicUsize;
// use rocket;
// use rocket::http::Status;
// use rocket::State;
// use rocket_contrib::Template;
// use rocket::response::{self, Response, Responder};
// use rocket::request::{FromRequest, Request};
// use rocket::Outcome;
// use rocket::Outcome::Success;
// use rocket::response::NamedFile;
// use rocket::http::{ContentType, Header, HeaderMap};
// use ::rocket::request::{FromRequest, FromForm, FormItems, FromFormValue, FromParam};
// use ::rocket::outcome::Outcome;
// use rocket::http::RawStr;
// use rocket::response::{content, NamedFile, Redirect, Flash};

use comrak::{markdown_to_html, ComrakOptions};
use twoway;
use titlecase::*;
use regex::Regex;

use ::serde::{Deserialize, Serialize};
use serde_json::{Value, Error};

use aho_corasick::{Automaton, AcAutomaton, Match};



// In the Responder for ContentRequest use the HashMap in ContentContext
//   to retrieve the body contents of the static file
// In /article route send the body contents to the toc generate function
//   only individual articles should show a table of contents

/*



*/

pub struct<'a> Head {
    // header level, 1-6
    pub level: u8,
    // string slice referencing the original body text of the header
    pub title: &'a str,
    // the id of the header tag (convert contents to lowercase then replace spaces with hypens)
    pub id: String,
}


// pub fn find_headers(body: &str) -> {
    
// }


pub fn generate(body: &str) -> String {
    lazy_static! {
        static ref FIND_HEADERS: Regex = Regex::new(r#"^<h[]></h>$"#).unwrap();
    }
    
    
    
}









    /*
    let aut = AcAutomaton::new(vec!["apple", "maple"]);
    let mut it = aut.find("I like maple apples.");
    assert_eq!(it.next(), Some(Match {
        pati: 1,
        start: 7,
        end: 12,
    }));
    assert_eq!(it.next(), Some(Match {
        pati: 0,
        start: 13,
        end: 18,
    }));
    
    // headers automation
    let hsa = AcAutomation::new(vec!["<h1>", ""])
    */
























