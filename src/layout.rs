

use rocket::response::Flash;
use rocket::request::FlashMessage;
use rocket::response::content::Html;
use blog::*;
use titlecase::titlecase;

use super::{BLOG_URL, USER_LOGIN_URL, ADMIN_LOGIN_URL};


// pub const UNAUTHORIZED_POST_MESSAGE: &'static str = "You are not authorized to post articles.  Please login as an administrator.<br><a href=\"http://localhost:8000/admin\">Admin Login</a>";
pub const UNAUTHORIZED_POST_MESSAGE: &'static str = "You are not authorized to post articles.  Please login as an administrator.<br><a href=\"admin\">Admin Login</a>";


// const HEADER: &'static str = include_str!("../static/template_header.html");
// const FOOTER: &'static str = include_str!("../static/template_footer.html");
const GENERIC_PAGE_START: &'static str = "<div class=\"v-content\">\n\t\t\t\t\t\t";
const GENERIC_PAGE_END: &'static str = "\n\t\t\t\t\t</div>";
const TABS: &'static str = "\t\t\t\t\t\t\t";


pub fn process_flash(flash_opt: Option<FlashMessage>) -> Option<String> {
    let fmsg: Option<String>;
    if let Some(flash) = flash_opt {
        if flash.name() == "error" {
            fmsg = Some(alert_danger( flash.msg() ));
        } else if flash.name() == "warning" {
            fmsg = Some(alert_warning( flash.msg() ));
        } else if flash.name() == "success" {
            fmsg = Some(alert_success( flash.msg() ));
        } else {
            fmsg = Some(alert_info( flash.msg() ));
        }
    }  else {
        fmsg = None;
    }
    fmsg
}

pub fn admin_nav_username(username: &str) -> String {
    format!(r##"
                        <li class="v-nav-item nav-item dropdown">
                            <a class="nav-link dropdown-toggle" href="#" id="navbarDropdown" role="button" data-toggle="dropdown" aria-haspopup="true" aria-expanded="false">
                                {user}
                            </a>
                            <div class="dropdown-menu" aria-labelledby="navbarDropdown">
                                <a class="dropdown-item" href="/insert">New Article</a>
                                <!-- <a class="dropdown-item" href="#">Something else here</a> -->
                                <div class="dropdown-divider"></div>
                                <a class="dropdown-item" href="/logout">Logout</a>
                            </div>
                        </li>
"##, user=username)
}

pub fn admin_nav() -> &'static str {
    r##"
                        <li class="v-nav-item nav-item dropdown">
                            <a class="nav-link dropdown-toggle" href="#" id="navbarDropdown" role="button" data-toggle="dropdown" aria-haspopup="true" aria-expanded="false">
                                {user}
                            </a>
                            <div class="dropdown-menu" aria-labelledby="navbarDropdown">
                                <a class="dropdown-item" href="/insert">New Article</a>
                                <!-- <a class="dropdown-item" href="#">Something else here</a> -->
                                <div class="dropdown-divider"></div>
                                <a class="dropdown-item" href="/logout">Logout</a>
                            </div>
                        </li>
"##
}

pub fn admin_nav_login() -> &'static str {
    r##"<li class="v-nav-item nav-item"><a class="nav-link" href="/admin">Login</a></li>"##
}







