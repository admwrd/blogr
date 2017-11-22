

use rocket::response::{NamedFile, Redirect, Flash};
use rocket::response::content::Html;
use rocket::request::{FlashMessage, Form};
use rocket::http::{Cookie, Cookies};
use r2d2_postgres::{PostgresConnectionManager, TlsMode};
use postgres::Connection;
use std::sync::Mutex;
use std::path::{Path, PathBuf};

use rocket_auth_login::authorization::*;
use rocket_auth_login::sanitization::*;
use ral_administrator::*;
use ral_user::*;

use ::templates::*;
use ::blog::*;
use ::data::*;
use ::layout::*;
use xpress::*;
use accept::*;

use super::*;
pub const URL_LOGIN_ADMIN: &'static str = "http://localhost:8000/admin_login";
pub const URL_LOGIN_USER: &'static str = "http://localhost:8000/user_login";

#[derive(Debug, Clone, FromForm)]
pub struct QueryUser {
    pub user: String,
}

/* Todo:
    Add a struct that implements the Responder trait
        Use this for adding an expiration header
    Add another struct that implements the Responder trait
        Use this for compressing the output using brotli/gzip/deflate
    Add structs that implement the Responder trait
        that will combine the expiration and compression responders
    Maybe even add a struct that will handle static file caching
        Database queries are cached by postgresql
            Look into how postgresql caches recent queries
            and look up how to increase how many queries are cached
            and when they are cached.  Try to get them cached sooner.
*/


#[get("/test")]
pub fn resp_test(encoding: AcceptCompression) -> Express {
    
    let template: String = hbs_template_string(TemplateBody::General(format!("Test successful. Encoding: {:?}", encoding), None), Some("Test Page".to_string()), String::from("/test"), None, None, None, None);
    // Express::From(template).compress(encoding)
    // Express::from_string(template).compress(encoding)
    // Express::from_string(template)
    let tempstr: Express = template.into();
    tempstr
}

#[get("/compress")]
pub fn compress_test(encoding: AcceptCompression) -> Express {
    
    let template: String = hbs_template_string(TemplateBody::General(format!("Test successful. Encoding: {:?}", encoding), None), Some("Test Page".to_string()), String::from("/test"), None, None, None, None);
    // Express::From(template).compress(encoding)
    // Express::from_string(template).compress(encoding)
    // Express::from_string(template)
    let tempstr: Express = template.into();
    tempstr.compress(encoding)
}

