// a simple page crawle

use regex::Regex;
use scraper::{Html, Selector};

use crate::errors::{ServiceError, ServiceResult};
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

#[derive(Debug, Clone, Default)]
pub struct WebPage {
    url: String,
    html: String,
    domain: String,
}

impl WebPage {
    pub fn new(url: &str) -> ServiceResult<Self> {
        let res = reqwest::get(url)?.text()?;

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
            &html, "head > title", "", "untitled"
        )
        .first()
        .unwrap_or(&String::from(""))
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
    
    let img_src: String = if og_img.len() == 0 {
        // random body img
        page_ele_paser(
            &html, "body img", "src", ""
        )
        .first()
        .unwrap_or(&String::from(""))
        .to_string()
    } else {
        og_img
    };

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

pub fn get_links(page: &WebPage) -> Vec<String> {
    let domain = &page.domain;
    let raw_links = page.extract_links();
    let mut links: Vec<String> = Vec::new();
    match domain.trim() {
        // Rust: team Blog
        "blog.rust-lang.org" => {
            for link in raw_links {
                if link.starts_with("/2020/") || link.starts_with("/2019/") {
                    let f_link = "https://blog.rust-lang.org".to_string() + &link;
                    links.push(f_link)
                }
            }
        }
        // Rust: Users forum
        "users.rust-lang.org" => {
            for link in raw_links {
                if link.starts_with("https://users.rust-lang.org/t/") {
                    links.push(link)
                }
            }
        }
        // Rust: Internal forum
        "internals.rust-lang.org" => {
            for link in raw_links {
                if link.starts_with("https://internals.rust-lang.org/t/") {
                    links.push(link)
                }
            }
        }
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
                if link.starts_with("/blog/") && !(link.contains("/tags/"))
                    && !(link.contains("/page/")) 
                {
                    let f_link = "https://async.rs".to_string() + &link;
                    links.push(f_link)
                }
            }
        }
        // Rust: tokio
        "tokio.rs" => {
            for link in raw_links {
                if link.starts_with("https://tokio.rs/blog/2") 
                    && !(link.contains("/#")) 
                {
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
                if link.starts_with("/2020/") || link.starts_with("/2019/") 
                    || link.starts_with("/2018/") 
                {
                    let f_link = "https://fnordig.de".to_string() + &link;
                    links.push(f_link)
                }
            }
        }
        // Rust: Pietro Albini
        "pietroalbini.org" => {
            for link in raw_links {
                if link.starts_with("/blog/") && !(link.contains(".xml")) {
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
                if link.starts_with("/2020/") || link.starts_with("/2019/") 
                    || link.starts_with("/2018/") 
                {
                    let f_link = "https://fitzgeraldnick.com".to_string() + &link;
                    links.push(f_link)
                }
            }
        }
        // Rust: Daniel Silverstone
        "blog.digital-scurf.org" => {
            for link in raw_links {
                if link.starts_with("./posts/") && link.len() > 8 
                    && !(link.contains("/#")) 
                {
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
                    && !(link.contains("/page/")) && !(link.contains(".xml")) 
                {
                    let f_link = "https://blog.x5ff.xyz".to_string() + &link;
                    links.push(f_link)
                }
            }
        }
        // Rust: Andre Bogus
        "llogiq.github.io" => {
            for link in raw_links {
                if link.starts_with("/2020/") 
                    || link.starts_with("/2019/") 
                    || link.starts_with("/2018/") 
                {
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
                if link.starts_with("https://blog.yoshuawuyts.com/") 
                    && !(link.contains(".xml"))  
                {
                    links.push(link)
                }
            }
        }
        // Rust: Sean McArthur
        "seanmonstar.com" => {
            for link in raw_links {
                if link.starts_with("https://seanmonstar.com/post/")  {
                    links.push(link)
                }
            }
        }
        // Rust: Ryan Levick
        "blog.ryanlevick.com" => {
            for link in raw_links {
                if link.starts_with("https://blog.ryanlevick.com/") 
                    && link.len() > 28  
                {
                    links.push(link)
                }
            }
        }
        // Rust: Aleksey Kladov
        "matklad.github.io" => {
            for link in raw_links {
                if link.starts_with("/2020/") 
                    || link.starts_with("/2019/")  
                {
                    let f_link = "https://matklad.github.io".to_string() + &link;
                    links.push(f_link)
                }
            }
        }
        // Rust: Embedded Rust
        "rust-embedded.github.io" => {
            for link in raw_links {
                if link.starts_with("https://rust-embedded.github.io/blog/") 
                    && link.len() > 38  
                    && !(link.contains(".xml")) 
                    && !(link.contains("/page/")) 
                {
                    links.push(link)
                }
            }
        }
        // Rust: Aleksey Kladov
        "blog.troutwine.us" => {
            for link in raw_links {
                if link.starts_with("/2020/") 
                    || link.starts_with("/2019/")  
                {
                    let f_link = "https://blog.troutwine.us".to_string() + &link;
                    links.push(f_link)
                }
            }
        }
        // Rust: Rust-Station
        "rustacean-station.org" => {
            for link in raw_links {
                if link.starts_with("/episode/") 
                    && link.len() > 10  
                {
                    let f_link = "https://rustacean-station.org".to_string() + &link;
                    links.push(f_link)
                }
            }
        }
        // Rust: Raph Levien
        "raphlinus.github.io" => {
            for link in raw_links {
                if link.starts_with("/rust/2020/") 
                    || link.starts_with("/rust/2019/")  
                {
                    let f_link = "https://raphlinus.github.io".to_string() + &link;
                    links.push(f_link)
                }
            }
        }
        // Rust: Armin Ronacher
        "lucumr.pocoo.org" => {
            for link in raw_links {
                if link.starts_with("/2020/") 
                    || link.starts_with("/2019/")  
                {
                    let f_link = "https://lucumr.pocoo.org".to_string() + &link;
                    links.push(f_link)
                }
            }
        }
         // CPP: fluentcpp
         "fluentcpp.com" => {
            for link in raw_links {
                if link.starts_with("https://www.fluentcpp.com/20") 
                    && link.len() > 32  
                    && !(link.contains("/#")) 
                {
                    // let f_link = "https://www.fluentcpp.com/".to_string() + &link;
                    links.push(link)
                }
            }
        }
        // Go: Team Blog
        "blog.golang.org" => {
            for link in raw_links {
                if link.starts_with("/") && link.len() > 2 
                    && !(link.contains("/index")) 
                    && !(link.contains("//")) 
                {
                    let f_link = "https://blog.golang.org".to_string() + &link;
                    links.push(f_link)
                }
            }
        }
        // Go: Russ Cox
        "research.swtch.com" => {
            for link in raw_links {
                if !link.starts_with("https://") && !(link.contains(".atom")) {
                    let f_link = "https://research.swtch.com/".to_string() + &link;
                    links.push(f_link)
                }
            }
        }
        // Web: Mozilla
        "hacks.mozilla.org" => {
            for link in raw_links {
                if link.starts_with("https://hacks.mozilla.org/2020/") 
                    || link.starts_with("https://hacks.mozilla.org/2019/") 
                {
                    links.push(link)
                }
            }
        }
        // Misc, Microsoft
        // "devblogs.microsoft.com" => {
        //     for link in raw_links {
        //         if link.starts_with("https://devblogs.microsoft.com/") 
        //         && !(link.contains("/#"))
        //         && !(link.contains("?"))
        //         && !(link.contains("/tag/"))
        //         && !(link.contains("/category/"))
        //         && !(link.contains("/blog/"))
        //         && !(link.contains("/page/"))
        //         && !(link.contains("/author/"))
        //         && !(link.contains("/wp-login"))
        //         && !(link.contains("/feed/"))
        //         {
        //             links.push(link)
        //         }
        //     }
        // }
        
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
        // Rust
        map.insert("blog.rust-lang.org", ("Rust Team", "Rust"));
        map.insert("users.rust-lang.org", ("Rust Forum", "Rust"));
        map.insert("internals.rust-lang.org", ("Rust Forum", "Rust"));
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
        map.insert("seanmonstar.com", ("Sean McArthur", "Rust"));
        map.insert("blog.ryanlevick.com", ("Ryan Levick", "Rust"));
        map.insert("matklad.github.io", ("Aleksey Kladov", "Rust"));
        map.insert("blog.troutwine.us", ("Troutwine", "Rust"));
        map.insert("rust-embedded.github.io", ("Embedded Rust", "Rust"));
        map.insert("rustacean-station.org", ("rustacean-station", "Rust"));
        map.insert("raphlinus.github.io", ("Raph Levien", "Rust")); 
        map.insert("lucumr.pocoo.org", ("Armin Ronacher", "Rust")); 
        // c++
        map.insert("fluentcpp.com", ("Jonathan Boccara", "CPP"));
        // Golang
        map.insert("blog.golang.org", ("Go Team", "Go"));
        map.insert("research.swtch.com", ("Russ Cox", "Go"));
        // Angular
        map.insert("blog.angular.io", ("Angular Team", "Angular"));
        // Web
        map.insert("hacks.mozilla.org", ("Mozilla", "Web"));
        // Misc
        map.insert("devblogs.microsoft.com", ("MicroSoft", "dotnet"));

        map
    };
}

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
