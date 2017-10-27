extern crate handlebars;

use super::serde::Serialize;
use super::{Engine, TemplateInfo};

pub use self::handlebars::Handlebars;

fn menu_helper (h: &Helper, _: &Handlebars, rc: &mut RenderContext) -> Result<(), RenderError> {
    // just for example, add error check for unwrap
    // let param = h.param(0).unwrap().value();
    // let rendered = format!("0x{:x}", param.as_u64().unwrap());
    // try!(rc.writer.write(rendered.into_bytes().as_ref()));
    // Ok(())
    
    let menu_opt = h.param(0);
    let page_opt = h.param(1);
    if menu_opt.is_none() || page_opt.is_none() {
        return Ok(());
    }
    let menu = menu_opt.unwrap().value();
    let page = page_opt.unwrap().value();
    if menu == page {
        // try!(rc.writer.write(rendered.into_bytes().as_ref()));
        try!(rc.writer.write(1u8));
    } else {
        // try!(rc.writer.write(rendered.into_bytes().as_ref()));
        try!(rc.writer.write(0u8));
    }
    Ok(())
}


impl Engine for Handlebars {
    const EXT: &'static str = "hbs";

    fn init(templates: &[(&str, &TemplateInfo)]) -> Option<Handlebars> {
        let mut hb = Handlebars::new();
        
        if let Err(e) = handlebars.register_helper("menu_item", Box::new(menu_helper)) {
            error!("Error in helper `menu_item`: '{}'", e);
            return None;
        }
        
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
