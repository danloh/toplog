
use futures::Future;
use actix::{Handler, Message};
use crate::errors::{ServiceError, ServiceResult};
use crate::api::auth::{verify_token, CheckUser};
use crate::api::item::{Item, QueryItems};
use crate::api::blog::{Blog, QueryBlogs};
use crate::view::TEMPLATE as tmpl;
use crate::{Dba, DbAddr, PooledConn};
use actix_http::http;
use actix_web::{
    web::{Data, Path},
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
