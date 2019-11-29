
use futures::{future::result, Future};
use actix::{Handler, Message};
use actix_web::{
    web::{Data, Json, Path, Query},
    Error, HttpResponse, ResponseError,
};
use base64::decode;
use diesel::prelude::*;
use diesel::{self, ExpressionMethods, QueryDsl, RunQueryDsl};
use chrono::{NaiveDateTime, NaiveDate, Utc};

use crate::errors::{ServiceError, ServiceResult};
use crate::api::{
    ReqQuery, ActionQuery, 
    auth::{CheckUser, CheckCan},
};
use crate::util::helper::gen_slug;
use crate::{Dba, DbAddr, PooledConn};
use crate::schema::{items, voteitems};

// POST: /api/items
// 
pub fn new(
    item: Json<NewItem>,
    _auth: CheckUser,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    db.send(item.into_inner())
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b)),
            Err(err) => Ok(err.error_response()),
        })
}

impl Handler<NewItem> for Dba {
    type Result = ServiceResult<Item>;

    fn handle(&mut self, na: NewItem, _: &mut Self::Context) -> Self::Result {
        let conn: &PooledConn = &self.0.get().unwrap();
        na.new(conn)
    }
}

// PUT: /api/items
// 
pub fn update(
    item: Json<UpdateItem>,
    _auth: CheckUser,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    db.send(item.into_inner())
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b)),
            Err(err) => Ok(err.error_response()),
        })
}

impl Handler<UpdateItem> for Dba {
    type Result = ServiceResult<Item>;

    fn handle(&mut self, b: UpdateItem, _: &mut Self::Context) -> Self::Result {
        let conn: &PooledConn = &self.0.get().unwrap();
        b.update(conn)
    }
}

// GET: /api/items/{slug}
// 
pub fn get(
    qb: Path<String>,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let item = QueryItem{
        slug: qb.into_inner(), 
        method: String::from("GET"),
        uname: String::new()
    };
    db.send(item)
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b)),
            Err(err) => Ok(err.error_response()),
        })
}

// PATCH: /api/items/{slug}
// 
pub fn toggle_top(
    qb: Path<String>,
    auth: CheckCan,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let item = QueryItem{
        slug: qb.into_inner(), 
        method: String::from("PUT"),
        uname: auth.uname
    };
    db.send(item)
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b.is_top)),
            Err(err) => Ok(err.error_response()),
        })
}

// PUT: /api/items/{slug}?action=vote|veto
// 
pub fn vote_or_veto(
    qb: Path<String>,
    aq: Query<ActionQuery>,
    auth: CheckUser,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let item = QueryItem{
        slug: qb.into_inner(), 
        method: aq.action.to_uppercase(),
        uname: auth.uname
    };
    db.send(item)
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b.vote)),
            Err(err) => Ok(err.error_response()),
        })
}

// DELETE: /api/items/{slug}
// 
pub fn del(
    qb: Path<String>,
    auth: CheckCan,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    
    let item = QueryItem{
        slug: qb.into_inner(), 
        method: String::from("DELETE"),
        uname: auth.uname
    };
    db.send(item)
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b.slug)),
            Err(err) => Ok(err.error_response()),
        })
}

impl Handler<QueryItem> for Dba {
    type Result = ServiceResult<Item>;

    fn handle(&mut self, qb: QueryItem, _: &mut Self::Context) -> Self::Result {
        let conn: &PooledConn = &self.0.get().unwrap();
        let method: &str = &qb.method.trim();

        match method {
            "GET" => { qb.get(conn) }
            "PUT" => { qb.toggle_top(conn) }
            "VOTE" => { qb.vote_or_veto(conn, "VOTE") }
            "VETO" => { qb.vote_or_veto(conn, "VETO") }
            "DELETE" => { qb.del(conn) }
            _ => { qb.get(conn) },
        }
    }
}

// GET: api/items?per=topic|author&kw=&page=p&perpage=42
// 
pub fn get_list(
    pq: Query<ReqQuery>,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let perpage = pq.perpage;
    let page = pq.page;
    let kw = pq.clone().kw;
    let per = pq.per.trim();
    let item = match per {
        "topic" => QueryItems::Topic(kw, perpage, page),
        "author" => QueryItems::Author(kw, perpage, page),
        "ty" => QueryItems::Ty(kw, perpage, page),
        "index" => QueryItems::Index(kw, perpage, page),
        // other: 
        // kw-topic: rust|go.., per-ty: art|book|..
        _ => QueryItems::Tt(kw, per.to_owned(), perpage, page),
    };
    db.send(item)
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b)),
            Err(err) => Ok(err.error_response()),
        })
}

