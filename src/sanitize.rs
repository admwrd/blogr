

use regex::Regex;
use htmlescape::*;

// escapes html tags and special characters
pub fn input_sanitize(string: String) -> String {
    string
}
// removes non-word characters
pub fn strict_sanitize(string: String) -> String {
    // use lazy_static! to make a regexp to remove everything but word characters
    lazy_static! {
        static ref STRICT: Regex = Regex::new(r#"\W+"#).unwrap();
    }
    STRICT.replace_all(&string, "").into_owned()
}
// leaves spaces, commas, hyphens, and underscores but removes all other non-word characters
pub fn medium_sanitize(string: String) -> String {
    encode_minimal(&string)
}

pub fn sanitize_sql(string: String) -> String {
    lazy_static! {
        static ref CLEAN_SQL: Regex = Regex::new(r#"(['"\\])"#).unwrap();
    }
    CLEAN_SQL.replace_all(&string, r"\\$1").into_owned()
    // string.replace("'", "\'")
    // .replace(r"\", r"\\");
    // .replace(r#"""#, r#"\""#);
}

pub fn str_is_numeric(string: String) -> bool {
    lazy_static! {
        static ref NUMERIC: Regex = Regex::new(r"^\d+$").unwrap();
    }
    NUMERIC.is_match(&string)
}

pub fn sanitize_attribute(string: String) -> String {
    encode_attribute(&string)
}

pub fn sanitize_body(string: String) -> String {
    // escape html entities/elements
    // unimplemented!()
    encode_minimal(&string)
}

pub fn sanitize_title(string: String) -> String {
    // set max length to 120 characters
    encode_minimal(&string)
    // unimplemented!()
}

pub fn sanitize_tags(string: String) -> String {
    encode_minimal(&string)
    // unimplemented!()
}
pub fn split_tags(string: String) -> Vec<String> {
    let tags: Vec<String> = string.to_lowercase().split(',').filter(|t| t != &"").map(|t| t.to_string()).collect();
    tags
}



