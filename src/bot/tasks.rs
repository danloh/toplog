use crate::bot::jobs::Environment;
use crate::errors::{SrvErrToStdErr, SrvError, SrvResult};
use crate::api::{
    item::{NewItem, Item},
    blog::{Blog},
};
use diesel::dsl::{any, sum};
use diesel::prelude::*;
use std::error::Error;
use swirl::PerformError;

// ===============================================================
// spider
//
// spider item and save to db
#[swirl::background_job]
pub fn spider_items(env: &Environment) -> Result<(), PerformError> {
    let conn = env.connection()?;
    spider_and_save_item(&conn)?;
    
    Ok(())
}

pub fn spider_and_save_item(conn: &PgConnection) -> QueryResult<()> {
    use crate::schema::items::dsl::*;
    use crate::bot::spider::{WebPage};

    // new WebPages and get all links
    let mut links: Vec<String> = Vec::new();
    let url_list = vec!(
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
        // ## Golang
        "https://blog.golang.org/index",
        "https://research.swtch.com/",  // Russ Cox
        // ## Angular
        // ## Web
        "https://hacks.mozilla.org/",
        
        // ## Mic
        "https://devblogs.microsoft.com",
    );

    for url in url_list {
        let page = WebPage::new(url);
        links.append(&mut page.clean_links());
    }
    // println!("{:?}", links);
    
    // diff the links w/ db
    use std::collections::HashSet;
    let mut links_set = HashSet::new();
    for l in links {
        // regex check url
        use crate::api::re_test_url;
        if re_test_url(&l) {
            links_set.insert(l);
        }
    }
    // TODO: limit the db query
    /*
    use chrono::{NaiveDate, Utc, Duration};
    let now = Utc::now().naive_utc();
    let limit_day = now - Duration::days(1);
    */
    
    let db_links: Vec<String> = items
        .filter(link.ne(""))
        //.filter(post_at.lt(limit_day))  // did nothing
        .select(link)
        .load::<String>(conn)?;
    let mut db_links_set = HashSet::new();
    for l in db_links {
        db_links_set.insert(l);
    }
    let diff_links = links_set.difference(&db_links_set);
    //println!("{:#?}", diff_links);
    // spider the diff_links and build item
    let mut new_items: Vec<NewItem> = Vec::new();
    for l in diff_links {
        let sp_item = WebPage::new(l).into_item();
        new_items.push(sp_item);
    }
    // save to db
    diesel::insert_into(items)
        .values(&new_items)
        .on_conflict_do_nothing()
        .execute(conn)?;

    Ok(())
}


// Cal 
//
// cal blog karma
#[swirl::background_job]
pub fn cal_blogs_karma(env: &Environment) -> Result<(), PerformError> {
    let conn = env.connection()?;
    update_blogs_karma(&conn)?;

    Ok(())
}

pub fn update_blogs_karma(conn: &PgConnection) -> QueryResult<()> {
    use crate::schema::blogs::dsl::*;
    use crate::schema::items::dsl::{items, author, vote};
    let blog_list = blogs.load::<Blog>(conn)?;
    for b in blog_list {
        let bname = &b.aname;
        let votes: Vec<i32> = items
            .filter(author.eq(bname))
            .select(vote)
            .load::<i32>(conn)?;
        let k: i32 = votes.iter().sum();
        let threshold: i32 = dotenv::var("THRESHOLD")
            .unwrap_or("42".to_owned())
            .parse().unwrap_or(42);
        let if_top = k > threshold || b.is_top;
        diesel::update(&b)
            .set((
                karma.eq(k), 
                is_top.eq(if_top)
            ))
            .execute(conn)?;
    }

    Ok(())
}


// statify the site
//
#[swirl::background_job]
pub fn gen_static_site(_env: &Environment) -> Result<(), PerformError> {
    // way-1: regenerate site
    // let host = dotenv::var("DOMAIN_HOST")
    //     .unwrap_or("https://newdin.com".to_string());
    // let url = host + "/api/generate-staticsite-noexpose";
    // reqwest::get(&url)?;

    // way-2: del htmls
    use crate::view::{ TOPIC_VEC, tmpl::del_html };
    let mut names: Vec<String> = Vec::new();
    for n in TOPIC_VEC.iter() {
        let name = n.to_string() + "-Misc";
        names.push(name);
    }

    for nm in names {
        del_html(&nm);
    }
    
    Ok(())
}