impl Handler<QueryItems> for Dba {
    type Result = ServiceResult<(Vec<Item>, i64)>;

    fn handle(&mut self, qbs: QueryItems, _: &mut Self::Context) -> Self::Result {
        let conn: &PooledConn = &self.0.get().unwrap();
        qbs.get(conn)
    }
}


// =================================================================================
// =================================================================================
// Model
// =================================================================================

#[derive(Clone, Debug, Serialize, Deserialize, Identifiable, Queryable)]
#[table_name = "items"]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub logo: String,
    pub author: String,
    pub ty: String,      // 1-item,2-translate,3-podcast,4-event,5-book
    pub lang: String,
    pub topic: String,
    pub link: String,
    pub link_host: String,
    pub origin_link: String, // for translate
    pub post_by: String,
    pub post_at: NaiveDateTime,
    pub pub_at: NaiveDate,
    pub is_top: bool,
    pub vote: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable, Default)]
#[table_name = "items"]
pub struct NewItem {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub logo: String,
    pub author: String,
    pub ty: String, 
    pub lang: String,
    pub topic: String,
    pub link: String,
    pub origin_link: String,
    pub post_by: String,
}

impl NewItem {
    fn new(
        self, 
        conn: &PooledConn,
    ) -> ServiceResult<Item> {
        use crate::schema::items::dsl::items;
        let title = self.title.trim();
        let a_slug = gen_slug(title);
        let new_item = NewItem {
            title: title.to_owned(),
            slug: a_slug,
            content: self.content.trim().to_owned(),  // do some trim
            logo: self.logo.trim().to_owned(),
            author: self.author.trim().to_owned(),
            ty: self.ty.trim().to_owned(),
            lang: self.lang.trim().to_owned(),
            topic: self.topic.trim().to_owned(),
            link: self.link.trim().to_owned(),
            origin_link: self.origin_link.trim().to_owned(),
            post_by: self.post_by.trim().to_owned(),
        };

        // save item's author to blog, for reference
        let aname = new_item.author.trim();
        if aname != "" {
            use crate::api::blog::NewBlog;
            NewBlog::save_name_as_blog(aname, conn);  // ignore potential error
        }

        let item_new = diesel::insert_into(items)
            .values(&new_item)
            .on_conflict_do_nothing()
            .get_result::<Item>(conn)?;
        
        Ok(item_new)
    }
}

impl Message for NewItem {
    type Result = ServiceResult<Item>;
}


#[derive(Clone, Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "items"]
pub struct UpdateItem {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub logo: String,
    pub author: String,
    pub ty: String, 
    pub lang: String,
    pub topic: String,
    pub link: String,
    pub origin_link: String,
    pub post_by: String,
    pub pub_at: NaiveDate,
}

impl UpdateItem {
    fn update(
        mut self, 
        conn: &PooledConn,
    ) -> ServiceResult<Item> {
        use crate::schema::items::dsl::*;
        let old = items.filter(id.eq(self.id))
            .get_result::<Item>(conn)?;
        
        // check if anything changed
        let new_title = self.title.trim();
        let new_content = self.content.trim();
        let new_logo = self.logo.trim();
        let new_author = self.author.trim();
        let new_ty = self.ty.trim();
        let new_lang = self.lang.trim();
        let new_topic = self.topic.trim();
        let new_link = self.link.trim();
        let new_origin = self.origin_link.trim();
        let new_pub_at = self.pub_at;

        let check_changed: bool = new_title != old.title.trim()
            || new_content != old.content.trim()
            || new_logo != old.logo.trim()
            || new_author != old.author.trim()
            || new_ty != old.ty.trim()
            || new_lang != old.lang.trim()
            || new_topic != old.topic.trim()
            || new_link != old.link.trim()
            || new_origin != old.origin_link.trim()
            || new_pub_at != old.pub_at;
        if !check_changed {
            return Err(ServiceError::BadRequest("Nothing Changed".to_owned()));
        }
        
        // check if title changed
        let check_title_changed: bool = &new_title != &old.title.trim();
        let a_slug = if check_title_changed {
            gen_slug(new_title) 
        } else {
            (&old).slug.to_owned()
        };
        let up = UpdateItem {
            title: new_title.to_owned(),
            slug: a_slug,
            content: new_content.to_owned(),  // do some trim
            logo: new_logo.to_owned(),
            author: new_author.to_owned(),
            ty: new_ty.to_owned(),
            lang: new_lang.to_owned(),
            topic: new_topic.to_owned(),
            link: new_link.to_owned(),
            origin_link: new_origin.to_owned(),
            pub_at: new_pub_at,
            ..self
        };

        // save item's author to blog, for referenc
        let aname = up.author.trim();
        if aname != "" && aname != old.author.trim() {
            use crate::api::blog::NewBlog;
            NewBlog::save_name_as_blog(aname, conn);  // ignore potential error
        }

        let item_update = diesel::update(&old)
            .set(up)
            .get_result::<Item>(conn)?;
        
        Ok(item_update)
    }
}

