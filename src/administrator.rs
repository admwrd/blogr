
use authorize::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdministratorCookie {
    pub userid: u32,
    pub username: String,
    pub display: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdministratorForm {
    pub username: String,
    pub password: String,
}

impl CookieId for AdministratorCookie {
    fn cookie_id<'a>() -> &'a str {
        "acid"
    }
}

impl CookieId for AdministratorForm {
    fn cookie_id<'a>() -> &'a str {
        "acid"
    }
} 


impl AuthorizeCookie for AdministratorCookie {
    // The store and retrieve methods are implemented automatically thanks to default methods
    
    // fn store(&self) -> String {
        
    // }
    // fn retrieve(data: String) -> Self {
        
    // }
}

impl AuthorizeForm for AdministratorForm {
    type CookieType = AdministratorCookie;
    // fn authenticate_user(username: &str, password: &str) -> Result<CookieType, (String, String)> {
    fn authenticate(&self) -> Result<Self::CookieType, (String, String)> {
        if &self.username == "andrew" && &self.password != "" {
            Ok(
                AdministratorCookie {
                    userid: 1,
                    username: "andrew",
                    display: "Andrew",
                }
            )
        } else {
            Err(self.username.to_string(), "Incorrect username".to_string())
        }
    }
    // fn fail_url(&self, msg: &str) -> String {
    fn fail_url(user: &str, msg: &str) -> String {
        let mut output = String::from(msg);
        output.push_str("?user=");
        // output.push_str(&self.username);
        output.push_str(user);
        output
    }
    fn new_form(user: &str, pass: &str) -> Self {
        AdministratorForm {
            username: user,
            password: pass, 
        }
    }
}
















