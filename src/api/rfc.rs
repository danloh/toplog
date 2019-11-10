
use futures::{future::result, Future};
use actix::{Handler, Message};
use actix_web::{
    web::{Data, Json, Path, Query},
    Error, HttpResponse, ResponseError,
};
use base64::decode;
use diesel::prelude::*;
use diesel::{self, dsl::any, ExpressionMethods, QueryDsl, RunQueryDsl};
use chrono::{NaiveDateTime, Utc};

use crate::errors::{ServiceError, ServiceResult};
use crate::api::{ReqQuery, auth::CheckUser};
use crate::util::helper::gen_slug;
use crate::{Dba, DbAddr, PooledConn};
use crate::schema::{issues, issuelabels};

// POST: /api/issues
// 
pub fn new(
    issue: Json<NewIssue>,
    _auth: CheckUser,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    db.send(issue.into_inner())
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b)),
            Err(err) => Ok(err.error_response()),
        })
}

impl Handler<NewIssue> for Dba {
    type Result = ServiceResult<Issue>;

    fn handle(&mut self, ni: NewIssue, _: &mut Self::Context) -> Self::Result {
        let conn: &PooledConn = &self.0.get().unwrap();
        ni.new(conn)
    }
}

// PUT: /api/issues
// 
pub fn update(
    issue: Json<UpdateIssue>,
    auth: CheckUser,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let up = UpdateIssue {
        author: auth.uname,
        ..issue.into_inner()
    };
    db.send(up)
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b)),
            Err(err) => Ok(err.error_response()),
        })
}

impl Handler<UpdateIssue> for Dba {
    type Result = ServiceResult<Issue>;

    fn handle(&mut self, u: UpdateIssue, _: &mut Self::Context) -> Self::Result {
        let conn: &PooledConn = &self.0.get().unwrap();
        u.update(conn)
    }
}

// GET: /api/issues/{id}
// 
pub fn get(
    qb: Path<String>,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let issue = QueryIssue{
        slug: qb.into_inner(), 
        method: String::from("GET"),
        uname: String::new()
    };
    db.send(issue)
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b)),
            Err(err) => Ok(err.error_response()),
        })
}

// DELETE: /api/issues/{id}
// 
pub fn del(
    qb: Path<String>,
    auth: CheckUser,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let issue = QueryIssue{
        slug: qb.into_inner(), 
        method: String::from("DELETE"),
        uname: auth.uname
    };
    db.send(issue)
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b)),
            Err(err) => Ok(err.error_response()),
        })
}

impl Handler<QueryIssue> for Dba {
    type Result = ServiceResult<Issue>;

    fn handle(&mut self, qb: QueryIssue, _: &mut Self::Context) -> Self::Result {
        let conn: &PooledConn = &self.0.get().unwrap();
        let method: &str = &qb.method.trim();
        if method == "GET" {
            qb.get(conn)
        } else {
            qb.del(conn)
        }
    }
}

// GET: api/issues?per=topic&kw=&perpage=20&page=p
// 
pub fn get_list(
    pq: Query<ReqQuery>,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let perpage = pq.perpage;
    let page = pq.page;
    let kw = pq.clone().kw;
    let per = pq.per.trim();
    let issue = match per {
        "label" => QueryIssues::Label(kw, perpage, page),
        _ => QueryIssues::Index(kw, perpage, page),
    };
    db.send(issue)
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b)),
            Err(err) => Ok(err.error_response()),
        })
}

impl Handler<QueryIssues> for Dba {
    type Result = ServiceResult<(Vec<Issue>, i64)>;

    fn handle(&mut self, qis: QueryIssues, _: &mut Self::Context) -> Self::Result {
        let conn: &PooledConn = &self.0.get().unwrap();
        qis.get(conn)
    }
}

// POST: /api/labelissues
// 
pub fn label_isuue(
    il: Json<NewIssueLabel>,
    auth: CheckUser,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let new_label = NewIssueLabel {
        uname: auth.uname,
        method: String::from("POST"),
        ..il.into_inner()
    };
    db.send(new_label)
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b)),
            Err(err) => Ok(err.error_response()),
        })
}

