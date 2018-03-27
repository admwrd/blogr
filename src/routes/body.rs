
use rocket_contrib::Template;
use rocket::{Request, Data, Outcome, Response};
// use rocket::response::{content, NamedFile, Redirect, Flash, Responder, Content};
use rocket::response::{NamedFile, Redirect, Flash, Responder, Content};
use rocket::response::content::Html;
use rocket::data::FromData;
use rocket::request::{FlashMessage, Form, FromForm, FormItems, FromRequest};
use rocket::http::{Cookie, Cookies, MediaType, ContentType, Status};
use rocket::State;

use std::fmt::Display;
use std::{env, str, thread};
use std::fs::{self, File, DirEntry};
use std::io::prelude::*;
use std::io::{self, Cursor, Read};
use std::path::{Path, PathBuf};
use std::time::{self, Instant, Duration};
use std::prelude::*;
use std::ffi::OsStr;
use std::collections::HashMap;
use std::sync::{Mutex, Arc, RwLock};
use std::sync::atomic::AtomicUsize;

use std::borrow::Cow;

use evmap::*;
use comrak::{markdown_to_html, ComrakOptions};


// mod body_options;
// mod page_routes;

// use super::super::*;
use super::*;
use blog::*;
use data::*;
use content::*;
use templates::*;
use xpress::*;


pub trait BodyContext {
    // fn content() -> String;
}

#[derive(Debug, Clone, Serialize)]
pub struct CtxBody<T: BodyContext>(T);

impl BodyContext for TemplateLogin {}
impl BodyContext for TemplateLoginData {}
impl BodyContext for TemplateCreate {}
impl BodyContext for TemplateEdit {}
impl BodyContext for TemplateGeneral {}
impl BodyContext for TemplateArticle {}
impl BodyContext for TemplateArticles {}
impl BodyContext for TemplateSearch {}
impl BodyContext for TemplateTags {}
impl BodyContext for TemplateArticlesPages {}
impl BodyContext for TemplateManage {}


// pub struct ArticleBody {
    
// }

// impl BodyContext for ArticleBody {}


// pub struct ArticlesBody {
    
// }

// impl BodyContext for ArticlesBody {}





pub struct CtxInfo {
    
}


// pub fn article(article_opt: Option<Article>) -> (CtxBody<TemplateArticle>, CtxInfo) {
pub fn article(article_opt: Option<Article>) -> CtxBody<TemplateArticle> {
    unimplemented!()
}
















