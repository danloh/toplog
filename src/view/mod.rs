// view redered by server side

pub mod tmpl;

use regex::Regex;
use serde_json::value::{to_value, Value};
use std::collections::HashMap;
use tera::{Result as TeraResult, Tera};
use unic_segment::GraphemeIndices;

lazy_static! {
    pub static ref TEMPLATE: Tera = {
        let mut tera = Tera::new("templates/**/*").unwrap_or_default();
        //tera.autoescape_on(vec!["html", ".sql"]);
        tera.register_filter("host", &host);
        tera.register_filter("md", &md);
        tera.register_filter("showless", &showless);
        tera.register_filter("enbase", &enbase);
        tera.register_filter("setbase", &setbase);
        tera
    };

    pub static ref TOPIC_VEC: Vec<&'static str> = {
        vec!(
            "all", 
            "Rust", "Go", "Swift", "TypeScript", "Angular", "Vue", "React", "Dart", "Flutter",
            "Python", "C-sharp", "C", "CPP", "JavaScript", "Java", "PHP", "Kotlin", "DataBase"
        )
    };

    pub static ref TY_VEC: Vec<&'static str> = {
        vec!(
            "Misc", "Article", "Book", "Event", "Job", "Media", "Product", "Translate"
        )
    };
}

// custom filters
//
// extract host from  url
pub fn host(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    use crate::util::helper::get_host;
    let s = try_get_value!("host", "value", String, value);
    let host = get_host(&s);

    Ok(to_value(&host).unwrap_or_default())
}

// markdown
pub fn md(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    use pulldown_cmark::{Parser, Options, html};
    let mut options = Options::all();
    // options.insert(Options::ENABLE_TABLES);
    // options.insert(Options::ENABLE_FOOTNOTES);
    // options.insert(Options::ENABLE_TASKLISTS);

    let s = try_get_value!("md", "value", String, value);
    let parser = Parser::new_ext(&s, options);

    let mut html_res = String::new();
    html::push_html(&mut html_res, parser);

    Ok(to_value(&html_res).unwrap_or_default())
}

// base64 encode
pub fn enbase(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    use crate::util::helper::en_base64;
    let s = try_get_value!("enbase", "value", String, value);
    let b64 = en_base64(&s);

    Ok(to_value(&b64).unwrap_or_default())
}

// set base url for href, NOTE: not decouple!
pub fn setbase(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    use crate::util::helper::en_base64;
    let s = try_get_value!("setbase", "value", String, value);
    // let check: String = match args.get("check") {
    //     Some(c) => try_get_value!("setbase", "check", String, c),
    //     None => "all".to_owned(),
    // };
    let check_s = s.trim();
    let base = if  check_s == "all" || check_s == "from" { 
        "/a/".to_owned() 
    } else { 
        "/t/".to_owned() + check_s + "/"
    };

    Ok(to_value(&base).unwrap_or_default())
}

// override tera filter trucate to avoid char boundary and cut html tag
pub fn showless(value: &Value, args: &HashMap<String, Value>) -> TeraResult<Value> {
    let s = try_get_value!("showless", "value", String, value);
    let graphemes = GraphemeIndices::new(&s).collect::<Vec<(usize, &str)>>();
    let length = graphemes.len();
    let least = match args.get("least") {
        Some(l) => try_get_value!("showless", "least", usize, l),
        None => 255,
    };

    if least >= length {
        return Ok(to_value(&s).unwrap_or_default());
    }

    let r_s = s[..graphemes[least].0].to_string();

    let last_link = r_s.rfind("<a").unwrap_or(0);
    let last_end_link = r_s.rfind("</a>").unwrap_or(0);
    let result = if last_link > last_end_link {
        r_s[..graphemes[last_link].0].to_string()
    } else {
        r_s
    };

    Ok(to_value(&result).unwrap_or_default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::value::to_value;
    use std::collections::HashMap;

    #[test]
    fn test_host() {
        let result = host(
            &to_value("https://toplog.cc/rlist/r-when").unwrap(),
            &HashMap::new(),
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), to_value("toplog.cc").unwrap());
    }
}
