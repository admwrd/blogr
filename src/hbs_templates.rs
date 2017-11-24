
use rocket_contrib::Template;
use handlebars::Handlebars;

use std::collections::{HashMap, BTreeMap};
use chrono::{NaiveDate, NaiveDateTime};
use titlecase::titlecase;
use std::time::{Instant, Duration};

// use cookie_data::*;

// not used anymore
// use admin_auth::*;
// use user_auth::*;

use super::BLOG_URL;
use blog::*;
use layout::*;
// use users;

use ral_administrator::*;
use ral_user::*;


use std::path::{Path, PathBuf};
use std::env;

#[derive(Debug, Clone, Serialize)]
pub struct TagCount {
    pub tag: String,
    pub url: String,
    pub count: u32,
    pub size: u16,
}


/// The TemplateBody struct determines which template is used and what info is passed to it
#[derive(Debug, Clone)]
pub enum TemplateBody {
    General(String, Option<String>),
    Article(Article, Option<String>),
    Articles(Vec<Article>, Option<String>), // articles and an optional message
    // Search(Vec<Article>, Option<String>, Option<String>), // articles and an optional message
    Search(Vec<Article>, Option<Search>, Option<String>), // articles and an optional message
    Login (
        String, // Form Action URL
        Option<String>, // username that was entered
        Option<String>, // fail message
    ),
    Create(String, Option<String>),
    Tags(Vec<TagCount>, Option<String>),
}

#[derive(Debug, Clone, Serialize)]
pub struct TemplateMenu {
    pub separator: bool,
    pub name: String,
    pub url: String,
    pub classes: String,
}

/// The TemplateInfo struct contains page metadata
#[derive(Debug, Clone, Serialize)]
pub struct TemplateInfo {
    pub title: String,
    pub logged_in: bool,
    pub is_admin: bool,
    pub is_user: bool,
    pub username: String,
    pub js: String,
    pub gentime: String,
    pub page: String,
    pub pages: Vec<TemplateMenu>,
    pub admin_pages: Vec<TemplateMenu>,
    pub base_url: &'static str,
}

// START TEMPLATEBODY STRUCTURES

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
    pub msg: String,
    pub info: TemplateInfo,
}
#[derive(Debug, Clone, Serialize)]
pub struct TemplateGeneral {
    pub body: String,
    pub msg: String,
    pub info: TemplateInfo,

}
#[derive(Debug, Clone, Serialize)]
pub struct TemplateArticle {
    pub body: ArticleDisplay,
    pub msg: String,
    pub info: TemplateInfo,

}
#[derive(Debug, Clone, Serialize)]
pub struct TemplateArticles {
    pub body: Vec<ArticleDisplay>,
    pub msg: String,
    pub info: TemplateInfo,
}
// #[derive(Debug, Clone, Serialize)]
// pub struct TemplateSearch {
//     pub body: Vec<ArticleDisplay>,
//     pub qry: String,
//     pub msg: String,
//     pub info: TemplateInfo,
// }
#[derive(Serialize)]
pub struct TemplateSearch {
    pub body: Vec<ArticleDisplay>,
    pub search: SearchDisplay,
    pub msg: String,
    pub info: TemplateInfo,
}
#[derive(Debug, Clone, Serialize)]
pub struct TemplateTags {
    pub tags: Vec<TagCount>,
    pub msg: String,
    pub info: TemplateInfo,
}

// END TEMPLATEBODY STRUCTS

// let end = start.elapsed();
// println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());


pub fn create_menu(page: &str, admin_opt: &Option<AdministratorCookie>, user_opt: &Option<UserCookie>) -> (Vec<TemplateMenu>, Vec<TemplateMenu>) {
    
    let mut pages: Vec<TemplateMenu> = vec![
        TemplateMenu::new(String::from("Home"), String::from("/"), page),
        TemplateMenu::new(String::from("About"), String::from("/about"), page),
        TemplateMenu::new(String::from("View Tags"), String::from("/all_tags"), page),
    ];
    
    let mut admin_pages: Vec<TemplateMenu> = Vec::new();
    if admin_opt.is_some() {
        admin_pages.push( TemplateMenu::new(String::from("Admin Dashboard"), String::from("/admin"), page) );
        admin_pages.push( TemplateMenu::new(String::from("New Article"), String::from("/create"), page) );
        admin_pages.push( TemplateMenu::separator() );
        admin_pages.push( TemplateMenu::new(String::from("Logout"), String::from("/admin_logout"), page) );
    } else if user_opt.is_some() {
        admin_pages.push( TemplateMenu::new(String::from("User Dashboard"), String::from("/user"), page) );
        admin_pages.push( TemplateMenu::separator() );
        admin_pages.push( TemplateMenu::new(String::from("Logout"), String::from("/user_logout"), page) );
    } else {
        pages.push( TemplateMenu::new(String::from("User Login"), String::from("/user"), page) );
        pages.push( TemplateMenu::new(String::from("Admin Login"), String::from("/admin"), page) );
    }
    
    (pages, admin_pages)
}


