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

//use std::collections::HashMap;
use chrono::NaiveDate;
//use regex::Regex;
pub use askama::Template;

use crate::api::item::{Item};
use crate::api::blog::{Blog};

#[derive(Template)]
#[template(path = "home.html")]
pub struct IndexTmpl<'a> {
    pub ty: &'a str,
    pub topic: &'a str,
    pub items: &'a Vec<Item>,
    pub blogs: &'a Vec<Blog>,
    pub tys: &'a Vec<&'a str>,
}

#[derive(Template)]
#[template(path = "item.html")]
pub struct ItemTmpl<'a> {
    pub ty: &'a str,
    pub topic: &'a str,
    pub item: &'a Item,
    pub tys: &'a Vec<&'a str>,
}

#[derive(Template)]
#[template(path = "more_item.html")]
pub struct ItemsTmpl<'a> {
    pub items: &'a Vec<Item>,
    pub topic: &'a str,
}


// ==================================================
// custom filters ===================================
//
mod filters {
    use askama::Result as TmplResult;
    use askama::Error as TmplError;
    use chrono::{
        format::{Item, StrftimeItems},
        DateTime, FixedOffset, NaiveDate, NaiveDateTime, Utc,
    };
    use chrono_tz::Tz;
    use unic_segment::GraphemeIndices;

    pub fn host(s: &str) -> TmplResult<String> {
        use crate::util::helper::get_host;
        let s_host = get_host(s);
        Ok(s_host)
    }

    pub fn num_unit(num: &i32) -> TmplResult<String> {
        let x = *num;
        let (n, u): (i32, &str) = if x >= 1000 {
            (num / 1000, "k")
        } else if x > 9000 {
            (9, "k+")
        } else {
            (x, "")
        };
        let res = n.to_string() + u;

        Ok(res)
    }

    pub fn md(s: &str) -> TmplResult<String> {
        use pulldown_cmark::{Parser, Options, html::push_html};
        use ammonia::clean;  // for HTML Sanitization
        let mut options = Options::all();
        // options.insert(Options::ENABLE_TABLES);
        // options.insert(Options::ENABLE_FOOTNOTES);
        // options.insert(Options::ENABLE_TASKLISTS);
    
        let parser = Parser::new_ext(s, options);
        let mut html_res = String::new();
        push_html(&mut html_res, parser);
        let clean_html = clean(&*html_res);
    
        Ok(clean_html)
    }

    pub fn pluralize(num: &i32, m: &str, s: &str) -> TmplResult<String> {
    
        let res = if (num.abs() - 1).abs() > 0 {
            num.to_string() + " " + m + s
        } else {
            num.to_string() + " " + m
        };

        Ok(res)
    }

    pub fn showless(s: &str, len: &usize) -> TmplResult<String> {

        let graphemes = GraphemeIndices::new(s).collect::<Vec<(usize, &str)>>();
        let length = graphemes.len();
        let least = *len;
    
        if least >= length {
            return Ok(s.to_string());
        }
    
        let r_s = s[..graphemes[least].0].to_string();
    
        let last_link = r_s.rfind("<a").unwrap_or(0);
        let last_end_link = r_s.rfind("</a>").unwrap_or(0);
        let result = if last_link > last_end_link {
            r_s[..graphemes[last_link].0].to_string()
        } else {
            r_s
        };
    
        Ok(result)
    }

    // base64 encode
    pub fn b64_encode(value: &str) -> TmplResult<String> {
        use crate::util::helper::en_base64;
        let b64 = en_base64(value);

        Ok(b64)
    }

    // set baseurl per topic
    pub fn set_baseurl(s: &str) -> TmplResult<String> {
        use crate::util::helper::en_base64;
        
        let check_s = s.trim();
        let baseurl = if  check_s == "all" || check_s == "from" { 
            "/a/".to_owned() 
        } else { 
            "/t/".to_owned() + check_s + "/"
        };
    
        Ok(baseurl)
    }

    pub fn date_fmt(
        value: &NaiveDate, 
        fmt: &str, 
    ) -> TmplResult<String> {
        
        let format: &str = if fmt.len() > 0 {
            fmt
        } else {
            "%Y-%m-%d"
        };
    
        let item = StrftimeItems::new(format);
        let formatted = format!("{}", value.format_with_items(item));
            
        Ok(formatted)
    }

    pub fn datetime_fmt(
        value: &dyn std::fmt::Display, 
        fmt: &str, 
        tz: &str,
    ) -> TmplResult<String> {
        let value = value.to_string();

        let format: &str = if fmt.len() > 0 {
            fmt
        } else {
            "%Y-%m-%d"
        };
    
        let items: Vec<Item> = StrftimeItems::new(format)
            .filter(|item| match item {
                Item::Error => true,
                _ => false,
            })
            .collect();
        if !items.is_empty() { return Ok(value) }
    
        let timezone =  if tz.len() > 0 {
            match tz.parse::<Tz>() {
                Ok(timezone) => Some(timezone),
                Err(_) => { return Ok(value) }
            }
        } else {
            None
        };

        let formatted = 
            if value.contains('T') {
                match value.parse::<DateTime<FixedOffset>>() {
                    Ok(val) => match timezone {
                        Some(timezone) => 
                            val.with_timezone(&timezone).format(format),
                        None => val.format(format),
                    },
                    Err(_) => match value.parse::<NaiveDateTime>() {
                        Ok(val) => val.format(format),
                        Err(_) => { return Ok(value) }
                    },
                }
            } else {
                match NaiveDate::parse_from_str(&value, "%Y-%m-%d") {
                    Ok(val) => DateTime::<Utc>::from_utc(
                            val.and_hms(0, 0, 0), Utc
                        )
                        .format(format),
                    Err(_) => { return Ok(value) }
                }
            };
    
        Ok(formatted.to_string())
    }
}
