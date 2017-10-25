
use std::collections::{HashMap, BTreeMap};
use rocket_contrib::Template;
use chrono::{NaiveDate, NaiveDateTime};
// use serde::ser::Serialize;

use blog::*;
use layout::*;

use cookie_data::*;
use admin_auth::*;
use user_auth::*;

use users::*;

// #[derive(Serialize)]
// pub struct TemplateMenu {
//     pub link_title: &'a str,
//     pub link_url: &'b str,
// }

#[derive(Debug, Clone)]
pub enum TemplateBody<'g, 's, 'm> {
    General(&'g str),
    Article(&'s Article),
    Articles(&'m Vec<Article>),
}

#[derive(Debug, Clone, Serialize)]
pub struct TemplateInfo<'t, 'u, 'j> {
    pub title: &'t str,
    pub logged_in: bool,
    pub is_admin: bool,
    pub is_user: bool,
    pub username: &'u str,
    pub js: &'j str,
}

#[derive(Debug, Clone, Serialize)]
pub struct TemplateGeneral<'b, 't, 'u, 'j> {
    pub body: &'b str,
    pub info: TemplateInfo<'t, 'u, 'j>,

}
#[derive(Debug, Clone, Serialize)]
pub struct TemplateArticle<'t, 'u, 'j> {
    pub body: ArticleDisplay,
    pub info: TemplateInfo<'t, 'u, 'j>,

}
#[derive(Debug, Clone, Serialize)]
pub struct TemplateArticles<'t, 'u, 'j> {
    pub body: Vec<ArticleDisplay>,
    pub info: TemplateInfo<'t, 'u, 'j>,

}

impl<'t, 'u, 'j> TemplateInfo<'t, 'u, 'j> {
    pub fn new<'p, 's>(title: Option<&'p str>, admin: Option<AdminCookie>, user: Option<UserCookie>, js: &'s str) -> TemplateInfo<'t, 'u, 'j> {
        TemplateInfo {
            title: if let Some(t) = title { t } else { "" },
            logged_in: if admin.is_some() || user.is_some() { true } else { false },
            is_admin: if admin.is_some() { true } else { false },
            is_user: if user.is_some() { true } else { false },
            username: if let Some(a) = admin { &a.username } else if let Some(u) = user { &u.username } else { "" },
            js,
        }
    }
    
}

// pub fn new(content: &'a str, title: Option<&'b str>, admin: Option<AdminCookie>, user: Option<UserCookie>, menu: Vec<(&str, &str)>, admin_menu: Vec<(&str, &str)>) -> TemplateItems {
// body: if let TemplateBody::General(contents) = content { contents } else { "Invalid contents." },
// body: if let TemplateBody::Article(article) = content { contents } else { Article {aid: 0, title: String::from("Invalid Article"), Local::now().naive_local(), body: String::from("Invalid template contents."), Vec::new(), String::from("Invalid article template.")} },
// body: if let TemplateBody::Article(article) = content { contents } else { Article {aid: 0, title: String::from("Invalid Article"), Local::now().naive_local(), body: String::from("Invalid template contents."), Vec::new(), String::from("Invalid article template.")} },

impl<'b, 't, 'u, 'j> TemplateGeneral<'b, 't, 'u, 'j>  {
    pub fn new<'c>(content: &'c str, info: TemplateInfo) -> TemplateGeneral<'b, 't, 'u, 'j> {
        TemplateGeneral {
            body: content,
            info
        }
    }
}
impl<'t, 'u, 'j> TemplateArticle<'t, 'u, 'j>  {
    pub fn new<'d>(content: &'d Article, info: TemplateInfo) -> TemplateArticle<'t, 'u, 'j> {
        TemplateArticle {
            body: content.to_display(),
            info,
        }
    }
}
impl<'t, 'u, 'j> TemplateArticles<'t, 'u, 'j>  {
    pub fn new<'e>(content: &'e Vec<Article>, info: TemplateInfo) -> TemplateArticles<'t, 'u, 'j> {
        let mut articles: Vec<ArticleDisplay> = content.iter().map(|a| a.to_display()).collect();
        TemplateArticles {
            body: articles,
            info,
        }
    }
}

pub fn hbs_template<'a, 'b>(content: TemplateBody, title: Option<&'b str>, admin_opt: Option<AdminCookie>, user_opt: Option<UserCookie>, javascript: Option<&str>) -> Template {
    // let mut context: HashMap<&str> = HashMap::new();
    // context.insert();
    let js = if let Some(j) = javascript { j } else { "" }; 
    let info = TemplateInfo::new(title, admin_opt, user_opt, js);
    match content {
        TemplateBody::General(contents) => {
            let context = TemplateGeneral::new(content, info);
            Template::render("general-template", &context)
        },
        TemplateBody::Article(article) => {
            let context = TemplateArticle::new(article, info);
            Template::render("article-template", &context)
        },
        TemplateBody::Articles(articles) => {
            // let context = TemplateArticles::new(articles, info);
            // Template::render("articles-template", &context)
            let context = TemplateGeneral::new("ARTICLES NOT YET IMPLEMENTED.", info);
            Template::render("general-template", &context)
        },
    }
    
}






























