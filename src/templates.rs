
use std::collections::{HashMap, BTreeMap};
use rocket_contrib::Template;

use blog::*;
use layout::*;

use cookie_data::*;
use admin_auth::*;
use user_auth::*;

use users::*;

#[derive(Serialize)]
pub struct<'a, 'b> TemplateMenu {
    pub link_title: &'a str,
    pub link_url: &'b str,
}

#[derive(Serialize)]
pub struct TemplateItems {
    pub body: &str,
    pub title: &str,
    pub menu: Vec<TemplateMenu>,
    pub js: &str,
}

/* Template Context:
    body            &str    Raw
    title           &str
    Menu            Menu
        link_title  &str    Raw
        link_url    &str    Raw
    js              &str    Raw
    

*/

pub fn hbs_generic_page<'a, 'b>(content: &'a str, title: Option<&'b str>, admin_opt: Option<AdminCookie>, user_opt: Option<UserCookie>) -> Template {
    // let mut context: HashMap<&str> = HashMap::new();
    // context.insert();
    
    
    Template::render("index", &context)
}





pub fn generic_template() -> Template {
    
}





























