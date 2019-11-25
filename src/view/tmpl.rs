
use futures::{Future, future::result};
use actix::{Handler, Message};
use crate::errors::{ServiceError, ServiceResult};
use crate::api::auth::{verify_token, CheckUser};
use crate::api::item::{Item, QueryItems};
use crate::api::blog::{Blog, QueryBlogs};
use crate::view::TEMPLATE as tmpl;
use crate::{Dba, DbAddr, PooledConn};
use actix_http::http;
use actix_web::{
    web::{Data, Path, Query},
    Error, HttpResponse, ResponseError,
};
use chrono::{SecondsFormat, Utc};

// GET /
//
pub fn index() -> Result<HttpResponse, Error> {
    let res = String::from_utf8(
        std::fs::read("www/index.html")
            .unwrap_or("Not Found".to_owned().into_bytes()), // handle not found
    )
    .unwrap_or_default();
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(res))
}

// GET /index
//
pub fn index_dyn(
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let home_msg = Home();
    db.send(home_msg).from_err().and_then(|res| match res {
        Ok(msg) => {
            let mut ctx = tera::Context::new();
            ctx.insert("items", &msg.items);
            ctx.insert("blogs", &msg.blogs);

            let h = tmpl.render("home.html", &ctx).map_err(|_| {
                ServiceError::InternalServerError("template failed".into())
            })?;
            std::fs::write("www/index.html", h.as_bytes())?;
            Ok(HttpResponse::Ok().content_type("text/html").body(h))
        }
        Err(e) => Ok(e.error_response()),
    })
}

pub fn spa_index() -> Result<HttpResponse, Error> {
    let res = String::from_utf8(
        std::fs::read("spa/index.html")
            .unwrap_or("Not Found".to_owned().into_bytes()),
    )
    .unwrap_or_default();
    Ok(HttpResponse::build(http::StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(res))
}

#[derive(Deserialize, Clone)]
pub struct PageQuery {
    page: i32,
    perpage: i32,
}

// GET /t/{topic}/{ty}?page=&perpage=42
//
pub fn topic(
    db: Data<DbAddr>,
    p: Path<(String, String)>,
    pq: Query<PageQuery>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let pa = p.into_inner();
    let topic = pa.0;
    let ty = pa.1;
    // extract Query
    let page = std::cmp::max(pq.page, 1);
    let perpage = pq.clone().perpage;

    let topic_msg = Topic{ topic, ty, page };
    result(
        topic_msg.validate()
    )
    .from_err()
    .and_then(move |_| db.send(topic_msg).from_err())
    .and_then(|res| match res {
        Ok(msg) => {
            let mut ctx = tera::Context::new();
            ctx.insert("items", &msg.items);
            ctx.insert("blogs", &msg.blogs);

            let h = tmpl.render("home.html", &ctx).map_err(|_| {
                ServiceError::InternalServerError("template failed".into())
            })?;
            let t_dir = "www/".to_owned() + &msg.message + ".html";
            std::fs::write(&t_dir, h.as_bytes())?;
            Ok(HttpResponse::Ok().content_type("text/html").body(h))
        }
        Err(e) => Ok(e.error_response()),
    })
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Home();

impl Message for Home {
    type Result = ServiceResult<HomeMsg>;
}

// result struct in response rut list
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct HomeMsg {
    pub status: i32,
    pub message: String,
    pub items: Vec<Item>,
    pub blogs: Vec<Blog>,
}

impl Handler<Home> for Dba {
    type Result = ServiceResult<HomeMsg>;

    fn handle(
        &mut self,
        _home: Home,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::items::dsl::*;
        use crate::schema::blogs::dsl::{blogs};
        let conn = &self.0.get()?;

        let (a_list, _) = QueryItems::Index("index".into(), 42, 1).get(conn)?;
        let (b_list, _) = QueryBlogs::Index("index".into(), 42, 1).get(conn)?;

        Ok(HomeMsg {
            status: 201,
            message: String::from("Success"),
            items: a_list,
            blogs: b_list,
        })
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Topic{
    pub topic: String,
    pub ty: String,
    pub page: i32,
}

impl Topic {
    fn validate(&self) -> ServiceResult<()> {
        let tp = &self.topic.trim().to_lowercase();
        let ty = &self.ty.trim().to_lowercase();
        let page = &self.page;
    
        let check = ty == "all" 
        || ty == "article" 
        || ty == "book" 
        || ty == "event" 
        || ty == "podcast" 
        || ty == "translate";

        if check {
            Ok(())
        } else {
            Err(ServiceError::BadRequest("Invalid Input".into()))
        }
    }
}

impl Message for Topic {
    type Result = ServiceResult<TopicMsg>;
}

// result struct in response rut list
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TopicMsg {
    pub status: i32,
    pub message: String,
    pub items: Vec<Item>,
    pub blogs: Vec<Blog>,
}

impl Handler<Topic> for Dba {
    type Result = ServiceResult<TopicMsg>;

    fn handle(
        &mut self,
        t: Topic,
        _: &mut Self::Context,
    ) -> Self::Result {
        use crate::schema::items::dsl::*;
        use crate::schema::blogs::dsl::{blogs};
        let conn = &self.0.get()?;
        let tpc = t.topic;
        let typ = t.ty;

        let tp = tpc.trim().to_lowercase();
        let te = typ.trim().to_lowercase();

        let query_item = if tp == "all" && te == "all" {
            QueryItems::Index("index".into(), 42, 1)
        } else if te == "all" {
            QueryItems::Topic(tpc.clone(), 42, t.page)
        } else {
            QueryItems::Tt(tpc.clone(), typ, 42, t.page)
        };
        let query_blog = QueryBlogs::Topic(tpc, 42, 1);

        let (i_list, _) = query_item.get(conn)?;
        let (b_list, _) = query_blog.get(conn)?;

        Ok(TopicMsg {
            status: 201,
            message: tp + "-" + &te,
            items: i_list,
            blogs: b_list,
        })
    }
}