const TEST_TEXT: &'static str = r#"
<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque iaculis molestie elit quis ullamcorper. Ut rutrum fermentum metus at volutpat. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Sed vehicula enim urna. Ut id vestibulum arcu. Interdum et malesuada fames ac ante ipsum primis in faucibus. Nam condimentum, mauris sed bibendum feugiat, mauris eros dictum dolor, in vehicula quam purus sed elit. Nullam vitae dignissim sem, consectetur fringilla nulla. Quisque eget interdum felis, ac euismod nisl. Nullam sit amet dui arcu. Aliquam vestibulum vel lorem egestas iaculis. Suspendisse tellus purus, dictum in justo at, scelerisque lacinia enim. Nunc aliquet eget sapien sed convallis. Phasellus sed odio tortor. Aenean convallis condimentum erat, vitae viverra est varius non.</p>
<p>Aenean lacinia eget ligula faucibus gravida. Morbi tempor sollicitudin lectus, nec dignissim mauris luctus ac. Nunc luctus nunc sagittis lorem imperdiet, ac interdum orci sagittis. Vivamus quis nisl dolor. Quisque egestas sapien viverra porttitor ultricies. Vivamus rhoncus enim sit amet lobortis convallis. Integer a semper neque, in eleifend ex. Quisque diam ex, molestie pretium tristique id, iaculis sit amet est. Etiam vel massa pharetra lectus accumsan luctus.</p>
<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed in facilisis enim. Maecenas a tortor enim. Donec a euismod risus. Nullam purus tellus, egestas a nibh et, lacinia egestas velit. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras nibh turpis, euismod id ultricies vitae, tristique sed sapien. In interdum dolor a efficitur hendrerit. Sed consectetur mauris ut nisl dapibus, sit amet aliquam tellus dapibus. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Vestibulum iaculis cursus ullamcorper. Aenean tempor quam quis dolor pharetra auctor. Aliquam erat volutpat.</p>
<p>Duis et egestas mi. Nunc quis pulvinar nisi. Fusce mollis diam augue, et sodales metus efficitur quis. Duis sollicitudin euismod sapien eu congue. In non mauris sit amet ligula rutrum rutrum. Quisque vulputate molestie erat, vel interdum nunc gravida quis. Cras volutpat euismod ex, at ullamcorper urna. Nunc molestie purus sed odio fermentum, a volutpat metus dignissim. Sed tempus, nisl ac interdum interdum, felis urna cursus est, vitae venenatis augue ex in sapien. Cras ultrices efficitur lobortis. Suspendisse eu placerat risus. Mauris diam magna, euismod quis aliquet tincidunt, fringilla a mauris.</p>
<p>Nullam pulvinar nec neque vel dapibus. Praesent iaculis hendrerit mauris ac porta. Suspendisse rhoncus sagittis enim, vel aliquet leo tempor vitae. Curabitur tortor nisi, tempor nec sapien quis, bibendum aliquam ex. Donec vitae tristique ex. Nullam porta nunc pharetra mauris malesuada commodo eu vel elit. Pellentesque eget mattis leo. Donec egestas iaculis enim. Nam feugiat erat in velit efficitur faucibus. Maecenas rhoncus bibendum euismod. Ut convallis, nisl vel consectetur vulputate, est orci maximus est, sit amet tincidunt magna purus eu leo. Nunc ante magna, lobortis ut eros vitae, pellentesque vestibulum risus.</p>
<p>Nam aliquam dui sed sem pulvinar hendrerit. Mauris turpis mi, consequat ut viverra id, pharetra id nisl. Pellentesque eu molestie arcu, nec semper orci. Donec volutpat vehicula pharetra. Maecenas cursus odio suscipit risus commodo pellentesque. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Nunc in dictum metus. Vestibulum quis volutpat metus. Aenean nulla elit, volutpat quis diam non, volutpat volutpat est. Cras a imperdiet dolor. Praesent gravida, augue vitae fermentum finibus, nisi erat blandit sem, quis congue urna urna id enim. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Aliquam sit amet egestas arcu. Praesent tristique consequat pulvinar. Nam et tempor ipsum. Quisque tristique turpis nec orci tristique, in egestas nulla pulvinar.</p>
<p>Proin eget tellus aliquet, bibendum diam a, aliquet dui. Nullam pulvinar quam sit amet est tincidunt, nec malesuada ipsum molestie. Nunc pharetra interdum eros, vitae interdum dui scelerisque ut. Nulla eu interdum nunc. Proin vitae turpis nec mauris molestie eleifend. Morbi vehicula pellentesque nulla molestie porttitor. Cras ullamcorper dolor ac lorem accumsan, nec commodo neque blandit. Ut mollis porta quam eu auctor.</p>
<p>Sed sodales nisi vel ligula dignissim, vel tristique odio varius. Donec hendrerit vulputate felis et mattis. Pellentesque vel libero justo. Suspendisse dignissim eget diam ac finibus. Quisque vel pulvinar nisi. In hendrerit, enim vitae interdum fermentum, erat eros vehicula velit, vitae convallis magna turpis sit amet ante. Maecenas elementum ipsum odio, quis fringilla enim lacinia vel. Curabitur vestibulum quis metus id iaculis. Praesent a purus non eros tincidunt cursus. Sed eu porttitor orci. Duis suscipit nibh et mi egestas bibendum. Cras dignissim ipsum vel blandit rhoncus massa nunc.</p>
<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque iaculis molestie elit quis ullamcorper. Ut rutrum fermentum metus at volutpat. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Sed vehicula enim urna. Ut id vestibulum arcu. Interdum et malesuada fames ac ante ipsum primis in faucibus. Nam condimentum, mauris sed bibendum feugiat, mauris eros dictum dolor, in vehicula quam purus sed elit. Nullam vitae dignissim sem, consectetur fringilla nulla. Quisque eget interdum felis, ac euismod nisl. Nullam sit amet dui arcu. Aliquam vestibulum vel lorem egestas iaculis. Suspendisse tellus purus, dictum in justo at, scelerisque lacinia enim. Nunc aliquet eget sapien sed convallis. Phasellus sed odio tortor. Aenean convallis condimentum erat, vitae viverra est varius non.</p>
<p>Aenean lacinia eget ligula faucibus gravida. Morbi tempor sollicitudin lectus, nec dignissim mauris luctus ac. Nunc luctus nunc sagittis lorem imperdiet, ac interdum orci sagittis. Vivamus quis nisl dolor. Quisque egestas sapien viverra porttitor ultricies. Vivamus rhoncus enim sit amet lobortis convallis. Integer a semper neque, in eleifend ex. Quisque diam ex, molestie pretium tristique id, iaculis sit amet est. Etiam vel massa pharetra lectus accumsan luctus.</p>
<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed in facilisis enim. Maecenas a tortor enim. Donec a euismod risus. Nullam purus tellus, egestas a nibh et, lacinia egestas velit. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras nibh turpis, euismod id ultricies vitae, tristique sed sapien. In interdum dolor a efficitur hendrerit. Sed consectetur mauris ut nisl dapibus, sit amet aliquam tellus dapibus. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Vestibulum iaculis cursus ullamcorper. Aenean tempor quam quis dolor pharetra auctor. Aliquam erat volutpat.</p>
<p>Duis et egestas mi. Nunc quis pulvinar nisi. Fusce mollis diam augue, et sodales metus efficitur quis. Duis sollicitudin euismod sapien eu congue. In non mauris sit amet ligula rutrum rutrum. Quisque vulputate molestie erat, vel interdum nunc gravida quis. Cras volutpat euismod ex, at ullamcorper urna. Nunc molestie purus sed odio fermentum, a volutpat metus dignissim. Sed tempus, nisl ac interdum interdum, felis urna cursus est, vitae venenatis augue ex in sapien. Cras ultrices efficitur lobortis. Suspendisse eu placerat risus. Mauris diam magna, euismod quis aliquet tincidunt, fringilla a mauris.</p>
<p>Nullam pulvinar nec neque vel dapibus. Praesent iaculis hendrerit mauris ac porta. Suspendisse rhoncus sagittis enim, vel aliquet leo tempor vitae. Curabitur tortor nisi, tempor nec sapien quis, bibendum aliquam ex. Donec vitae tristique ex. Nullam porta nunc pharetra mauris malesuada commodo eu vel elit. Pellentesque eget mattis leo. Donec egestas iaculis enim. Nam feugiat erat in velit efficitur faucibus. Maecenas rhoncus bibendum euismod. Ut convallis, nisl vel consectetur vulputate, est orci maximus est, sit amet tincidunt magna purus eu leo. Nunc ante magna, lobortis ut eros vitae, pellentesque vestibulum risus.</p>
<p>Nam aliquam dui sed sem pulvinar hendrerit. Mauris turpis mi, consequat ut viverra id, pharetra id nisl. Pellentesque eu molestie arcu, nec semper orci. Donec volutpat vehicula pharetra. Maecenas cursus odio suscipit risus commodo pellentesque. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Nunc in dictum metus. Vestibulum quis volutpat metus. Aenean nulla elit, volutpat quis diam non, volutpat volutpat est. Cras a imperdiet dolor. Praesent gravida, augue vitae fermentum finibus, nisi erat blandit sem, quis congue urna urna id enim. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Aliquam sit amet egestas arcu. Praesent tristique consequat pulvinar. Nam et tempor ipsum. Quisque tristique turpis nec orci tristique, in egestas nulla pulvinar.</p>
<p>Proin eget tellus aliquet, bibendum diam a, aliquet dui. Nullam pulvinar quam sit amet est tincidunt, nec malesuada ipsum molestie. Nunc pharetra interdum eros, vitae interdum dui scelerisque ut. Nulla eu interdum nunc. Proin vitae turpis nec mauris molestie eleifend. Morbi vehicula pellentesque nulla molestie porttitor. Cras ullamcorper dolor ac lorem accumsan, nec commodo neque blandit. Ut mollis porta quam eu auctor.</p>
<p>Sed sodales nisi vel ligula dignissim, vel tristique odio varius. Donec hendrerit vulputate felis et mattis. Pellentesque vel libero justo. Suspendisse dignissim eget diam ac finibus. Quisque vel pulvinar nisi. In hendrerit, enim vitae interdum fermentum, erat eros vehicula velit, vitae convallis magna turpis sit amet ante. Maecenas elementum ipsum odio, quis fringilla enim lacinia vel. Curabitur vestibulum quis metus id iaculis. Praesent a purus non eros tincidunt cursus. Sed eu porttitor orci. Duis suscipit nibh et mi egestas bibendum. Cras dignissim ipsum vel blandit rhoncus massa nunc.</p>
<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque iaculis molestie elit quis ullamcorper. Ut rutrum fermentum metus at volutpat. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Sed vehicula enim urna. Ut id vestibulum arcu. Interdum et malesuada fames ac ante ipsum primis in faucibus. Nam condimentum, mauris sed bibendum feugiat, mauris eros dictum dolor, in vehicula quam purus sed elit. Nullam vitae dignissim sem, consectetur fringilla nulla. Quisque eget interdum felis, ac euismod nisl. Nullam sit amet dui arcu. Aliquam vestibulum vel lorem egestas iaculis. Suspendisse tellus purus, dictum in justo at, scelerisque lacinia enim. Nunc aliquet eget sapien sed convallis. Phasellus sed odio tortor. Aenean convallis condimentum erat, vitae viverra est varius non.</p>
<p>Aenean lacinia eget ligula faucibus gravida. Morbi tempor sollicitudin lectus, nec dignissim mauris luctus ac. Nunc luctus nunc sagittis lorem imperdiet, ac interdum orci sagittis. Vivamus quis nisl dolor. Quisque egestas sapien viverra porttitor ultricies. Vivamus rhoncus enim sit amet lobortis convallis. Integer a semper neque, in eleifend ex. Quisque diam ex, molestie pretium tristique id, iaculis sit amet est. Etiam vel massa pharetra lectus accumsan luctus.</p>
<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed in facilisis enim. Maecenas a tortor enim. Donec a euismod risus. Nullam purus tellus, egestas a nibh et, lacinia egestas velit. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras nibh turpis, euismod id ultricies vitae, tristique sed sapien. In interdum dolor a efficitur hendrerit. Sed consectetur mauris ut nisl dapibus, sit amet aliquam tellus dapibus. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Vestibulum iaculis cursus ullamcorper. Aenean tempor quam quis dolor pharetra auctor. Aliquam erat volutpat.</p>
<p>Duis et egestas mi. Nunc quis pulvinar nisi. Fusce mollis diam augue, et sodales metus efficitur quis. Duis sollicitudin euismod sapien eu congue. In non mauris sit amet ligula rutrum rutrum. Quisque vulputate molestie erat, vel interdum nunc gravida quis. Cras volutpat euismod ex, at ullamcorper urna. Nunc molestie purus sed odio fermentum, a volutpat metus dignissim. Sed tempus, nisl ac interdum interdum, felis urna cursus est, vitae venenatis augue ex in sapien. Cras ultrices efficitur lobortis. Suspendisse eu placerat risus. Mauris diam magna, euismod quis aliquet tincidunt, fringilla a mauris.</p>
<p>Nullam pulvinar nec neque vel dapibus. Praesent iaculis hendrerit mauris ac porta. Suspendisse rhoncus sagittis enim, vel aliquet leo tempor vitae. Curabitur tortor nisi, tempor nec sapien quis, bibendum aliquam ex. Donec vitae tristique ex. Nullam porta nunc pharetra mauris malesuada commodo eu vel elit. Pellentesque eget mattis leo. Donec egestas iaculis enim. Nam feugiat erat in velit efficitur faucibus. Maecenas rhoncus bibendum euismod. Ut convallis, nisl vel consectetur vulputate, est orci maximus est, sit amet tincidunt magna purus eu leo. Nunc ante magna, lobortis ut eros vitae, pellentesque vestibulum risus.</p>
<p>Nam aliquam dui sed sem pulvinar hendrerit. Mauris turpis mi, consequat ut viverra id, pharetra id nisl. Pellentesque eu molestie arcu, nec semper orci. Donec volutpat vehicula pharetra. Maecenas cursus odio suscipit risus commodo pellentesque. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Nunc in dictum metus. Vestibulum quis volutpat metus. Aenean nulla elit, volutpat quis diam non, volutpat volutpat est. Cras a imperdiet dolor. Praesent gravida, augue vitae fermentum finibus, nisi erat blandit sem, quis congue urna urna id enim. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Aliquam sit amet egestas arcu. Praesent tristique consequat pulvinar. Nam et tempor ipsum. Quisque tristique turpis nec orci tristique, in egestas nulla pulvinar.</p>
<p>Proin eget tellus aliquet, bibendum diam a, aliquet dui. Nullam pulvinar quam sit amet est tincidunt, nec malesuada ipsum molestie. Nunc pharetra interdum eros, vitae interdum dui scelerisque ut. Nulla eu interdum nunc. Proin vitae turpis nec mauris molestie eleifend. Morbi vehicula pellentesque nulla molestie porttitor. Cras ullamcorper dolor ac lorem accumsan, nec commodo neque blandit. Ut mollis porta quam eu auctor.</p>
<p>Sed sodales nisi vel ligula dignissim, vel tristique odio varius. Donec hendrerit vulputate felis et mattis. Pellentesque vel libero justo. Suspendisse dignissim eget diam ac finibus. Quisque vel pulvinar nisi. In hendrerit, enim vitae interdum fermentum, erat eros vehicula velit, vitae convallis magna turpis sit amet ante. Maecenas elementum ipsum odio, quis fringilla enim lacinia vel. Curabitur vestibulum quis metus id iaculis. Praesent a purus non eros tincidunt cursus. Sed eu porttitor orci. Duis suscipit nibh et mi egestas bibendum. Cras dignissim ipsum vel blandit rhoncus massa nunc.</p>
<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque iaculis molestie elit quis ullamcorper. Ut rutrum fermentum metus at volutpat. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Sed vehicula enim urna. Ut id vestibulum arcu. Interdum et malesuada fames ac ante ipsum primis in faucibus. Nam condimentum, mauris sed bibendum feugiat, mauris eros dictum dolor, in vehicula quam purus sed elit. Nullam vitae dignissim sem, consectetur fringilla nulla. Quisque eget interdum felis, ac euismod nisl. Nullam sit amet dui arcu. Aliquam vestibulum vel lorem egestas iaculis. Suspendisse tellus purus, dictum in justo at, scelerisque lacinia enim. Nunc aliquet eget sapien sed convallis. Phasellus sed odio tortor. Aenean convallis condimentum erat, vitae viverra est varius non.</p>
<p>Aenean lacinia eget ligula faucibus gravida. Morbi tempor sollicitudin lectus, nec dignissim mauris luctus ac. Nunc luctus nunc sagittis lorem imperdiet, ac interdum orci sagittis. Vivamus quis nisl dolor. Quisque egestas sapien viverra porttitor ultricies. Vivamus rhoncus enim sit amet lobortis convallis. Integer a semper neque, in eleifend ex. Quisque diam ex, molestie pretium tristique id, iaculis sit amet est. Etiam vel massa pharetra lectus accumsan luctus.</p>
<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed in facilisis enim. Maecenas a tortor enim. Donec a euismod risus. Nullam purus tellus, egestas a nibh et, lacinia egestas velit. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras nibh turpis, euismod id ultricies vitae, tristique sed sapien. In interdum dolor a efficitur hendrerit. Sed consectetur mauris ut nisl dapibus, sit amet aliquam tellus dapibus. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Vestibulum iaculis cursus ullamcorper. Aenean tempor quam quis dolor pharetra auctor. Aliquam erat volutpat.</p>
<p>Duis et egestas mi. Nunc quis pulvinar nisi. Fusce mollis diam augue, et sodales metus efficitur quis. Duis sollicitudin euismod sapien eu congue. In non mauris sit amet ligula rutrum rutrum. Quisque vulputate molestie erat, vel interdum nunc gravida quis. Cras volutpat euismod ex, at ullamcorper urna. Nunc molestie purus sed odio fermentum, a volutpat metus dignissim. Sed tempus, nisl ac interdum interdum, felis urna cursus est, vitae venenatis augue ex in sapien. Cras ultrices efficitur lobortis. Suspendisse eu placerat risus. Mauris diam magna, euismod quis aliquet tincidunt, fringilla a mauris.</p>
<p>Nullam pulvinar nec neque vel dapibus. Praesent iaculis hendrerit mauris ac porta. Suspendisse rhoncus sagittis enim, vel aliquet leo tempor vitae. Curabitur tortor nisi, tempor nec sapien quis, bibendum aliquam ex. Donec vitae tristique ex. Nullam porta nunc pharetra mauris malesuada commodo eu vel elit. Pellentesque eget mattis leo. Donec egestas iaculis enim. Nam feugiat erat in velit efficitur faucibus. Maecenas rhoncus bibendum euismod. Ut convallis, nisl vel consectetur vulputate, est orci maximus est, sit amet tincidunt magna purus eu leo. Nunc ante magna, lobortis ut eros vitae, pellentesque vestibulum risus.</p>
<p>Nam aliquam dui sed sem pulvinar hendrerit. Mauris turpis mi, consequat ut viverra id, pharetra id nisl. Pellentesque eu molestie arcu, nec semper orci. Donec volutpat vehicula pharetra. Maecenas cursus odio suscipit risus commodo pellentesque. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Nunc in dictum metus. Vestibulum quis volutpat metus. Aenean nulla elit, volutpat quis diam non, volutpat volutpat est. Cras a imperdiet dolor. Praesent gravida, augue vitae fermentum finibus, nisi erat blandit sem, quis congue urna urna id enim. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Aliquam sit amet egestas arcu. Praesent tristique consequat pulvinar. Nam et tempor ipsum. Quisque tristique turpis nec orci tristique, in egestas nulla pulvinar.</p>
<p>Proin eget tellus aliquet, bibendum diam a, aliquet dui. Nullam pulvinar quam sit amet est tincidunt, nec malesuada ipsum molestie. Nunc pharetra interdum eros, vitae interdum dui scelerisque ut. Nulla eu interdum nunc. Proin vitae turpis nec mauris molestie eleifend. Morbi vehicula pellentesque nulla molestie porttitor. Cras ullamcorper dolor ac lorem accumsan, nec commodo neque blandit. Ut mollis porta quam eu auctor.</p>
<p>Sed sodales nisi vel ligula dignissim, vel tristique odio varius. Donec hendrerit vulputate felis et mattis. Pellentesque vel libero justo. Suspendisse dignissim eget diam ac finibus. Quisque vel pulvinar nisi. In hendrerit, enim vitae interdum fermentum, erat eros vehicula velit, vitae convallis magna turpis sit amet ante. Maecenas elementum ipsum odio, quis fringilla enim lacinia vel. Curabitur vestibulum quis metus id iaculis. Praesent a purus non eros tincidunt cursus. Sed eu porttitor orci. Duis suscipit nibh et mi egestas bibendum. Cras dignissim ipsum vel blandit rhoncus massa nunc.</p>
<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque iaculis molestie elit quis ullamcorper. Ut rutrum fermentum metus at volutpat. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Sed vehicula enim urna. Ut id vestibulum arcu. Interdum et malesuada fames ac ante ipsum primis in faucibus. Nam condimentum, mauris sed bibendum feugiat, mauris eros dictum dolor, in vehicula quam purus sed elit. Nullam vitae dignissim sem, consectetur fringilla nulla. Quisque eget interdum felis, ac euismod nisl. Nullam sit amet dui arcu. Aliquam vestibulum vel lorem egestas iaculis. Suspendisse tellus purus, dictum in justo at, scelerisque lacinia enim. Nunc aliquet eget sapien sed convallis. Phasellus sed odio tortor. Aenean convallis condimentum erat, vitae viverra est varius non.</p>
<p>Aenean lacinia eget ligula faucibus gravida. Morbi tempor sollicitudin lectus, nec dignissim mauris luctus ac. Nunc luctus nunc sagittis lorem imperdiet, ac interdum orci sagittis. Vivamus quis nisl dolor. Quisque egestas sapien viverra porttitor ultricies. Vivamus rhoncus enim sit amet lobortis convallis. Integer a semper neque, in eleifend ex. Quisque diam ex, molestie pretium tristique id, iaculis sit amet est. Etiam vel massa pharetra lectus accumsan luctus.</p>
<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed in facilisis enim. Maecenas a tortor enim. Donec a euismod risus. Nullam purus tellus, egestas a nibh et, lacinia egestas velit. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras nibh turpis, euismod id ultricies vitae, tristique sed sapien. In interdum dolor a efficitur hendrerit. Sed consectetur mauris ut nisl dapibus, sit amet aliquam tellus dapibus. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Vestibulum iaculis cursus ullamcorper. Aenean tempor quam quis dolor pharetra auctor. Aliquam erat volutpat.</p>
<p>Duis et egestas mi. Nunc quis pulvinar nisi. Fusce mollis diam augue, et sodales metus efficitur quis. Duis sollicitudin euismod sapien eu congue. In non mauris sit amet ligula rutrum rutrum. Quisque vulputate molestie erat, vel interdum nunc gravida quis. Cras volutpat euismod ex, at ullamcorper urna. Nunc molestie purus sed odio fermentum, a volutpat metus dignissim. Sed tempus, nisl ac interdum interdum, felis urna cursus est, vitae venenatis augue ex in sapien. Cras ultrices efficitur lobortis. Suspendisse eu placerat risus. Mauris diam magna, euismod quis aliquet tincidunt, fringilla a mauris.</p>
<p>Nullam pulvinar nec neque vel dapibus. Praesent iaculis hendrerit mauris ac porta. Suspendisse rhoncus sagittis enim, vel aliquet leo tempor vitae. Curabitur tortor nisi, tempor nec sapien quis, bibendum aliquam ex. Donec vitae tristique ex. Nullam porta nunc pharetra mauris malesuada commodo eu vel elit. Pellentesque eget mattis leo. Donec egestas iaculis enim. Nam feugiat erat in velit efficitur faucibus. Maecenas rhoncus bibendum euismod. Ut convallis, nisl vel consectetur vulputate, est orci maximus est, sit amet tincidunt magna purus eu leo. Nunc ante magna, lobortis ut eros vitae, pellentesque vestibulum risus.</p>
<p>Nam aliquam dui sed sem pulvinar hendrerit. Mauris turpis mi, consequat ut viverra id, pharetra id nisl. Pellentesque eu molestie arcu, nec semper orci. Donec volutpat vehicula pharetra. Maecenas cursus odio suscipit risus commodo pellentesque. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Nunc in dictum metus. Vestibulum quis volutpat metus. Aenean nulla elit, volutpat quis diam non, volutpat volutpat est. Cras a imperdiet dolor. Praesent gravida, augue vitae fermentum finibus, nisi erat blandit sem, quis congue urna urna id enim. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Aliquam sit amet egestas arcu. Praesent tristique consequat pulvinar. Nam et tempor ipsum. Quisque tristique turpis nec orci tristique, in egestas nulla pulvinar.</p>
<p>Proin eget tellus aliquet, bibendum diam a, aliquet dui. Nullam pulvinar quam sit amet est tincidunt, nec malesuada ipsum molestie. Nunc pharetra interdum eros, vitae interdum dui scelerisque ut. Nulla eu interdum nunc. Proin vitae turpis nec mauris molestie eleifend. Morbi vehicula pellentesque nulla molestie porttitor. Cras ullamcorper dolor ac lorem accumsan, nec commodo neque blandit. Ut mollis porta quam eu auctor.</p>
<p>Sed sodales nisi vel ligula dignissim, vel tristique odio varius. Donec hendrerit vulputate felis et mattis. Pellentesque vel libero justo. Suspendisse dignissim eget diam ac finibus. Quisque vel pulvinar nisi. In hendrerit, enim vitae interdum fermentum, erat eros vehicula velit, vitae convallis magna turpis sit amet ante. Maecenas elementum ipsum odio, quis fringilla enim lacinia vel. Curabitur vestibulum quis metus id iaculis. Praesent a purus non eros tincidunt cursus. Sed eu porttitor orci. Duis suscipit nibh et mi egestas bibendum. Cras dignissim ipsum vel blandit rhoncus massa nunc.</p>
<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque iaculis molestie elit quis ullamcorper. Ut rutrum fermentum metus at volutpat. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Sed vehicula enim urna. Ut id vestibulum arcu. Interdum et malesuada fames ac ante ipsum primis in faucibus. Nam condimentum, mauris sed bibendum feugiat, mauris eros dictum dolor, in vehicula quam purus sed elit. Nullam vitae dignissim sem, consectetur fringilla nulla. Quisque eget interdum felis, ac euismod nisl. Nullam sit amet dui arcu. Aliquam vestibulum vel lorem egestas iaculis. Suspendisse tellus purus, dictum in justo at, scelerisque lacinia enim. Nunc aliquet eget sapien sed convallis. Phasellus sed odio tortor. Aenean convallis condimentum erat, vitae viverra est varius non.</p>
<p>Aenean lacinia eget ligula faucibus gravida. Morbi tempor sollicitudin lectus, nec dignissim mauris luctus ac. Nunc luctus nunc sagittis lorem imperdiet, ac interdum orci sagittis. Vivamus quis nisl dolor. Quisque egestas sapien viverra porttitor ultricies. Vivamus rhoncus enim sit amet lobortis convallis. Integer a semper neque, in eleifend ex. Quisque diam ex, molestie pretium tristique id, iaculis sit amet est. Etiam vel massa pharetra lectus accumsan luctus.</p>
<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed in facilisis enim. Maecenas a tortor enim. Donec a euismod risus. Nullam purus tellus, egestas a nibh et, lacinia egestas velit. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras nibh turpis, euismod id ultricies vitae, tristique sed sapien. In interdum dolor a efficitur hendrerit. Sed consectetur mauris ut nisl dapibus, sit amet aliquam tellus dapibus. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Vestibulum iaculis cursus ullamcorper. Aenean tempor quam quis dolor pharetra auctor. Aliquam erat volutpat.</p>
<p>Duis et egestas mi. Nunc quis pulvinar nisi. Fusce mollis diam augue, et sodales metus efficitur quis. Duis sollicitudin euismod sapien eu congue. In non mauris sit amet ligula rutrum rutrum. Quisque vulputate molestie erat, vel interdum nunc gravida quis. Cras volutpat euismod ex, at ullamcorper urna. Nunc molestie purus sed odio fermentum, a volutpat metus dignissim. Sed tempus, nisl ac interdum interdum, felis urna cursus est, vitae venenatis augue ex in sapien. Cras ultrices efficitur lobortis. Suspendisse eu placerat risus. Mauris diam magna, euismod quis aliquet tincidunt, fringilla a mauris.</p>
<p>Nullam pulvinar nec neque vel dapibus. Praesent iaculis hendrerit mauris ac porta. Suspendisse rhoncus sagittis enim, vel aliquet leo tempor vitae. Curabitur tortor nisi, tempor nec sapien quis, bibendum aliquam ex. Donec vitae tristique ex. Nullam porta nunc pharetra mauris malesuada commodo eu vel elit. Pellentesque eget mattis leo. Donec egestas iaculis enim. Nam feugiat erat in velit efficitur faucibus. Maecenas rhoncus bibendum euismod. Ut convallis, nisl vel consectetur vulputate, est orci maximus est, sit amet tincidunt magna purus eu leo. Nunc ante magna, lobortis ut eros vitae, pellentesque vestibulum risus.</p>
<p>Nam aliquam dui sed sem pulvinar hendrerit. Mauris turpis mi, consequat ut viverra id, pharetra id nisl. Pellentesque eu molestie arcu, nec semper orci. Donec volutpat vehicula pharetra. Maecenas cursus odio suscipit risus commodo pellentesque. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Nunc in dictum metus. Vestibulum quis volutpat metus. Aenean nulla elit, volutpat quis diam non, volutpat volutpat est. Cras a imperdiet dolor. Praesent gravida, augue vitae fermentum finibus, nisi erat blandit sem, quis congue urna urna id enim. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Aliquam sit amet egestas arcu. Praesent tristique consequat pulvinar. Nam et tempor ipsum. Quisque tristique turpis nec orci tristique, in egestas nulla pulvinar.</p>
<p>Proin eget tellus aliquet, bibendum diam a, aliquet dui. Nullam pulvinar quam sit amet est tincidunt, nec malesuada ipsum molestie. Nunc pharetra interdum eros, vitae interdum dui scelerisque ut. Nulla eu interdum nunc. Proin vitae turpis nec mauris molestie eleifend. Morbi vehicula pellentesque nulla molestie porttitor. Cras ullamcorper dolor ac lorem accumsan, nec commodo neque blandit. Ut mollis porta quam eu auctor.</p>
<p>Sed sodales nisi vel ligula dignissim, vel tristique odio varius. Donec hendrerit vulputate felis et mattis. Pellentesque vel libero justo. Suspendisse dignissim eget diam ac finibus. Quisque vel pulvinar nisi. In hendrerit, enim vitae interdum fermentum, erat eros vehicula velit, vitae convallis magna turpis sit amet ante. Maecenas elementum ipsum odio, quis fringilla enim lacinia vel. Curabitur vestibulum quis metus id iaculis. Praesent a purus non eros tincidunt cursus. Sed eu porttitor orci. Duis suscipit nibh et mi egestas bibendum. Cras dignissim ipsum vel blandit rhoncus massa nunc.</p>
<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque iaculis molestie elit quis ullamcorper. Ut rutrum fermentum metus at volutpat. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Sed vehicula enim urna. Ut id vestibulum arcu. Interdum et malesuada fames ac ante ipsum primis in faucibus. Nam condimentum, mauris sed bibendum feugiat, mauris eros dictum dolor, in vehicula quam purus sed elit. Nullam vitae dignissim sem, consectetur fringilla nulla. Quisque eget interdum felis, ac euismod nisl. Nullam sit amet dui arcu. Aliquam vestibulum vel lorem egestas iaculis. Suspendisse tellus purus, dictum in justo at, scelerisque lacinia enim. Nunc aliquet eget sapien sed convallis. Phasellus sed odio tortor. Aenean convallis condimentum erat, vitae viverra est varius non.</p>
<p>Aenean lacinia eget ligula faucibus gravida. Morbi tempor sollicitudin lectus, nec dignissim mauris luctus ac. Nunc luctus nunc sagittis lorem imperdiet, ac interdum orci sagittis. Vivamus quis nisl dolor. Quisque egestas sapien viverra porttitor ultricies. Vivamus rhoncus enim sit amet lobortis convallis. Integer a semper neque, in eleifend ex. Quisque diam ex, molestie pretium tristique id, iaculis sit amet est. Etiam vel massa pharetra lectus accumsan luctus.</p>
<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed in facilisis enim. Maecenas a tortor enim. Donec a euismod risus. Nullam purus tellus, egestas a nibh et, lacinia egestas velit. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras nibh turpis, euismod id ultricies vitae, tristique sed sapien. In interdum dolor a efficitur hendrerit. Sed consectetur mauris ut nisl dapibus, sit amet aliquam tellus dapibus. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Vestibulum iaculis cursus ullamcorper. Aenean tempor quam quis dolor pharetra auctor. Aliquam erat volutpat.</p>
<p>Duis et egestas mi. Nunc quis pulvinar nisi. Fusce mollis diam augue, et sodales metus efficitur quis. Duis sollicitudin euismod sapien eu congue. In non mauris sit amet ligula rutrum rutrum. Quisque vulputate molestie erat, vel interdum nunc gravida quis. Cras volutpat euismod ex, at ullamcorper urna. Nunc molestie purus sed odio fermentum, a volutpat metus dignissim. Sed tempus, nisl ac interdum interdum, felis urna cursus est, vitae venenatis augue ex in sapien. Cras ultrices efficitur lobortis. Suspendisse eu placerat risus. Mauris diam magna, euismod quis aliquet tincidunt, fringilla a mauris.</p>
<p>Nullam pulvinar nec neque vel dapibus. Praesent iaculis hendrerit mauris ac porta. Suspendisse rhoncus sagittis enim, vel aliquet leo tempor vitae. Curabitur tortor nisi, tempor nec sapien quis, bibendum aliquam ex. Donec vitae tristique ex. Nullam porta nunc pharetra mauris malesuada commodo eu vel elit. Pellentesque eget mattis leo. Donec egestas iaculis enim. Nam feugiat erat in velit efficitur faucibus. Maecenas rhoncus bibendum euismod. Ut convallis, nisl vel consectetur vulputate, est orci maximus est, sit amet tincidunt magna purus eu leo. Nunc ante magna, lobortis ut eros vitae, pellentesque vestibulum risus.</p>
<p>Nam aliquam dui sed sem pulvinar hendrerit. Mauris turpis mi, consequat ut viverra id, pharetra id nisl. Pellentesque eu molestie arcu, nec semper orci. Donec volutpat vehicula pharetra. Maecenas cursus odio suscipit risus commodo pellentesque. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Nunc in dictum metus. Vestibulum quis volutpat metus. Aenean nulla elit, volutpat quis diam non, volutpat volutpat est. Cras a imperdiet dolor. Praesent gravida, augue vitae fermentum finibus, nisi erat blandit sem, quis congue urna urna id enim. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Aliquam sit amet egestas arcu. Praesent tristique consequat pulvinar. Nam et tempor ipsum. Quisque tristique turpis nec orci tristique, in egestas nulla pulvinar.</p>
<p>Proin eget tellus aliquet, bibendum diam a, aliquet dui. Nullam pulvinar quam sit amet est tincidunt, nec malesuada ipsum molestie. Nunc pharetra interdum eros, vitae interdum dui scelerisque ut. Nulla eu interdum nunc. Proin vitae turpis nec mauris molestie eleifend. Morbi vehicula pellentesque nulla molestie porttitor. Cras ullamcorper dolor ac lorem accumsan, nec commodo neque blandit. Ut mollis porta quam eu auctor.</p>
<p>Sed sodales nisi vel ligula dignissim, vel tristique odio varius. Donec hendrerit vulputate felis et mattis. Pellentesque vel libero justo. Suspendisse dignissim eget diam ac finibus. Quisque vel pulvinar nisi. In hendrerit, enim vitae interdum fermentum, erat eros vehicula velit, vitae convallis magna turpis sit amet ante. Maecenas elementum ipsum odio, quis fringilla enim lacinia vel. Curabitur vestibulum quis metus id iaculis. Praesent a purus non eros tincidunt cursus. Sed eu porttitor orci. Duis suscipit nibh et mi egestas bibendum. Cras dignissim ipsum vel blandit rhoncus massa nunc.</p>
<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque iaculis molestie elit quis ullamcorper. Ut rutrum fermentum metus at volutpat. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Sed vehicula enim urna. Ut id vestibulum arcu. Interdum et malesuada fames ac ante ipsum primis in faucibus. Nam condimentum, mauris sed bibendum feugiat, mauris eros dictum dolor, in vehicula quam purus sed elit. Nullam vitae dignissim sem, consectetur fringilla nulla. Quisque eget interdum felis, ac euismod nisl. Nullam sit amet dui arcu. Aliquam vestibulum vel lorem egestas iaculis. Suspendisse tellus purus, dictum in justo at, scelerisque lacinia enim. Nunc aliquet eget sapien sed convallis. Phasellus sed odio tortor. Aenean convallis condimentum erat, vitae viverra est varius non.</p>
<p>Aenean lacinia eget ligula faucibus gravida. Morbi tempor sollicitudin lectus, nec dignissim mauris luctus ac. Nunc luctus nunc sagittis lorem imperdiet, ac interdum orci sagittis. Vivamus quis nisl dolor. Quisque egestas sapien viverra porttitor ultricies. Vivamus rhoncus enim sit amet lobortis convallis. Integer a semper neque, in eleifend ex. Quisque diam ex, molestie pretium tristique id, iaculis sit amet est. Etiam vel massa pharetra lectus accumsan luctus.</p>
<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed in facilisis enim. Maecenas a tortor enim. Donec a euismod risus. Nullam purus tellus, egestas a nibh et, lacinia egestas velit. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras nibh turpis, euismod id ultricies vitae, tristique sed sapien. In interdum dolor a efficitur hendrerit. Sed consectetur mauris ut nisl dapibus, sit amet aliquam tellus dapibus. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Vestibulum iaculis cursus ullamcorper. Aenean tempor quam quis dolor pharetra auctor. Aliquam erat volutpat.</p>
<p>Duis et egestas mi. Nunc quis pulvinar nisi. Fusce mollis diam augue, et sodales metus efficitur quis. Duis sollicitudin euismod sapien eu congue. In non mauris sit amet ligula rutrum rutrum. Quisque vulputate molestie erat, vel interdum nunc gravida quis. Cras volutpat euismod ex, at ullamcorper urna. Nunc molestie purus sed odio fermentum, a volutpat metus dignissim. Sed tempus, nisl ac interdum interdum, felis urna cursus est, vitae venenatis augue ex in sapien. Cras ultrices efficitur lobortis. Suspendisse eu placerat risus. Mauris diam magna, euismod quis aliquet tincidunt, fringilla a mauris.</p>
<p>Nullam pulvinar nec neque vel dapibus. Praesent iaculis hendrerit mauris ac porta. Suspendisse rhoncus sagittis enim, vel aliquet leo tempor vitae. Curabitur tortor nisi, tempor nec sapien quis, bibendum aliquam ex. Donec vitae tristique ex. Nullam porta nunc pharetra mauris malesuada commodo eu vel elit. Pellentesque eget mattis leo. Donec egestas iaculis enim. Nam feugiat erat in velit efficitur faucibus. Maecenas rhoncus bibendum euismod. Ut convallis, nisl vel consectetur vulputate, est orci maximus est, sit amet tincidunt magna purus eu leo. Nunc ante magna, lobortis ut eros vitae, pellentesque vestibulum risus.</p>
<p>Nam aliquam dui sed sem pulvinar hendrerit. Mauris turpis mi, consequat ut viverra id, pharetra id nisl. Pellentesque eu molestie arcu, nec semper orci. Donec volutpat vehicula pharetra. Maecenas cursus odio suscipit risus commodo pellentesque. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Nunc in dictum metus. Vestibulum quis volutpat metus. Aenean nulla elit, volutpat quis diam non, volutpat volutpat est. Cras a imperdiet dolor. Praesent gravida, augue vitae fermentum finibus, nisi erat blandit sem, quis congue urna urna id enim. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Aliquam sit amet egestas arcu. Praesent tristique consequat pulvinar. Nam et tempor ipsum. Quisque tristique turpis nec orci tristique, in egestas nulla pulvinar.</p>
<p>Proin eget tellus aliquet, bibendum diam a, aliquet dui. Nullam pulvinar quam sit amet est tincidunt, nec malesuada ipsum molestie. Nunc pharetra interdum eros, vitae interdum dui scelerisque ut. Nulla eu interdum nunc. Proin vitae turpis nec mauris molestie eleifend. Morbi vehicula pellentesque nulla molestie porttitor. Cras ullamcorper dolor ac lorem accumsan, nec commodo neque blandit. Ut mollis porta quam eu auctor.</p>
<p>Sed sodales nisi vel ligula dignissim, vel tristique odio varius. Donec hendrerit vulputate felis et mattis. Pellentesque vel libero justo. Suspendisse dignissim eget diam ac finibus. Quisque vel pulvinar nisi. In hendrerit, enim vitae interdum fermentum, erat eros vehicula velit, vitae convallis magna turpis sit amet ante. Maecenas elementum ipsum odio, quis fringilla enim lacinia vel. Curabitur vestibulum quis metus id iaculis. Praesent a purus non eros tincidunt cursus. Sed eu porttitor orci. Duis suscipit nibh et mi egestas bibendum. Cras dignissim ipsum vel blandit rhoncus massa nunc.</p>
<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque iaculis molestie elit quis ullamcorper. Ut rutrum fermentum metus at volutpat. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Sed vehicula enim urna. Ut id vestibulum arcu. Interdum et malesuada fames ac ante ipsum primis in faucibus. Nam condimentum, mauris sed bibendum feugiat, mauris eros dictum dolor, in vehicula quam purus sed elit. Nullam vitae dignissim sem, consectetur fringilla nulla. Quisque eget interdum felis, ac euismod nisl. Nullam sit amet dui arcu. Aliquam vestibulum vel lorem egestas iaculis. Suspendisse tellus purus, dictum in justo at, scelerisque lacinia enim. Nunc aliquet eget sapien sed convallis. Phasellus sed odio tortor. Aenean convallis condimentum erat, vitae viverra est varius non.</p>
<p>Aenean lacinia eget ligula faucibus gravida. Morbi tempor sollicitudin lectus, nec dignissim mauris luctus ac. Nunc luctus nunc sagittis lorem imperdiet, ac interdum orci sagittis. Vivamus quis nisl dolor. Quisque egestas sapien viverra porttitor ultricies. Vivamus rhoncus enim sit amet lobortis convallis. Integer a semper neque, in eleifend ex. Quisque diam ex, molestie pretium tristique id, iaculis sit amet est. Etiam vel massa pharetra lectus accumsan luctus.</p>
<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed in facilisis enim. Maecenas a tortor enim. Donec a euismod risus. Nullam purus tellus, egestas a nibh et, lacinia egestas velit. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras nibh turpis, euismod id ultricies vitae, tristique sed sapien. In interdum dolor a efficitur hendrerit. Sed consectetur mauris ut nisl dapibus, sit amet aliquam tellus dapibus. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Vestibulum iaculis cursus ullamcorper. Aenean tempor quam quis dolor pharetra auctor. Aliquam erat volutpat.</p>
<p>Duis et egestas mi. Nunc quis pulvinar nisi. Fusce mollis diam augue, et sodales metus efficitur quis. Duis sollicitudin euismod sapien eu congue. In non mauris sit amet ligula rutrum rutrum. Quisque vulputate molestie erat, vel interdum nunc gravida quis. Cras volutpat euismod ex, at ullamcorper urna. Nunc molestie purus sed odio fermentum, a volutpat metus dignissim. Sed tempus, nisl ac interdum interdum, felis urna cursus est, vitae venenatis augue ex in sapien. Cras ultrices efficitur lobortis. Suspendisse eu placerat risus. Mauris diam magna, euismod quis aliquet tincidunt, fringilla a mauris.</p>
<p>Nullam pulvinar nec neque vel dapibus. Praesent iaculis hendrerit mauris ac porta. Suspendisse rhoncus sagittis enim, vel aliquet leo tempor vitae. Curabitur tortor nisi, tempor nec sapien quis, bibendum aliquam ex. Donec vitae tristique ex. Nullam porta nunc pharetra mauris malesuada commodo eu vel elit. Pellentesque eget mattis leo. Donec egestas iaculis enim. Nam feugiat erat in velit efficitur faucibus. Maecenas rhoncus bibendum euismod. Ut convallis, nisl vel consectetur vulputate, est orci maximus est, sit amet tincidunt magna purus eu leo. Nunc ante magna, lobortis ut eros vitae, pellentesque vestibulum risus.</p>
<p>Nam aliquam dui sed sem pulvinar hendrerit. Mauris turpis mi, consequat ut viverra id, pharetra id nisl. Pellentesque eu molestie arcu, nec semper orci. Donec volutpat vehicula pharetra. Maecenas cursus odio suscipit risus commodo pellentesque. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Nunc in dictum metus. Vestibulum quis volutpat metus. Aenean nulla elit, volutpat quis diam non, volutpat volutpat est. Cras a imperdiet dolor. Praesent gravida, augue vitae fermentum finibus, nisi erat blandit sem, quis congue urna urna id enim. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Aliquam sit amet egestas arcu. Praesent tristique consequat pulvinar. Nam et tempor ipsum. Quisque tristique turpis nec orci tristique, in egestas nulla pulvinar.</p>
<p>Proin eget tellus aliquet, bibendum diam a, aliquet dui. Nullam pulvinar quam sit amet est tincidunt, nec malesuada ipsum molestie. Nunc pharetra interdum eros, vitae interdum dui scelerisque ut. Nulla eu interdum nunc. Proin vitae turpis nec mauris molestie eleifend. Morbi vehicula pellentesque nulla molestie porttitor. Cras ullamcorper dolor ac lorem accumsan, nec commodo neque blandit. Ut mollis porta quam eu auctor.</p>
<p>Sed sodales nisi vel ligula dignissim, vel tristique odio varius. Donec hendrerit vulputate felis et mattis. Pellentesque vel libero justo. Suspendisse dignissim eget diam ac finibus. Quisque vel pulvinar nisi. In hendrerit, enim vitae interdum fermentum, erat eros vehicula velit, vitae convallis magna turpis sit amet ante. Maecenas elementum ipsum odio, quis fringilla enim lacinia vel. Curabitur vestibulum quis metus id iaculis. Praesent a purus non eros tincidunt cursus. Sed eu porttitor orci. Duis suscipit nibh et mi egestas bibendum. Cras dignissim ipsum vel blandit rhoncus massa nunc.</p>
<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Quisque iaculis molestie elit quis ullamcorper. Ut rutrum fermentum metus at volutpat. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia Curae; Sed vehicula enim urna. Ut id vestibulum arcu. Interdum et malesuada fames ac ante ipsum primis in faucibus. Nam condimentum, mauris sed bibendum feugiat, mauris eros dictum dolor, in vehicula quam purus sed elit. Nullam vitae dignissim sem, consectetur fringilla nulla. Quisque eget interdum felis, ac euismod nisl. Nullam sit amet dui arcu. Aliquam vestibulum vel lorem egestas iaculis. Suspendisse tellus purus, dictum in justo at, scelerisque lacinia enim. Nunc aliquet eget sapien sed convallis. Phasellus sed odio tortor. Aenean convallis condimentum erat, vitae viverra est varius non.</p>
<p>Aenean lacinia eget ligula faucibus gravida. Morbi tempor sollicitudin lectus, nec dignissim mauris luctus ac. Nunc luctus nunc sagittis lorem imperdiet, ac interdum orci sagittis. Vivamus quis nisl dolor. Quisque egestas sapien viverra porttitor ultricies. Vivamus rhoncus enim sit amet lobortis convallis. Integer a semper neque, in eleifend ex. Quisque diam ex, molestie pretium tristique id, iaculis sit amet est. Etiam vel massa pharetra lectus accumsan luctus.</p>
<p>Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed in facilisis enim. Maecenas a tortor enim. Donec a euismod risus. Nullam purus tellus, egestas a nibh et, lacinia egestas velit. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras nibh turpis, euismod id ultricies vitae, tristique sed sapien. In interdum dolor a efficitur hendrerit. Sed consectetur mauris ut nisl dapibus, sit amet aliquam tellus dapibus. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Vestibulum iaculis cursus ullamcorper. Aenean tempor quam quis dolor pharetra auctor. Aliquam erat volutpat.</p>
<p>Duis et egestas mi. Nunc quis pulvinar nisi. Fusce mollis diam augue, et sodales metus efficitur quis. Duis sollicitudin euismod sapien eu congue. In non mauris sit amet ligula rutrum rutrum. Quisque vulputate molestie erat, vel interdum nunc gravida quis. Cras volutpat euismod ex, at ullamcorper urna. Nunc molestie purus sed odio fermentum, a volutpat metus dignissim. Sed tempus, nisl ac interdum interdum, felis urna cursus est, vitae venenatis augue ex in sapien. Cras ultrices efficitur lobortis. Suspendisse eu placerat risus. Mauris diam magna, euismod quis aliquet tincidunt, fringilla a mauris.</p>
<p>Nullam pulvinar nec neque vel dapibus. Praesent iaculis hendrerit mauris ac porta. Suspendisse rhoncus sagittis enim, vel aliquet leo tempor vitae. Curabitur tortor nisi, tempor nec sapien quis, bibendum aliquam ex. Donec vitae tristique ex. Nullam porta nunc pharetra mauris malesuada commodo eu vel elit. Pellentesque eget mattis leo. Donec egestas iaculis enim. Nam feugiat erat in velit efficitur faucibus. Maecenas rhoncus bibendum euismod. Ut convallis, nisl vel consectetur vulputate, est orci maximus est, sit amet tincidunt magna purus eu leo. Nunc ante magna, lobortis ut eros vitae, pellentesque vestibulum risus.</p>
<p>Nam aliquam dui sed sem pulvinar hendrerit. Mauris turpis mi, consequat ut viverra id, pharetra id nisl. Pellentesque eu molestie arcu, nec semper orci. Donec volutpat vehicula pharetra. Maecenas cursus odio suscipit risus commodo pellentesque. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Nunc in dictum metus. Vestibulum quis volutpat metus. Aenean nulla elit, volutpat quis diam non, volutpat volutpat est. Cras a imperdiet dolor. Praesent gravida, augue vitae fermentum finibus, nisi erat blandit sem, quis congue urna urna id enim. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Aliquam sit amet egestas arcu. Praesent tristique consequat pulvinar. Nam et tempor ipsum. Quisque tristique turpis nec orci tristique, in egestas nulla pulvinar.</p>
<p>Proin eget tellus aliquet, bibendum diam a, aliquet dui. Nullam pulvinar quam sit amet est tincidunt, nec malesuada ipsum molestie. Nunc pharetra interdum eros, vitae interdum dui scelerisque ut. Nulla eu interdum nunc. Proin vitae turpis nec mauris molestie eleifend. Morbi vehicula pellentesque nulla molestie porttitor. Cras ullamcorper dolor ac lorem accumsan, nec commodo neque blandit. Ut mollis porta quam eu auctor.</p>
<p>Sed sodales nisi vel ligula dignissim, vel tristique odio varius. Donec hendrerit vulputate felis et mattis. Pellentesque vel libero justo. Suspendisse dignissim eget diam ac finibus. Quisque vel pulvinar nisi. In hendrerit, enim vitae interdum fermentum, erat eros vehicula velit, vitae convallis magna turpis sit amet ante. Maecenas elementum ipsum odio, quis fringilla enim lacinia vel. Curabitur vestibulum quis metus id iaculis. Praesent a purus non eros tincidunt cursus. Sed eu porttitor orci. Duis suscipit nibh et mi egestas bibendum. Cras dignissim ipsum vel blandit rhoncus massa nunc.</p>
"#;

