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
        tera.register_filter("showless", &showless);
        tera
    };
}

// custom filters
//
// extract host from  url
pub fn host(value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
    use crate::util::helper::get_host;
    let s = try_get_value!("host", "value", String, value);
    let host = get_host(&s);

    Ok(to_value(&host).unwrap())
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
        return Ok(to_value(&s).unwrap());
    }

    let r_s = s[..graphemes[least].0].to_string();

    let last_link = r_s.rfind("<a").unwrap_or(0);
    let last_end_link = r_s.rfind("</a>").unwrap_or(0);
    let result = if last_link > last_end_link {
        r_s[..graphemes[last_link].0].to_string()
    } else {
        r_s
    };

    Ok(to_value(&result).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::value::to_value;
    use std::collections::HashMap;

    #[test]
    fn test_host() {
        let result = host(
            &to_value("https://ruthub.com/rlist/r-when").unwrap(),
            &HashMap::new(),
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), to_value("ruthub.com").unwrap());
    }
}
