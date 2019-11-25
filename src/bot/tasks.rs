use crate::bot::jobs::Environment;
use crate::errors::{SrvErrToStdErr, SrvError, SrvResult};
use crate::api::{
    item::{NewItem},
};
use diesel::dsl::any;
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
    let npr = WebPage::new("https://www.npr.org/books/");
    links.append(&mut npr.clean_links());
    

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
    use crate::util::helper::gen_slug;
    let mut new_items: Vec<NewItem> = Vec::new();
    for l in diff_links {
        let sp_item = WebPage::new(l).into_item();
        let i_slug = gen_slug(&sp_item.title);
        let new_item = NewItem {
            slug: i_slug,
            ..sp_item
        };
        new_items.push(new_item);
    }
    // save to db
    diesel::insert_into(items)
        .values(&new_items)
        .on_conflict_do_nothing()
        .execute(conn)?;

    Ok(())
}
