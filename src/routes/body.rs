
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

use super::super::*;
use super::*;
use ::blog::*;
use ::data::*;
use ::content::*;
use ::templates::*;
use ::xpress::*;
use ::ral_user::*;
use ::ral_administrator::*;


pub trait BodyContext {
    // fn content() -> String;
}

#[derive(Debug, Clone, Serialize)]
pub struct CtxBody<T: BodyContext>(T);

impl BodyContext for TemplateArticlesPages {}
impl BodyContext for TemplateGeneral {}
impl BodyContext for TemplateArticle {}
impl BodyContext for TemplateTags {}

// Admin pages should not be cached so the structs below should not
// need to be used, they may but they are not needed (most likely)
impl BodyContext for TemplateLogin {}
impl BodyContext for TemplateLoginData {}
impl BodyContext for TemplateCreate {}
impl BodyContext for TemplateEdit {}
impl BodyContext for TemplateSearch {}
impl BodyContext for TemplateManage {}
impl BodyContext for TemplateArticles {} // Is this still used??


// pub struct ArticleBody {
    
// }

// impl BodyContext for ArticleBody {}


// pub struct ArticlesBody {
    
// }

// impl BodyContext for ArticlesBody {}





// pub struct CtxInfo {
    
// }


// pub fn article(article_opt: Option<Article>) -> (CtxBody<TemplateArticle>, CtxInfo) {
// pub fn article(article_opt: Option<Article>) -> CtxBody<TemplateArticle> {
// pub fn article<T: BodyContext>(body: CtxBody<T>, admin: Option<AdministratorCookie>, user: Option<UserCookie>, uhits: Option<UniqueHits>, gen: Option<GenTimer>, msg: Option<String>) -> CtxBody<TemplateArticle> {
// pub fn article<T: BodyContext>(body: Option<Article>, admin: Option<AdministratorCookie>, user: Option<UserCookie>, uhits: Option<UniqueHits>, gen: Option<GenTimer>, msg: Option<String>) -> CtxBody<TemplateArticle> {

pub fn article(body: Option<Article>, admin: Option<AdministratorCookie>, user: Option<UserCookie>, uhits: Option<UniqueHits>, gen: Option<GenTimer>, msg: Option<String>) -> CtxBody<TemplateArticle> {
    unimplemented!()
}



mod article {
    use super::*;
    // Could make it look like:
    // serve(body: CtxBody<BodyArticle>, info: CtxInfo)
    // with CtxInfo looking like:
    /*   CtxInfo {
            admin: Option<AdministratorCookie>, 
            user: Option<UserCookie>, 
            uhits: Option<UniqueHits>, 
            gen: Option<GenTimer>, 
            msg: Option<String>
        }
    */ 
    pub fn serve(body: Option<Article>, admin: Option<AdministratorCookie>, user: Option<UserCookie>, uhits: Option<UniqueHits>, gen: Option<GenTimer>, msg: Option<String>) -> CtxBody<TemplateArticle> {
        unimplemented!()
    }
    // pub fn 
}

mod articles {
    use super::*;
    pub fn serve(body: Option<Vec<Article>>, admin: Option<AdministratorCookie>, user: Option<UserCookie>, uhits: Option<UniqueHits>, gen: Option<GenTimer>, msg: Option<String>) -> CtxBody<TemplateArticle> {
        unimplemented!()
        
    }
    
}


















