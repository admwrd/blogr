
use rocket::config::{Config, Environment};

const COOKIE_IDENTIFIER_CONFIG_KEY: &'static str = "simpleauth_cookie_identifier";
const ADMIN_COOKIE_KEY: &'static str = "admin_cookie_identifier";
const USER_COOKIE_KEY: &'static str = "user_cookie_identifier";
const SECRET_KEY: &'static str = "8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=";


pub fn get_config() -> Config{
    Config::build(Environment::active().unwrap())
        .secret_key(SECRET_KEY)
        .extra(COOKIE_IDENTIFIER_CONFIG_KEY, "sid")
        .unwrap()
}
pub fn get_cookie_identifier() -> String{
    let config = get_config();
    config.get_str(COOKIE_IDENTIFIER_CONFIG_KEY).unwrap().to_owned()
}


pub fn get_user_config() -> Config {
    Config::build(Environment::active().unwrap())
        .secret_key(SECRET_KEY)
        .extra(USER_COOKIE_KEY, "sid")
        .unwrap()
}
pub fn user_cookie_identifier() -> String{
    let config = get_user_config();
    config.get_str(USER_COOKIE_KEY).unwrap().to_owned()
}


pub fn get_admin_config() -> Config {
    Config::build(Environment::active().unwrap())
        .secret_key(SECRET_KEY)
        .extra(ADMIN_COOKIE_KEY, "sid")
        .unwrap()
}
pub fn admin_cookie_identifier() -> String{
    let config = get_admin_config();
    config.get_str(ADMIN_COOKIE_KEY).unwrap().to_owned()
}

