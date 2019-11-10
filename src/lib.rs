#![allow(warnings)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate tera;

use actix::prelude::*;
use actix::{Actor, SyncContext};
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{
    middleware::{Compress, Logger},
    web::{delete, get, post, put, resource, route, scope},
    App, HttpResponse, HttpServer,
};

use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

// #[macro_use]
// pub mod macros;

pub mod api;
pub mod bot;
pub mod errors;
pub mod schema;
pub mod util;
pub mod view;

// some type alias
pub type PoolConn = Pool<ConnectionManager<PgConnection>>;
pub type PooledConn = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

// This is db executor actor
pub struct Dba(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for Dba {
    type Context = SyncContext<Self>;
}

pub type DbAddr = Addr<Dba>;

pub fn init_dba() -> DbAddr {
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let cpu_num = num_cpus::get();
    let pool_num = std::cmp::max(10, cpu_num * 2 + 1) as u32;
    // p_num subject to c_num??
    let conn = Pool::builder()
        .max_size(pool_num)
        .build(manager)
        .expect("Failed to create pool.");

    SyncArbiter::start(cpu_num * 2 + 1, move || Dba(conn.clone()))
}

pub fn init_fern_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{},{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.level(),
                record.target(),
                record.line().unwrap_or(0),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(fern::log_file("rut.log")?)
        .apply()?;

    Ok(())
}

pub fn init_server() -> std::io::Result<()> {
    // init logger
    init_fern_logger().unwrap_or_default();
    // new runtime
    let sys = actix_rt::System::new("rut-server-rust");
    // init actor
    let addr: DbAddr = init_dba();

    let bind_host =
        dotenv::var("BIND_ADDRESS").unwrap_or("127.0.0.1:8083".to_string());
    // config Server, App, AppState, middleware, service
    HttpServer::new(move || {
        App::new()
            .data(addr.clone())
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(Cors::default())
            // everything under '/api/' route
            .service(scope("/api")
                // to auth
                .service(
                    resource("/signin")
                        .route(post().to_async(api::auth::signin))
                )
                // to register
                .service(
                    resource("/signup")
                        .route(post().to_async(api::auth::signup))
                )
                .service(
                    resource("/reset")   // reset-1: request rest psw, send mail
                        .route(post().to_async(api::auth::reset_psw_req))
                )
                .service(
                    resource("/reset/{token}")   // reset-2: copy token, new psw
                        .route(post().to_async(api::auth::reset_psw))
                )
                .service(
                    resource("/users/{uname}")
                        .route(get().to_async(api::auth::get))
                        .route(post().to_async(api::auth::update))
                        .route(put().to_async(api::auth::change_psw))
                )
                .service(
                    resource("/blogs")
                        .route(post().to_async(api::blog::new))
                        .route(put().to_async(api::blog::update))
                        // get_list: ?per=topic&kw=&perpage=20&page=p
                        .route(get().to_async(api::blog::get_list)) 
                )
                .service(
                    resource("/blogs/{id}")
                        .route(get().to_async(api::blog::get))
                        .route(delete().to_async(api::blog::del))
                )
                .service(
                    resource("/articles")
                        .route(post().to_async(api::article::new))
                        .route(put().to_async(api::article::update))
                        // get_list: ?per=topic&kw=&perpage=20&page=p
                        .route(get().to_async(api::article::get_list)) 
                )
                .service(
                    resource("/articles/{slug}")
                        .route(get().to_async(api::article::get))
                        .route(delete().to_async(api::article::del))
                )
                .service(
                    resource("/issues")
                        .route(post().to_async(api::rfc::new))
                        .route(put().to_async(api::rfc::update))
                        // get_list: ?per=label&kw=&perpage=20&page=p
                        .route(get().to_async(api::rfc::get_list)) 
                )
                .service(
                    resource("/issues/{slug}")
                        .route(get().to_async(api::rfc::get))
                        .route(delete().to_async(api::rfc::del))
                )
                .service(
                    resource("/labelissues")
                        .route(post().to_async(api::rfc::label_isuue))
                        .route(delete().to_async(api::rfc::del_label_isuue))
                )
                .default_service(route().to(|| HttpResponse::NotFound()))
            )
            //.service(scope("/api")
               
            .default_service(route().to(|| HttpResponse::NotFound()))
        //.default_service(route().to(view::tmpl::not_found))
    })
    .bind(&bind_host)
    .expect("Can not bind to host")
    .start();

    println!("Starting http server: {}", bind_host);

    // start runtime
    sys.run()
}