impl Message for UpdateItem {
    type Result = ServiceResult<Item>;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct QueryItem {
    pub slug: String,
    pub method: String, // get|put|delete|vote
    pub uname: String,
}

impl QueryItem {
    fn get(
        &self, 
        conn: &PooledConn,
    ) -> ServiceResult<Item> {
        use crate::schema::items::dsl::{items, slug};
        let item = items.filter(slug.eq(&self.slug))
            .get_result::<Item>(conn)?;
        Ok(item)
    }

    fn vote_or_veto(
        &self, 
        conn: &PooledConn,
        action: &str,
    ) -> ServiceResult<Item> {
        use crate::schema::items::dsl::{items, slug, vote, is_top};

        let old = items
            .filter(slug.eq(&self.slug))
            .get_result::<Item>(conn)?;
        let old_vote = old.vote;
        let old_is_top = old.is_top;
        let act = action.to_uppercase();

        use crate::schema::voteitems::dsl::{voteitems};
        let itemid = old.id;
        let new_vote = VoteItem {
            uname: self.uname.to_owned(),
            item_id: itemid,
            vote_at: Utc::now().naive_utc(),
            vote_as: if act == "VOTE" { 1 } else { -1 },
        };
        let as_vote = new_vote.new(conn).unwrap_or(0) as i32;

        let incr = if act == "VOTE" { as_vote } else { 0 - as_vote };
        let threshold: i32 = dotenv::var("THRESHOLD")
            .unwrap_or("42".to_owned())
            .parse().unwrap_or(42);
        let if_top = old_vote > threshold || old_is_top;

        let item = diesel::update(&old)
            .set((
                vote.eq(vote + incr),
                is_top.eq(if_top)
            ))
            .get_result::<Item>(conn)?;

        Ok(item)
    }

    fn toggle_top(
        &self, 
        conn: &PooledConn,
    ) -> ServiceResult<Item> {
        use crate::schema::items::dsl::{items, slug, is_top};
        let old = items
            .filter(slug.eq(&self.slug))
            .get_result::<Item>(conn)?;
        let check_top: bool = old.is_top;
        let item = diesel::update(&old)
            .set(is_top.eq(!check_top))
            .get_result::<Item>(conn)?;

        Ok(item)
    }