// DELETE: /api/labelissues
// 
pub fn del_label_isuue(
    il: Json<NewIssueLabel>,
    auth: CheckUser,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let new_label = NewIssueLabel {
        uname: auth.uname,
        method: String::from("DELETE"),
        ..il.into_inner()
    };
    db.send(new_label)
        .from_err()
        .and_then(move |res| match res {
            Ok(b) => Ok(HttpResponse::Ok().json(b)),
            Err(err) => Ok(err.error_response()),
        })
}

impl Handler<NewIssueLabel> for Dba {
    type Result = ServiceResult<IssueLabel>;

    fn handle(&mut self, ni: NewIssueLabel, _: &mut Self::Context) -> Self::Result {
        let conn: &PooledConn = &self.0.get().unwrap();
        let method: &str = ni.method.trim();
        if method == "POST" {
            ni.new(conn)
        } else {
            ni.del(conn)
        }
    }
}


// =================================================================================
// =================================================================================
// Model
// =================================================================================

#[derive(Clone, Debug, Serialize, Deserialize, Identifiable, Queryable)]
#[table_name = "issues"]
pub struct Issue {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub author: String,
    pub post_at: NaiveDateTime,
    pub vote: i32,
    pub is_closed: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable)]
#[table_name = "issues"]
pub struct NewIssue {
    pub title: String,
    pub slug: String,
    pub content: String,
    pub author: String,
}

impl NewIssue {
    fn new(
        self, 
        conn: &PooledConn,
    ) -> ServiceResult<Issue> {
        use crate::schema::issues::dsl::issues;
        let i_slug = gen_slug(&self.title);
        let new_issue = NewIssue {
            slug: i_slug,
            ..self
        };
        let issue_new = diesel::insert_into(issues)
            .values(&new_issue)
            .on_conflict_do_nothing()
            .get_result::<Issue>(conn)?;

        Ok(issue_new)
    }
}

impl Message for NewIssue {
    type Result = ServiceResult<Issue>;
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, AsChangeset)]
#[table_name = "issues"]
pub struct UpdateIssue {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub author: String,
}

impl UpdateIssue {
    fn update(
        mut self,
        conn: &PooledConn,
    ) -> ServiceResult<Issue> {
        use crate::schema::issues::dsl::*;
        let old = issues.filter(id.eq(self.id))
            .get_result::<Issue>(conn)?;
        // check permission
        let admin_env = dotenv::var("ADMIN").unwrap_or("".to_string());
        let check_permission: bool = 
            &self.author == &admin_env || &self.author == &old.author;
        if !check_permission {
            return Err(ServiceError::Unauthorized);
        }

        // check if anything chenged
        let check_changed: bool = self.title.trim() != old.title.trim()
            || self.content.trim() != old.content.trim();
        if !check_changed {
            return Err(ServiceError::BadRequest("Nothing Changed".to_owned()));
        }

        // check if title changed
        let check_title_changed: bool = &self.title.trim() == &old.title.trim();
        let i_slug = if check_title_changed {
            (&old).slug.to_owned()
        } else {
            gen_slug(&self.title)
        };
        let up = UpdateIssue {
            slug: i_slug,
            author: (&old).author.to_owned(),
            ..self
        };

        let issue_update = 
            diesel::update(&old).set(&up).get_result::<Issue>(conn)?;
        Ok(issue_update)
    }
}

impl Message for UpdateIssue {
    type Result = ServiceResult<Issue>;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct QueryIssue {
    pub slug: String,
    pub method: String, // get|delete
    pub uname: String,
}

impl QueryIssue {
    fn get(
        &self, 
        conn: &PooledConn,
    ) -> ServiceResult<Issue> {
        use crate::schema::issues::dsl::{issues, slug};
        let issue = issues.filter(&slug.eq(&self.slug)).get_result::<Issue>(conn)?;
        Ok(issue)
    }

