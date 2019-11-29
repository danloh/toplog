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
        let dmn = domain.trim();
        match dmn {
            _ => {
                let page = parse_common_page(url, html);
                let title = page.title.trim();
                NewItem {
                    title: replace_space(title, " "),
                    slug: gen_slug(title),
                    content: page.content.trim().to_owned(),
                    logo: page.img.trim().to_owned(),
                    author: get_author_topic(dmn).0,
                    ty: "Article".to_owned(),
                    lang: "English".to_owned(),
                    topic: get_author_topic(dmn).1,
                    link: page.url.trim().to_owned(),
                    origin_link: "".to_owned(),
                    post_by: "bot".to_owned(),
                }
            }
        }
    }

}


pub fn parse_common_page(url: String, html: Html) -> PageInfo {
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
        // Rust: Nicholas Matsakis 
        "smallcultfollowing.com" => {
            for link in raw_links {
                if link.starts_with("/babysteps/blog/2") {
                    let f_link = "http://smallcultfollowing.com".to_string() + &link;
                    links.push(f_link)
                }
            }
        }
        // Rust: async-std
        "async.rs" => {
            for link in raw_links {
                if link.starts_with("/blog/") && !(link.contains("/tags/")) && !(link.contains("/page/")) {
                    let f_link = "https://async.rs".to_string() + &link;
                    links.push(f_link)
                }
            }
        }
        // Rust: tokio
        "tokio.rs" => {
            for link in raw_links {
                if link.starts_with("https://tokio.rs/blog/2") && !(link.contains("/#")) {
                    links.push(link)
                }
            }
        }
        // Rust: Guillaume Gomez 
        "blog.guillaume-gomez.fr" => {
            for link in raw_links {
                if link.starts_with("https://blog.guillaume-gomez.fr/articles/2") {
                    links.push(link)
                }
            }
        }
        // Rust: Ralf Jung
        "ralfj.de" => {
            for link in raw_links {
                if link.starts_with("/blog/2") {
                    let f_link = "https://www.ralfj.de".to_string() + &link;
                    links.push(f_link)
                }
            }
        }
        // Rust: Steve Klabnik
        "words.steveklabnik.com" => {
            for link in raw_links {
                if link.starts_with("https://words.steveklabnik.com/") {
                    links.push(link)
                }
            }
        }
        // Rust: Jorge Aparicio
        "blog.japaric.io" => {
            for link in raw_links {
                if link.starts_with("https://blog.japaric.io/") && link.len() > 25 {
                    links.push(link)
                }
            }
        }
        // Rust: Nick Cameron
        "ncameron.org" => {
            for link in raw_links {
                if link.starts_with("/blog/") && !(link.contains("/author/")) {
                    let f_link = "https://www.ncameron.org".to_string() + &link;
                    links.push(f_link)
                }
            }
        }
        // Rust: Manish Goregaokar
        "manishearth.github.io" => {
            for link in raw_links {
                if link.starts_with("/blog/2") {
                    let f_link = "https://manishearth.github.io".to_string() + &link;
                    links.push(f_link)
                }
            }
        }
        // Rust: Simonas Kazlauskas
        "kazlauskas.me" => {
            for link in raw_links {
                if link.starts_with("./entries/") {
                    let f_link = "https://kazlauskas.me".to_string() 
                        + &link.replacen("./", "/", 1);
                    links.push(f_link)
                }
            }
        }
        // Rust: Jan-Erik Rediger
        "fnordig.de" => {
            for link in raw_links {
                if link.starts_with("/2020/") || link.starts_with("/2019/") || link.starts_with("/2018/") {
                    let f_link = "https://fnordig.de".to_string() + &link;
                    links.push(f_link)
                }
            }
        }
        // Rust: Pietro Albini
        "pietroalbini.org" => {
            for link in raw_links {
                if link.starts_with("/blog/") {
                    let f_link = "https://www.pietroalbini.org".to_string() + &link;
                    links.push(f_link)
                }
            }
        }
        // Rust: Without Boats
        "boats.gitlab.io" => {
            for link in raw_links {
                if link.starts_with("https://boats.gitlab.io/blog/post/") {
                    links.push(link)
                }
            }
        }
        // Rust: Pascal Hertleif
        "deterministic.space" => {
            for link in raw_links {
                if link.starts_with("/") && !(link.contains(".xml")) {
                    let f_link = "https://deterministic.space".to_string() + &link;
                    links.push(f_link)
                }
            }
        }
        // Rust: Nick Fitzgerald
        "fitzgeraldnick.com" => {
            for link in raw_links {
                if link.starts_with("/2020/") || link.starts_with("/2019/") || link.starts_with("/2018/") {
                    let f_link = "https://fitzgeraldnick.com".to_string() + &link;
                    links.push(f_link)
                }
            }
        }
        // Rust: Daniel Silverstone
        "blog.digital-scurf.org" => {
            for link in raw_links {
                if link.starts_with("./posts/") && link.len() > 8 && !(link.contains("/#")) {
                    let f_link = "https://blog.digital-scurf.org".to_string() 
                        + &link.replacen("./", "/", 1);
                    links.push(f_link)
                }
            }
        }
        // Rust: Claus Matzinger
        "blog.x5ff.xyz" => {
            for link in raw_links {
                if link.starts_with("/blog/") && link.len() > 6 
                    && !(link.contains("/page/")) && !(link.contains(".xml")) {
                    let f_link = "https://blog.x5ff.xyz".to_string() + &link;
                    links.push(f_link)
                }
            }
        }
        // Rust: Andre Bogus
        "llogiq.github.io" => {
            for link in raw_links {
                if link.starts_with("/2020/") || link.starts_with("/2019/") || link.starts_with("/2018/") {
                    let f_link = "https://llogiq.github.io".to_string() + &link;
                    links.push(f_link)
                }
            }
        }
        // Rust: Tony Arcieri
        "tonyarcieri.com" => {
            for link in raw_links {
                if link.starts_with("https://tonyarcieri.com/") {
                    links.push(link)
                }
            }
        }
        // Rust: Yoshua Wuyts
        "blog.yoshuawuyts.com" => {
            for link in raw_links {
                if link.starts_with("https://blog.yoshuawuyts.com/") && !(link.contains(".xml"))  {
                    links.push(link)
                }
            }
        }
        
        _ => {}  // to deal with
    }
    
    links.sort();
    links.dedup();
    links.reverse();
    links
}


