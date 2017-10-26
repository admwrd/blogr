
use std::collections::{HashMap, BTreeMap};
use rocket_contrib::Template;
use chrono::{NaiveDate, NaiveDateTime};
use titlecase::titlecase;

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
pub enum TemplateBody {
    General(String),
    Article(Article),
    Articles(Vec<Article>),
}

#[derive(Debug, Clone, Serialize)]
pub struct TemplateInfo {
    pub title: String,
    pub logged_in: bool,
    pub is_admin: bool,
    pub is_user: bool,
    pub username: String,
    pub js: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TemplateGeneral {
    pub body: String,
    pub info: TemplateInfo,

}
#[derive(Debug, Clone, Serialize)]
pub struct TemplateArticle {
    pub body: ArticleDisplay,
    pub info: TemplateInfo,

}
#[derive(Debug, Clone, Serialize)]
pub struct TemplateArticles {
    pub body: Vec<ArticleDisplay>,
    pub info: TemplateInfo,

}

impl TemplateInfo {
    pub fn new(title: Option<String>, admin: Option<AdminCookie>, user: Option<UserCookie>, js: String) -> TemplateInfo {
        TemplateInfo {
            title: if let Some(t) = title { t } else { String::new() },
            logged_in: if admin.is_some() || user.is_some() { true } else { false },
            is_admin: if admin.is_some() { true } else { false },
            is_user: if user.is_some() { true } else { false },
            username: if let Some(a) = admin { titlecase(&a.username.clone()) } else if let Some(u) = user { titlecase(&u.username.clone()) } else { String::new() },
            js,
        }
    }
    
}

// pub fn new(content: &'a str, title: Option<&'b str>, admin: Option<AdminCookie>, user: Option<UserCookie>, menu: Vec<(&str, &str)>, admin_menu: Vec<(&str, &str)>) -> TemplateItems {
// body: if let TemplateBody::General(contents) = content { contents } else { "Invalid contents." },
// body: if let TemplateBody::Article(article) = content { contents } else { Article {aid: 0, title: String::from("Invalid Article"), Local::now().naive_local(), body: String::from("Invalid template contents."), Vec::new(), String::from("Invalid article template.")} },
// body: if let TemplateBody::Article(article) = content { contents } else { Article {aid: 0, title: String::from("Invalid Article"), Local::now().naive_local(), body: String::from("Invalid template contents."), Vec::new(), String::from("Invalid article template.")} },

impl TemplateGeneral  {
    pub fn new(content: String, info: TemplateInfo) -> TemplateGeneral {
        TemplateGeneral {
            body: content,
            info
        }
    }
}
impl TemplateArticle {
    pub fn new(content: Article, info: TemplateInfo) -> TemplateArticle {
        TemplateArticle {
            body: content.to_display(),
            info,
        }
    }
}
impl TemplateArticles {
    pub fn new(content: Vec<Article>, info: TemplateInfo) -> TemplateArticles {
        let mut articles: Vec<ArticleDisplay> = content.iter().map(|a| a.to_display()).collect();
        TemplateArticles {
            body: articles,
            info,
        }
    }
}

pub fn hbs_template(content: TemplateBody, title: Option<String>, admin_opt: Option<AdminCookie>, user_opt: Option<UserCookie>, javascript: Option<String>) -> Template {
    // let mut context: HashMap<&str> = HashMap::new();
    // context.insert();
    let js = if let Some(j) = javascript { j } else { "".to_string() }; 
    let info = TemplateInfo::new(title, admin_opt, user_opt, js);
    match content {
        TemplateBody::General(contents) => {
            let context = TemplateGeneral::new(contents, info);
            Template::render("general-template", &context)
        },
        TemplateBody::Article(article) => {
            let context = TemplateArticle::new(article, info);
            Template::render("article-template", &context)
        },
        TemplateBody::Articles(articles) => {
            let context = TemplateArticles::new(articles, info);
            Template::render("articles-template", &context)
            // let context = TemplateGeneral::new("ARTICLES NOT YET IMPLEMENTED.".to_string(), info);
            // Template::render("general-template", &context)
        },
    }
    
}






