#[get("/uncompressed")]
pub fn uncompressed() ->  Template {
    hbs_template(TemplateBody::General(TEST_TEXT.to_string(), None), Some("Test Page".to_string()), String::from("/test"), None, None, None, None)
}

#[get("/compress2")]
pub fn compress_test2(encoding: AcceptCompression) -> Express {
    
    // let text
    
    let template: String = hbs_template_string(TemplateBody::General(TEST_TEXT.to_string(), None), Some("Test Page".to_string()), String::from("/test"), None, None, None, None);
    // Express::From(template).compress(encoding)
    // Express::from_string(template).compress(encoding)
    // Express::from_string(template)
    let tempstr: Express = template.into();
    // tempstr.compress(encoding)
    tempstr
}

#[get("/compress3")]
pub fn compress_test3(encoding: AcceptCompression) -> Express {
    
    // let text
    
    // let template_string: String = hbs_template_string(TemplateBody::General(TEST_TEXT.to_string(), None), Some("Test Page".to_string()), String::from("/test"), None, None, None, None);
    let template_template: Template = hbs_template(TemplateBody::General(TEST_TEXT.to_string(), None), Some("Test Page".to_string()), String::from("/test"), None, None, None, None);
    
    // Express::From(template).compress(encoding)
    // Express::from_string(template).compress(encoding)
    // Express::from_string(template)
    
    // let tempstr: Express = template_string.into();
    let tempstr: Express = template_template.into();
    // tempstr.compress(encoding)
    tempstr.clone()
}

