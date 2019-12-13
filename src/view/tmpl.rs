
use futures::{Future, future::result};
use actix::{Handler, Message};
use crate::errors::{ServiceError, ServiceResult};
use crate::api::auth::{verify_token, CheckUser, CheckCan};
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
        std::fs::read("www/all-index.html")
            .unwrap_or("Not Found".to_owned().into_bytes()), // handle not found
    )
    .unwrap_or_default();
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(res))
}

// GET /a/{ty} // special: /index, /Misc
//
// static file default, otherwise generate
pub fn index_either(
    db: Data<DbAddr>,
    p: Path<String>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let home_msg = TopicEither { 
        topic: String::from("all"), 
        ty: p.into_inner(),
    };
    
    db.send(home_msg).from_err().and_then(|res| match res {
        Ok(msg) => {
            if msg.status == 201 {
                let mut ctx = tera::Context::new();
                ctx.insert("items", &msg.items.unwrap_or(Vec::new()));
                ctx.insert("blogs", &msg.blogs.unwrap_or(Vec::new()));

                let mesg: Vec<&str> = (&msg.message).split("-").collect();
                let typ = mesg[1];
                ctx.insert("ty", &typ);
                ctx.insert("topic", "all");

                let h = tmpl.render("home.html", &ctx).map_err(|_| {
                    ServiceError::NotFound("failed".into())
                })?;
                let dir = "www/".to_owned() + &msg.message + ".html";
                std::fs::write(dir, h.as_bytes())?;
                Ok(HttpResponse::Ok().content_type("text/html").body(h))
            } else {
                let html = msg.html.unwrap_or_default();
                Ok(HttpResponse::build(http::StatusCode::OK)
                    .content_type("text/html; charset=utf-8")
                    .body(html))
            }
        }
        Err(e) => Ok(e.error_response()),
    })
}

// GET /a/{ty}/dyn // special: /index, /Misc, /newest
//
// response dynamically
pub fn index_dyn(
    db: Data<DbAddr>,
    p: Path<String>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let ty = p.into_inner();
    let home_msg = Topic { 
        topic: String::from("all"), 
        ty,
        page: 1, 
    };
    
    db.send(home_msg).from_err().and_then(|res| match res {
        Ok(msg) => {
            let mut ctx = tera::Context::new();
            ctx.insert("items", &msg.items);
            ctx.insert("blogs", &msg.blogs);

            let mesg: Vec<&str> = (&msg.message).split("-").collect();
            let typ = mesg[1];
            ctx.insert("ty", &typ);
            ctx.insert("topic", "all");

            let h = tmpl.render("home.html", &ctx).map_err(|_| {
                ServiceError::NotFound("failed".into())
            })?;
            let dir = "www/".to_owned() + &msg.message + ".html";
            std::fs::write(dir, h.as_bytes())?;
            Ok(HttpResponse::Ok().content_type("text/html").body(h))
        }
        Err(e) => Ok(e.error_response()),
    })
}

// GET /index
//
// redirect to index_dyn
pub fn dyn_index(
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let p: Path<String> = String::from("index").into();
    index_dyn(db, p)
}

// GET /all/newest
//
// redirect to index_dyn
pub fn index_newest(
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let p: Path<String> = String::from("newest").into();
    index_dyn(db, p)
}

// GET /t/{topic}/{ty}/dyn
//
// response dynamically
pub fn topic_dyn(
    db: Data<DbAddr>,
    p: Path<(String, String)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let pa = p.into_inner();
    let topic = pa.0;
    let ty = pa.1;

    let topic_msg = Topic{ topic, ty, page: 1 };
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
            let mesg: Vec<&str> = (&msg.message).split("-").collect();
            let tpc = mesg[0];
            let typ = mesg[1];
            ctx.insert("ty", typ);
            ctx.insert("topic", tpc);

            let h = tmpl.render("home.html", &ctx).map_err(|_| {
                ServiceError::NotFound("failed".into())
            })?;
            let t_dir = "www/".to_owned() + &msg.message + ".html";
            std::fs::write(&t_dir, h.as_bytes())?;
            Ok(HttpResponse::Ok().content_type("text/html").body(h))
        }
        Err(e) => Ok(e.error_response()),
    })
}

