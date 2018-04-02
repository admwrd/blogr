



#[cfg(production)]
pub const PRODUCTION: bool = true;
#[cfg(not(production))]
pub const PRODUCTION: bool = false;

include!("private.rs");

/*
private.rs will contain:

#[cfg(not(production))]
pub const DATABASE_URL: &'static str = "postgres://dbuser:dbpass@localhost/blog";

#[cfg(production)]
pub const DATABASE_URL: &'static str = "postgres://dbuser:dbpass@localhost/blog";
*/



// DEVELOPMENT SETTINGS

#[cfg(not(production))]
pub const BLOG_URL: &'static str = "http://localhost:8000/";
#[cfg(not(production))]
pub const USER_LOGIN_URL: &'static str = "http://localhost:8000/user";
#[cfg(not(production))]
pub const ADMIN_LOGIN_URL: &'static str = "http://localhost:8000/admin";
#[cfg(not(production))]
pub const TEST_LOGIN_URL: &'static str = "http://localhost:8000/login";
#[cfg(not(production))]
pub const CREATE_FORM_URL: &'static str = "http://localhost:8000/create";
#[cfg(not(production))]
pub const EDIT_FORM_URL: &'static str = "http://localhost:8000/edit";
#[cfg(not(production))]
pub const MANAGE_URL: &'static str = "http://localhost:8000/manage";
#[cfg(not(production))]
pub const MAX_CREATE_TITLE: usize = 120;
#[cfg(not(production))]
pub const MAX_CREATE_DESCRIPTION: usize = 400;
#[cfg(not(production))]
pub const MAX_CREATE_TAGS: usize = 250;
#[cfg(not(production))]
const MAX_ATTEMPTS: i16 = 8; // 8
#[cfg(not(production))]
const LOCKOUT_DURATION: u32 = 8; // 6 seconds // 900 seconds = 15 minutes
#[cfg(not(production))]
const DB_BACKUP_SCRIPT: &'static str = r"scripts\db_backup-dev.bat";
// After the specified number of attempts, the next account lock will permanently lock the account
#[cfg(not(production))]
const ADMIN_LOCK: i16 = 20;
// After the specified number of attempts, the next account lock will permanently lock the account
#[cfg(not(production))]
const USER_LOCK: i16 = 40;
// Path to the article header images internally
#[cfg(not(production))]
const INTERNAL_IMGS: &'static str = r"c:\code\lang\rust\proj\blogr\static\imgs";
#[cfg(not(production))]
pub const HITS_SAVE_INTERVAL: usize = 2;
// Default http caching value (max age value)
#[cfg(not(production))]
const DEFAULT_TTL: isize = 3600;  // 1*60*60 = 1 hour, 43200=1/2 day, 86400=1 day
// If no description is found take a specified amount of characters from the article body
#[cfg(not(production))]
pub const DESC_LIMIT: usize = 300;
// The directory to load static pages from
#[cfg(not(production))]
const STATIC_PAGES_DIR: &'static str = "pages";
// Multi Segment Paths are those that have multiple / in the uri
// ex: /article/<aid>/Search_Engine_Friendly_Title
// const MULTI_SEGMENT_PATHS: Vec<&'static str> = vec!["article", "search", "tag"];
#[cfg(not(production))]
const MULTI_SEGMENT_PATHS: &[&'static str] = &["article", "search", "tag"];
// const MULTI_SEGMENT_PATHS: &[&'static str] = [""];
#[cfg(not(production))]
const PAGE_TEMPLATES: &[&'static str] = &["page-template", "page-code-template", "page-blank-template"];
#[cfg(not(production))]
const DEFAULT_PAGE_TEMPLATE: &'static str = "page-template";
#[cfg(not(production))]
const HIT_COUNTER_LOG: &'static str = "logs/page_stats.json";
#[cfg(not(production))]
const TOTAL_HITS_LOG: &'static str = "logs/total_views.json";
#[cfg(not(production))]
const UNIQUE_HITS_LOG: &'static str = "logs/unique_stats.json";
#[cfg(not(production))]
// const DOWNLOADABLE_LOGS: &[&'static str] = vec!["page_stats.json", "unique_stats.json", "total_views.json"];
const DOWNLOADABLE_LOGS: &[&'static str] = &["page_stats.json", "unique_stats.json", "total_views.json"];
// Enables fallback database queries if cache is disabled
#[cfg(not(production))]
const CACHE_FALLBACK: bool = true;
#[cfg(not(production))]
const CACHE_ENABLED: bool = true;
#[cfg(not(production))]
const ENABLE_DEBUG_OUTPUT: bool = false;

// #[cfg(not(production))]
// static DEFAULT_PAGE_MENU: Option<Vec<TemplateMenu>> = Some(vec![TemplateMenu::new("Rust Tutorials".to_owned(), "/content/tutorials".to_owned(), "")]);
// static DEFAULT_PAGE_MENU: Option<Vec<TemplateMenu>> = Some(vec![TemplateMenu {name: "Rust Tutorials".to_owned(), url: format!("{}content/tutorials", BLOG_URL), separator: false, classes: String::new()}]);


// Comrak Markdown rendering default settings
#[cfg(not(production))]
pub const COMRAK_OPTIONS: ComrakOptions = ComrakOptions {
    hardbreaks: true,            // \n => <br>\n
    width: 120usize,             
    github_pre_lang: false,      
    ext_strikethrough: true,     // hello ~world~ person.
    ext_tagfilter: true,         // filters out certain html tags
    ext_table: true,             // | a | b |\n|---|---|\n| c | d |
    ext_autolink: true,          
    ext_tasklist: true,          // * [x] Done\n* [ ] Not Done
    ext_superscript: true,       // e = mc^2^
    ext_header_ids: None,        // None / Some("some-id-prefix-".to_string())
    ext_footnotes: true,         // Hi[^x]\n\n[^x]: A footnote here\n
};










// PRODUCTION SETTINGS

#[cfg(production)]
pub const BLOG_URL: &'static str = "https://vishus.net/";
#[cfg(production)]
pub const USER_LOGIN_URL: &'static str = "https://vishus.net/user";
#[cfg(production)]
pub const ADMIN_LOGIN_URL: &'static str = "https://vishus.net/admin";
#[cfg(production)]
pub const TEST_LOGIN_URL: &'static str = "https://vishus.net/login";
#[cfg(production)]
pub const CREATE_FORM_URL: &'static str = "https://vishus.net/create";
#[cfg(production)]
pub const EDIT_FORM_URL: &'static str = "https://vishus.net/edit";
#[cfg(production)]
pub const MANAGE_URL: &'static str = "https://vishus.net/manage";
#[cfg(production)]
pub const MAX_CREATE_TITLE: usize = 50;
#[cfg(production)]
pub const MAX_CREATE_DESCRIPTION: usize = 500;
#[cfg(production)]
pub const MAX_CREATE_TAGS: usize = 250;
#[cfg(production)]
const MAX_ATTEMPTS: i16 = 5; // 8
#[cfg(production)]
const LOCKOUT_DURATION: u32 = 900; // 6 seconds // 900 seconds = 15 minutes
#[cfg(production)]
const DB_BACKUP_SCRIPT: &'static str = r"bash";
#[cfg(production)]
const DB_BACKUP_ARG: &'static str = r"scripts/db_backup-prod.sh";
// After the specified number of attempts, the next account lock will permanently lock the account
#[cfg(production)]
const ADMIN_LOCK: i16 = 15;
// After the specified number of attempts, the next account lock will permanently lock the account
#[cfg(production)]
const USER_LOCK: i16 = 40;
// Path to the article header images internally
#[cfg(production)]
const INTERNAL_IMGS: &'static str = r"/var/www/html/imgs";
#[cfg(production)]
pub const HITS_SAVE_INTERVAL: usize = 35;
// Default http caching value (max age value)
#[cfg(production)]
const DEFAULT_TTL: isize = 3600;  // 1*60*60 = 1 hour, 43200=1/2 day, 86400=1 day
// If no description is found take a specified amount of characters from the article body
#[cfg(production)]
pub const DESC_LIMIT: usize = 300;
// The directory to load static pages from
#[cfg(production)]
const STATIC_PAGES_DIR: &'static str = "pages";
// Multi Segment Paths are those that have multiple / in the uri
// ex: /article/<aid>/Search_Engine_Friendly_Title
// const MULTI_SEGMENT_PATHS: Vec<&'static str> = vec!["article", "search", "tag"];
#[cfg(production)]
const MULTI_SEGMENT_PATHS: &[&'static str] = &["article", "search", "tag"];
#[cfg(production)]
const PAGE_TEMPLATES: &[&'static str] = &["page-template", "page-code-template", "page-blank-template"];
#[cfg(production)]
const DEFAULT_PAGE_TEMPLATE: &'static str = "page-template";
#[cfg(production)]
const HIT_COUNTER_LOG: &'static str = "logs/page_stats.json";
#[cfg(production)]
const TOTAL_HITS_LOG: &'static str = "logs/total_views.json";
#[cfg(production)]
const UNIQUE_HITS_LOG: &'static str = "logs/unique_stats.json";
#[cfg(production)]
// const DOWNLOADABLE_LOGS = vec!["page_stats.json", "unique_stats.json", "total_views.json"];
const DOWNLOADABLE_LOGS: &[&'static str] = &["page_stats.json", "unique_stats.json", "total_views.json"];
// Enables fallback database queries if cache is disabled
#[cfg(production)]
const CACHE_FALLBACK: bool = true;
#[cfg(production)]
const CACHE_ENABLED: bool = true;
#[cfg(production)]
const ENABLE_DEBUG_OUTPUT: bool = false;





// #[cfg(production)]
// static DEFAULT_PAGE_MENU: Option<Vec<TemplateMenu>> = Some(vec![TemplateMenu::new("Rust Tutorials".to_owned(), "/content/tutorials".to_owned(), "")]);
// static DEFAULT_PAGE_MENU: Option<Vec<TemplateMenu>> = Some(vec![TemplateMenu {name: "Rust Tutorials".to_owned(), url: format!("{}content/tutorials", BLOG_URL), separator: false, classes: String::new()}]);

// Comrak Markdown rendering default settings
#[cfg(production)]
pub const COMRAK_OPTIONS: ComrakOptions = ComrakOptions {
    hardbreaks: true,            // \n => <br>\n
    width: 120usize,             
    github_pre_lang: false,      
    ext_strikethrough: true,     // hello ~world~ person.
    ext_tagfilter: true,         // filters out certain html tags
    ext_table: true,             // | a | b |\n|---|---|\n| c | d |
    ext_autolink: true,          
    ext_tasklist: true,          // * [x] Done\n* [ ] Not Done
    ext_superscript: true,       // e = mc^2^
    ext_header_ids: None,        // None / Some("some-id-prefix-".to_string())
    ext_footnotes: true,         // Hi[^x]\n\n[^x]: A footnote here\n
};









lazy_static! {
    static ref BASE: &'static str = if BLOG_URL.ends_with("/") {
        &BLOG_URL[..BLOG_URL.len()-1]
    } else {
        &BLOG_URL
    };
}

    
// The production and development versions of the menu items allows
// links to be added with different addresses depending on dev/prod environment
//   ex: the links in the dev version could use http://localhost:8000/
//       where as the prod version could use https://your_domain.com/
#[cfg(production)]
lazy_static! {
    static ref DEFAULT_PAGE_MENU: Option<Vec<TemplateMenu>> = Some(
        vec![
            TemplateMenu::new("Home".to_owned(), "/".to_owned(), ""),
            TemplateMenu::new("Rust Tutorials".to_owned(), "/content/tutorials".to_owned(), ""),
            TemplateMenu::new("Tags".to_owned(), "/all_tags".to_owned(), ""),
            TemplateMenu::new("About".to_owned(), "/content/about-me".to_owned(), ""),
        ]
    );
}
#[cfg(not(production))]
lazy_static! {
    static ref DEFAULT_PAGE_MENU: Option<Vec<TemplateMenu>> = Some(
        vec![
            TemplateMenu::new("Home".to_owned(), "/".to_owned(), ""),
            TemplateMenu::new("Rust Tutorials".to_owned(), "/content/tutorials".to_owned(), ""),
            TemplateMenu::new("Tags".to_owned(), "/all_tags".to_owned(), ""),
            TemplateMenu::new("About".to_owned(), "/content/about-me".to_owned(), ""),
        ]
    );
}

#[cfg(production)]
lazy_static! {
    static ref DEFAULT_PAGE_DROPDOWN: Option<Vec<TemplateMenu>> = None;
}
#[cfg(not(production))]
lazy_static! {
    static ref DEFAULT_PAGE_DROPDOWN: Option<Vec<TemplateMenu>> = None;
}




