
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
use crate::api::{*};
use crate::util::helper::gen_slug;
use crate::{DbAddr, PooledConn};


#[derive(Clone, Debug, Serialize, Deserialize, Identifiable, Queryable)]
#[table_name = "articles"]
pub struct Article {
    pub id: i32,
    pub title: String, // unique, person's name
    pub content: String,
    pub author: String,
    pub ty: i32,        // from blog or translate
    pub topic: String,
    pub link: String,
    pub link_host: String,
    pub post_at: NaiveDateTime,
    pub pub_at: NaiveDateTime,
    pub vote: i32,
}
