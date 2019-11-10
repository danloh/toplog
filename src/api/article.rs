
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
use crate::schema::{articles};

// POST: /api/articles
// 
pub fn new(
    article: Json<NewArticle>,
    _auth: CheckUser,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    db.send(article.into_inner())
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b)),
            Err(err) => Ok(err.error_response()),
        })
}

impl Handler<NewArticle> for Dba {
    type Result = ServiceResult<Article>;

    fn handle(&mut self, na: NewArticle, _: &mut Self::Context) -> Self::Result {
        let conn: &PooledConn = &self.0.get().unwrap();
        na.new(conn)
    }
}

// PUT: /api/articles
// 
pub fn update(
    article: Json<UpdateArticle>,
    _auth: CheckUser,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    db.send(article.into_inner())
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b)),
            Err(err) => Ok(err.error_response()),
        })
}

impl Handler<UpdateArticle> for Dba {
    type Result = ServiceResult<Article>;

    fn handle(&mut self, b: UpdateArticle, _: &mut Self::Context) -> Self::Result {
        let conn: &PooledConn = &self.0.get().unwrap();
        b.update(conn)
    }
}

// GET: /api/articles/{slug}
// 
pub fn get(
    qb: Path<String>,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let article = QueryArticle{
        slug: qb.into_inner(), 
        method: String::from("GET"),
        uname: String::new()
    };
    db.send(article)
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b)),
            Err(err) => Ok(err.error_response()),
        })
}

// DELETE: /api/articles/{slug}
// 
pub fn del(
    qb: Path<String>,
    auth: CheckUser,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let article = QueryArticle{
        slug: qb.into_inner(), 
        method: String::from("DELETE"),
        uname: auth.uname
    };
    db.send(article)
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b)),
            Err(err) => Ok(err.error_response()),
        })
}

impl Handler<QueryArticle> for Dba {
    type Result = ServiceResult<Article>;

    fn handle(&mut self, qb: QueryArticle, _: &mut Self::Context) -> Self::Result {
        let conn: &PooledConn = &self.0.get().unwrap();
        let method: &str = &qb.method.trim();
        if method == "GET" {
            qb.get(conn)
        } else {
            qb.del(conn)
        }
    }
}

// GET: api/articles?per=topic|author&kw=&perpage=42&page=p
// 
pub fn get_list(
    pq: Query<ReqQuery>,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let perpage = pq.perpage;
    let page = pq.page;
    let kw = pq.clone().kw;
    let per = pq.per.trim();
    let article = match per {
        "topic" => QueryArticles::Topic(kw, perpage, page),
        "author" => QueryArticles::Author(kw, perpage, page),
        _ => QueryArticles::Index(kw, perpage, page),
    };
    db.send(article)
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b)),
            Err(err) => Ok(err.error_response()),
        })
}

impl Handler<QueryArticles> for Dba {
    type Result = ServiceResult<(Vec<Article>, i64)>;

    fn handle(&mut self, qbs: QueryArticles, _: &mut Self::Context) -> Self::Result {
        let conn: &PooledConn = &self.0.get().unwrap();
        qbs.get(conn)
    }
}


// =================================================================================
// =================================================================================
// Model
// =================================================================================

#[derive(Clone, Debug, Serialize, Deserialize, Identifiable, Queryable)]
#[table_name = "articles"]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub author: String,
    pub ty: i32,        // from article or translate
    pub language: String,
    pub topic: String,
    pub link: String,
    pub link_host: String,
    pub post_by: String,
    pub post_at: NaiveDateTime,
    pub pub_at: NaiveDateTime,
    pub vote: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "articles"]
pub struct NewArticle {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub author: String,
    pub ty: i32,        // from article or translate
    pub language: String,
    pub topic: String,
    pub link: String,
    pub link_host: String,
    pub post_by: String,
}

