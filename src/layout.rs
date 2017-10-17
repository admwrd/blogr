

use rocket::response::content::Html;
use blog::*;

const ADMIN_LOGIN_FULL: &'static str = r##"
<!DOCTYPE html>
<html lang="en">
    <head>
        <!-- Required meta tags -->
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no">
        
        <!-- Font Awesome -->
        <link rel="stylesheet" href="css/font-awesome.min.css">
        
        <!-- Bootstrap CSS -->
        <link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/4.0.0-beta/css/bootstrap.min.css" integrity="sha384-/Y6pD6FV/Vv2HJnA6t+vslU6fwYXjCFtcEpHbNJ0lyAFsXTsjBbfaDjzALeQsN6M" crossorigin="anonymous">
        
        <!-- Custom CSS -->
        <link id="pageStyleSheet" type="text/css" href="css/blog.css" rel="stylesheet" />
        
        <!-- JavaScript -->
        <!-- <script src="js-head.js"></script>  -->
        
        <style>
        
        </style>
    </head>
    <body>
        <div id="mainWrapper" class="main-wrapper">
            
            <form action="http://localhost:8000/admin" name="login_form" method="post" onsubmit="return validate_form()">
                <div class="form-group" id="userGroup">
                    <label for="usernameField">Email Address</label>
                    <input type="text" name="username" class="form-control" id="usernameField" aria-describedby="idHelp" placeholder="Enter email">
                    <small id="idHelp" class="form-text text-muted">Your email address will not be shared with anyone else.</small>
                </div>
                <div class="form-group" id="passGroup">
                    <label for="passwordField">Password</label>
                    <input type="password" name="password" class="form-control" id="passwordField" placeholder="Password">
                </div>
                <!-- <div class="form-check">
                  <label class="form-check-label">
                      <input type="checkbox" class="form-check-input">
                  Check me out
                    </label>
                </div> -->
                <button type="submit" class="btn btn-primary">Submit</button>
            </form>
            
        </div>
        <!-- jQuery first, then Popper.js, then Bootstrap JS -->
        <script src="https://code.jquery.com/jquery-3.2.1.slim.min.js" integrity="sha384-KJ3o2DKtIkvYIK3UENzmM7KCkRr/rE9/Qpg6aAZGJwFDMVNA/GpGFF93hXpG5KkN" crossorigin="anonymous"></script>
        <script src="https://cdnjs.cloudflare.com/ajax/libs/popper.js/1.11.0/umd/popper.min.js" integrity="sha384-b/U6ypiBEHpOf/4+1nzFpr53nxSS+GLCkfwBdFNTxtclqqenISfwAzpKaMNFNmj4" crossorigin="anonymous"></script>
        <script src="https://maxcdn.bootstrapcdn.com/bootstrap/4.0.0-beta/js/bootstrap.min.js" integrity="sha384-h0AbiXch4ZDo7tp9hKZ4TsHbi047NrKGLO3SEJAg45jXxnGIfYzk4Si90RDIqNm1" crossorigin="anonymous"></script>
        
        <!-- Custom JavaScript -->
        <script src="blog.js">
        <script type="text/javascript">
        </script>
        
    </body>
</html>
"##;

const USER_LOGIN_FULL: &'static str = r##"
<!DOCTYPE html>
<html lang="en">
    <head>
        <!-- Required meta tags -->
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no">
        
        <!-- Font Awesome -->
        <link rel="stylesheet" href="css/font-awesome.min.css">
        
        <!-- Bootstrap CSS -->
        <link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/bootstrap/4.0.0-beta/css/bootstrap.min.css" integrity="sha384-/Y6pD6FV/Vv2HJnA6t+vslU6fwYXjCFtcEpHbNJ0lyAFsXTsjBbfaDjzALeQsN6M" crossorigin="anonymous">
        
        <!-- Custom CSS -->
        <link id="pageStyleSheet" type="text/css" href="css/blog.css" rel="stylesheet" />
        
        <!-- JavaScript -->
        <!-- <script src="js-head.js"></script>  -->
        
        <style>
        
        </style>
    </head>
    <body>
        <div id="mainWrapper" class="main-wrapper">
            
            <form action="http://localhost:8000/user" name="login_form" method="post" onsubmit="return validate_form()">
                <div class="form-group" id="userGroup">
                    <label for="usernameField">Email Address</label>
                    <input type="text" name="username" class="form-control" id="usernameField" aria-describedby="idHelp" placeholder="Enter email">
                    <small id="idHelp" class="form-text text-muted">Your email address will not be shared with anyone else.</small>
                </div>
                <div class="form-group" id="passGroup">
                    <label for="passwordField">Password</label>
                    <input type="password" name="password" class="form-control" id="passwordField" placeholder="Password">
                </div>
                <!-- <div class="form-check">
                  <label class="form-check-label">
                      <input type="checkbox" class="form-check-input">
                  Check me out
                    </label>
                </div> -->
                <button type="submit" class="btn btn-primary">Submit</button>
            </form>
            
        </div>
        <!-- jQuery first, then Popper.js, then Bootstrap JS -->
        <script src="https://code.jquery.com/jquery-3.2.1.slim.min.js" integrity="sha384-KJ3o2DKtIkvYIK3UENzmM7KCkRr/rE9/Qpg6aAZGJwFDMVNA/GpGFF93hXpG5KkN" crossorigin="anonymous"></script>
        <script src="https://cdnjs.cloudflare.com/ajax/libs/popper.js/1.11.0/umd/popper.min.js" integrity="sha384-b/U6ypiBEHpOf/4+1nzFpr53nxSS+GLCkfwBdFNTxtclqqenISfwAzpKaMNFNmj4" crossorigin="anonymous"></script>
        <script src="https://maxcdn.bootstrapcdn.com/bootstrap/4.0.0-beta/js/bootstrap.min.js" integrity="sha384-h0AbiXch4ZDo7tp9hKZ4TsHbi047NrKGLO3SEJAg45jXxnGIfYzk4Si90RDIqNm1" crossorigin="anonymous"></script>
        
        <!-- Custom JavaScript -->
        <script src="blog.js">
        <script type="text/javascript">
        // $(".alert).alert();
        </script>
        
    </body>
