
use articles::*;

use regex::Regex;
use chrono::prelude::*;

#[derive(Debug, Clone)]
pub struct Tag {
    tag: String, // Limit to 32 characters
}

#[derive(Debug, Clone)]
pub struct Category {
    category: u32,
    name: String,
}

pub fn sanitize_output(string: String) -> String {
    // escape html tags
    unimplemented!()
}

pub fn shorten(string: &str, length: u32) -> String {
    if string.len() > length {
        let shortened = &string[..length];
        sanitize_output(shortened.to_string())
    } else {
        sanitize_output(string.to_string())
    }
}

impl Category {
    fn display(&self) -> String {
        // format!("{title}", title=self.name);
        self.name.clone()
    }
    fn retrieve_articles() -> Vec<Article> {
        unimplemented!()
    }
}

impl Tag {
    fn display(&self) -> String {
        self.tag.clone()
    }
    fn retrieve_articles() -> Vec<Article> {
        unimplemented!()
    }
}