pub fn alert_danger(msg: &str) -> String {
    format!(r##"
                        <div class="v-centered-msg alert alert-danger" role="alert">
                            {why}
                        </div>
"##, why=msg)
}
pub fn alert_success(msg: &str) -> String {
    format!(r##"
                        <div class="v-centered-msg alert alert-success" role="alert">
                            {why}
                        </div>
"##, why=msg)
}
pub fn alert_info(msg: &str) -> String {
    format!(r##"
                        <div class="v-centered-msg alert alert-info" role="alert">
                            {why}
                        </div>
"##, why=msg)
}
pub fn alert_warning(msg: &str) -> String {
    format!(r##"
                        <div class="v-centered-msg alert alert-warning" role="alert">
                            {why}
                        </div>
"##, why=msg)
}
pub fn alert_primary(msg: &str) -> String {
    format!(r##"
                        <div class="v-centered-msg alert alert-primary" role="alert">
                            {why}
                        </div>
"##, why=msg)
}


pub fn login_form(url: &str) -> String {
    format!(r##"
                        <form id="needs-validation" action="{url}" name="login_form" method="post" novalidate>
                            <div class="form-group" id="userGroup">
                                <label for="usernameField">Email Address</label>
                                <div class="col-md-9 mb-3">
                                    <input type="text" name="username" value="" class="form-control" id="usernameField" aria-describedby="idHelp" placeholder="Username" required>
                                    <div class="invalid-feedback">
                                        Please specify a username
                                    </div>
                                </div>
                                <!-- <small id="idHelp" class="form-text text-muted">Your email address will not be shared with anyone else.</small> -->
                            </div>
                            <div class="form-group" id="passGroup">
                                <label for="passwordField">Password</label>
                                <div class="col-md-9 mb-3">
                                    <input type="password" name="password" class="form-control" id="passwordField" placeholder="Password" required>
                                    <div class="invalid-feedback">
                                        A password is requierd.
                                    </div>
                                    <input type="password" id="passwordHidden" class="hidden-pass form-control">
                                </div>
                            </div>
                            <div class="v-submit">
                                <button type="submit" class="btn btn-primary" id="submit-button-id">Login</button>
                            </div>
                            <!-- <button type="submit" class="btn btn-faded" id="submit-button-id">Login</button> -->
                            <!-- <button type="submit" class="btn btn-dark" id="submit-button-id">Login</button> -->
                            <!-- <button type="submit" class="btn btn-success" id="submit-button-id">Login</button> -->
                        </form>
"##, url=url)
}

// http://localhost:8000/admin
pub fn login_form_fail(url: &str, user: &str, why: &str) -> String {
    format!(r##"
                        {alert}
                        <form id="needs-validation" action="{url}" name="login_form" method="post" novalidate>
                            <div class="form-group" id="userGroup">
                                <label for="usernameField">Email Address</label>
                                <div class="col-md-9 mb-3">
                                    <input type="text" name="username" value="{user}" class="form-control" id="usernameField" aria-describedby="idHelp" placeholder="Username" required>
                                    <div class="invalid-feedback">
                                        Please specify a username
                                    </div>
                                </div>
                                <!-- <small id="idHelp" class="form-text text-muted">Your email address will not be shared with anyone else.</small> -->
                            </div>
                            <div class="form-group" id="passGroup">
                                <label for="passwordField">Password</label>
                                <div class="col-md-9 mb-3">
                                    <input type="password" name="password" class="form-control" id="passwordField" placeholder="Password" required>
                                    <div class="invalid-feedback">
                                        A password is requierd.
                                    </div>
                                    <input type="password" id="passwordHidden" class="hidden-pass form-control">
                                </div>
                            </div>
                            <div class="v-submit">
                                <button type="submit" class="btn btn-primary" id="submit-button-id">Login</button>
                            </div>
                            <!-- <button type="submit" class="btn btn-faded" id="submit-button-id">Login</button> -->
                            <!-- <button type="submit" class="btn btn-dark" id="submit-button-id">Login</button> -->
                            <!-- <button type="submit" class="btn btn-success" id="submit-button-id">Login</button> -->
                        </form>
"##, url=url, user=user, alert=alert_danger(&format!("Login failed: {}", why)))
}


// pub fn template(body: &str) -> Html<String> {
    
//     let mut webpage = HEADER.to_string();
    
//     webpage.reserve(FOOTER.len() + body.len() + GENERIC_PAGE_START.len() + GENERIC_PAGE_END.len() + 200);
//     webpage.push_str(GENERIC_PAGE_START);
//     // do not add tabs to first line, add TABS to every newline after
//     webpage.push_str(body);
//     webpage.push_str(GENERIC_PAGE_END);
//     webpage.push_str(FOOTER);
    
//     Html(webpage)
// }

pub fn link_tags(tags: &Vec<String>) -> String {
    let mut contents = String::new();
    for t in tags {
        contents.push_str(&format!(" <a href=\"{url}tag?tag={tag}\">{tag}</a>", url=BLOG_URL, tag=t));
    }
    contents
}

// pub fn template_create_article(url: &str) -> String {
//     // Do not add the <div class="v-content">  line as GENERIC_PAGE_START does this
//     format!(r##"
//                         <form method="post" action="{url}article" name="insert_form">
//                             <div class="col-md-7 mx-auto mb-3">
//                                 <input name="title" type="text" class="v-centered-input v-form-control form-control" id="inputTitle" placeholder="Title">
//                             </div>
//                             <div class="form-group">
//                                 <label for="input_body" class="v-form-label">Contents</label>
//                                 <textarea class="form-control" name="body" id="insert_body" rows="3"></textarea>
//                             </div>
//                             <div class="col-md-8 mx-auto">
//                                 <label for="insert-tags" class="v-center-label v-form-label">Tags -Comma Separated</label>
//                                     <input name="tags" id="insert-tags" type="text" class="v-centered-input v-form-control form-control" placeholder="Tags">
//                                     <div class="v-form-message" id="tag-msg">
//                                         Did you mean to <b>Comma Separate</b> the tags?
//                                     </div>
//                             </div>
//                             <div class="v-submit">
//                                 <button type="submit" class="btn btn-primary">Create Article</button>
//                             </div>
//                         </form>
//                         <script>
//                             StartText();
//                         </script>
//         "##, url=url)
// }

// pub fn full_template_article_new(article: &Article, is_admin: bool, is_user: bool, username: Option<String>) -> Html<String> {
//     unimplemented!()
// }

// pub fn template_article(article: &Article, is_admin: bool, is_user: bool, username: Option<String>) -> String {
//     // display created time, and modified time if it differs from created date
//     // display how long ago it was created if modified == created
//     //   or if modified != created display how long ago it was modified
//     let mut contents = String::with_capacity(article.body.len() + 50);
//     let mut bodytxt = if article.body.trim().starts_with("<") {
//         article.body.clone()
//     } else {
//         let mut bt = String::new();
//         bt.push_str("<p>");
//         bt.push_str(&article.body);
//         bt.push_str("</p>");
//         bt
//     };
//     // let mut indented = String::new();
//     // // ensure correct tab formatting
//     // // extremely needless but it's removable
//     // for line in bodytxt.lines() {
//     //     if line.starts_with(TABS) {
//     //         indented.push_str(line);
//     //         indented.push_str("\n");
//     //     } else {
//     //         indented.push_str(TABS);
//     //         indented.push_str(line.trim_left());
//     //         indented.push_str("\n");
//     //     }
//     // }
//     // bodytxt = indented;
//     contents.push_str(&format!(r##"
//                     <article class="v-article">
//                         <header class="v-article-header">
//                             <h2 class="v-article-title"><a href="/article?aid={aid}">{title}</a></h2>
//                             <div class="row">
//                                 <date class="v-article-date" datetime="{date_machine}">{date}</date>
//                                 <!-- YYYY-MM-DDThh:mm:ssTZD OR PTDHMS -->
//                             </div>
//                         </header>
//                         {body}
//                         <div class="v-article-tags">Tags:{tags}</div>
//                     </article>
// "##, aid=article.aid, title=titlecase(&article.title), date_machine=article.posted.format("%Y-%m-%dT%H:%M:%S"), date=article.posted.format("%Y-%m-%d @ %I:%M%P"), body=bodytxt, tags=link_tags(&article.tags)));
//     contents
// }

// pub fn full_template_article(article: &Article, is_admin: bool, is_user: bool, username: Option<String>) -> Html<String> {
//     let mut contents: String = String::from(HEADER);
//     contents.push_str(&template_article(article, is_admin, is_user, username));
//     contents.push_str(FOOTER);
//     Html(contents)
// }
// pub fn template_articles_msg(msg: &str, generic_template: bool, articles: Vec<Article>, is_admin: bool, is_user: bool, username: Option<String>) -> Html<String> {
//     let mut contents: String = String::from(HEADER);
//     if generic_template { contents.push_str(GENERIC_PAGE_START); }
//     contents.push_str(msg);
//     if generic_template { contents.push_str(GENERIC_PAGE_END); }
//     for a in articles {
//         contents.push_str(&template_article(&a, is_admin, is_user, username.clone()));
//     }
//     contents.push_str(FOOTER);
//     Html(contents)
// }

// pub fn template_articles(articles: Vec<Article>, is_admin: bool, is_user: bool, username: Option<String>) -> Html<String> {
//     let mut contents: String = String::from(HEADER);
//     for a in articles {
//         contents.push_str(&template_article(&a, is_admin, is_user, username.clone()));
//     }
//     contents.push_str(FOOTER);
//     Html(contents)
// }

// pub fn template_list_articles(articles: &Vec<u32>, title: String) -> Html<String> {
//     // lookup each aid and return author,
//     // the title shortened to 128 characters, 
//     // and body shortened to 512 characters)
//     unimplemented!()
// }

// pub fn template_header() -> &'static str {
//     HEADER
// }

// pub fn template_footer() -> &'static str {
//     FOOTER
// }
