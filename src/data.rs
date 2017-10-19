
use rocket;
use ::rocket::request::{FromRequest, FromForm, FormItems};
use ::rocket::Request;
use ::rocket::outcome::Outcome;
use ::rocket::config::{Config, Environment};

use regex::Regex;
use chrono::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};

use blog::*;
use users::*;
use cookie_data::*;

use std::env;
use dotenv::dotenv;
// use diesel::prelude::*;
// use diesel::pg::PgConnection;



pub fn establish_connection() {
    dotenv.ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect("Error connecting to {}", database_url);
}


pub trait HasUsername {
    fn username(&self) -> String;
    fn get_user(&self) -> Self {
        
    }
}

impl HasUsername for AdminCookie {
    fn username(&self) -> String {
        self.username
    }
}





