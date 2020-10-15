// view redered by server side

pub mod tmpl;
pub mod form;

use regex::Regex;
use std::collections::HashMap;
use unic_segment::GraphemeIndices;
use chrono::NaiveDate;
pub use askama::Template;

use crate::api::item::{Item};
use crate::api::blog::{Blog};
use crate::api::auth::CheckUser;


lazy_static! {
    pub static ref TOPIC_VEC: Vec<&'static str> = {
        vec!(
            "all", 
            "Rust", "Go", "Swift", "TypeScript", "Dart", 
            "Python", "C-sharp", "C", "CPP", "JavaScript", "Java", "PHP", "Kotlin", "DataBase"
        )
    };

    pub static ref TY_VEC: Vec<&'static str> = {
        vec!(
            "Article", "Book", "Event", "Job", "Media", "Project", "Translate", "Misc"
        )
    };
}


#[derive(Template)]
#[template(path = "collection.html")]
pub struct CollectionTmpl<'a> {
    pub ty: &'a str,
    pub topic: &'a str,
    pub items: &'a Vec<Item>,
    pub blogs: &'a Vec<Blog>,
    pub tys: &'a Vec<&'a str>,
}

#[derive(Template)]
#[template(path = "item.html")]
pub struct ItemTmpl<'a> {
    pub item: &'a Item,
}

#[derive(Template)]
#[template(path = "more_item.html")]
pub struct ItemsTmpl<'a> {
    pub items: &'a Vec<Item>,
    pub topic: &'a str,
}

#[derive(Template)]
#[template(path = "profile.html")]
pub struct ProfileTmpl<'a> {
    pub user: &'a CheckUser,
    pub is_self: bool,
}

#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTmpl();

#[derive(Template)]
#[template(path = "sitemap/sitemap.xml")]
pub struct SiteMapTmpl<'a> {
    pub tys: &'a Vec<&'a str>,
    pub topics: &'a Vec<&'a str>,
    pub lastmod: &'a str,
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

    pub fn dt_fmt(
        value: &NaiveDateTime, 
        fmt: &str, 
    ) -> TmplResult<String> {
        
        let format: &str = if fmt.len() > 0 {
            fmt
        } else {
            "%Y-%m-%d %R"
        };
        // https://docs.rs/chrono/0.4.15/chrono/format/strftime/index.html
        let formatted = value.format(format).to_string();
            
        Ok(formatted)
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
