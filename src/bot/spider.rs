// a simple page crawle

use regex::Regex;
use reqwest;
use scraper::{Html, Selector};

use crate::api::item::NewItem;
use crate::api::{re_test_img_url, replace_sep, trim_url_qry};
use crate::util::helper::gen_slug;


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PageInfo {
    pub title: String,
    pub url: String,
    pub img: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct WebPage {
    url: String,
    html: String,
    domain: String,
}

impl WebPage {
    pub fn new(url: &str) -> Self {
        let res = reqwest::get(url).unwrap().text().unwrap();

        lazy_static! {
            static ref Scheme_re: Regex = Regex::new(r"https?://").unwrap();
            static ref Path_re: Regex = Regex::new(r"/.*").unwrap();
        }

        let uri = Scheme_re.replace_all(url, "");
        let host = Path_re.replace_all(&uri, "");
        let domain = host.replace("www.", "");

        Self {
            url: url.to_string(),
            html: res,
            domain,
        }
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
        match domain.trim() {
            _ => {
                let page = parse_other_page(url, html);
                let title = page.title;
                NewItem {
                    title: title.clone(),
                    slug: gen_slug(&title),
                    content: page.content,
                    logo: page.img,
                    ty: "Article".to_owned(),
                    lang: "English".to_owned(),
                    topic: "Rust".to_owned(),  // ?
                    link: page.url,
                    post_by: "bot".to_owned(),
                    ..NewItem::default()
                }
            }
        }
    }

}


pub fn parse_other_page(url: String, html: Html) -> PageInfo {
    let title_selector = Selector::parse("head > title").unwrap();
    let img_selector = Selector::parse("img").unwrap();
    let meta_selector = Selector::parse(r#"meta[name="description"]"#).unwrap();

    // get title
    let titles: Vec<_> = html.select(&title_selector).collect();
    let title_text: String = if let Some(t) = titles.first() {
        t.inner_html()
    } else {
        "untitled, please help to update".to_owned()
    };

    // get image url
    let imgs: Vec<_> = html.select(&img_selector).collect();
    let img_src: String = if let Some(img) = imgs.first() {
        match img.value().attr("src") {
            Some(src) => {
                if re_test_img_url(src) {
                    src.to_owned()
                } else {
                    "".to_owned()
                }
            }
            None => "".to_owned(),
        }
    } else {
        "".to_owned()
    };

    // get content -- meta description or og
    // for og:description
    let mut og_descript = |html: Html| -> String {
        let og_selector = 
            Selector::parse(r#"meta[property="og:description"]"#).unwrap();
        let descript: Vec<_> = html.select(&og_selector).collect();
        let content: String = if let Some(p) = descript.first() {
            match p.value().attr("content") {
                Some(c) => { c.to_owned() }
                None => { title_text.clone() }
            }
        } else {
            title_text.clone()
        };

        content
    };
    // meta description
    let descript: Vec<_> = html.select(&meta_selector).collect();
    let content: String = if let Some(p) = descript.first() {
        match p.value().attr("content") {
            Some(c) => { c.to_owned() }
            None => { og_descript(html) }
        }
    } else {
        og_descript(html)
    };

    PageInfo {
        title: title_text,
        url: url,
        img: img_src,
        content: content,
    }
}

pub fn get_links(page: &WebPage) -> Vec<String> {
    let domain = &page.domain;
    let raw_links = page.extract_links();
    let mut links: Vec<String> = Vec::new();
    match domain.trim() {
        "npr.org" => {
            for link in raw_links {
                if link.starts_with("https://www.npr.org/2") {
                    links.push(link.to_owned())
                }
            }
        }
        
        _ => {}  // to deal with
    }
    
    links.sort();
    links.dedup();
    links
}


// TODO: more specific parse