// GET /t/{topic}/{ty}
//
// static file default, otherwise generate
pub fn topic_either(
    db: Data<DbAddr>,
    p: Path<(String, String)>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let pa = p.into_inner();
    let topic = pa.0;
    let ty = pa.1;

    let topic_msg = TopicEither{ topic, ty };
    result(
        topic_msg.validate()
    )
    .from_err()
    .and_then(move |_| db.send(topic_msg).from_err())
    .and_then(|res| match res {
        Ok(msg) => {
            if msg.status == 201 {
                // println!(">> via dyn 201");
                let mut ctx = tera::Context::new();
                ctx.insert("items", &msg.items.unwrap_or(Vec::new()));
                ctx.insert("blogs", &msg.blogs.unwrap_or(Vec::new()));

                let mesg: Vec<&str> = (&msg.message).split("-").collect();
                let tpc = mesg[0];
                let typ = mesg[1];
                ctx.insert("topic", tpc);
                ctx.insert("ty", typ);

                let h = tmpl.render("home.html", &ctx).map_err(|_| {
                    ServiceError::NotFound("failed".into())
                })?;
                let t_dir = "www/".to_owned() + &msg.message + ".html";
                std::fs::write(&t_dir, h.as_bytes())?;
                Ok(HttpResponse::Ok().content_type("text/html").body(h))
            } else {
                let html = msg.html.unwrap_or_default();
                Ok(HttpResponse::build(http::StatusCode::OK)
                    .content_type("text/html; charset=utf-8")
                    .body(html))
            }
        }
        Err(e) => Ok(e.error_response()),
    })
}

// GET /from?by=
//
// response dynamically
pub fn item_from(
    db: Data<DbAddr>,
    bq: Query<ByQuery>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    // extract Query
    let bq_by = bq.into_inner().by.unwrap_or_default();
    use crate::util::helper::de_base64;
    let by = de_base64(&bq_by);

    let topic_msg = Topic { 
        topic: String::from("from"),
        ty: by,
        page: 1, 
    };
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
            let mesg: Vec<&str> = (&msg.message).split("-").collect();
            let by = mesg[1];
            ctx.insert("ty", by);
            ctx.insert("topic", "from");

            let h = tmpl.render("home.html", &ctx).map_err(|_| {
                ServiceError::NotFound("failed".into())
            })?;
            // let t_dir = "www/".to_owned() + &msg.message + ".html";
            // std::fs::write(&t_dir, h.as_bytes())?;
            Ok(HttpResponse::Ok().content_type("text/html").body(h))
        }
        Err(e) => Ok(e.error_response()),
    })
}

// GET /more/{topic}/{ty}?page=&perpage=42
//
pub fn more_item(
    db: Data<DbAddr>,
    p: Path<(String, String)>,
    pq: Query<PageQuery>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let pa = p.into_inner();
    let topic = pa.0;
    let p_ty = pa.1;
    let ty = if topic.trim() == "from" { 
        use crate::util::helper::de_base64;
        de_base64(&p_ty)
    } else { 
        p_ty 
    };
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

            let h = tmpl.render("more_item.html", &ctx).map_err(|_| {
                ServiceError::NotFound("failed".into())
            })?;
            Ok(HttpResponse::Ok().content_type("text/html").body(h))
        }
        Err(e) => Ok(e.error_response()),
    })
}

// GET /item/{slug}
//
pub fn item_view(
    db: Data<DbAddr>,
    p: Path<String>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let slug = p.into_inner();
    use crate::api::item::QueryItem;

    let item_msg = QueryItem { 
        slug, 
        method: String::from("GET"), 
        uname: String::new(),
    };

    db.send(item_msg).from_err().and_then(|res| match res {
        Ok(msg) => {
            let mut ctx = tera::Context::new();
            ctx.insert("item", &msg);
            ctx.insert("ty", "all");
            ctx.insert("topic", "all");

            let h = tmpl.render("item.html", &ctx).map_err(|_| {
                ServiceError::NotFound("failed".into())
            })?;
            Ok(HttpResponse::Ok().content_type("text/html").body(h))
        }
        Err(e) => Ok(e.error_response()),
    })
}

// GET /me/index.html // spa
// try_uri for spa
// 
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

