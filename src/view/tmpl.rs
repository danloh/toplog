
//use futures::{Future};
use actix::{Handler, Message};
use crate::errors::{ServiceError, ServiceResult};
use crate::api::auth::{verify_token, QueryUser, CheckUser, CheckCan};
use crate::api::item::{Item, QueryItems};
use crate::api::blog::{Blog, QueryBlogs};
use crate::{Dba, DbAddr, PooledConn};
use actix_web::{
    web::{Data, Path, Query},
    Error, HttpResponse, ResponseError,
    Result
};
use chrono::{SecondsFormat, Utc};
use crate::view::{
    Template, TY_VEC, TOPIC_VEC, 
    CollectionTmpl, ItemTmpl, ItemsTmpl, AboutTmpl, ProfileTmpl,
    SiteMapTmpl
};

// for extrct query param
// 
#[derive(Deserialize, Clone)]
pub struct PageQuery {
    page: i32,
    perpage: i32,
}

#[derive(Deserialize, Clone)]
pub struct PerQuery {
    ty: Option<String>,    // Article|Book...
    tpc: Option<String>,   // topic: Rust|Golang...
    ord: Option<String>,   // order
}

#[derive(Deserialize, Clone)]
pub struct FromQuery {
    by: Option<String>,
    site: Option<String>,  // TODO
    ord: Option<String>,   // TODO
}


// GET /index  // reserve
//
pub async fn dyn_index(
    db: Data<DbAddr>,
) -> ServiceResult<HttpResponse> {
    let q = Query(PerQuery{
        ty: None,
        tpc: None,
        ord: None
    });
    collection_dyn(db, q).await
}

// GET /collection?ty=&tpc=&ord=
//
// static file default, otherwise generate
pub async fn collection_either(
    db: Data<DbAddr>,
    q: Query<PerQuery>,
) -> ServiceResult<HttpResponse> {
    let pq = q.clone();
    let ty = pq.ty.unwrap_or(String::from("index"));
    let mut tpc = pq.tpc.unwrap_or(String::from("all"));
    if tpc.trim() == "from" {
        tpc = String::from("all");
    }
    let dir = "www/collection/".to_owned() + &tpc +"-" + &ty + ".html";
    let s_html = std::fs::read(dir);
    match s_html {
        Ok(s) => {
            let html = String::from_utf8(s).unwrap_or_default();
            return Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(html)
            )
        }
        _ => {
            return collection_dyn(db, q).await
        }
    }
}

pub async fn collection_dyn(
    db: Data<DbAddr>,
    q: Query<PerQuery>,
) -> ServiceResult<HttpResponse> {
    let pq = q.clone();
    let ty = pq.ty.unwrap_or(String::from("index"));
    let mut topic = pq.tpc.unwrap_or(String::from("all"));
    if topic.trim() == "from" {
        topic = String::from("all");
    }
    let tpc_msg = Topic { 
        topic: topic.clone(), 
        ty: ty.clone(),
        page: 1,
    };
    
    let res = db.send(tpc_msg).await?;
    match res {
        Ok(msg) => {
            // let mesg: Vec<&str> = (&msg.message).split("-").collect();
            // let tp = mesg[0];
            // let typ = mesg[1];

            let tmpl = CollectionTmpl {
                ty: &ty,
                topic: &topic,
                items: &msg.items,
                blogs: &msg.blogs,
                tys: &TY_VEC,
            };

            let h = tmpl.render().unwrap_or("Rendering failed".into());
            let dir = "www/collection/".to_owned() + &msg.message + ".html";
            std::fs::write(dir, h.as_bytes())?;
            Ok(HttpResponse::Ok().content_type("text/html").body(h))
        }
        Err(e) => Ok(e.error_response()),
    }
}

// GET /from?by=
//
// response dynamically
pub async fn item_from(
    db: Data<DbAddr>,
    bq: Query<FromQuery>,
) -> ServiceResult<HttpResponse> {
    // extract Query
    let bq_by = bq.into_inner().by.unwrap_or_default();
    use crate::util::helper::de_base64;
    let by = de_base64(&bq_by);

    let topic_msg = Topic { 
        topic: String::from("from"),
        ty: by,
        page: 1, 
    };
    
    if let Err(e) = topic_msg.validate() {
        return Ok(e.error_response());
    }

    let res = db.send(topic_msg).await?;
    match res {
        Ok(msg) => {
            let mesg: Vec<&str> = (&msg.message).split("-").collect();
            let by = mesg[1];

            let by_tmpl = CollectionTmpl {
                ty: by,
                topic: "from",
                items: &msg.items,
                blogs: &msg.blogs,
                tys: &TY_VEC,
            };

            let h = by_tmpl.render().unwrap_or("Rendering failed".into());
            // let t_dir = "www/collection/".to_owned() + &msg.message + ".html";
            // std::fs::write(&t_dir, h.as_bytes())?;
            Ok(HttpResponse::Ok().content_type("text/html").body(h))
        }
        Err(e) => Ok(e.error_response()),
    }
}

