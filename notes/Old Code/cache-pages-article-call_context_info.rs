macro_rules! ctx_info {
            // ( $title:expr; and $page:expr ) => {
            ( $title:expr, $page:expr ) => {
                // let t_opt = if $title == "" { None } else { $title.to_owned() };
                info::info(if $title == "" { None } else { Some($title.to_owned()) }, $page.to_owned(), admin, user, gen, uhits, encoding, javascript, msg)
                
            }
        }
        
        /* let info = |title: &str, page: &str| {
            // let t_opt: Option<String> = if title == "" { None } else { let temp: String = title.into(); Some(temp) };
            let t_opt: Option<String> = if title == "" { None } else { Some(title.to_owned()) };
            // let p_opt: Option<String> = if page == "" { None } else { let temp: String = page.into(); Some(temp) };
            // let p_opt: Option<String> = if page == "" { None } else { Some(page.to_owned()) };
            let p: String = page.to_owned();
            // let t: String = title.into();
            // let p: String = page.into();
            info::info(t_opt, p, admin, user, gen, uhits, encoding, javascript, msg)
        }; */