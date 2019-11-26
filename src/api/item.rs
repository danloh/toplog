
use futures::{future::result, Future};
use actix::{Handler, Message};
use actix_web::{
    web::{Data, Json, Path, Query},
    Error, HttpResponse, ResponseError,
};
use base64::decode;
use diesel::prelude::*;
use diesel::{self, ExpressionMethods, QueryDsl, RunQueryDsl};
use chrono::{NaiveDateTime, Utc};

use crate::errors::{ServiceError, ServiceResult};
use crate::api::{ReqQuery, auth::CheckUser};
use crate::util::helper::gen_slug;
use crate::{Dba, DbAddr, PooledConn};
use crate::schema::{items};

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

// DELETE: /api/items/{slug}
// 
pub fn del(
    qb: Path<String>,
    auth: CheckUser,
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
            Ok(b) => Ok(HttpResponse::Ok().json(b)),
            Err(err) => Ok(err.error_response()),
        })
}

impl Handler<QueryItem> for Dba {
    type Result = ServiceResult<Item>;

    fn handle(&mut self, qb: QueryItem, _: &mut Self::Context) -> Self::Result {
        let conn: &PooledConn = &self.0.get().unwrap();
        let method: &str = &qb.method.trim();
        if method == "GET" {
            qb.get(conn)
        } else {
            qb.del(conn)
        }
    }
}

// GET: api/items?per=topic|author&kw=&perpage=42&page=p
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
    pub pub_at: NaiveDateTime,
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
        let a_slug = gen_slug(&self.title);
        let new_item = NewItem {
            slug: a_slug,
            ..self
        };
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
}

impl UpdateItem {
    fn update(
        mut self, 
        conn: &PooledConn,
    ) -> ServiceResult<Item> {
        use crate::schema::items::dsl::*;
        let old = items.filter(id.eq(self.id))
            .get_result::<Item>(conn)?;
        // check if title changed
        let check_title_changed: bool = &self.title.trim() == &old.title.trim();
        let a_slug = if check_title_changed {
            (&old).slug.to_owned()
        } else {
            gen_slug(&self.title)
        };
        let up = UpdateItem {
            slug: a_slug,
            ..self
        };

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
    pub method: String, // get|delete
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
            QueryItems::Index(t, o, p) => {
                if t.trim() == "index" {
                    item_list = items
                        .filter(is_top.eq(true))
                        .order(post_at.desc())
                        .limit(42)
                        .load::<Item>(conn)?;
                    item_count = item_list.len() as i64;
                } else {
                    let p_o = std::cmp::max(0, p-1);
                    if t.trim() == "Misc" {
                        let query = items
                            .filter(is_top.eq(false));
                        item_count = query.clone().count().get_result(conn)?;
                        item_list = query
                            .order(post_at.desc())
                            .limit(o.into())
                            .offset((o * p_o).into())
                            .load::<Item>(conn)?;
                    } else {
                        let query = items
                            .filter(is_top.eq(true))
                            .filter(ty.eq(t));
                        item_count = query.clone().count().get_result(conn)?;
                        item_list = query
                            .order(post_at.desc())
                            .limit(o.into())
                            .offset((o * p_o).into())
                            .load::<Item>(conn)?;
                    }
                }
            }
            QueryItems::Tt(t, typ, o, p) => {
                let p_o = std::cmp::max(0, p-1);
                if typ.trim() == "Misc" {
                    let query = items
                        .filter(is_top.eq(false))
                        .filter(topic.eq(t));
                    item_count = query.clone().count().get_result(conn)?;
                    item_list = query
                        .order(post_at.desc())
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
                        .order(post_at.desc())
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
                    .order(post_at.desc())
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
                    .order(post_at.desc())
                    .limit(o.into())
                    .offset((o * p_o).into())
                    .load::<Item>(conn)?;
            }
            QueryItems::Author(a, o, p) => {
                let query = items
                    .filter(is_top.eq(true))
                    .filter(author.eq(a));
                let p_o = std::cmp::max(0, p-1);
                item_count = query.clone().count().get_result(conn)?;
                item_list = query
                    .order(post_at.desc())
                    .limit(o.into())
                    .offset((o * p_o).into())
                    .load::<Item>(conn)?;
            }
            _ => {
                item_list = items
                    .filter(is_top.eq(true))
                    .order(post_at.desc())
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
