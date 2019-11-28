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
        "http://smallcultfollowing.com/babysteps/"
    );
    let babystep = WebPage::new("http://smallcultfollowing.com/babysteps/");
    links.append(&mut babystep.clean_links());
    let tokio = WebPage::new("https://tokio.rs/blog/2019-11-tokio-0-2/");
    links.append(&mut tokio.clean_links());
    let asystd = WebPage::new("https://async.rs/blog/");
    links.append(&mut asystd.clean_links());
    let ggomez = WebPage::new("https://blog.guillaume-gomez.fr/");
    links.append(&mut ggomez.clean_links());
    let ralf = WebPage::new("https://www.ralfj.de/blog/");
    links.append(&mut ralf.clean_links());
    let steve = WebPage::new("https://words.steveklabnik.com/");
    links.append(&mut steve.clean_links());
    let jorge = WebPage::new("https://blog.japaric.io/");
    links.append(&mut jorge.clean_links());
    let nrc = WebPage::new("https://www.ncameron.org/blog/");
    links.append(&mut nrc.clean_links());
    let mg = WebPage::new("https://manishearth.github.io/");
    links.append(&mut mg.clean_links());
    let nagisa = WebPage::new("https://kazlauskas.me/");
    links.append(&mut nagisa.clean_links());
    let badboy = WebPage::new("https://fnordig.de/posts/");
    links.append(&mut badboy.clean_links());
    let pietroalbini = WebPage::new("https://www.pietroalbini.org/");
    links.append(&mut pietroalbini.clean_links());
    let boats = WebPage::new("https://boats.gitlab.io/blog/");
    links.append(&mut boats.clean_links());
    let pascal = WebPage::new("https://deterministic.space/");
    links.append(&mut pascal.clean_links());
    let fitz = WebPage::new("https://fitzgeraldnick.com/");
    links.append(&mut fitz.clean_links());
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
    let db_links: Vec<String> = items
        .filter(link.ne(""))
        .select(link)
        .load::<String>(conn)?;
    let mut db_links_set = HashSet::new();
    for l in db_links {
        db_links_set.insert(l);
    }
    let diff_links = links_set.difference(&db_links_set);
    //println!("{:#?}", diff_links);
    // spider the diff_links and build Rut
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
        let if_top = k > 100 || b.is_top;
        diesel::update(&b)
            .set((
                karma.eq(k), 
                is_top.eq(if_top)
            ))
            .execute(conn)?;
    }

    Ok(())
}
