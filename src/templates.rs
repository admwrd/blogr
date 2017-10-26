
use std::collections::{HashMap, BTreeMap};
use rocket_contrib::Template;
use chrono::{NaiveDate, NaiveDateTime};
use titlecase::titlecase;
use std::time::{Instant, Duration};

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
    Login (
        String, // Form Action URL
        Option<String>, // username that was entered
        Option<String>, // fail message
    ),
    Create(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct TemplateInfo {
    pub title: String,
    pub logged_in: bool,
    pub is_admin: bool,
    pub is_user: bool,
    pub username: String,
    pub js: String,
    pub gentime: String,
}

// #[derive(Debug, Clone, Serialize)]
// pub struct TemplateLogin {
//     pub tried_user: String,
//     pub msg: String,
//     pub title: String,
//     pub logged_in: bool,
//     pub is_admin: bool,
//     pub is_user: bool,
//     pub username: String,
//     pub js: String,
//     pub gentime: String,
// }

#[derive(Debug, Clone, Serialize)]
pub struct TemplateLogin {
    pub action_url: String,
    pub tried_user: String,
    pub msg: String,
    pub info: TemplateInfo,
}

#[derive(Debug, Clone, Serialize)]
pub struct TemplateCreate {
    pub action_url: String,
    pub info: TemplateInfo,
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

// let end = start.elapsed();
// println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());

impl TemplateInfo {
    pub fn new(title: Option<String>, admin: Option<AdminCookie>, user: Option<UserCookie>, js: String, gen: Option<Instant>) -> TemplateInfo {
        TemplateInfo {
            title: if let Some(t) = title { t } else { String::new() },
            logged_in: if admin.is_some() || user.is_some() { true } else { false },
            is_admin: if admin.is_some() { true } else { false },
            is_user: if user.is_some() { true } else { false },
            username: if let Some(a) = admin { titlecase(&a.username.clone()) } else if let Some(u) = user { titlecase(&u.username.clone()) } else { String::new() },
            js,
            gentime: if let Some(inst) = gen {
                let end = inst.elapsed();
                format!("{}.{:08}", end.as_secs(), end.subsec_nanos())
            } else { String::new() },
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
impl TemplateLogin {
        pub fn new(action_url: String, tried: Option<String>, fail: Option<String>, info: TemplateInfo) -> TemplateLogin {
        TemplateLogin {
            action_url,
            tried_user: if let Some(tuser) = tried { tuser } else { String::new() },
            msg: if let Some(fmsg) = fail { fmsg } else { String::new() },
            info,
        }
    }
}

impl TemplateCreate {
        pub fn new(action_url: String, info: TemplateInfo) -> TemplateCreate {
        TemplateCreate {
            action_url,
            info,
        }
    }
}

pub fn hbs_template(content: TemplateBody, title: Option<String>, admin_opt: Option<AdminCookie>, user_opt: Option<UserCookie>, javascript: Option<String>, gen: Option<Instant>) -> Template {
    // let mut context: HashMap<&str> = HashMap::new();
    // context.insert();
    let js = if let Some(j) = javascript { j } else { "".to_string() }; 
    let info = TemplateInfo::new(title, admin_opt, user_opt, js, gen);
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
        TemplateBody::Login(action, tried, fail) => {
            let context = TemplateLogin::new(action, tried, fail, info);
            Template::render("login-template", &context)
        },
        TemplateBody::Create(action) => {
            let context = TemplateCreate::new(action, info);
            Template::render("create-template", &context)
        },
    }
    
}

// pub fn hbs_login(tried_user: Option<String>, fail_msg: Option<String>, title: Option<String>, admin_opt: Option<AdminCookie>, user_opt: Option<UserCookie>, javascript: Option<String>, gen: Option<Instant>) -> Template {
    
// }





























