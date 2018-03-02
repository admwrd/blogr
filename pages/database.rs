
use std::ops::Deref;
use std;
use std::env;

use ::rocket::request::{self, FromRequest, FromForm, FormItems};
use rocket::{Request, State, Outcome};
use ::rocket::config::{Config, Environment};
use rocket::http::Status;
use r2d2;
use r2d2_postgres;
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use postgres::Connection;
use postgres;
use postgres::params::{ConnectParams, Host};

type Pool = r2d2::Pool<PostgresConnectionManager>;

// Creates the database connection pool
pub fn init_pg_pool() -> Pool {
    let config = r2d2::Config::default();
    let manager = PostgresConnectionManager::new("postgres://postgres:password@localhost/blogdb", TlsMode::None).expect("Could not connect to database using specified connection string.");
    r2d2::Pool::new(config, manager).expect("Could not create database pool")
}

pub struct DbConn(
    pub r2d2::PooledConnection<PostgresConnectionManager>
);

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

// For the convenience of using &DbConn
impl Deref for DbConn {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

