
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
use crate::api::auth::CheckUser;
use crate::{Dba, DbAddr, PooledConn};
use crate::schema::{blogs};

// POST: /api/blogs
// 
pub fn new(
    blog: Json<NewBlog>,
    _auth: CheckUser,
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
    blog: Json<Blog>,
    _auth: CheckUser,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    db.send(blog.into_inner())
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b)),
            Err(err) => Ok(err.error_response()),
        })
}

impl Handler<Blog> for Dba {
    type Result = ServiceResult<Blog>;

    fn handle(&mut self, b: Blog, _: &mut Self::Context) -> Self::Result {
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
        method: String::from("GET")
    };
    db.send(blog)
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b)),
            Err(err) => Ok(err.error_response()),
        })
}

// DELETE: /api/blogs/{id}
// 
pub fn del(
    qb: Path<i32>,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let blog = QueryBlog{
        id: qb.into_inner(), 
        method: String::from("DELETE")
    };
    db.send(blog)
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b)),
            Err(err) => Ok(err.error_response()),
        })
}

impl Handler<QueryBlog> for Dba {
    type Result = ServiceResult<Blog>;

    fn handle(&mut self, qb: QueryBlog, _: &mut Self::Context) -> Self::Result {
        let conn: &PooledConn = &self.0.get().unwrap();
        let method: &str = &qb.method.trim();
        if method == "GET" {
            qb.get(conn)
        } else {
            qb.del(conn)
        }
    }
}

// GET: api/blogs?per=topic&kw=&perpage=20&page=p
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

#[derive(
    Clone, Debug, Serialize, Deserialize, Default, 
    Identifiable, Queryable, AsChangeset
)]
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

impl Blog {
    fn update(
        mut self, 
        conn: &PooledConn,
    ) -> ServiceResult<Blog> {
        use crate::schema::blogs::dsl::*;
        let old_blog = blogs.filter(id.eq(self.id))
            .get_result::<Blog>(conn)?;

        let blog_update = diesel::update(
            blogs.filter(id.eq(self.id))
        )
        .set(&self)
        .get_result::<Blog>(conn)?;

        Ok(blog_update)
    }
}

impl Message for Blog {
    type Result = ServiceResult<Blog>;
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
}

impl Message for NewBlog {
    type Result = ServiceResult<Blog>;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct QueryBlog {
    pub id: i32,
    pub method: String, // get|delete
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

    fn del(
        &self, 
        conn: &PooledConn,
    ) -> ServiceResult<Blog> {
        use crate::schema::blogs::dsl::{blogs, id};
        diesel::delete(
            blogs.filter(id.eq(self.id))
        ).execute(conn)?;
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
}

impl QueryBlogs {
    fn get(
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
            _ => {
                blog_list = blogs.order(karma.desc()).limit(10).load::<Blog>(conn)?;
                blog_count = blog_list.len() as i64;
            }
        }
        Ok((blog_list, blog_count))
    }
}

impl Message for QueryBlogs {
    type Result = ServiceResult<(Vec<Blog>, i64)>;
}
