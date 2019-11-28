
use futures::{future::result, Future};
use actix::{Handler, Message};
use actix_web::{
    web::{Data, Json, Path, Query},
    Error, HttpResponse, ResponseError,
};
use base64::decode;
use diesel::prelude::*;
use diesel::{self, ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::errors::{ServiceError, ServiceResult};
use crate::api::{ReqQuery};
use crate::api::auth::{CheckUser, CheckCan};
use crate::{Dba, DbAddr, PooledConn};
use crate::schema::{blogs};

// POST: /api/blogs
// 
pub fn new(
    blog: Json<NewBlog>,
    _can: CheckCan,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    db.send(blog.into_inner())
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b)),
            Err(err) => Ok(err.error_response()),
        })
}

impl Handler<NewBlog> for Dba {
    type Result = ServiceResult<Blog>;

    fn handle(&mut self, nb: NewBlog, _: &mut Self::Context) -> Self::Result {
        let conn: &PooledConn = &self.0.get().unwrap();
        nb.new(conn)
    }
}

// PUT: /api/blogs
// 
pub fn update(
    blog: Json<UpdateBlog>,
    _can: CheckCan,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    db.send(blog.into_inner())
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b)),
            Err(err) => Ok(err.error_response()),
        })
}

impl Handler<UpdateBlog> for Dba {
    type Result = ServiceResult<Blog>;

    fn handle(&mut self, b: UpdateBlog, _: &mut Self::Context) -> Self::Result {
        let conn: &PooledConn = &self.0.get().unwrap();
        b.update(conn)
    }
}

// GET: /api/blogs/{id}
// 
pub fn get(
    qb: Path<i32>,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let blog = QueryBlog{
        id: qb.into_inner(), 
        method: String::from("GET"),
        uname: String::new()
    };
    db.send(blog)
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b)),
            Err(err) => Ok(err.error_response()),
        })
}

// PUT: /api/blogs/{id}
// 
pub fn toggle_top(
    qb: Path<i32>,
    auth: CheckCan,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let blog = QueryBlog{
        id: qb.into_inner(), 
        method: String::from("PUT"),
        uname: auth.uname
    };
    db.send(blog)
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b.is_top)),
            Err(err) => Ok(err.error_response()),
        })
}

// DELETE: /api/blogs/{id}
// 
pub fn del(
    qb: Path<i32>,
    auth: CheckCan,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let blog = QueryBlog{
        id: qb.into_inner(), 
        method: String::from("DELETE"),
        uname: auth.uname
    };
    db.send(blog)
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b.aname)),
            Err(err) => Ok(err.error_response()),
        })
}

impl Handler<QueryBlog> for Dba {
    type Result = ServiceResult<Blog>;

    fn handle(&mut self, qb: QueryBlog, _: &mut Self::Context) -> Self::Result {
        let conn: &PooledConn = &self.0.get().unwrap();
        let method: &str = &qb.method.trim();

        match method {
            "GET" => { qb.get(conn) }
            "PUT" => { qb.toggle_top(conn) }
            "DELETE" => { qb.del(conn) }
            _ => { qb.get(conn) },
        }
    }
}

// GET: api/blogs?per=topic&kw=&page=p&perpage=42
// 
pub fn get_list(
    pq: Query<ReqQuery>,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let perpage = pq.perpage;
    let page = pq.page;
    let kw = pq.clone().kw;
    let per = pq.per.trim();
    let blog = match per {
        "topic" => QueryBlogs::Topic(kw, perpage, page),
        "top" => QueryBlogs::Top(kw, perpage, page),
        _ => QueryBlogs::Index(kw, perpage, page),
    };
    db.send(blog)
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b)),
            Err(err) => Ok(err.error_response()),
        })
}

impl Handler<QueryBlogs> for Dba {
    type Result = ServiceResult<(Vec<Blog>, i64)>;

    fn handle(&mut self, qbs: QueryBlogs, _: &mut Self::Context) -> Self::Result {
        let conn: &PooledConn = &self.0.get().unwrap();
        qbs.get(conn)
    }
}


// =================================================================================
// =================================================================================
// Model
// =================================================================================

#[derive(Clone, Debug, Serialize, Deserialize, Default, Identifiable, Queryable)]
#[table_name = "blogs"]
pub struct Blog {
    pub id: i32,
    pub aname: String, // unique, person's name
    pub avatar: String,
    pub intro: String,
    pub topic: String,
    pub blog_link: String,
    pub blog_host: String,
    pub tw_link: String,
    pub gh_link: String,
    pub other_link: String,
    pub is_top: bool,
    pub karma: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, Insertable)]
#[table_name = "blogs"]
pub struct NewBlog {
    pub aname: String,
    pub avatar: String,
    pub intro: String,
    pub topic: String,
    pub blog_link: String,
    pub blog_host: String,
    pub tw_link: String,
    pub gh_link: String,
    pub other_link: String,
    pub is_top: bool,
}

impl NewBlog {
    fn new(
        &self, 
        conn: &PooledConn,
    ) -> ServiceResult<Blog> {
        use crate::schema::blogs::dsl::blogs;
        let blog_new = diesel::insert_into(blogs)
            .values(self)
            .on_conflict_do_nothing()
            .get_result::<Blog>(conn)?;

        Ok(blog_new)
    }

    pub fn save_name_as_blog(
        name: &str,
        conn: &PooledConn,
    ) -> ServiceResult<Blog> {
        let new_blog = NewBlog {
            aname: name.to_owned(),
            is_top: false,
            ..NewBlog::default()
        };
        new_blog.new(conn)
    }
}