impl NewArticle {
    fn new(
        self, 
        conn: &PooledConn,
    ) -> ServiceResult<Article> {
        use crate::schema::articles::dsl::articles;
        let a_slug = gen_slug(&self.title);
        let new_article = NewArticle {
            slug: a_slug,
            ..self
        };
        let article_new = diesel::insert_into(articles)
            .values(&new_article)
            .on_conflict_do_nothing()
            .get_result::<Article>(conn)?;

        Ok(article_new)
    }
}

impl Message for NewArticle {
    type Result = ServiceResult<Article>;
}


#[derive(Clone, Debug, Serialize, Deserialize, AsChangeset)]
#[table_name = "articles"]
pub struct UpdateArticle {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub author: String,
    pub ty: i32,        // from article or translate
    pub language: String,
    pub topic: String,
    pub link: String,
    pub post_by: String,
}

impl UpdateArticle {
    fn update(
        mut self, 
        conn: &PooledConn,
    ) -> ServiceResult<Article> {
        use crate::schema::articles::dsl::*;
        let old = articles.filter(id.eq(self.id))
            .get_result::<Article>(conn)?;
        // check if title changed
        let check_title_changed: bool = &self.title.trim() == &old.title.trim();
        let a_slug = if check_title_changed {
            (&old).slug.to_owned()
        } else {
            gen_slug(&self.title)
        };
        let up = UpdateArticle {
            slug: a_slug,
            ..self
        };

        let article_update = diesel::update(&old)
            .set(up)
            .get_result::<Article>(conn)?;

        Ok(article_update)
    }
}

impl Message for UpdateArticle {
    type Result = ServiceResult<Article>;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct QueryArticle {
    pub slug: String,
    pub method: String, // get|delete
    pub uname: String,
}

impl QueryArticle {
    fn get(
        &self, 
        conn: &PooledConn,
    ) -> ServiceResult<Article> {
        use crate::schema::articles::dsl::{articles, slug};
        let article = articles.filter(slug.eq(&self.slug))
            .get_result::<Article>(conn)?;
        Ok(article)
    }

    fn del(
        &self, 
        conn: &PooledConn,
    ) -> ServiceResult<Article> {
        use crate::schema::articles::dsl::{articles, slug};
        // check permission
        let admin_env = dotenv::var("ADMIN").unwrap_or("".to_string());
        let check_permission: bool = self.uname == admin_env;
        if !check_permission {
            return Err(ServiceError::Unauthorized);
        }

        let article = diesel::delete(articles.filter(slug.eq(&self.slug)))
            .get_result::<Article>(conn)?;
        Ok(article)
    }
}

impl Message for QueryArticle {
    type Result = ServiceResult<Article>;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum QueryArticles {
    Index(String, i32, i32),
    Topic(String, i32, i32), // topic, perpage, page
    Author(String, i32, i32),  // aname, ..
}

impl QueryArticles {
    fn get(
        self, 
        conn: &PooledConn,
    ) -> ServiceResult<(Vec<Article>, i64)> {
        use crate::schema::articles::dsl::*;
        let mut article_list: Vec<Article> = Vec::new();
        let mut article_count = 0;
        match self {
            QueryArticles::Topic(t, o, p) => {
                let query = articles.filter(topic.eq(t));
                let p_o = std::cmp::max(0, p-1);
                article_count = query.clone().count().get_result(conn)?;
                article_list = query
                    .order(vote.desc())
                    .limit(o.into())
                    .offset((o * p_o).into())
                    .load::<Article>(conn)?;
            }
            QueryArticles::Author(a, o, p) => {
                let query = articles.filter(author.eq(a));
                let p_o = std::cmp::max(0, p-1);
                article_count = query.clone().count().get_result(conn)?;
                article_list = query
                    .order(vote.desc())
                    .limit(o.into())
                    .offset((o * p_o).into())
                    .load::<Article>(conn)?;
            }
            _ => {
                article_list = articles
                    .order(vote.desc())
                    .limit(42)
                    .load::<Article>(conn)?;
                article_count = article_list.len() as i64;
            }
        }
        Ok((article_list, article_count))
    }
}

impl Message for QueryArticles {
    type Result = ServiceResult<(Vec<Article>, i64)>;
}
