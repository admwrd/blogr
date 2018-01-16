/* the pages variable is populated with:
    Current page - pulled from query string/uri (call the parse_query method of the settings)
    Current route (pulled from request.uri())
    Settings (specified by Paginate<Settings>)
        Items per page
*/
#[get("/pagination/<num_items_opt>")]
fn pagination_test(start: GenTimer, num_items_opt: Option<u32>, pages: Page<Pagination>, conn: DbConn, admin: Option<AdministratorCookie>, user: Option<UserCookie>, encoding: AcceptCompression) -> Express {
    // let pages = PaginateDefaults::new()
    // let qrystr = ::gen_query("SELECT * FROM articles", Some("posted"));
    // let articles = conn.articles(qrystr);
    
    let contents: String;
    if let Some(num_items) = num_items_opt {
        contents = format!("There were {} items found.<br>\nYou are viewing page {} out of {} pages.<br>\nNavigation:<br>\n{}", num_items, pages.cur_page, pages.num_pages(num_items), pages.navigation(num_items));
    } else {
        let num_items = 10;
        contents = format!("There were {} items found.<br>\nYou are viewing page {} out of {} pages.<br>\nNavigation:<br>\n{}", num_items, pages.cur_page, pages.num_pages(num_items), pages.navigation(num_items));
        // contents = format!("You are viewing page {} out of {} pages.\n<br>Navigation:<br>{}", pages.cur_page, num_pages, pages.navigation(num_pages));
    }
    
    // enum TemplateBody :: PaginateArticles( Vec<Article>, T: Collate )
    // let output = hbs_template(TemplateBody::PaginateArticles(articles, ), None, String::from("/"), admin, user, None, Some(start.0));
    let output: Template = hbs_template(TemplateBody::General(contents), None, Some("Pagination Test".to_string()), String::from("/pagination"), admin, user, None, Some(start.0));
    
    let express: Express = output.into();
    express.compress( encoding )
    
    // if let Ok(qry) = conn.query(qrystr, &[]) {
    //     if !qry.is_empty() && qry.len() != 0 {
            
    //     }
    // }
}
