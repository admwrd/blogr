















main.rs

fn main() {

    /* ... */

    let (articles_reader, mut articles_writer) = evmap::new();
    match routes::load_article_cache(&all_articles, &mut articles_writer, &conn) {
        Ok( num ) => {
            if !PRODUCTION {
                println!("Article cache loaded with {} articles.", num);
            }
        },
        Err( err ) => { panic!("{}", err); },
    }
    
    let article_reader_cache = ArticleCacheReader{ cache: Arc::new(articles_reader) };
    
    
    let (pages_reader, mut pages_writer) = evmap::new();
    match routes::load_pages(&mut pages_writer, &conn) {
        Ok( num ) => {
            if !PRODUCTION {
                println!("Pages cache loaded with {} pages.", num);
            }
        },
        Err( err ) => { panic!("{}", err); },
    }
}    

    /*
    
    all_tags
    /tag/<tag>
        /tag?<tag>
    
    /article?<aid>
        /article/<aid>
        /article/<aid>/<title>
    /article (hbs_article_not_found)
    /rss.xml
    /author/<authorid>
        /author/<authorid>/<authorname>
    /about
    
    
    /pageviews
    /pagestats
    /pagestats/<show_errors>
    /manage/<sortstr>/<orderstr>
    /manage
    
    
    
    */



routes/mod.rs

// pub fn load_articles(&mut evmap::WriteHandle<String, Article>) -> Result<usize, String> {

// pub fn load_article_cache(articles: &Vec<Article>, writer: &mut WriteHandle<String, &Article>, conn: &DbConn) -> Result<usize, String> {
// pub fn load_article_cache(articles_arc: &Arc<Vec<Article>>, writer: &mut WriteHandle<u32, &Article>, conn: &DbConn) -> Result<usize, String> {
// pub fn load_article_cache(articles: &Vec<Article>, writer: &mut WriteHandle<u32, &Article>, conn: &DbConn) -> Result<usize, String> {
pub fn load_article_cache<'v, 'd, 'w>(articles: &'v Vec<Article>, writer: &'w mut WriteHandle<u32, &'v Article>, conn: &'d DbConn) -> Result<usize, String> {
    // unimplemented!()
    // let articles = articles_arc.clone();
    if articles.len() == 0 {
        Ok(0usize)
    } else {
        let mut count = 0usize;
        // for article in articles.iter() {
        for article in articles {
            writer.insert(article.aid, &article);
            count += 1;
        }
        writer.refresh();
        if count == articles.len() {
            Ok(count)
        } else {
            Err(format!("Error loading article cache: inconsistent vector sizes, {} inserted vs original", count/*, articles.len()*/))
        }
    }
}

pub fn load_pages(writer: &mut evmap::WriteHandle<String, String>, conn: &DbConn) -> Result<usize, String> {
    unimplemented!()
}