// TODO: more specific parse


// some helpers
// 
// maintain a hashmap to map {host: (author, topic)}
use std::collections::HashMap;

lazy_static! {
    pub static ref MAP_HOST: HashMap<&'static str, (&'static str, &'static str)> = {
        let mut map = HashMap::new();
        map.insert("smallcultfollowing.com", ("Nicholas Matsakis", "Rust"));
        map.insert("tokio.rs", ("Tokio Team", "Rust"));
        map.insert("async.rs", ("async-std", "Rust"));
        map.insert("blog.guillaume-gomez.fr", ("Guillaume Gomez", "Rust"));
        map.insert("ralfj.de", ("Ralf Jung", "Rust"));
        map.insert("words.steveklabnik.com", ("Steve Klabnik", "Rust"));
        map.insert("blog.japaric.io", ("Jorge Aparicio", "Rust"));
        map.insert("ncameron.org", ("Nick Cameron", "Rust"));
        map.insert("manishearth.github.io", ("Manish Goregaokar", "Rust"));
        map.insert("kazlauskas.me", ("Simonas Kazlauskas", "Rust"));
        map.insert("fnordig.de", ("Jan-Erik Rediger", "Rust"));
        map.insert("pietroalbini.org", ("Pietro Albini", "Rust"));
        map.insert("boats.gitlab.io", ("Without Boats", "Rust"));
        map.insert("deterministic.space", ("Pascal Hertleif", "Rust"));
        map.insert("fitzgeraldnick.com", ("Nick Fitzgerald", "Rust"));
        map.insert("blog.digital-scurf.org", ("Daniel Silverstone", "Rust"));
        map.insert("blog.x5ff.xyz", ("Claus Matzinger", "Rust"));
        map.insert("llogiq.github.io", ("Andre Bogus", "Rust"));
        map.insert("tonyarcieri.com", ("Tony Arcieri", "Rust"));
        map.insert("blog.yoshuawuyts.com", ("Yoshua Wuyts", "Rust"));

        map
    };
}

fn get_author_topic(host: &str) -> (String, String) {
    let map = &MAP_HOST;
    let default = &(host,"Rust");
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
