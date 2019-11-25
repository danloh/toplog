// database pool

use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use parking_lot::{ReentrantMutex, ReentrantMutexGuard};
use std::ops::Deref;
use std::sync::Arc;

use crate::errors::SrvResult;

#[allow(missing_debug_implementations)]
#[derive(Clone)]
pub enum DieselPool {
    Pool(r2d2::Pool<ConnectionManager<PgConnection>>),
    Test(Arc<ReentrantMutex<PgConnection>>),
}

impl DieselPool {
    pub fn get(&self) -> SrvResult<DieselPooledConn<'_>> {
        match self {
            DieselPool::Pool(pool) => Ok(DieselPooledConn::Pool(pool.get()?)),
            DieselPool::Test(conn) => Ok(DieselPooledConn::Test(conn.lock())),
        }
    }

    pub fn state(&self) -> r2d2::State {
        match self {
            DieselPool::Pool(pool) => pool.state(),
            DieselPool::Test(_) => panic!("Cannot get the state of a test pool"),
        }
    }

    fn test_conn(conn: PgConnection) -> Self {
        DieselPool::Test(Arc::new(ReentrantMutex::new(conn)))
    }
}

#[allow(missing_debug_implementations)]
pub enum DieselPooledConn<'a> {
    Pool(r2d2::PooledConnection<ConnectionManager<PgConnection>>),
    Test(ReentrantMutexGuard<'a, PgConnection>),
}

unsafe impl<'a> Send for DieselPooledConn<'a> {}

impl Deref for DieselPooledConn<'_> {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        match self {
            DieselPooledConn::Pool(conn) => conn.deref(),
            DieselPooledConn::Test(conn) => conn.deref(),
        }
    }
}

pub fn connect_now() -> ConnectionResult<PgConnection> {
    let url = dotenv::var("DATABASE_URL").expect("Invalid database URL");
    PgConnection::establish(&url.to_string())
}

pub fn diesel_pool(
    url: String,
    config: r2d2::Builder<ConnectionManager<PgConnection>>,
) -> DieselPool {
    let manager = ConnectionManager::new(url);
    DieselPool::Pool(config.build(manager).unwrap())
}
