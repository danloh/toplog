// cfg spider to get webpage links

use std::collections::HashMap;
use crate::bot::spider::WebPage;

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
        // Rust: Stjepan Glavina
        "stjepang.github.io" => {
            for link in raw_links {
                if link.starts_with("/2020/") 
                    || link.starts_with("/2019/")  
                {
                    let f_link = "https://stjepang.github.io".to_string() + &link;
                    links.push(f_link)
                }
            }
        }
        // Rust: bastion.rs
        "blog.bastion.rs" => {
            for link in raw_links {
                if link.starts_with("/2020/")  
                {
                    let f_link = "https://blog.bastion.rs".to_string() + &link;
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
 
// maintain a hashmap to map {host: (author, topic)}
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
        map.insert("stjepang.github.io", ("Stjepan Glavina", "Rust")); 
        map.insert("blog.bastion.rs", ("bastion.rs", "Rust")); 
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

    pub static ref LINK_VEC: Vec<&'static str> = {
        vec!(
            // ## Rust
            "https://blog.rust-lang.org/",
            "https://users.rust-lang.org/top",
            "https://internals.rust-lang.org/top",
            "http://smallcultfollowing.com/babysteps/", // babystep
            "https://tokio.rs/blog/2019-11-tokio-0-2/", // tokio
            "https://async.rs/blog/", // async-std
            "https://blog.guillaume-gomez.fr/", // Gomez
            "https://www.ralfj.de/blog/", // ralf
            "https://fitzgeraldnick.com/", // fitz
            "https://deterministic.space/", // pascal
            "https://boats.gitlab.io/blog/", // boats
            "https://www.pietroalbini.org/", 
            "https://fnordig.de/posts/",  // badboy
            "https://kazlauskas.me/", // nagisa
            "https://manishearth.github.io/", // mg
            "https://www.ncameron.org/blog/",  // nrc
            "https://blog.japaric.io/",  // jorge
            "https://words.steveklabnik.com/",
            "https://blog.digital-scurf.org/", // Daniel Silverstone
            "https://blog.x5ff.xyz/blog/",  // Claus Matzinger
            "https://llogiq.github.io/", // Andre Bogus
            "https://tonyarcieri.com/",  // Tony Arcieri
            "https://blog.yoshuawuyts.com/", // Yoshua Wuyts
            "https://seanmonstar.com/",
            "https://blog.ryanlevick.com/", // Ryan Levick
            "https://matklad.github.io/",  // Aleksey Kladov
            "https://blog.troutwine.us/", // 
            "https://rust-embedded.github.io/blog/", // Embedded Rust Working Group
            "https://rustacean-station.org/",
            "https://raphlinus.github.io/", // Raph Levien
            "https://lucumr.pocoo.org/",  //  Armin Ronacher
            "https://stjepang.github.io/",  // Stjepan Glavina
            "https://blog.bastion.rs/", // bastion.rs
            // cpp
            "https://www.fluentcpp.com/",
            // ## Golang
            "https://blog.golang.org/index",
            "https://research.swtch.com/",  // Russ Cox
            // ## Angular
            // ## Web
            "https://hacks.mozilla.org/",
            
            // ## Mic
            //"https://devblogs.microsoft.com",
        )
    };
}