// GET /site/{name}
//
// site: about, help, terms, etc.
pub fn site(p_info: Path<String>) -> Result<HttpResponse, Error> {
    let p = p_info.into_inner();
    let tpl_dir = p + ".html";
    let dir = "www/".to_owned() + &tpl_dir;
    
    let t = tmpl.render(&tpl_dir, &tera::Context::new())
        .map_err(|_| ServiceError::NotFound("404".into()))?;
    std::fs::write(dir, t.as_bytes())?;

    Ok(HttpResponse::build(http::StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(t))
}

pub fn gen_html(
    topic: String,
    ty: String,
    conn: &PooledConn,
) -> ServiceResult<()> {

    let dir = topic.clone() + "-" + &ty;
    let mut ctx = tera::Context::new();
    ctx.insert("ty", &ty);
    ctx.insert("topic", &topic);

    let tp = topic.trim().to_lowercase();

    let (query_item, query_blog) = match tp.trim() {
        "all" => {
            (
                QueryItems::Index(ty, 42, 1),
                QueryBlogs::Index("index".into(), 42, 1)
            )
        }
        "from" => {
            (
                QueryItems::Author(ty.clone(), 42, 1),
                QueryBlogs::Name(ty, 42, 1)
            )
        } 
        _ => {
            (
                QueryItems::Tt(topic.clone(), ty, 42, 1),
                QueryBlogs::Top(topic, 42, 1)
            )
        }
    };

    let (i_list, _) = query_item.get(conn)?;
    let (b_list, _) = query_blog.get(conn)?;

    ctx.insert("items", &i_list);
    ctx.insert("blogs", &b_list);

    let h = tmpl.render("home.html", &ctx).map_err(|_| {
        ServiceError::NotFound("failed".into())
    })?;
    let t_dir = "www/".to_owned() + &dir + ".html";
    std::fs::write(&t_dir, h.as_bytes())?;
            
    Ok(())
}

// GET /generate-staticsite
//
// statify site
pub fn statify_site(
    db: Data<DbAddr>,
    _auth: CheckCan,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let ss = StaticSite();
    db.send(ss).from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(e) => Ok(e.error_response()),
    })
}

// alt statify site
//
// non auth, only for background job,  do not expose!
pub fn statify_site_(
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let ss = StaticSite();
    db.send(ss).from_err().and_then(|res| match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(e) => Ok(e.error_response()),
    })
}

pub struct StaticSite();

impl Message for StaticSite {
    type Result = ServiceResult<String>;
}

impl Handler<StaticSite> for Dba {
    type Result = ServiceResult<String>;

    fn handle(&mut self, ss: StaticSite, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;
        gen_static(conn);

        Ok(String::from("Done"))
    }
}

pub fn gen_static(conn: &PooledConn) -> ServiceResult<()> {
    let tpcs = vec!(
        "all", "Rust", "Go", 
        "TypeScript", "Angular", "Vue", "React", "Dart"
    );
    let typs = vec!(
        "index", "Article", "Book", "Event", "Job", "Media", 
        "Product", "Translate", "Misc"
    );

    for tpc in tpcs {
        for typ in typs.clone() {
            gen_html(tpc.into(), typ.into(), conn);
        }
    }
          
    Ok(())
}

// GET /generate-sitemap
//
pub fn gen_sitemap(_auth: CheckCan)-> ServiceResult<HttpResponse> {
    let mut s_ctx = tera::Context::new();
    s_ctx.insert(
        "lastmod",
        &Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
    );
    let s = tmpl.render("sitemap/sitemap.xml", &s_ctx).map_err(|_| {
        ServiceError::InternalServerError("tmpl failed".into())
    })?;
    std::fs::write("www/sitemap.xml", s.as_bytes())?;

    Ok(HttpResponse::Ok().json("Done".to_owned()))
}

// =====================================================================
// type model
// =====================================================================

// for extrct query param
#[derive(Deserialize, Clone)]
pub struct PageQuery {
    page: i32,
    perpage: i32,
}

// for extrct query param
#[derive(Deserialize, Clone)]
pub struct ByQuery {
    by: Option<String>,
    site: Option<String>,  // TODO
    ord: Option<String>,   // TODO
}