#[get("/compress4")]
pub fn compress_test4(encoding: AcceptCompression) -> Express {
    
    // let text
    
    // let template_string: String = hbs_template_string(TemplateBody::General(TEST_TEXT.to_string(), None), Some("Test Page".to_string()), String::from("/test"), None, None, None, None);
    let template_template: String = hbs_template_string(TemplateBody::General(TEST_TEXT.to_string(), None), Some("Test Page".to_string()), String::from("/test"), None, None, None, None);
    
    // Express::From(template).compress(encoding)
    // Express::from_string(template).compress(encoding)
    // Express::from_string(template)
    
    // let tempstr: Express = template_string.into();
    let express: Express = template_template.into();
    // tempstr.compress(encoding)
    express.compress(encoding)
}

#[get("/gzip")]
pub fn compress_gzip(encoding: AcceptCompression) -> Express {
    let template_template: Template = hbs_template(TemplateBody::General(TEST_TEXT.to_string(), None), Some("Test Page".to_string()), String::from("/test"), None, None, None, None);

    let express: Express = template_template.into();
    express.compress( encoding.checked_gzip() )
}


#[get("/deflate")]
pub fn compress_deflate(encoding: AcceptCompression) -> Express {
    let template_template: Template = hbs_template(TemplateBody::General(TEST_TEXT.to_string(), None), Some("Test Page".to_string()), String::from("/test"), None, None, None, None);

    let express: Express = template_template.into();
    express.compress( encoding.checked_deflate() )
}


