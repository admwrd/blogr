extern crate handlebars;

use super::serde::Serialize;
use super::{Engine, TemplateInfo};

// pub use self::handlebars::{Helper, RenderError, Handlebars, RenderContext};
// pub use self::handlebars::{Handlebars, RenderError, RenderContext, Helper, Context, Renderable, HelperResult};
pub use self::handlebars::{Handlebars, RenderError, RenderContext, Helper, Context, Renderable};
// use hbs::{Handlebars, RenderError, RenderContext, Helper, Context, Renderable};

// http://sunng87.github.io/handlebars-rust/handlebars/index.html#custom-helper

/* https://medium.com/@ericdreichert/rusts-iron-framework-handlebars-helpers-3f5c4775a9ba
    Used with:
<tbody>
    {{#each task_records}}
        {{#if-multiple-of 2 @index}}
            <tr>
        {{else}}
            <tr class="row_highlight">
        {{/if-multiple-of}}
            <td>{{this.date}}</td>
            <td>{{this.start_time}}</td>
            <td>{{this.end_time}}</td>
            <td>{{this.project_name}}</td>
            <td id="desc_data">{{this.description}}</td>
        </tr>
    {{/each}}
</tbody>
*/

// const FACTOR_OF_INTEREST_IDX: usize = 0;
// const CANDIDATE_IDX: usize = 1;
pub fn current_page(_: &Context, helper: &Helper, hbars: &Handlebars, render_ctx: &mut RenderContext) -> Result<(), RenderError> {
    let page_opt = helper.param(0);
    let menu_opt = helper.param(1);
    if page_opt.is_none() || menu_opt.is_none() {
        if page_opt.is_none() && menu_opt.is_none() {
            return Err(RenderError::new("A page and a menu item must be specified."));
        } else if page_opt.is_none() {
            return Err(RenderError::new("First parameter (page) must be set."));
        } else if menu_opt.is_none() {
            return Err(RenderError::new("Second parameter (menu) must be set."));
        } else {
            return Err(RenderError::new("Wtf."));
        }
    }
    let page = page_opt.unwrap().value();
    let menu = menu_opt.unwrap().value();
    let output_template = if page == menu {
        helper.template()
    } else {
        helper.inverse()
    };
    match output_template {
        Some(t) => t.render(hbars, render_ctx),
        None => Ok(()),
    }
    
    // let factor_of_interest = try!(
    //     helper.param(FACTOR_OF_INTEREST_IDX)
    //         .map(|json| json.value())
    //         .and_then(|val| val.as_u64())
    //         .and_then(|u64_val| if u64_val > 0 { Some(u64_val) } else { None } )
    //         .ok_or_else(|| RenderError::new("Factor of interest must be a number greater than 0."))
    // );

    // let candidate = try!(
    //     helper.param(CANDIDATE_IDX)
    //         .map(|json| json.value())
    //         .and_then(|val| val.as_u64())
    //         .ok_or_else(|| RenderError::new("Candidate must be a number greater than or equal to 0."))
    // );

    // let possible_template = if candidate % factor_of_interest == 0 {
    //     helper.template()
    // } else {
    //     helper.inverse()
    // };

    // match possible_template {
    //     Some(t) => t.render(ctx, hbars, render_ctx),
    //     None => Ok(()),
    // }
}

// fn menu_helper (h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
//     // just for example, add error check for unwrap
//     // let param = h.param(0).unwrap().value();
//     // let rendered = format!("0x{:x}", param.as_u64().unwrap());
//     // try!(rc.writer.write(rendered.into_bytes().as_ref()));
//     // Ok(())
    
//     let menu_opt = h.param(0);
//     let page_opt = h.param(1);
//     if menu_opt.is_none() || page_opt.is_none() {
//         return Ok(());
//     }
//     let menu = menu_opt.unwrap().value();
//     let page = page_opt.unwrap().value();
//     if menu == page {
//         // try!(rc.writer.write(rendered.into_bytes().as_ref()));
//         try!(rc.writer.write(&[1u8]));
//     } else {
//         // try!(rc.writer.write(rendered.into_bytes().as_ref()));
//         try!(rc.writer.write(&[0u8]));
//     }
//     Ok(())
// }


impl Engine for Handlebars {
    const EXT: &'static str = "hbs";

    fn init(templates: &[(&str, &TemplateInfo)]) -> Option<Handlebars> {
        let mut hb = Handlebars::new();
        
        // if let None = hb.register_helper("menu_item", Box::new(menu_helper)) {
        
        // hb.register_helper("is-current-page", Box::new(self::current_page));
        
        // if let None = hb.register_helper("is-current-page", Box::new(current_page)) {
        //     error!("Error registering helper `current_page`");
        //     return None;
        // }
        // hb.register_helper("size-tag",
        // Box::new(|helper: &Helper, hbars: &Handlebars, render_ctx: &mut RenderContext| -> Result<(), RenderError> {
            
        // } );
            
        hb.register_helper("is-current-page",
        Box::new(|helper: &Helper, hbars: &Handlebars, render_ctx: &mut RenderContext| -> Result<(), RenderError> {
            let page_opt = helper.param(0);
            let menu_opt = helper.param(1);
            if page_opt.is_none() || menu_opt.is_none() {
                if page_opt.is_none() && menu_opt.is_none() {
                    return Err(RenderError::new("A page and a menu item must be specified."));
                } else if page_opt.is_none() {
                    return Err(RenderError::new("First parameter (page) must be set."));
                } else if menu_opt.is_none() {
                    return Err(RenderError::new("Second parameter (menu) must be set."));
                } else {
                    return Err(RenderError::new("Wtf."));
                }
            }
            let page = page_opt.unwrap().value();
            let menu = menu_opt.unwrap().value();
            let output_template = if page == menu {
                helper.template()
            } else {
                helper.inverse()
            };
            match output_template {
                Some(t) => t.render(hbars, render_ctx),
                None => Ok(()),
            }
        }));
        
        for &(name, info) in templates {
            let path = &info.path;
            if let Err(e) = hb.register_template_file(name, path) {
                error!("Error in Handlebars template '{}'.", name);
                info_!("{}", e);
                info_!("Template path: '{}'.", path.to_string_lossy());
                return None;
            }
        }

        Some(hb)
    }

    fn render<C: Serialize>(&self, name: &str, context: C) -> Option<String> {
        if self.get_template(name).is_none() {
            error_!("Handlebars template '{}' does not exist.", name);
            return None;
        }

        match Handlebars::render(self, name, &context) {
            Ok(string) => Some(string),
            Err(e) => {
                error_!("Error rendering Handlebars template '{}': {}", name, e);
                None
            }
        }
    }
}
