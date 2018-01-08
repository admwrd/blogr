

use regex::Regex;
use htmlescape::*;

pub fn sanitize_tag(tag: &str) -> String {
    lazy_static! {
        static ref IS_SANITARY_TAG: Regex = Regex::new(r#"^[\w\d\s]*$"#).unwrap();
        static ref SANITIZE_TAG: Regex = Regex::new(r#"[^\w\d\s]+"#).unwrap();
    }
    if IS_SANITARY_TAG.is_match(tag) {
        tag.to_string()
    } else {
        SANITIZE_TAG.replace_all(tag, "").to_string()
    }
}


// used to sanitize usernames
pub fn sanitize(string: &str) -> String {
    lazy_static! {
        static ref SANITARY: Regex = Regex::new(r#"^\w+$"#).unwrap();
        static ref SANITIZE: Regex = Regex::new(r#"\W+"#).unwrap();
    }
    if SANITARY.is_match(string) {
        string.to_string()
    } else {
        SANITIZE.replace_all(string, "").to_string()
    }
}

// used to sanitize passwords
pub fn sanitize_password(string: &str) -> String {
    lazy_static! {
        static ref SANITARY_PASSWORD: Regex = Regex::new(r#"^[A-Fa-f0-9]+$"#).unwrap();
        static ref SANITIZE_PASSWORD: Regex = Regex::new(r#"[^A-Fa-f0-9]+"#).unwrap();
    }
    if SANITARY_PASSWORD.is_match(string) {
        string.to_string()
    } else {
        SANITIZE_PASSWORD.replace_all(string, "").to_string()
    }
}


// escapes html tags and special characters
pub fn input_sanitize(string: String) -> String {
    encode_minimal(&string)
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

/// Postgres uses two single quotes to specify a single quote in text.
/// The extra single quote acts as an escape since postgresql uses this by default
/// instead of the Posix backslash escape which can be specified by prefixing the
/// string in the sql statement with an E.
pub fn escape_sql_pg(mut string: String) -> String {
    string.replace("'", "''")
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
    let tags: Vec<String> = string.to_lowercase().split(',').filter(|t| t != &"").map(|t| sanitize_tag(t.trim())).collect();
    tags
}



