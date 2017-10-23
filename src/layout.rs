

use rocket::response::content::Html;
use blog::*;
use titlecase::titlecase;

use super::{BLOG_URL, USER_LOGIN_URL, ADMIN_LOGIN_URL};

pub const UNAUTHORIZED_POST_MESSAGE: &'static str = "You are not authorized to post.  Please login as an administrator.<br><a href=\"http://localhost:8000/admin\">Admin Login</a>";


const HEADER: &'static str = include_str!("../static/template_header.html");
const FOOTER: &'static str = include_str!("../static/template_footer.html");
const GENERIC_PAGE_START: &'static str = "<div class=\"v-content\">\n\t\t\t\t\t\t";
const GENERIC_PAGE_END: &'static str = "\n\t\t\t\t\t</div>";
const TABS: &'static str = "\t\t\t\t\t\t\t";

        // <form>
        //     <div class="form-group row">
        //         <label for="inputEmail3" class="col-sm-2 col-form-label">Email</label>
        //         <div class="col-sm-10">
        //             <input type="email" class="form-control" id="inputEmail3" placeholder="Email">
        //         </div>
        //     </div>
        //     <div class="form-group row">
        //         <label for="inputPassword3" class="col-sm-2 col-form-label">Password</label>
        //         <div class="col-sm-10">
        //             <input type="password" class="form-control" id="inputPassword3" placeholder="Password">
        //         </div>
        //     </div>

