// background jobs setup

use std::error::Error;
use std::panic::AssertUnwindSafe;
use std::sync::Arc;
use swirl::PerformError;

use crate::db::{DieselPool, DieselPooledConn};
use crate::errors::{SrvErrToStdErr, SrvError, SrvResult};

impl<'a> swirl::db::BorrowedConnection<'a> for DieselPool {
    type Connection = DieselPooledConn<'a>;
}

impl swirl::db::DieselPool for DieselPool {
    type Error = SrvErrToStdErr;

    fn get(&self) -> Result<swirl::db::DieselPooledConn<'_, Self>, Self::Error> {
        self.get().map_err(SrvErrToStdErr)
    }
}

#[allow(missing_debug_implementations)]
pub struct Environment {
    // FIXME: https://github.com/sfackler/r2d2/pull/70
    pub conn_pool: AssertUnwindSafe<DieselPool>,
}

// FIXME: AssertUnwindSafe should be `Clone`, this can be replaced with
// `#[derive(Clone)]` if that is fixed in the standard lib
impl Clone for Environment {
    fn clone(&self) -> Self {
        Self {
            conn_pool: AssertUnwindSafe(self.conn_pool.0.clone()),
        }
    }
}

impl Environment {
    pub fn new(conn_pool: DieselPool) -> Self {
        Self {
            conn_pool: AssertUnwindSafe(conn_pool),
        }
    }

    pub fn connection(&self) -> Result<DieselPooledConn<'_>, PerformError> {
        self.conn_pool.get().map_err(|e| SrvErrToStdErr(e).into())
    }
}