// result struct in response
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ItemBlogMsg {
    pub status: i32,
    pub message: String,
    pub items: Vec<Item>,
    pub blogs: Vec<Blog>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Topic{
    pub topic: String,  // special case: all, from
    pub ty: String,     // special case: index, Misc, newest
    pub page: i32,
}

impl Topic {
    fn validate(&self) -> ServiceResult<()> {
        let tp: &str = &self.topic.trim();
        let ty: &str = &self.ty.trim();
        let page = &self.page;
    
        let check = if tp != "from" { checker(&ty) } else { true };
        if check {
            Ok(())
        } else {
            Err(ServiceError::BadRequest("Invalid Input".into()))
        }
    }
}

impl Message for Topic {
    type Result = ServiceResult<ItemBlogMsg>;
}

// per topic and ty
impl Handler<Topic> for Dba {
    type Result = ServiceResult<ItemBlogMsg>;

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
        let msg = tpc.clone() + "-" + &typ;

        let tp = tpc.trim().to_lowercase();

        let (query_item, query_blog) = match tp.trim() {
            "all" => {
                (
                    QueryItems::Index(typ, 42, t.page),
                    QueryBlogs::Index("index".into(), 42, 1)
                )
            }
            "from" => {
                (
                    QueryItems::Author(typ.clone(), 42, t.page),
                    QueryBlogs::Name(typ, 42, 1)
                )
            } 
            _ => {
                (
                    QueryItems::Tt(tpc.clone(), typ, 42, t.page),
                    QueryBlogs::Top(tpc, 42, 1)
                )
            }
        };

        let (i_list, _) = query_item.get(conn)?;
        let (b_list, _) = query_blog.get(conn)?;

        Ok(ItemBlogMsg {
            status: 201,
            message: msg, // send back the ty and topic info
            items: i_list,
            blogs: b_list,
        })
    }
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TopicEither{
    pub topic: String,
    pub ty: String,
}

impl TopicEither {
    fn validate(&self) -> ServiceResult<()> {
        let tp: &str = &self.topic.trim();
        let ty: &str = &self.ty.trim();
    
        let check = checker(&ty);
        if check {
            Ok(())
        } else {
            Err(ServiceError::BadRequest("Invalid Input".into()))
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EitherMsg {
    pub status: i32,
    pub message: String,
    pub items: Option<Vec<Item>>,
    pub blogs: Option<Vec<Blog>>,
    pub html: Option<String>,
}

impl Message for TopicEither {
    type Result = ServiceResult<EitherMsg>;
}

// per topic and ty
impl Handler<TopicEither> for Dba {
    type Result = ServiceResult<EitherMsg>;

    fn handle(
        &mut self,
        t: TopicEither,
        _: &mut Self::Context,
    ) -> Self::Result {
        let tpc = t.topic;
        let typ = t.ty;
        let msg = tpc.clone() + "-" + &typ;

        let dir = "www/".to_owned() + &msg + ".html";
        let i_html = std::fs::read(dir);

        match i_html {
            Ok(s) => {
                let html = String::from_utf8(s).unwrap_or_default();
                // println!(">> via static");
                Ok(EitherMsg {
                    status: 200,
                    message: msg,
                    items: None,
                    blogs: None,
                    html: Some(html),
                })
            }
            _ => {
                use crate::schema::items::dsl::*;
                use crate::schema::blogs::dsl::{blogs};
                let conn = &self.0.get()?;

                let tp = tpc.trim().to_lowercase();

                let (query_item, query_blog) = if tp == "all" {
                    (
                        QueryItems::Index(typ, 42, 1),
                        QueryBlogs::Index("index".into(), 42, 1)
                    )
                } else {
                    (
                        QueryItems::Tt(tpc.clone(), typ, 42, 1),
                        QueryBlogs::Top(tpc, 42, 1)
                    )
                };

                let (i_list, _) = query_item.get(conn)?;
                let (b_list, _) = query_blog.get(conn)?;

                Ok(EitherMsg {
                    status: 201,
                    message: msg,
                    items: Some(i_list),
                    blogs: Some(b_list),
                    html: None
                })
            }
        }
    }
}

//
// a checker
fn checker(ty: &str) -> bool {
    let check = ty == "index"
        || ty == "Article" 
        || ty == "Book" 
        || ty == "Event" 
        || ty == "Job" 
        || ty == "Media" 
        || ty == "Product" 
        || ty == "Translate"
        || ty == "Misc"
        || ty == "newest";

    check
}