pub fn login_form(url: &str) -> String {
    format!(r##"
            <form action="{url}" name="login_form" method="post" onsubmit="return validate_form()">
                <div class="form-group" id="userGroup">
                    <label for="usernameField">Email Address</label>
                    <input type="text" name="username" value="" class="form-control" id="usernameField" aria-describedby="idHelp" placeholder="Username">
                    <small id="idHelp" class="form-text text-muted">Your email address will not be shared with anyone else.</small>
                </div>
                <div class="form-group" id="passGroup">
                    <label for="passwordField">Password</label>
                    <input type="password" name="password" class="form-control" id="passwordField" placeholder="Password">
                    <input type="password" id="passwordHidden" class="hidden-pass form-control">
                </div>
                <button type="submit" class="btn btn-primary" id="submit-button-id">Submit</button>
            </form>
"##, url=url)
}

// http://localhost:8000/admin
pub fn login_form_fail(url: &str, user: &str, why: &str) -> String {
    format!(r##"
            <div class="alert alert-danger" role="alert">
                Login failed: {why}
            </div>
            <form action="{url}" name="login_form" method="post" onsubmit="return validate_form()">
                <div class="form-group" id="userGroup">
                    <label for="usernameField">Email Address</label>
                    <input type="text" name="username" value="{user}" class="form-control" id="usernameField" aria-describedby="idHelp" placeholder="Username">
                    <small id="idHelp" class="form-text text-muted">Your email address will not be shared with anyone else.</small>
                </div>
                <div class="form-group" id="passGroup">
                    <label for="passwordField">Password</label>
                    <input type="password" name="password" class="form-control" id="passwordField" placeholder="Password">
                    <input type="password" id="passwordHidden" class="hidden-pass form-control">
                </div>
                <button type="submit" class="btn btn-primary" id="submit-button-id">Submit</button>
            </form>
"##, url=url, user=user, why=why)
}




// pub fn template_login_admin() -> String {
    
// }
// pub fn template_login_admin_fail() -> String {
    
// }
// pub fn template_login_user() -> String {
    
// }
// pub fn template_login_user_fail() -> String {
    
// }



pub fn template(body: &str) -> Html<String> {
    
    // let mut webpage = String::with_capacity( (859 + 914 + body.len()) );
    // webpage.push_str(template_header().to_string());
    let mut webpage = HEADER.to_string();
    
    webpage.reserve(FOOTER.len() + body.len() + GENERIC_PAGE_START.len() + GENERIC_PAGE_END.len() + 200);
    webpage.push_str(GENERIC_PAGE_START);
    // do not add tabs to first line, add TABS to every newline after
    webpage.push_str(body);
    webpage.push_str(GENERIC_PAGE_END);
    webpage.push_str(FOOTER);
    
    Html(webpage)
}

pub fn link_tags(tags: &Vec<String>) -> String {
    let mut contents = String::new();
    for t in tags {
        contents.push_str(&format!(" <a href=\"{url}tag?tag={tag}\">{tag}</a>", url=BLOG_URL, tag=t));
        // contents.push_str("<a href=\"");  contents.push_str(BLOG_URL);  contents.push_str("tag?tag=");  contents.push_str(&t);  contents.push_str("\">");  contents.push_str(&t);  contents.push_str("</a>");
    }
    contents
}

pub fn template_create_article(url: &str) -> String {
    format!(r##"
            <form method="post" action="{url}article" name="insert_form">
                            <div class="form-group">
                                <label for="inputTitle" class="v-form-label">Title</label>
                                <input name="title" type="text" class="v-form-control form-control" id="inputTitle" placeholder="Title">
                            </div>
                            <div class="form-group">
                                <label for="inputDesc" class="v-form-label">Description</label>
                                <input name="description" type="text" class="v-form-control form-control" id="inputDesc" placeholder="Description">
                            </div>
                            <div class="form-group">
                                <label for="input_body" class="v-form-label">Contents</label>
                                <textarea class="form-control" name="body" id="insert_body" rows="3"></textarea>
                            </div>
                            <div class="form-group">
                                <label for="inputTags" class="v-form-label">Tags -Comma Separated</label>
                                <input name="tags" type="text" class="v-form-control form-control" id="inputTags" placeholder="Tags">
                            </div>
                            <button type="submit" class="btn btn-primary">Submit</button>
                        </form>
                        <script>
                            StartText();
                        </script>
        "##, url=url)
}

pub fn full_template_article_new(article: &Article, is_admin: bool, is_user: bool, username: Option<String>) -> Html<String> {
    unimplemented!()
}

pub fn template_article(article: &Article, is_admin: bool, is_user: bool, username: Option<String>) -> String {
    // display created time, and modified time if it differs from created date
    // display how long ago it was created if modified == created
    //   or if modified != created display how long ago it was modified
    // 
    // unimplemented!()
    // let mut contents = String::from("You are viewing article ");
    // contents.push_str(&format!("{}<br>\n", article.aid));
    // contents.push_str(&article.info());
    let mut contents = String::with_capacity(article.body.len() + 50);
    let mut bodytxt = if article.body.trim().starts_with("<") {
        article.body.clone()
    } else {
        let mut bt = String::new();
        bt.push_str("<p>");
        bt.push_str(&article.body);
        bt.push_str("</p>");
        bt
    };
    // let mut indented = String::new();
    // // ensure correct tab formatting
    // // extremely needless but it's removable
    // for line in bodytxt.lines() {
    //     if line.starts_with(TABS) {
    //         indented.push_str(line);
    //         indented.push_str("\n");
    //     } else {
    //         indented.push_str(TABS);
    //         indented.push_str(line.trim_left());
    //         indented.push_str("\n");
    //     }
    // }
    // bodytxt = indented;
    contents.push_str(&format!(r##"
                    <article class="v-article">
                        <header class="v-article-header">
                            <h2 class="v-article-title"><a href="/article?aid={aid}">{title}</a></h2>
                            <div class="row">
                                <date class="v-article-date" datetime="{date_machine}">{date}</date>
                                <!-- YYYY-MM-DDThh:mm:ssTZD OR PTDHMS -->
                            </div>
                        </header>
                        {body}
                        <div class="v-article-tags">Tags:{tags}</div>
                    </article>
"##, aid=article.aid, title=titlecase(&article.title), date_machine=article.posted.format("%Y-%m-%dT%H:%M:%S"), date=article.posted.format("%Y-%m-%d @ %I:%M%P"), body=bodytxt, tags=link_tags(&article.tags)));
    contents
}

pub fn full_template_article(article: &Article, is_admin: bool, is_user: bool, username: Option<String>) -> Html<String> {
    let mut contents: String = String::from(HEADER);
    contents.push_str(&template_article(article, is_admin, is_user, username));
    contents.push_str(FOOTER);
    Html(contents)
}

pub fn template_articles(articles: Vec<Article>, is_admin: bool, is_user: bool, username: Option<String>) -> Html<String> {
    let mut contents: String = String::from(HEADER);
    for a in articles {
        contents.push_str(&template_article(&a, is_admin, is_user, username.clone()));
    }
    contents.push_str(FOOTER);
    Html(contents)
}

pub fn template_list_articles(articles: &Vec<u32>, title: String) -> Html<String> {
    // lookup each aid and return author,
    // the title shortened to 128 characters, 
    // and body shortened to 512 characters)
    unimplemented!()
}

// pub fn template_login_user() -> &'static str {
//     let mut contents = String::from(HEADER);
//     contents.push_str();
//     contents.push_str(FOOTER);
//     USER_LOGIN_FULL
// }

// pub fn template_login_admin() -> &'static str {
//     ADMIN_LOGIN_FULL
// }

pub fn template_header() -> &'static str {
    HEADER
}

pub fn template_footer() -> &'static str {
    FOOTER
}