    fn del(
        &self, 
        conn: &PooledConn,
    ) -> ServiceResult<Item> {
        use crate::schema::items::dsl::{items, slug};
        // check permission
        let admin_env = dotenv::var("ADMIN").unwrap_or("".to_string());
        let check_permission: bool = self.uname == admin_env;
        if !check_permission {
            return Err(ServiceError::Unauthorized);
        }

        let item = diesel::delete(items.filter(slug.eq(&self.slug)))
            .get_result::<Item>(conn)?;
        Ok(item)
    }
}

impl Message for QueryItem {
    type Result = ServiceResult<Item>;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum QueryItems {
    Index(String, i32, i32),
    Topic(String, i32, i32), // topic, perpage, page
    Ty(String, i32, i32), // ty, perpage, page
    Tt(String, String, i32, i32), // topic, ty, perpage, page
    Author(String, i32, i32),  // aname, ..
}

impl QueryItems {
    pub fn get(
        self, 
        conn: &PooledConn,
    ) -> ServiceResult<(Vec<Item>, i64)> {
        use crate::schema::items::dsl::*;
        let mut item_list: Vec<Item> = Vec::new();
        let mut item_count = 0;
        match self {
            QueryItems::Index(t, o, p) => {  // topic = all
                let p_o = std::cmp::max(0, p-1);
                match t.to_lowercase().trim() {
                    "index" => {
                        item_list = items
                            .filter(is_top.eq(true))
                            .order(pub_at.desc())
                            .limit(42)
                            .load::<Item>(conn)?;
                        item_count = item_list.len() as i64;
                    }
                    "misc" => {
                        let query = items
                            .filter(is_top.eq(false));
                        item_count = query.clone().count().get_result(conn)?;
                        item_list = query
                            .order(pub_at.desc())
                            .limit(o.into())
                            .offset((o * p_o).into())
                            .load::<Item>(conn)?;
                    }
                    _ => {
                        let query = items
                            .filter(is_top.eq(true))
                            .filter(ty.eq(t));
                        item_count = query.clone().count().get_result(conn)?;
                        item_list = query
                            .order(pub_at.desc())
                            .limit(o.into())
                            .offset((o * p_o).into())
                            .load::<Item>(conn)?;
                    }

                }
            }
            QueryItems::Tt(t, typ, o, p) => {
                let p_o = std::cmp::max(0, p-1);
                if typ.trim().to_lowercase() == "misc" {
                    let query = items
                        .filter(is_top.eq(false))
                        .filter(topic.eq(t));
                    item_count = query.clone().count().get_result(conn)?;
                    item_list = query
                        .order(pub_at.desc())
                        .limit(o.into())
                        .offset((o * p_o).into())
                        .load::<Item>(conn)?;
                } else {
                    let query = items
                        .filter(is_top.eq(true))
                        .filter(topic.eq(t))
                        .filter(ty.eq(typ));
                    item_count = query.clone().count().get_result(conn)?;
                    item_list = query
                        .order(pub_at.desc())
                        .limit(o.into())
                        .offset((o * p_o).into())
                        .load::<Item>(conn)?;
                }
            }
            QueryItems::Topic(t, o, p) => {
                let query = items
                    .filter(is_top.eq(true))
                    .filter(topic.eq(t));
                let p_o = std::cmp::max(0, p-1);
                item_count = query.clone().count().get_result(conn)?;
                item_list = query
                    .order(pub_at.desc())
                    .limit(o.into())
                    .offset((o * p_o).into())
                    .load::<Item>(conn)?;
            }
            QueryItems::Ty(t, o, p) => {
                let query = items
                    .filter(is_top.eq(true))
                    .filter(ty.eq(t));
                let p_o = std::cmp::max(0, p-1);
                item_count = query.clone().count().get_result(conn)?;
                item_list = query
                    .order(pub_at.desc())
                    .limit(o.into())
                    .offset((o * p_o).into())
                    .load::<Item>(conn)?;
            }
            QueryItems::Author(a, o, p) => {
                let query = items
                    //.filter(is_top.eq(true))
                    .filter(author.eq(a));
                let p_o = std::cmp::max(0, p-1);
                item_count = query.clone().count().get_result(conn)?;
                item_list = query
                    .order(pub_at.desc())
                    .limit(o.into())
                    .offset((o * p_o).into())
                    .load::<Item>(conn)?;
            }
            _ => {
                item_list = items
                    .filter(is_top.eq(true))
                    .order(pub_at.desc())
                    .limit(42)
                    .load::<Item>(conn)?;
                item_count = item_list.len() as i64;
            }
        }
        Ok((item_list, item_count))
    }
}

impl Message for QueryItems {
    type Result = ServiceResult<(Vec<Item>, i64)>;
}

#[derive(
    Clone, Debug, Serialize, Deserialize, 
    Identifiable, Queryable, Insertable, AsChangeset
)]
#[primary_key(uname, item_id)]
#[table_name = "voteitems"]
pub struct VoteItem {
    pub uname: String,
    pub item_id: i32,
    pub vote_at: NaiveDateTime,
    pub vote_as: i16,
}

impl VoteItem {
    fn new(
        &self, 
        conn: &PooledConn,
    ) -> ServiceResult<usize> {
        use crate::schema::voteitems::dsl::{voteitems};
        let vote_count = diesel::insert_into(voteitems)
            .values(self)
            .on_conflict_do_nothing()
            .execute(conn)?;

        Ok(vote_count)
    }

    fn del(
        &self, 
        conn: &PooledConn,
    ) -> ServiceResult<usize> {
        use crate::schema::voteitems::dsl::{voteitems, uname, item_id};

        let unvote = diesel::delete(
            voteitems
                .filter(uname.eq(&self.uname))
                .filter(item_id.eq(&self.item_id))
            ).execute(conn)?;

        Ok(unvote)
    }
}
