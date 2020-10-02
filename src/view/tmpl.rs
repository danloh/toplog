
//use futures::{Future};
use actix::{Handler, Message};
use crate::errors::{ServiceError, ServiceResult};
use crate::api::auth::{verify_token, CheckUser, CheckCan};
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
    IndexTmpl, ItemTmpl, ItemsTmpl, AboutTmpl, SiteMapTmpl
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

// GET /
//
pub async fn index() -> ServiceResult<HttpResponse> {
    let res = String::from_utf8(
        std::fs::read("www/all-index.html")
            .unwrap_or("Not Found".to_owned().into_bytes()), // handle not found
    )
    .unwrap_or_default();
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(res)
    )
}

// GET /a/{ty} // special: /index, /Misc
//
// static file default, otherwise generate
pub async fn index_either(
    db: Data<DbAddr>,
    p: Path<String>,
) -> ServiceResult<HttpResponse> {
    let ty = p.clone();
    let dir = "www/".to_owned() + "all-" + &ty + ".html"; 
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
            return index_dyn(db, p).await
        }
    }
}

// GET /a/{ty}/dyn // special: /index, /Misc, /newest
//
// response dynamically
pub async fn index_dyn(
    db: Data<DbAddr>,
    p: Path<String>,
) -> ServiceResult<HttpResponse> {
    let ty = p.into_inner();
    let home_msg = Topic { 
        topic: String::from("all"), 
        ty,
        page: 1,
    };
    
    let res = db.send(home_msg).await?;
    match res {
        Ok(msg) => {
            let mesg: Vec<&str> = (&msg.message).split("-").collect();
            let typ = mesg[1];

            let index_tmpl = IndexTmpl {
                ty: &typ,
                topic: "all",
                items: &msg.items,
                blogs: &msg.blogs,
                tys: &TY_VEC,
            };

            let h = index_tmpl.render().unwrap_or("Rendering failed".into());
            let dir = "www/".to_owned() + &msg.message + ".html";
            std::fs::write(dir, h.as_bytes())?;
            Ok(HttpResponse::Ok().content_type("text/html").body(h))
        }
        Err(e) => Ok(e.error_response()),
    }
}

// GET /index
//
// redirect to index_dyn
pub async fn dyn_index(
    db: Data<DbAddr>,
) -> ServiceResult<HttpResponse> {
    let p: Path<String> = String::from("index").into();
    index_dyn(db, p).await
}

// GET /all/newest
//
// redirect to index_dyn
pub async fn index_newest(
    db: Data<DbAddr>,
) -> ServiceResult<HttpResponse> {
    let p: Path<String> = String::from("newest").into();
    index_dyn(db, p).await
}

// GET /t/{topic}/{ty}/dyn
//
// response dynamically
pub async fn topic_dyn(
    db: Data<DbAddr>,
    p: Path<(String, String)>,
) -> ServiceResult<HttpResponse> {
    let pa = p.into_inner();
    let topic = pa.0;
    let ty = pa.1;

    let topic_msg = Topic{ topic, ty, page: 1 };
    
    if let Err(e) = topic_msg.validate() {
        return Ok(e.error_response());
    }
    
    let res = db.send(topic_msg).await?;
    match res {
        Ok(msg) => {
            let mesg: Vec<&str> = (&msg.message).split("-").collect();
            let tpc = mesg[0];
            let typ = mesg[1];

            let tpc_tmpl = IndexTmpl {
                ty: typ,
                topic: tpc,
                items: &msg.items,
                blogs: &msg.blogs,
                tys: &TY_VEC,
            };

            let h = tpc_tmpl.render().unwrap_or("Rendering failed".into());
            let t_dir = "www/".to_owned() + &msg.message + ".html";
            std::fs::write(&t_dir, h.as_bytes())?;
            Ok(HttpResponse::Ok().content_type("text/html").body(h))
        }
        Err(e) => Ok(e.error_response()),
    }
}

