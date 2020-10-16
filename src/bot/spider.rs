// a simple page crawle

use regex::Regex;
use chrono::{NaiveDate, Utc};
use scraper::{Html, Selector};

use crate::errors::{ServiceError, ServiceResult};
use crate::api::item::NewItem;
use crate::api::{re_test_img_url, replace_sep, trim_url_qry};
use crate::util::helper::gen_slug;
use crate::bot::cfg::{get_links, MAP_HOST};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PageInfo {
    pub title: String,
    pub url: String,
    pub img: String,
    pub content: String,
}

#[derive(Debug, Clone, Default)]
pub struct WebPage {
    pub url: String,
    pub html: String,
    pub domain: String,
}

impl WebPage {
    pub fn new(url: &str) -> ServiceResult<Self> {
        // let res = reqwest::get(url)?.text()?;

        let default_html = String::from("");
        let res = match reqwest::blocking::get(url) {
            Ok(resp) => {
                let mut resp = resp;
                match resp.text() {
                    Ok(s) => s,
                    _ => default_html
                }
            },
            _ => default_html
        };

        lazy_static! {
            static ref Scheme_re: Regex = Regex::new(r"https?://").unwrap();
            static ref Path_re: Regex = Regex::new(r"/.*").unwrap();
        }

        let uri = Scheme_re.replace_all(url, "");
        let host = Path_re.replace_all(&uri, "");
        let domain = host.replace("www.", "");

        let page = Self {
            url: url.to_string(),
            html: res,
            domain,
        };
        Ok(page)
    }

    // URL getter
    pub fn get_url(&self) -> String {
        self.url.clone()
    }

    // Domain getter
    pub fn get_domain(&self) -> String {
        self.domain.clone()
    }

    // HTML parser
    pub fn get_html(&self) -> Html {
        Html::parse_document(&self.html)
    }

    // extract links in html page
    pub fn extract_links(&self) -> Vec<String> {
        let link_selector = Selector::parse("a").unwrap();
        let html = self.get_html();
        let link_refs: Vec<_> = html.select(&link_selector).collect();
        let mut links: Vec<String> = Vec::new();
        for link in link_refs {
            match link.value().attr("href") {
                Some(href) => {
                    links.push(href.to_string())
                }
                None => {}
            }
        }

        links
    }

    pub fn clean_links(&self) -> Vec<String> {
        get_links(self)
    }

    pub fn into_item(&self) -> NewItem {
        let url = self.get_url();
        let html = self.get_html();
        let domain = self.get_domain();
        let dmn = domain.trim();
        match dmn {
            _ => {
                let page = parse_common_page(html, &url);
                let title = page.title.trim();
                NewItem {
                    title: replace_space(title, " "),
                    content: page.content.trim().to_owned(),
                    logo: page.img.trim().to_owned(),
                    author: get_author_topic(dmn).0,
                    ty: "Article".to_owned(),
                    topic: get_author_topic(dmn).1,
                    link: page.url.trim().to_owned(),
                    post_by: "bot".to_owned(),
                    pub_at: Utc::today().naive_utc()
                }
            }
        }
    }
}

pub fn page_ele_paser(
    html: &Html,
    sel_str: &str,
    attr_str: &str,
    alt_txt: &str,
) -> Vec<String> {
    let a_selector = Selector::parse(sel_str).unwrap();
    let a_vec: Vec<_> = html.select(&a_selector).collect();

    if a_vec.len() == 0 {
        return Vec::new();
    }

    let mut a_txt_vec = Vec::new();
    for a in a_vec {
        let a_txt =  if attr_str.trim().len() == 0 {
            a.inner_html()
        } else {
            match a.value().attr(attr_str) {
                Some(s) => s.to_owned(),
                None => String::from(alt_txt),
            }
        };
        a_txt_vec.push(a_txt);
    }
    
    // test
    // println!(">>{:?} -> {:?}", sel_str, a_txt_vec);

    a_txt_vec
}

pub fn parse_common_page(html: Html, url: &str) -> PageInfo {
    
    // get title
    let title_text: String = 
        page_ele_paser(
            &html, "head > title", "", url
        )
        .first()
        .unwrap_or(&String::from(url))
        .to_string();

    // get image url
    //
    // og:image og:image:url
    let og_img: String = page_ele_paser(
            &html, r#"meta[property="og:image"]"#, "content", ""
        )
        .first()
        .unwrap_or(&String::from(""))
        .to_string();
    
    let img_src: String = og_img;
    // if og_img.len() == 0 {
    //     // random body img
    //     page_ele_paser(
    //         &html, "body img", "src", ""
    //     )
    //     .first()
    //     .unwrap_or(&String::from(""))
    //     .to_string()
    // } else {
    //     og_img
    // };

    // get content descript -- meta description or og:description
    //
    // meta description
    let meta_descript: String = page_ele_paser(
        &html, r#"meta[name="description"]"#, "content", ""
    )
    .first()
    .unwrap_or(&String::from(""))
    .to_string();

    let content: String = if meta_descript.len() == 0 {
        // og:description
        page_ele_paser(
            &html, r#"meta[property="og:description"]"#, "content", ""
        )
        .first()
        .unwrap_or(&String::from(""))
        .to_string()
    } else {
        meta_descript
    };

    // get canonical link
    let c_link: String = page_ele_paser(
        &html, r#"link[rel="canonical"]"#, "href", url
    )
    .first()
    .unwrap_or(&String::from(url))
    .to_string();
    
    PageInfo {
        title: title_text,
        url: c_link,
        img: img_src,
        content,
    }
}

// some helpers

fn get_author_topic(host: &str) -> (String, String) {
    let map = &MAP_HOST;
    let default = &(host, "Rust");
    let tup = map.get(host).unwrap_or(default);

    (tup.0.to_owned(), tup.1.to_owned())
}

// replace whitespance, \n \t \r \f...
pub fn replace_space(text: &str, rep: &str) -> String {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"\s").unwrap(); // let fail in test
    }
    RE.replace_all(text, rep).into_owned()
}
