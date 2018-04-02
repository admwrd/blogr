
/* 
pub trait Cachable {
    type Index;
    type StateType;
    // type Output;
    fn new(Self::Index) -> Self;
    // fn retrieve(self, State<Self::StateType>, Option<&DbConn>) -> Option<Self::Output>;
    fn retrieve(self, State<Self::StateType>, Option<&DbConn>) -> Express;
    
}

pub struct SingleArticle(u32);

pub struct MultiArticles(Vec<u32>);

pub struct GenericInfo{
    
}

impl Cachable for SingleArticle {
    type Index = u32;
    type StateType = ArticleCacheLock;
    // type Output = Express;
    
    fn new(aid: u32) -> SingleArticle {
        SingleArticle(aid)
    }
    // pub fn retrieve(self, conn_opt: Option<&DbConn>) -> Option<Article> {
    fn retrieve(self, articles_state: State<Self::StateType>, conn_opt: Option<&DbConn>) -> Express {
        let aid = self.0;
        
        
        // let output: Template;
        // // let output = 
        // if let Ok(a) = articles_state.lock.read() {
        //     if let Some(article) = a.articles.get(&aid) {
        //         let title = article.title.clone();
        //         // println!("Article {}\n{:?}", article.aid, &article);
        //         // output = hbs_template(TemplateBody::Article(article.clone()), None, Some(title), String::from("/article"), admin, user, Some("enable_toc(true);".to_owned()), Some(start.0));
        //         hbs_template(TemplateBody::Article(article.clone()), None, Some(title), String::from("/article"), admin, user, Some("enable_toc(true);".to_owned()), Some(start.0))
        //     } else {
        //         // output = hbs_template(TemplateBody::General(alert_danger(&format!("Article {} not found.", aid))), None, Some("Article Not Found".to_string()), String::from("/article"), admin, user, None, Some(start.0));
        //         hbs_template(TemplateBody::General(alert_danger(&format!("Article {} not found.", aid))), None, Some("Article Not Found".to_string()), String::from("/article"), admin, user, None, Some(start.0))
        //     }
        // } else {
        //     // output =  hbs_template(TemplateBody::General(alert_danger(&format!("Failed to acquire cache lock for article {}.", aid))), None, Some("Internal Error".to_string()), String::from("/article"), admin, user, None, Some(start.0));
        //     hbs_template(TemplateBody::General(alert_danger(&format!("Failed to acquire cache lock for article {}.", aid))), None, Some("Internal Error".to_string()), String::from("/article"), admin, user, None, Some(start.0))
        // }
        
        
        // let output: Template;
        // // let output = 
        // if let Ok(a) = articles_state.lock.read() {
        //     if let Some(article) = a.articles.get(&aid) {
        //         let title = article.title.clone();
        //         // println!("Article {}\n{:?}", article.aid, &article);
        //         // output = hbs_template(TemplateBody::Article(article.clone()), None, Some(title), String::from("/article"), admin, user, Some("enable_toc(true);".to_owned()), Some(start.0));
        //         hbs_template(TemplateBody::Article(article.clone()), None, Some(title), String::from("/article"), admin, user, Some("enable_toc(true);".to_owned()), Some(start.0))
        //     } else {
        //         // output = hbs_template(TemplateBody::General(alert_danger(&format!("Article {} not found.", aid))), None, Some("Article Not Found".to_string()), String::from("/article"), admin, user, None, Some(start.0));
        //         hbs_template(TemplateBody::General(alert_danger(&format!("Article {} not found.", aid))), None, Some("Article Not Found".to_string()), String::from("/article"), admin, user, None, Some(start.0))
        //     }
        // } else {
        //     // output =  hbs_template(TemplateBody::General(alert_danger(&format!("Failed to acquire cache lock for article {}.", aid))), None, Some("Internal Error".to_string()), String::from("/article"), admin, user, None, Some(start.0));
        //     hbs_template(TemplateBody::General(alert_danger(&format!("Failed to acquire cache lock for article {}.", aid))), None, Some("Internal Error".to_string()), String::from("/article"), admin, user, None, Some(start.0))
        // }
        // // };
        // output
        
        let express: Express = String::new().into();
        express
    }
}

 */