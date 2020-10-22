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
    use crate::bot::cfg::LINK_VEC;
    let url_list = &LINK_VEC;
    for url in url_list.iter() {
        // println!("{}", url);
        let page = WebPage::new(url).unwrap_or_default();
        links.append(&mut page.clean_links());
    }
    // println!("{:?}", links);
    
    // diff the links w/ db
    //
    // extracted new links
    use std::collections::HashSet;
    let mut links_set = HashSet::new();
    for l in links {
        // regex check url
        use crate::api::re_test_url;
        if re_test_url(&l) {
            links_set.insert(l);
        }
    }

    // deserde spidered links from Json as reduce query db
    //
    use crate::util::helper::{deserde_links, serde_links, serde_add_links};
    let sped_links = deserde_links();
    let sp_links = if sped_links.len() > 0 {
        sped_links
    } else {
        let db_links: Vec<String> = items
            .filter(link.ne(""))
            //.filter(post_at.lt(limit_day))  // did nothing
            .select(link)
            .load::<String>(conn)?;
        serde_links(db_links.clone());
        db_links
    };
    let mut spd_links_set = HashSet::new();
    for l in sp_links {
        spd_links_set.insert(l);
    }

    // diff the real new links to feed spider
    let diff_links = links_set.difference(&spd_links_set);
    //println!("{:#?}", diff_links);
    let mut new_links: Vec<String> = Vec::new();
    // spider the diff_links and build item
    let mut new_items: Vec<NewItem> = Vec::new();
    for l in diff_links {
        let sp_item = WebPage::new(l)
            .unwrap_or_default()
            .into_item();
        new_items.push(sp_item);
        new_links.push(l.to_string());
    }

    // save new items to db
    diesel::insert_into(items)
        .values(&new_items)
        .on_conflict_do_nothing()
        .execute(conn)?;
    
    // save new links to json
    serde_add_links(new_links);

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

    use crate::view::tmpl::del_dir;
    del_dir("www/collection").unwrap_or(());
    del_dir("www/item").unwrap_or(());
    
    Ok(())
}
