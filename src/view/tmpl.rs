
use crate::errors::ServiceError;
use crate::api::auth::{verify_token, CheckUser};
use crate::view::TEMPLATE as tmpl;
use crate::DbAddr;
use actix_http::http;
use actix_web::{
    web::{Data, Path},
    Error, HttpResponse, ResponseError,
};
use chrono::{SecondsFormat, Utc};
use futures::Future;

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
    let home_msg = HomeRutsTags();
    db.send(home_msg).from_err().and_then(|res| match res {
        Ok(msg) => {
            let mut ctx = tera::Context::new();
            ctx.insert("ruts", &msg.ruts);
            ctx.insert("tags", &msg.tags);

            let h = tmpl.render("home.html", &ctx).map_err(|_| {
                ServiceError::InternalServerError("template failed".into())
            })?;
            std::fs::write("www/index.html", h.as_bytes())?;
            Ok(HttpResponse::Ok().content_type("text/html").body(h))
        }
        Err(e) => Ok(e.error_response()),
    })
}

