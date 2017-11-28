

use std::time::{Instant, Duration, SystemTime};

use std::{env, str};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use std::io::{self, BufReader, BufWriter, Cursor, Read};
use std::io::prelude::*;
use std::fs::File;

use rocket::State;

use xpress::*;
use chashmap::*;


pub const MAX_CACHE_SIZE: usize = 80000;
pub const FILE_LIMIT: usize = 8000;
pub const UPDATE_AGE: u64 = 7200;


pub struct VEntry {
    pub added: SystemTime,
    pub access: SystemTime,
    pub created: SystemTime,
    pub modified: SystemTime,
    pub uses: usize,
    pub path: PathBuf,
    pub data: Vec<u8>,
}

lazy_static! {
    static ref CACHE_SIZE: Mutex<u64> = Mutex::new(0);
}

pub struct VCache (pub CHashMap<PathBuf, VEntry> );

unsafe impl Send for VCache {}


impl VCache {
    pub fn retrieve(path: PathBuf, cache: State<VCache>) -> Option<Vec<u8>> {
    // pub fn retrieve(&self, path: PathBuf, cache: State<VCache>) -> Option<Vec<u8>> {
        // check if the file is in the cache
        
        println!("Retreiving cache item: {}", &path.display());
        
        if let Some(mut cached_item) = cache.0.get_mut(&path) {
            if let Ok(mut file) = File::open(&path) {
                if let Ok(metadata) = file.metadata() {
                    // check if the cached item needs to be updated
                    if cached_item.added.elapsed().unwrap_or(Duration::new(UPDATE_AGE+1, 0)) > Duration::new(UPDATE_AGE, 0) {
                        if metadata.created().unwrap() > cached_item.created || metadata.modified().unwrap() > cached_item.modified {
                            
                            println!("Cache item is expired, updating {}", &path.display());
                            
                            let mut buffer: Vec<u8> = Vec::new();
                            let bytes = file.read_to_end(&mut buffer);
                            
                            // Todo: Cache Size:
                            //      subtract the original data size from the cache size
                            //      add the new data size to the cache size
                            
                            cached_item.data = buffer;
                            cached_item.added = if metadata.modified().unwrap() > metadata.created().unwrap() {
                                metadata.modified().unwrap()
                            } else {
                                metadata.created().unwrap()
                            };
                            
                        }
                    }
                    
                    println!("Updating cache item `{}` access count and returning {} bytes", &path.display(), cached_item.data.len());
                    
                    cached_item.access = SystemTime::now();
                    cached_item.uses += 1;
                    cached_item.access = SystemTime::now();
                    Some(cached_item.data.clone())
                } else {
                    println!("Cache item `{}` retrieval failed 1", &path.display());
                    None
                }
            } else {
                println!("Cache item `{}` retrieval failed 2", &path.display());
                None
            }
        } else {
            let file_opt = File::open(&path);
            if let Ok(mut file) = file_opt {
                if let Ok(metadata) = file.metadata() {
                    let mut data: Vec<u8> = Vec::new();
                    let bytes = file.read_to_end(&mut data);
                    
                    // Todo: limit the size of the cache
                    // {
                        // let current_size = CACHE_SIZE.lock().expect("Could not unlock cache size mutex");
                        // if ( data.len()+current_size ) > MAX_CACHE_SIZE {
                            // iterate through the cache and remove the oldest items
                            // keep removing oldest items from oldest until newer until
                            // the total size freed is equal to the new item size
                        // }
                    // }
                    
                    let output = data.clone();
                    let item = VEntry {
                                    added: SystemTime::now(),
                                    access: SystemTime::now(),
                                    created: metadata.created().expect("Could not retrieve cache item metadata"),
                                    modified: metadata.modified().expect("Could not retrieve cache item metadata."),
                                    uses: 1,
                                    path: path.clone(),
                                    data,
                    };
                    
                    println!("Inserting cache item: {} with {} bytes", &path.display(), item.data.len());
                    
                    cache.0.insert_new(path, item);
                    
                    // Todo: Cache Size
                    // add the size of data to the cache size
                    
                    Some(output)
                } else {
                    println!("Cache item `{}` retrieval failed 3", &path.display());
                    None
                }
            } else {
                println!("Cache item `{}` retrieval failed 4", &path.display());
                None
            }
        }
    }
    
}