#[get("/brotli")]
pub fn compress_brotli(encoding: AcceptCompression) -> Express {
    let template_template: Template = hbs_template(TemplateBody::General(TEST_TEXT.to_string(), None), Some("Test Page".to_string()), String::from("/test"), None, None, None, None);

    let express: Express = template_template.into();
    express.compress( encoding.checked_brotli() )
}



#[get("/admin_dashboard")]
pub fn dashboard_admin_authorized(admin: AdministratorCookie, conn: DbConn) -> Template {
    let start = Instant::now();
    
    let output: Template = hbs_template(TemplateBody::General(format!("Welcome Administrator {user}.  You are viewing the administrator dashboard page.", user=admin.username), None), Some("Dashboard".to_string()), String::from("/admin_dashboard"), None, None, None, Some(start));
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/admin_dashboard", rank = 2)]
pub fn dashboard_admin_unauthorized() -> Template {
    hbs_template(
        TemplateBody::General(
            "You are not logged in. <a href=\"/admin_login\">Administrator Login</a>".to_string(), None
        ), 
        Some("Administrator Login".to_string()), 
        String::from("/admin_dashboard_error"), 
        None, 
        None, 
        None, 
        None
    )
}

#[get("/admin_login", rank = 1)]
pub fn dashboard_admin_login() -> Template {
    hbs_template(TemplateBody::Login(URL_LOGIN_ADMIN.to_string(), None, None), Some("Administrator Login".to_string()), String::from("/admin_login"), None, None, None, None)
}

#[get("/admin_login?<user>")]
// fn dashboard_retry_user(user: UserQuery, flash_msg_opt: Option<FlashMessage>) -> Template {
// fn dashboard_retry_user(mut user: String, flash_msg_opt: Option<FlashMessage>) -> Template {
pub fn dashboard_admin_retry_user(mut user: QueryUser, flash_msg_opt: Option<FlashMessage>) -> Template {
    let start = Instant::now();
    // user = login::sanitization::sanitize(&user);
    let username = if &user.user != "" { Some(user.user.clone() ) } else { None };
    let flash = if let Some(f) = flash_msg_opt { Some(alert_danger(f.msg())) } else { None };
    let output = hbs_template(TemplateBody::Login(URL_LOGIN_ADMIN.to_string(), username, flash), Some("Administrator Login".to_string()), String::from("/admin_login"), None, None, None, Some(start));
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/admin_login", rank = 2)]
pub fn dashboard_admin_retry_flash(flash_msg: FlashMessage) -> Template {
    let start = Instant::now();
    
    let flash = Some( alert_danger(flash_msg.msg()) );
    let output = hbs_template(TemplateBody::Login(URL_LOGIN_ADMIN.to_string(), None, flash), Some("Administrator Login".to_string()), String::from("/admin_login"), None, None, None, Some(start));
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[allow(unused_mut)]
#[post("/admin_login", data = "<form>")]
// fn process_admin_login(form: Form<LoginCont<AdminLogin>>, mut cookies: Cookies) -> Result<Redirect, Flash<Redirect>> {
// fn process_admin_login(form: Form<LoginCont<AdministratorForm>>, mut cookies: Cookies) -> Result<Redirect, Flash<Redirect>> {
// fn process_admin_login(form: Form<AdministratorForm>, mut cookies: Cookies) -> Result<Redirect, Flash<Redirect>> {
pub fn process_admin_login(form: Form<LoginCont<AdministratorForm>>, mut cookies: Cookies) -> Result<Redirect, Flash<Redirect>> {
    let start = Instant::now();
    
    let login: AdministratorForm = form.get().form();
    // let login: AdministratorForm = form.into_inner().form;
    let output = login.flash_redirect("/admin_dashboard", "/admin_login", cookies);
    
    let end = start.elapsed();
    println!("Processed in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/admin_logout")]
pub fn logout_admin(admin: Option<AdministratorCookie>, mut cookies: Cookies) -> Result<Flash<Redirect>, Redirect> {
    if let Some(_) = admin {
        // cookies.remove_private(Cookie::named(AdministratorCookie::cookie_id()));
        AdministratorCookie::delete_cookie(cookies);
        Ok(Flash::success(Redirect::to("/"), "Successfully logged out."))
    } else {
        Err(Redirect::to("/admin_login"))
    }
}









#[get("/user_dashboard")]
pub fn dashboard_user_authorized(admin: UserCookie, conn: DbConn) -> Template {
    let start = Instant::now();
    
    let output: Template = hbs_template(TemplateBody::General(format!("Welcome User {user}.  You are viewing the User dashboard page.", user=admin.username), None), Some("User Dashboard".to_string()), String::from("/user_dashboard"), None, None, None, Some(start));
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/user_login", rank = 1)]
pub fn dashboard_user_login() -> Template {
    hbs_template(TemplateBody::Login(URL_LOGIN_USER.to_string(), None, None), Some("User Login".to_string()), String::from("/user_login"), None, None, None, None)
}

#[get("/user_dashboard", rank = 2)]
pub fn dashboard_user_unauthorized() -> Template {
    hbs_template(
        TemplateBody::General(
            "You are not logged in. <a href=\"/user_login\">User Login</a>".to_string(), None,
        ), 
        Some("User Login".to_string()),
        String::from("/user_dashboard_error"), 
        None, 
        None, 
        None, 
        None
    )
}

#[get("/user_login?<user>")]
// fn dashboard_retry_user(user: UserQuery, flash_msg_opt: Option<FlashMessage>) -> Template {
// fn dashboard_retry_user(mut user: String, flash_msg_opt: Option<FlashMessage>) -> Template {
pub fn dashboard_user_retry_user(mut user: QueryUser, flash_msg_opt: Option<FlashMessage>) -> Template {
    let start = Instant::now();
    // user = login::sanitization::sanitize(&user);
    let username = if &user.user != "" { Some(user.user.clone() ) } else { None };
    let flash = if let Some(f) = flash_msg_opt { Some(alert_danger(f.msg())) } else { None };
    let output = hbs_template(TemplateBody::Login(URL_LOGIN_USER.to_string(), username, flash), Some("User Login".to_string()), String::from("/user_login"), None, None, None, Some(start));
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/user_login", rank = 2)]
pub fn dashboard_user_retry_flash(flash_msg: FlashMessage) -> Template {
    let start = Instant::now();
    
    let flash = Some( alert_danger(flash_msg.msg()) );
    let output = hbs_template(TemplateBody::Login(URL_LOGIN_USER.to_string(), None, flash), Some("User Login".to_string()), String::from("/user_login"), None, None, None, None);
    
    let end = start.elapsed();
    println!("Served in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[allow(unused_mut)]
#[post("/user_login", data = "<form>")]
// fn process_admin_login(form: Form<LoginCont<AdminLogin>>, mut cookies: Cookies) -> Result<Redirect, Flash<Redirect>> {
// fn process_admin_login(form: Form<LoginCont<AdministratorForm>>, mut cookies: Cookies) -> Result<Redirect, Flash<Redirect>> {
// fn process_admin_login(form: Form<AdministratorForm>, mut cookies: Cookies) -> Result<Redirect, Flash<Redirect>> {
pub fn process_user_login(form: Form<LoginCont<UserForm>>, mut cookies: Cookies) -> Result<Redirect, Flash<Redirect>> {
    let start = Instant::now();
    
    let login: UserForm = form.get().form();
    // let login: AdministratorForm = form.into_inner().form;
    let output = login.flash_redirect("/user_dashboard", "/user_login", cookies);
    
    let end = start.elapsed();
    println!("Processed in {}.{:08} seconds", end.as_secs(), end.subsec_nanos());
    output
}

#[get("/user_logout")]
pub fn logout_user(admin: Option<UserCookie>, mut cookies: Cookies) -> Result<Flash<Redirect>, Redirect> {
    if let Some(_) = admin {
        // cookies.remove_private(Cookie::named(UserCookie::cookie_id()));
        UserCookie::delete_cookie(cookies);
        Ok(Flash::success(Redirect::to("/"), "Successfully logged out."))
    } else {
        Err(Redirect::to("/user_login"))
    }
}