// GET /t/{topic}/{ty}
//
// static file default, otherwise generate
pub async fn topic_either(
    db: Data<DbAddr>,
    p: Path<(String, String)>,
) -> ServiceResult<HttpResponse> {
    let pa = p.clone();
    let topic = pa.0;
    let ty = pa.1;

    let dir = "www/".to_owned() + &topic + "-" + &ty + ".html";
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
            return topic_dyn(db, p).await
        }
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

            let by_tmpl = IndexTmpl {
                ty: by,
                topic: "from",
                items: &msg.items,
                blogs: &msg.blogs,
                tys: &TY_VEC,
            };

            let h = by_tmpl.render().unwrap_or("Rendering failed".into());
            // let t_dir = "www/".to_owned() + &msg.message + ".html";
            // std::fs::write(&t_dir, h.as_bytes())?;
            Ok(HttpResponse::Ok().content_type("text/html").body(h))
        }
        Err(e) => Ok(e.error_response()),
    }
}

// GET /more/{topic}/{ty}?page=&perpage=42
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

// GET /item/{slug}
//
pub async fn item_view(
    db: Data<DbAddr>,
    p: Path<String>,
) -> ServiceResult<HttpResponse> {
    let slug = p.into_inner();
    use crate::api::item::QueryItem;

    let item_msg = QueryItem { 
        slug, 
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

// generate static html
pub fn gen_html(
    topic: String,
    ty: String,
    conn: &PooledConn,
) -> ServiceResult<()> {

    let dir = topic.clone() + "-" + &ty;
    let tp = topic.trim().to_lowercase();

    let (query_item, query_blog) = match tp.trim() {
        "all" => {
            (
                QueryItems::Index(ty.clone(), 42, 1),
                QueryBlogs::Index("index".into(), 42, 1)
            )
        }
        "from" => {
            (
                QueryItems::Author(ty.clone(), 42, 1),
                QueryBlogs::Name(ty.clone(), 42, 1)
            )
        } 
        _ => {
            (
                QueryItems::Tt(topic.clone(), ty.clone(), 42, 1),
                QueryBlogs::Top(topic.clone(), 42, 1)
            )
        }
    };

    let (i_list, _) = query_item.get(conn)?;
    let (b_list, _) = query_blog.get(conn)?;

    let by_tmpl = IndexTmpl {
        ty: &ty,
        topic: &topic,
        items: &i_list,
        blogs: &b_list,
        tys: &TY_VEC,
    };

    let h = by_tmpl.render().unwrap_or("Rendering failed".into());
    let t_dir = "www/".to_owned() + &dir + ".html";
    std::fs::write(&t_dir, h.as_bytes())?;
            
    Ok(())
}

// del static html
pub fn del_html(name: &str) -> ServiceResult<()> {
    let to_del_html = "www/".to_owned() + name + ".html";
    std::fs::remove_file(to_del_html)?;
    Ok(())
}

// GET /api/generate-staticsite
//
// statify site
pub async fn statify_site(
    db: Data<DbAddr>,
    _auth: CheckCan,
) -> ServiceResult<HttpResponse> {
    let ss = StaticSite();
    let res = db.send(ss).await?; 
    match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(e) => Ok(e.error_response()),
    }
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

// GET /api/generate-staticsite-noexpose
// alt statify site
//
// non auth, only for background job,  do not expose!
pub async fn statify_site_(
    db: Data<DbAddr>,
) -> ServiceResult<HttpResponse> {
    let ss = StaticSite();
    let res = db.send(ss).await?; 
    match res {
        Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
        Err(e) => Ok(e.error_response()),
    }
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
        "all", 
        "Rust", "Go", "Swift", "TypeScript", "Angular", "Vue", "React", "Dart", "Flutter",
        "Python", "C-sharp", "C", "CPP", "JavaScript", "Java", "PHP", "Kotlin", "DataBase"
    );
    let typs = vec!(
        "index", "Misc", 
        "Article", "Book", "Event", "Job", "Media", 
        "Product", "Translate"
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
        || ty == "Product" 
        || ty == "Translate"
        || ty == "Misc"
        || ty == "newest";

    check
}
