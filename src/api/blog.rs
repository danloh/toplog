
use futures::{future::result, Future};
use actix::{Handler, Message};
use actix_web::{
    web::{Data, Json, Path},
    Error, HttpResponse, ResponseError,
};
use base64::decode;
use diesel::prelude::*;
use diesel::{self, ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::errors::{ServiceError, ServiceResult};
use crate::api::{AuthMsg, UserMsg};
use crate::util::helper::gen_slug;
use crate::{DbAddr, PooledConn};


#[derive(Clone, Debug, Serialize, Deserialize, Identifiable, Queryable)]
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
