
// use rocket;
use ::rocket::request::{self, FromRequest, FromForm, FormItems};
use rocket::{Request, State, Outcome};
use ::rocket::config::{Config, Environment};
use rocket::http::Status;

use regex::Regex;
use chrono::prelude::*;
use chrono::{NaiveDate, NaiveDateTime};

use titlecase::titlecase;

use blog::*;
// not used anymore
// use users::*;
// use cookie_data::*;

// use rocket::request::{self, FromRequest};

use r2d2;
use r2d2_postgres;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
// use postgres::{Connection, TlsMode};
use postgres::Connection;
use postgres;
use postgres::params::{ConnectParams, Host};
// use postgres::SslMode;
// use postgres::TlsMode;

use std::ops::Deref;
use std;
use std::env;
// use dotenv::dotenv;
use super::{DATABASE_URL, DESC_LIMIT};
// use diesel;
// use diesel::prelude::*;
// use diesel::pg::PgConnection;


// https://github.com/sfackler/rust-postgres/issues/128
// let stmt = try!(conn.prepare("INSERT INTO foo (bar) VALUES ('baz') RETURNING id"));
// let id: i32 = try!(stmt.query(&[])).iter().next().unwrap().get(0);




// https://sfackler.github.io/r2d2-postgres/doc/v0.9.2/r2d2_postgres/struct.PostgresConnectionManager.html
// https://medium.com/@aergonaut/writing-a-github-webhook-with-rust-part-1-rocket-4426dd06d45d
// https://github.com/aergonaut/railgun/blob/master/src/railgun/db.rs

/// Type alias for the r2d2 connection pool. Use this as a State<T> parameter
/// in handlers that need a database connection.
// pub type ConnectionPool = r2d2::Pool<r2d2_diesel::ConnectionManager<diesel::pg::PgConnection>>;
type Pool = r2d2::Pool<PostgresConnectionManager>;



/// Creates the database connection pool
pub fn init_pg_pool() -> Pool {
    // let conn_str = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let config = r2d2::Config::default();
    
    // let manager = PostgresConnectionManager::new("postgres://postgres:andrew@localhost/blog", TlsMode::None).unwrap();
    
    // dotenv().ok();
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = PostgresConnectionManager::new(DATABASE_URL, TlsMode::None).expect("Could not connect to database using specified connection string.");
    
    r2d2::Pool::new(config, manager).expect("Could not create database pool")
}

pub fn init_pg_conn() -> Connection {
    // let conn_str = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // Connection::connect("postgres://postgres:andrew@localhost/blog", postgres::TlsMode::None).unwrap()
    Connection::connect(DATABASE_URL, postgres::TlsMode::None).unwrap()
}

pub struct DbConn(
    pub r2d2::PooledConnection<PostgresConnectionManager>
);

impl DbConn {
    /// If called like: conn.articles("") it will return all articles.  The description of the article is used if it exists otherwise a truncated body is returned; to return articles will their full body contents use `conn.articles_full("")`.
    pub fn articles(&self, qrystr: &str) -> Option<Vec<Article>> {
        
        let qryrst: Result<_, _> = if qrystr != "" {
            self.query(qrystr, &[])
        } else {
            self.query(&format!("SELECT a.aid, a.title, a.posted, description({}, a.body, a.description), a.tag, a.description, u.userid, u.display, u.username, a.image, a.markdown, a.modified FROM articles a JOIN users u ON (a.author = u.userid)", DESC_LIMIT), &[])
        };
        if let Ok(result) = qryrst {
            let mut articles: Vec<Article> = Vec::new();
            for row in &result {
                
                let display: Option<String> = row.get(7);
                let username: String = if let Some(disp) = display { disp } else { row.get(8) };
                let image: String = row.get_opt(9).unwrap_or(Ok(String::new())).unwrap_or(String::new());
                
                let a = Article {
                    aid: row.get(0),
                    title: row.get(1),
                    posted: row.get(2),
                    body: row.get(3),
                    tags: row.get_opt(4).unwrap_or(Ok(Vec::<String>::new())).unwrap_or(Vec::<String>::new()).into_iter().map(|s| s.trim_matches('\'').trim().to_string()).filter(|s| s.as_str() != "").collect(),
                    // tags: row.get_opt(4).unwrap_or(Ok(Vec::<String>::new())).unwrap_or(Vec::<String>::new()).into_iter().map(|s| s.trim().trim_matches('\'').to_string()).collect(),
                    description: row.get_opt(5).unwrap_or(Ok(String::new())).unwrap_or(String::new()),
                    userid: row.get(6),
                    username: titlecase( &username ),
                    markdown: row.get_opt(10).unwrap_or(Ok(String::new())).unwrap_or(String::new()),
                    image,
                    modified: row.get(11),
                };
                articles.push(a);
            }
            Some(articles)
        } else {
            None
        }
    }
    /// Runs a query returning articles from the database.  If the text passed in is equal to "" then the default 
    /// query is to return all articles with full body content.
    pub fn articles_full(&self, qrystr: &str) -> Option<Vec<Article>> {
        let qry = if qrystr != "" { qrystr } else { "SELECT a.aid, a.title, a.posted, a.body, a.tag, a.description, u.userid, u.display, u.username, a.image, a.markdown, a.modified  FROM articles a JOIN users u ON (a.author = u.userid)" };
        self.articles(qry)
    }
}

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

// For the convenience of using an &DbConn as an &SqliteConnection.
impl Deref for DbConn {
    // type Target = SqliteConnection;
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


pub fn establish_connection() -> Connection {
    // dotenv().ok();
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // PgConnection::establish(&database_url).expect("Error connecting to {}", database_url);
    // Connection::connect("postgres://postgres@localhost:5433", TlsMode::None).unwrap()
    // Connection::connect(database_url, postgres::TlsMode::None).unwrap()
    Connection::connect(DATABASE_URL, postgres::TlsMode::None).unwrap()
}

// Commented out because the DotEnv crate isn't required anywhere else
// and these functions are not used.  They are left here because they
// could be useful at some point, someday but not immediately.
//
// pub fn establish_connection_dotenv() -> Connection {
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     Connection::connect(database_url, postgres::TlsMode::None).unwrap()
// }



// Possible way to retrieve a user
// pub trait HasUsername {
//     fn username(&self) -> String;
//     fn retrieve_user(&self) -> Self;
// }

// impl HasUsername for AdminCookie {
//     fn username(&self) -> String {
//         self.username.clone()
//     }
//     fn retrieve_user(&self) -> Self {
//         unimplemented!()
//     }
// }





