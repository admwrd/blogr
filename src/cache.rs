


use rocket_contrib::Template;
use rocket::{Request, Data, Outcome, Response};
use rocket::response::{content, NamedFile, Redirect, Flash, Responder, Content};
use rocket::http::{Cookie, Cookies, MediaType, ContentType, Status};

// use concurrent_hashmap::*;

use chrono::{Local, DateTime, TimeZone};
use std::{env, str, fs};
use std::io::{self, BufReader, BufWriter, Cursor, Read};
use std::io::prelude::*;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, Instant, SystemTime};


use super::{FILE_CACHE, CURRENT_CACHE_SIZE};

pub const MAX_CACHE_SIZE: usize = 80000;
pub const FILE_LIMIT: usize = 8000;
pub const UPDATE_AGE: usize = 7200; // 7200 = 2 hrs (3600*2). Check for updated content after this age


// pub struct Cache {
//     pub total_limit: usize,
//     pub file_max: usize,
//     pub max_age: usize, // check last modified and created date
//     pub current_size: usize,
// }
// impl Cache {
//     pub fn new(total_limit: usize, file_max: usize) -> Self {
//         Cache {
//             total_limit,
//             file_max,
//         }
//     }
    
// }


pub struct CacheEntry {
    pub path: PathBuf,
    pub file: File,
    pub uses: usize,
    pub last_access: SystemTime,
    pub added: SystemTime,
    pub modified: SystemTime,
    pub created: SystemTime,
    pub data: Vec<u8>,
}


// https://stackoverflow.com/questions/34832583/global-mutable-hashmap-in-a-library
// Access the hashmap:
// let mut hashmap = HASHMAP.lock().unwrap();

impl CacheEntry {
    
    /// Checks to see if the file needs to be updated
    pub fn is_current(&self) -> bool {
        if let Ok(metadata) = self.path.metadata() {
            let file_created = metadata.created().expect("Could not get file created time");
            let file_modified = metadata.modified().expect("Could not get file modified time");
            
            if self.modified >= file_modified && self.created >= file_created {
                true
            } else {
                false
            }
        } else {
            println!("Failed to retrieve metadata on {}", &self.path.display());
            false
        }
    }
    
    // Maybe change return value to Option<Vec<u8>> ???0
    pub fn retrieve(path: &Path) -> Option<Vec<u8>> {
        let pathbuf = path.to_path_buf();
        
        
        let mut cache = FILE_CACHE.lock().unwrap();
        
        // check existence in cache
        // get a mutable reference to the file's CacheEntry
        if !path.exists() && cache.contains_key(&pathbuf) {
            cache.remove(&pathbuf);
            return None;
        }
        { // used to separate mutable calls cache.get_mut() and cache.insert(pathbuf, cache_entry)
            if let Some(mut cache_entry) = cache.get_mut(&pathbuf) {
                // if exists in cache check if the elapsed time since it was added exceeds the max time
                
                if !cache_entry.is_current() {
                    if let Ok(metadata) = pathbuf.metadata() {
                        if !metadata.is_file() {
                            return None;
                        } else {
                            let mut buffer: Vec<u8> = Vec::new();
                            let rst = cache_entry.file.read_to_end(&mut buffer);
                            cache_entry.data = buffer;
                            cache_entry.uses += 1;
                            cache_entry.last_access = SystemTime::now();
                            return Some(cache_entry.data.clone());
                        }
                    } else {
                        return None;
                    }
                } else {
                    cache_entry.uses += 1;
                    cache_entry.last_access = SystemTime::now();
                    return Some(cache_entry.data.clone());
                }
            } 
        }
        if let Some(cache_entry) = CacheEntry::get_file(&pathbuf) {
            let inserted = cache.insert(pathbuf, cache_entry);
            Some(inserted.expect("error extracting inserted value").data.clone())
        } else {
            None
        }
    }
    
    pub fn get_file(pathbuf: &Path) -> Option<Self> {
        if !pathbuf.exists() {
            None
        } else if let Ok(metadata) = pathbuf.metadata() {
            if !metadata.is_file() {
                None
            } else {
                let file_opt = File::open(pathbuf);
                if let Ok(mut file) = file_opt {
                    let mut buffer: Vec<u8> = Vec::new();
                    let rst = file.read_to_end(&mut buffer);
                    let cache_entry = CacheEntry {
                        path: pathbuf.to_path_buf(),
                        file: file,
                        uses: 1,
                        last_access: SystemTime::now(),
                        added: SystemTime::now(),
                        modified: metadata.modified().unwrap_or(SystemTime::now()),
                        created: metadata.created().unwrap_or(SystemTime::now()),
                        data: buffer,
                    };
                    Some(cache_entry)
                    
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}








