impl Message for NewBlog {
    type Result = ServiceResult<Blog>;
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, AsChangeset)]
#[table_name = "blogs"]
pub struct UpdateBlog {
    pub id: i32,
    pub aname: String,
    pub avatar: String,
    pub intro: String,
    pub topic: String,
    pub blog_link: String,
    pub blog_host: String,
    pub tw_link: String,
    pub gh_link: String,
    pub other_link: String,
    pub is_top: bool,
}

impl UpdateBlog {
    fn update(
        mut self,
        conn: &PooledConn,
    ) -> ServiceResult<Blog> {
        use crate::schema::blogs::dsl::*;
        let old = blogs.filter(id.eq(self.id))
            .get_result::<Blog>(conn)?;
        // check if anything chenged
        let new_aname = self.aname.trim();
        let check_changed: bool = new_aname != old.aname.trim()
            || self.avatar.trim() != old.avatar.trim()
            || self.intro.trim() != old.intro.trim()
            || self.topic.trim() != old.topic.trim()
            || self.blog_link.trim() != old.blog_link.trim()
            || self.tw_link.trim() != old.tw_link.trim()
            || self.gh_link.trim() != old.gh_link.trim()
            || self.other_link.trim() != old.other_link.trim()
            || self.is_top != old.is_top;
        if !check_changed {
            return Err(ServiceError::BadRequest("Nothing Changed".to_owned()));
        }

        // update item's author if aname chenged
        if new_aname != old.aname.trim() && new_aname != "" {
            use crate::api::item::Item;
            use crate::schema::items::dsl::{items, author};
            diesel::update(
                items.filter(author.eq(old.aname.trim()))
            )
            .set(author.eq(new_aname))
            .execute(conn)?;
        }

        let blog_update = diesel::update(&old).set(&self).get_result::<Blog>(conn)?;

        Ok(blog_update)
    }
}

impl Message for UpdateBlog {
    type Result = ServiceResult<Blog>;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct QueryBlog {
    pub id: i32,
    pub method: String, // get|delete
    pub uname: String,
}

impl QueryBlog {
    fn get(
        &self, 
        conn: &PooledConn,
    ) -> ServiceResult<Blog> {
        use crate::schema::blogs::dsl::{blogs, id};
        let blog = blogs.filter(id.eq(self.id)).get_result::<Blog>(conn)?;
        Ok(blog)
    }

    fn toggle_top(
        &self, 
        conn: &PooledConn,
    ) -> ServiceResult<Blog> {
        use crate::schema::blogs::dsl::{blogs, id, is_top};
        let old = blogs
            .filter(id.eq(&self.id))
            .get_result::<Blog>(conn)?;
        let check_top: bool = old.is_top;
        let blog = diesel::update(&old)
            .set(is_top.eq(!check_top))
            .get_result::<Blog>(conn)?;

        Ok(blog)
    }

    fn del(
        &self, 
        conn: &PooledConn,
    ) -> ServiceResult<Blog> {
        use crate::schema::blogs::dsl::{blogs, id};
        // // check permission
        // let admin_env = dotenv::var("ADMIN").unwrap_or("".to_string());
        // let check_permission: bool = self.uname == admin_env;
        // if !check_permission {
        //     return Err(ServiceError::Unauthorized);
        // }

        diesel::delete(blogs.filter(id.eq(self.id))).execute(conn)?;
        Ok(Blog::default())
    }
}

impl Message for QueryBlog {
    type Result = ServiceResult<Blog>;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum QueryBlogs {
    Index(String, i32, i32),
    Topic(String, i32, i32),
    Top(String, i32, i32),  // topic, perpage-42, page
}

impl QueryBlogs {
    pub fn get(
        self, 
        conn: &PooledConn,
    ) -> ServiceResult<(Vec<Blog>, i64)> {
        use crate::schema::blogs::dsl::*;
        let mut blog_list: Vec<Blog> = Vec::new();
        let mut blog_count = 0;
        match self {
            QueryBlogs::Topic(t, o, p) => {
                let query = blogs.filter(topic.eq(t));
                let p_o = std::cmp::max(0, p-1);
                blog_count = query.clone().count().get_result(conn)?;
                blog_list = query
                    .order(karma.desc())
                    .limit(o.into())
                    .offset((o * p_o).into())
                    .load::<Blog>(conn)?;
            }
            QueryBlogs::Top(t, o, p) => {
                let query = blogs.filter(is_top.eq(true)).filter(topic.eq(t));
                let p_o = std::cmp::max(0, p-1);
                blog_count = query.clone().count().get_result(conn)?;
                blog_list = query
                    .order(karma.desc())
                    .limit(o.into())
                    .offset((o * p_o).into())
                    .load::<Blog>(conn)?;
            }
            _ => {
                blog_list = blogs
                    .filter(is_top.eq(true))
                    .order(karma.desc()).limit(42).load::<Blog>(conn)?;
                blog_count = blog_list.len() as i64;
            }
        }
        Ok((blog_list, blog_count))
    }
}

impl Message for QueryBlogs {
    type Result = ServiceResult<(Vec<Blog>, i64)>;
}

// TODO
//#[derive(Clone, Debug, Serialize, Deserialize, Default, Identifiable, Queryable)]
//#[table_name = "topics"]
pub struct Topic {
    pub id: i32,
    pub tname: String,
    pub logo: String,
    pub intro: String,
}