// GET /moreitems/{topic}/{ty}?page=&perpage=42
//
pub async fn more_item(
    db: Data<DbAddr>,
    p: Path<(String, String)>,
    pq: Query<PageQuery>,
) -> ServiceResult<HttpResponse> {
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

    if let Err(e) = topic_msg.validate() {
        return Ok(e.error_response());
    }

    let res =  db.send(topic_msg).await?;
    match res {
        Ok(msg) => {
            let mesg: Vec<&str> = (&msg.message).split("-").collect();
            let tpc = mesg[0];
            //let typ = mesg[1];

            let items_tmpl = ItemsTmpl {
                items: &msg.items,
                topic: tpc,
            };

            let h = items_tmpl.render().unwrap_or("Rendering failed".into());

            Ok(HttpResponse::Ok().content_type("text/html").body(h))
        }
        Err(e) => Ok(e.error_response()),
    }
}

// GET /item/{id}
//
pub async fn item_view(
    db: Data<DbAddr>,
    p: Path<i32>,
) -> ServiceResult<HttpResponse> {
    let id = p.into_inner();
    use crate::api::item::QueryItem;

    let item_msg = QueryItem { 
        id, 
        method: String::from("GET"), 
        uname: String::new(),
    };

    let res = db.send(item_msg).await?; 
    match res {
        Ok(msg) => {
            let item_tmpl = ItemTmpl {
                item: &msg,
            };

            let h = item_tmpl.render().unwrap_or("Rendering failed".into());
            Ok(HttpResponse::Ok().content_type("text/html").body(h))
        }
        Err(e) => Ok(e.error_response()),
    }
}

// profile
// GET /@{uname}
//
pub async fn profile(
    db: Data<DbAddr>,
    auth: CheckUser,
    name: Path<String>,
) -> ServiceResult<HttpResponse> {
    let uname = name.into_inner();
    let authuname = auth.uname;
    let is_self = if uname == authuname { true } else { false };
    let user = db.send(QueryUser { uname }).await??;

    let profile = ProfileTmpl {
        user: &user,
        is_self,
    };
    let s = profile.render().unwrap_or("Rendering failed".into());

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(s)
    )
}

// GET /site/{name}
//
// site: about, help, terms, etc.
pub async fn site(p_info: Path<String>) -> ServiceResult<HttpResponse>{
    let p = p_info.into_inner();
    let tpl_dir = p + ".html";
    let dir = "www/".to_owned() + &tpl_dir;
    let about = AboutTmpl();
    let s = about.render().unwrap_or("Rendering failed".into());
    std::fs::write(dir, s.as_bytes())?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(s)
    )
}

// GET /api/generate-staticsite
//
// just del cached file 
pub async fn statify_site(
    db: Data<DbAddr>,
    _auth: CheckCan,
) -> ServiceResult<HttpResponse> {
    del_dir("www/collection");

    Ok(HttpResponse::Ok().json(String::from("done")))
}

// GET /generate-sitemap
//
pub async fn gen_sitemap(_auth: CheckCan)-> ServiceResult<HttpResponse> {
    let sitemap_tmpl = SiteMapTmpl {
        tys: &TY_VEC,
        topics: &TOPIC_VEC,
        lastmod: &Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true),
    };

    let h = sitemap_tmpl.render().unwrap_or("Rendering failed".into());
    std::fs::write("www/sitemap.xml", h.as_bytes())?;

    Ok(HttpResponse::Ok().json("Done".to_owned()))
}

// DELETE /api/stfile/{t-t}  // any potential issue??
//
// delete static file.
pub async fn del_static_file(
    p: Path<String>
) -> ServiceResult<HttpResponse> {
    del_html(&p.into_inner())?;

    Ok(HttpResponse::Ok().json(String::from("delete")))
}


// del static html
pub fn del_html(name: &str) -> ServiceResult<()> {
    let to_del_html = "www/".to_owned() + name + ".html";
    std::fs::remove_file(to_del_html)?;
    Ok(())
}

// del static dir and re-create dir
pub fn del_dir(dir_name: &str) -> ServiceResult<()> {
    std::fs::remove_dir_all(dir_name)?;
    std::fs::create_dir(dir_name)?;
    Ok(())
}

// =====================================================================
// type model
// =====================================================================

// result struct in response
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ItemBlogMsg {
    pub status: i32,
    pub message: String,  // send back topic-ty
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
            message: msg, // send back the topic-ty info
            items: i_list,
            blogs: b_list,
        })
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
        || ty == "Project" 
        || ty == "Translate"
        || ty == "Misc"
        || ty == "newest";

    check
}