</html>
"##;


const HEADER: &'static str = include_str!("../static/header.html");
const FOOTER: &'static str = include_str!("../static/footer.html");

pub fn template_admin_login_fail(user: &str, reason: &str) -> String {
    format!(r##"
            <div class="alert alert-danger" role="alert">
                Login failed: {why}
            </div>
            <form action="http://localhost:8000/admin" name="login_form" method="post" onsubmit="return validate_form()">
                <div class="form-group" id="userGroup">
                    <label for="usernameField">Email Address</label>
                    <input type="text" name="username" value="{user}" class="form-control" id="usernameField" aria-describedby="idHelp" placeholder="Enter email">
                    <small id="idHelp" class="form-text text-muted">Your email address will not be shared with anyone else.</small>
                </div>
                <div class="form-group" id="passGroup">
                    <label for="passwordField">Password</label>
                    <input type="password" name="password" class="form-control" id="passwordField" placeholder="Password">
                </div>
                <!-- <div class="form-check">
                  <label class="form-check-label">
                      <input type="checkbox" class="form-check-input">
                  Check me out
                    </label>
                </div> -->
                <button type="submit" class="btn btn-primary">Submit</button>
            </form>
"##, user=user, why=reason)
}

pub fn template_user_login_fail(user: &str, reason: &str) -> String {
    format!(r##"
            <div class="alert alert-danger" role="alert">
                Login failed: {why}
            </div>
            <form action="http://localhost:8000/user" name="login_form" method="post" onsubmit="return validate_form()">
                <div class="form-group" id="userGroup">
                    <label for="usernameField">Email Address</label>
                    <input type="text" name="username" value="{user}" class="form-control" id="usernameField" aria-describedby="idHelp" placeholder="Enter email">
                    <small id="idHelp" class="form-text text-muted">Your email address will not be shared with anyone else.</small>
                </div>
                <div class="form-group" id="passGroup">
                    <label for="passwordField">Password</label>
                    <input type="password" name="password" class="form-control" id="passwordField" placeholder="Password">
                </div>
                <!-- <div class="form-check">
                  <label class="form-check-label">
                      <input type="checkbox" class="form-check-input">
                  Check me out
                    </label>
                </div> -->
                <button type="submit" class="btn btn-primary">Submit</button>
            </form>
"##, user=user, why=reason)
}

pub fn template(body: &str) -> Html<String> {
    
    // let mut webpage = String::with_capacity( (859 + 914 + body.len()) );
    // webpage.push_str(template_header().to_string());
    let mut webpage = HEADER.to_string();
    
    webpage.reserve(FOOTER.len() + body.len() + 50);
    
    webpage.push_str(body);
    webpage.push_str(HEADER);
    
    Html(webpage)
}

pub fn template_article(article: &Article, is_admin: bool, is_user: bool, username: Option<&str>) ->String {
    // display created time, and modified time if it differs from created date
    // display how long ago it was created if modified == created
    //   or if modified != created display how long ago it was modified
    // 
    unimplemented!()
}

pub fn template_list_articles(articles: Vec<u64>, title: String) -> Html<String> {
    // lookup each aid and return author,
    // the title shortened to 128 characters, 
    // and body shortened to 512 characters)
    unimplemented!()
}

pub fn template_login_user() -> &'static str {
    USER_LOGIN_FULL
}

pub fn template_login_admin() -> &'static str {
    ADMIN_LOGIN_FULL
}

pub fn template_header() -> &'static str {
    HEADER
}

pub fn template_footer() -> &'static str {
    FOOTER
}
