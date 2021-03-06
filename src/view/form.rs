pub use askama::Template;
use actix_web::{
    web::{Data, Path, Query},
    Error, HttpResponse, ResponseError,
    Result
};
use crate::errors::{ServiceError, ServiceResult};
use crate::api::auth::{CheckAuth, CheckUser, CheckCan, CheckCsrf, generate_token};

#[derive(Template)]
#[template(path = "auth_form.html")]
pub struct AuthFormTmpl<'a> {
    pub csrf_tok: &'a str,
    pub uname: &'a str,
}

pub async fn auth_form(
    auth: CheckAuth,
) -> Result<HttpResponse, ServiceError> {
    let uname = auth.0;
    let tok = generate_token(&uname, "auth@uname", 1*24*3600)?;
    let af = AuthFormTmpl {
        csrf_tok: &tok,
        uname: &uname,
    };
    let s = af.render().unwrap_or("Rendering failed".into());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(s)
    )
}

#[derive(Template)]
#[template(path = "0_item_form.html")]
pub struct ItemFormTmpl<'a> {
    pub csrf_tok: &'a str,
    pub is_new: bool,
}

// GET /newitem
//
pub async fn new_item(
    auth: CheckUser,
) -> Result<HttpResponse, ServiceError> {
    let tok = generate_token(&auth.uname, "item@new", 1*24*3600)?;
    let ns = ItemFormTmpl {
        csrf_tok: &tok,
        is_new: true,
    };
    let s = ns.render().unwrap_or("Rendering failed".into());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(s)
    )
}

// GET /edititem?id=
//
pub async fn edit_item(
    auth: CheckUser,
) -> Result<HttpResponse, ServiceError> {
    let tok = generate_token(&auth.uname, "item@edit", 1*24*3600)?;
    let ns = ItemFormTmpl {
        csrf_tok: &tok,
        is_new: false,
    };
    let s = ns.render().unwrap_or("Rendering failed".into());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(s)
    )
}

#[derive(Template)]
#[template(path = "0_blog_form.html")]
pub struct BlogFormTmpl<'a> {
    pub csrf_tok: &'a str,
    pub is_new: bool,
}

// GET /newblog
//
pub async fn new_blog(
    auth: CheckUser,
) -> Result<HttpResponse, ServiceError> {
    let tok = generate_token(&auth.uname, "blog@new", 1*24*3600)?;
    let ns = BlogFormTmpl {
        csrf_tok: &tok,
        is_new: true,
    };
    let s = ns.render().unwrap_or("Rendering failed".into());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(s)
    )
}

// GET /editblog?id=
//
pub async fn edit_blog(
    auth: CheckUser,
) -> Result<HttpResponse, ServiceError> {
    let tok = generate_token(&auth.uname, "blog@edit", 1*24*3600)?;
    let ns = BlogFormTmpl {
        csrf_tok: &tok,
        is_new: false,
    };
    let s = ns.render().unwrap_or("Rendering failed".into());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(s)
    )
}

#[derive(Template)]
#[template(path = "0_submit_form.html")]
pub struct SubmitToTmpl<'a> {
    pub csrf_tok: &'a str,
}

pub async fn submit_to (
    auth: CheckAuth,
) -> Result<HttpResponse, ServiceError> {
    let tok = generate_token(&auth.0, "submit@item", 1*24*3600)?;
    let na = SubmitToTmpl {
        csrf_tok: &tok,
    };
    let s = na.render().unwrap_or("Rendering failed".into());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(s)
    )
}
