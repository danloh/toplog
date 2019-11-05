
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

use crate::util::helper::gen_slug;
use crate::{DbAddr, PooledConn};


#[derive(Clone, Debug, Serialize, Deserialize, Identifiable, Queryable)]
#[table_name = "issues"]
pub struct Issue {
    pub id: i32,
    pub title: String, // unique, person's name
    pub content: String,
    pub author: String,
    pub post_at: NaiveDateTime,
    pub vote: i32,
    pub is_closed: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Identifiable, Queryable, Insertable)]
#[table_name = "issuelabels"]
pub struct Issue {
    pub issue_id: i32,
    pub lable: String,
    pub label_at: NaiveDateTime,
}
