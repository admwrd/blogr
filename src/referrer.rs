

use rocket::http::Header;

use rocket::request::{FromRequest, Request};
use rocket::Outcome;



pub struct Referrer(pub Option<String>);

impl<'a, 'r> FromRequest<'a, 'r> for Referrer {
    type Error = ();
    
    fn from_request(req: &'a Request<'r>) -> ::rocket::request::Outcome<Referrer,Self::Error> {
        let referrer: Option<&str> = req.headers().get("Referer").next();
        if let Some(refer) = referrer {
            // println!("Referer: {}", refer);
            Outcome::Success(Referrer(Some(refer.to_string())))
        } else {
            Outcome::Success(Referrer(None))
            // Outcome::Forward(())
        }
    }
}