    fn del(
        &self, 
        conn: &PooledConn,
    ) -> ServiceResult<Issue> {
        use crate::schema::issues::dsl::{issues, slug};
        // check permission
        let admin_env = dotenv::var("ADMIN").unwrap_or("".to_string());
        let check_permission: bool = &self.uname == &admin_env;
        if !check_permission {
            return Err(ServiceError::Unauthorized);
        }

        let issue = diesel::delete(
            issues.filter(&slug.eq(&self.slug))
        ).get_result::<Issue>(conn)?;
        Ok(issue)
    }
}

impl Message for QueryIssue {
    type Result = ServiceResult<Issue>;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum QueryIssues {
    Index(String, i32, i32),
    Label(String, i32, i32),
}

impl QueryIssues {
    fn get(
        self, 
        conn: &PooledConn,
    ) -> ServiceResult<(Vec<Issue>, i64)> {
        use crate::schema::issues::dsl::*;
        let mut issue_list: Vec<Issue> = Vec::new();
        let mut issue_count = 0;
        match self {
            QueryIssues::Label(t, o, p) => {
                use crate::schema::issuelabels::dsl::{issuelabels, label, issue_id};
                let p_o = std::cmp::max(0, p-1);
                let query = issuelabels.filter(label.eq(t));
                let issue_ids = query.clone()
                    .select(issue_id)
                    .order(issue_id.desc())
                    .limit(o.into())
                    .offset((o * p_o).into())
                    .load::<i32>(conn)?;
                
                issue_list = issues.filter(&id.eq(any(&issue_ids)))
                    .order(post_at.desc())
                    .load::<Issue>(conn)?;
                issue_count = query.count().get_result(conn)?;
            }
            _ => {
                issue_list = issues
                    .order(post_at.desc()).limit(42).load::<Issue>(conn)?;
                issue_count = issue_list.len() as i64;
            }
        }
        Ok((issue_list, issue_count))
    }
}

impl Message for QueryIssues {
    type Result = ServiceResult<(Vec<Issue>, i64)>;
}


#[derive(Clone, Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "issuelabels"]
pub struct IssueLabel {
    pub issue_id: i32,
    pub label: String,
    pub label_at: NaiveDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewIssueLabel {
    pub issue_id: i32,
    pub label: String,
    pub uname: String,
    pub method: String,
}

impl NewIssueLabel {
    fn new(
        self, 
        conn: &PooledConn,
    ) -> ServiceResult<IssueLabel> {
        use crate::schema::issuelabels::dsl::{issuelabels, issue_id, label};
        // check permission
        let admin_env = dotenv::var("ADMIN").unwrap_or("".to_string());
        let check_permission: bool = self.uname == admin_env;
        if !check_permission {
            return Err(ServiceError::Unauthorized);
        }

        let issue_new = diesel::insert_into(issuelabels)
            .values((
                &issue_id.eq(&self.issue_id),
                &label.eq(&self.label)
            ))
            .on_conflict_do_nothing()
            .get_result::<IssueLabel>(conn)?;

        Ok(issue_new)
    }

    fn del(
        &self, 
        conn: &PooledConn,
    ) -> ServiceResult<IssueLabel> {
        use crate::schema::issuelabels::dsl::{issuelabels, issue_id, label};
        // check permission
        let admin_env = dotenv::var("ADMIN").unwrap_or("".to_string());
        let check_permission: bool = self.uname == admin_env;
        if !check_permission {
            return Err(ServiceError::Unauthorized);
        }

        let issuelabel = diesel::delete(
            issuelabels
            .filter(&issue_id.eq(&self.issue_id))
            .filter(&label.eq(&self.label))
        ).get_result::<IssueLabel>(conn)?;
        Ok(issuelabel)
    }
}

impl Message for NewIssueLabel {
    type Result = ServiceResult<IssueLabel>;
}