impl TemplateMenu {
    pub fn new(name: String, url: String, current_page: &str) -> TemplateMenu {
        let classes = if &url == current_page {
            "active".to_string()
        } else {
            String::new()
        };
        TemplateMenu {
            separator: false,
            classes,
            name,
            url,
        }
    }
    pub fn separator() -> TemplateMenu {
        TemplateMenu {
            separator: true,
            name: String::new(),
            url: String::new(),
            classes: String::new(),
        }
    }
}



impl TemplateInfo {
    pub fn new( title: Option<String>, 
                admin: Option<AdministratorCookie>, 
                user: Option<UserCookie>, 
                js: String, 
                gen: Option<Instant>, 
                page: String,
                pages: Vec<TemplateMenu>,
                admin_pages: Vec<TemplateMenu>,
              ) -> TemplateInfo {
        
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
            page,
            pages,
            admin_pages,
            base_url: BLOG_URL,
        }
    }
}



impl TemplateGeneral  {
    pub fn new(content: String, msg: Option<String>, info: TemplateInfo) -> TemplateGeneral {
        TemplateGeneral {
            body: content,
            msg: if let Some(m) = msg { m } else { String::new() },
            info
        }
    }
}
impl TemplateArticle {
    pub fn new(content: Article, msg: Option<String>, info: TemplateInfo) -> TemplateArticle {
        TemplateArticle {
            body: content.to_display(),
            msg: if let Some(m) = msg { m } else { String::new() },
            info,
        }
    }
}
impl TemplateArticles {
    pub fn new(content: Vec<Article>, msg: Option<String>, info: TemplateInfo) -> TemplateArticles {
        let mut articles: Vec<ArticleDisplay> = content.iter().map(|a| a.to_display()).collect();
        TemplateArticles {
            body: articles,
            msg: if let Some(m) = msg { m } else { String::new() },
            info,
        }
    }
}
impl TemplateSearch {
    pub fn new(content: Vec<Article>, search: Option<Search>, msg: Option<String>, info: TemplateInfo) -> TemplateSearch {
        let mut articles: Vec<ArticleDisplay> = content.iter().map(|a| a.to_display()).collect();
        TemplateSearch {
            body: articles,
            search: if let Some(s) = search { s.to_display() } else { SearchDisplay::default() },
            msg: if let Some(m) = msg { m } else { String::new() },
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
        pub fn new(action_url: String, msg: Option<String>, info: TemplateInfo) -> TemplateCreate {
        TemplateCreate {
            action_url,
            msg: if let Some(m) = msg { m } else { String::new() },
            info,
        }
    }
}
impl TemplateTags {
    pub fn new(tags: Vec<TagCount>, msg: Option<String>, info: TemplateInfo) -> TemplateTags {
        TemplateTags {
            tags,
            msg: if let Some(m) = msg { m } else { String::new() },
            info,
        }
    }
}





pub fn hbs_template(content: TemplateBody, title: Option<String>, page: String, admin_opt: Option<AdministratorCookie>, user_opt: Option<UserCookie>, javascript: Option<String>, gen: Option<Instant>) -> Template {
    // let mut context: HashMap<&str> = HashMap::new();
    // context.insert();
    let js = if let Some(j) = javascript { j } else { "".to_string() }; 
    // let info = TemplateInfo::new(title, admin_opt, user_opt, js, gen);
    
    let (pages, admin_pages) = create_menu(&page, &admin_opt, &user_opt);
    
    let info = TemplateInfo::new(title, admin_opt, user_opt, js, gen, page, pages, admin_pages);
    
    match content {
        TemplateBody::General(contents, msg) => {
            let context = TemplateGeneral::new(contents, msg, info);
            Template::render("general-template", &context)
        },
        TemplateBody::Article(article, msg) => {
            let context = TemplateArticle::new(article, msg, info);
            Template::render("article-template", &context)
        },
        TemplateBody::Articles(articles, msg) => {
            let context = TemplateArticles::new(articles, msg, info);
            Template::render("articles-template", &context)
            // let context = TemplateGeneral::new("ARTICLES NOT YET IMPLEMENTED.".to_string(), info);
            // Template::render("general-template", &context)
        },
        // TemplateBody::Search(articles, qry, msg) => {
        //     let context = TemplateArticles::new(articles, qry, msg, info);
        //     Template::render("articles-template", &context)
        //     // let context = TemplateGeneral::new("ARTICLES NOT YET IMPLEMENTED.".to_string(), info);
        //     // Template::render("general-template", &context)
        // },
        TemplateBody::Search(articles, search, msg) => {
            let context = TemplateSearch::new(articles, search, msg, info);
            Template::render("search-template", &context)
            // let context = TemplateGeneral::new("ARTICLES NOT YET IMPLEMENTED.".to_string(), info);
            // Template::render("general-template", &context)
        },
        TemplateBody::Login(action, tried, fail) => {
            let context = TemplateLogin::new(action, tried, fail, info);
            Template::render("login-template", &context)
        },
        TemplateBody::Create(action, msg) => {
            let context = TemplateCreate::new(action, msg, info);
            Template::render("create-template", &context)
        },
        TemplateBody::Tags(tags, msg) => {
            let context = TemplateTags::new(tags, msg, info);
            Template::render("tags-template", &context)
        },
    }
    
}






