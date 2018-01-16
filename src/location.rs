
use rocket::request::{FromRequest, Request};
use rocket::Outcome;
use rocket::response::Redirect;



pub struct Location(pub String);

impl<'a, 'r> FromRequest<'a, 'r> for Location {
    type Error = ();
    
    fn from_request(req: &'a Request<'r>) -> ::rocket::request::Outcome<Location, Self::Error> {
        let route = req.uri().as_str();
        Outcome::Success( Location( route.to_owned() ) )
    }
}


// pub fn admin_login(request: &Request) -> Redirect {
// pub fn admin_login(route: String) -> Redirect {
pub fn admin_login(route: Location) -> Redirect {
    // let route = request.uri().as_str();
    let mut redir = String::with_capacity(route.0.len() + 20);
    redir.push_str("/admin?referrer=");
    redir.push_str(&route.0);
    
    Redirect::to(&redir)
}