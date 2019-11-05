// error wrapper, TODO: wrap more error

use actix::MailboxError;
use actix_web::{error::ResponseError, HttpResponse};
use base64::DecodeError;
use derive_more::Display;
use diesel::r2d2::PoolError;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use oauth2::TokenError;
use reqwest::Error as ReqError;
use std::convert::{From, Into};
use std::error::Error as StdError;
use swirl::errors::PerformError;
//use jsonwebtoken::errors::{Error as JwtError, ErrorKind as JwtErrorKind};

#[derive(Debug, Display)]
pub enum ServiceError {
    // 400
    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    // 401
    #[display(fmt = "Unauthorized")]
    Unauthorized,

    // 404
    #[display(fmt = "Not Found: {}", _0)]
    NotFound(String),

    // 500+
    #[display(fmt = "Internal Server Error: {}", _0)]
    InternalServerError(String),
}

pub type ServiceResult<T> = Result<T, ServiceError>;

impl StdError for ServiceError {}

// impl ResponseError trait allows to convert errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::InternalServerError(ref message) => {
                HttpResponse::InternalServerError().json(message)
            }
            ServiceError::BadRequest(ref message) => {
                HttpResponse::BadRequest().json(message)
            }
            ServiceError::Unauthorized => {
                HttpResponse::Unauthorized().json("Unauthorized")
            }
            ServiceError::NotFound(ref message) => {
                HttpResponse::NotFound().json(message)
            }
        }
    }
}

impl From<MailboxError> for ServiceError {
    fn from(_error: MailboxError) -> Self {
        ServiceError::InternalServerError("Mailbox".into())
    }
}

impl From<DieselError> for ServiceError {
    fn from(error: DieselError) -> ServiceError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DieselError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let msg =
                        info.details().unwrap_or_else(|| info.message()).to_string();
                    return ServiceError::BadRequest(msg);
                }
                ServiceError::InternalServerError("datebase".into())
            }
            DieselError::NotFound => {
                ServiceError::NotFound("requested record was not found".into())
            }
            _ => ServiceError::InternalServerError("datebase".into()),
        }
    }
}

impl From<PoolError> for ServiceError {
    fn from(_error: PoolError) -> Self {
        ServiceError::InternalServerError("pool".into())
    }
}

// Base64 decode
impl From<DecodeError> for ServiceError {
    fn from(_error: DecodeError) -> Self {
        ServiceError::BadRequest("Invalid Base64 Code".into())
    }
}

// reqwest
impl From<ReqError> for ServiceError {
    fn from(_error: ReqError) -> Self {
        ServiceError::InternalServerError("reqwest".into())
    }
}

// reqwest
impl From<TokenError> for ServiceError {
    fn from(_error: TokenError) -> Self {
        ServiceError::InternalServerError("oauth token".into())
    }
}

// swirl back-job
// impl Into<PerformError> for ServiceError {
//     fn into(self) -> PerformError {
//         Box::new(self)
//     }
// }

// jwt
// impl From<JwtError> for ServiceError {
//     fn from(error: JwtError) -> Self {
//         match error.kind() {
//             JwtErrorKind::InvalidToken => {
//                 ServiceError::BadRequest("Invalid Token".into())
//             },
//             JwtErrorKind::InvalidIssuer => {
//                 ServiceError::BadRequest("Invalid Issuer".into())
//             },
//             _ => ServiceError::Unauthorized,
//         }
//     }
// }

// ############################################################################
// ## Error
// ############################################################################

use chrono::NaiveDateTime;
use std::any::{Any, TypeId};
use std::fmt;

// SrvError trait

pub trait SrvError: Send + Sync + fmt::Display + fmt::Debug + 'static {
    fn description(&self) -> &str;
    fn cause(&self) -> Option<&(dyn SrvError)> {
        None
    }

    fn response(&self) -> Option<HttpResponse> {
        if self.human() {
            Some(
                HttpResponse::InternalServerError()
                    .json(self.description().to_string()),
            )
        } else {
            self.cause().and_then(SrvError::response)
        }
    }
    fn human(&self) -> bool {
        false
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
}

impl dyn SrvError {
    pub fn is<T: Any>(&self) -> bool {
        self.get_type_id() == TypeId::of::<T>()
    }

    pub fn from_std_error(err: Box<dyn StdError + Send>) -> Box<dyn SrvError> {
        Self::try_convert(&*err).unwrap_or_else(|| internal(&err))
    }

    fn try_convert(err: &(dyn StdError + Send + 'static)) -> Option<Box<Self>> {
        match err.downcast_ref() {
            Some(DieselError::NotFound) => Some(Box::new(NotFound)),
            Some(DieselError::DatabaseError(_, info))
                if info.message().ends_with("read-only transaction") =>
            {
                Some(Box::new(ReadOnlyMode))
            }
            _ => None,
        }
    }
}

impl SrvError for Box<dyn SrvError> {
    fn description(&self) -> &str {
        (**self).description()
    }
    fn cause(&self) -> Option<&dyn SrvError> {
        (**self).cause()
    }
    fn human(&self) -> bool {
        (**self).human()
    }
    fn response(&self) -> Option<HttpResponse> {
        (**self).response()
    }
}

pub type SrvResult<T> = Result<T, Box<dyn SrvError>>;

// =============================================================================
// Chaining errors

pub trait ChainError<T> {
    fn chain_error<E, F>(self, callback: F) -> SrvResult<T>
    where
        E: SrvError,
        F: FnOnce() -> E;
}

#[derive(Debug)]
struct ChainedError<E> {
    error: E,
    cause: Box<dyn SrvError>,
}

impl<T, F> ChainError<T> for F
where
    F: FnOnce() -> SrvResult<T>,
{
    fn chain_error<E, C>(self, callback: C) -> SrvResult<T>
    where
        E: SrvError,
        C: FnOnce() -> E,
    {
        self().chain_error(callback)
    }
}

impl<T, E: SrvError> ChainError<T> for Result<T, E> {
    fn chain_error<E2, C>(self, callback: C) -> SrvResult<T>
    where
        E2: SrvError,
        C: FnOnce() -> E2,
    {
        self.map_err(move |err| {
            Box::new(ChainedError {
                error: callback(),
                cause: Box::new(err),
            }) as Box<dyn SrvError>
        })
    }
}

impl<T> ChainError<T> for Option<T> {
    fn chain_error<E, C>(self, callback: C) -> SrvResult<T>
    where
        E: SrvError,
        C: FnOnce() -> E,
    {
        match self {
            Some(t) => Ok(t),
            None => Err(Box::new(callback())),
        }
    }
}

impl<E: SrvError> SrvError for ChainedError<E> {
    fn description(&self) -> &str {
        self.error.description()
    }
    fn cause(&self) -> Option<&dyn SrvError> {
        Some(&*self.cause)
    }
    fn response(&self) -> Option<HttpResponse> {
        self.error.response()
    }
    fn human(&self) -> bool {
        self.error.human()
    }
}

impl<E: SrvError> fmt::Display for ChainedError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} caused by {}", self.error, self.cause)
    }
}

// =============================================================================
// Error impls

impl<E: StdError + Sync + Send + 'static> SrvError for E {
    fn description(&self) -> &str {
        StdError::description(self)
    }
}

impl<E: StdError + Sync + Send + 'static> From<E> for Box<dyn SrvError> {
    fn from(err: E) -> Box<dyn SrvError> {
        SrvError::try_convert(&err).unwrap_or_else(|| Box::new(err))
    }
}
// =============================================================================
// Concrete errors

#[derive(Debug)]
struct ConcreteSrvError {
    description: String,
    detail: Option<String>,
    cause: Option<Box<dyn SrvError>>,
    human: bool,
}

impl fmt::Display for ConcreteSrvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)?;
        if let Some(ref s) = self.detail {
            write!(f, " ({})", s)?;
        }
        Ok(())
    }
}

impl SrvError for ConcreteSrvError {
    fn description(&self) -> &str {
        &self.description
    }
    fn cause(&self) -> Option<&dyn SrvError> {
        self.cause.as_ref().map(|c| &**c)
    }
    fn human(&self) -> bool {
        self.human
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NotFound;

impl SrvError for NotFound {
    fn description(&self) -> &str {
        "not found"
    }

    fn response(&self) -> Option<HttpResponse> {
        let response = HttpResponse::NotFound().json("Not Found");
        Some(response)
    }
}

impl fmt::Display for NotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "Not Found".fmt(f)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Unauthorized;

impl SrvError for Unauthorized {
    fn description(&self) -> &str {
        "unauthorized"
    }

    fn response(&self) -> Option<HttpResponse> {
        let response = HttpResponse::Unauthorized().json("Unauthorized");
        Some(response)
    }
}

impl fmt::Display for Unauthorized {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "must be logged in to perform that action".fmt(f)
    }
}

#[derive(Debug)]
struct BadRequest(String);

impl SrvError for BadRequest {
    fn description(&self) -> &str {
        self.0.as_ref()
    }

    fn response(&self) -> Option<HttpResponse> {
        let response = HttpResponse::BadRequest().json("Bad Request");
        Some(response)
    }
}

impl fmt::Display for BadRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

pub fn internal_error(error: &str, detail: &str) -> Box<dyn SrvError> {
    Box::new(ConcreteSrvError {
        description: error.to_string(),
        detail: Some(detail.to_string()),
        cause: None,
        human: false,
    })
}

pub fn internal<S: ToString + ?Sized>(error: &S) -> Box<dyn SrvError> {
    Box::new(ConcreteSrvError {
        description: error.to_string(),
        detail: None,
        cause: None,
        human: false,
    })
}

pub fn human<S: ToString + ?Sized>(error: &S) -> Box<dyn SrvError> {
    Box::new(ConcreteSrvError {
        description: error.to_string(),
        detail: None,
        cause: None,
        human: true,
    })
}

/// This is intended to be used for errors being sent back to the Ember
/// frontend, not to cargo as cargo does not handle non-200 response codes well
/// (see <https://github.com/rust-lang/cargo/issues/3995>), but Ember requires
/// non-200 response codes for its stores to work properly.
///
/// Since this is going back to the UI these errors are treated the same as
/// `human` errors, other than the HTTP status code.
pub fn bad_request<S: ToString + ?Sized>(error: &S) -> Box<dyn SrvError> {
    Box::new(BadRequest(error.to_string()))
}

#[derive(Debug)]
pub struct SrvErrToStdErr(pub Box<dyn SrvError>);

impl StdError for SrvErrToStdErr {
    fn description(&self) -> &str {
        self.0.description()
    }
}

impl fmt::Display for SrvErrToStdErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)?;

        let mut err = &*self.0;
        while let Some(cause) = err.cause() {
            err = cause;
            write!(f, "\nCaused by: {}", err)?;
        }

        Ok(())
    }
}

pub(crate) fn std_error(e: Box<dyn SrvError>) -> Box<dyn StdError + Send> {
    Box::new(SrvErrToStdErr(e))
}

pub(crate) fn std_error_no_send(e: Box<dyn SrvError>) -> Box<dyn StdError> {
    Box::new(SrvErrToStdErr(e))
}

#[derive(Debug, Clone, Copy)]
pub struct ReadOnlyMode;

impl SrvError for ReadOnlyMode {
    fn description(&self) -> &str {
        "tried to write in read only mode"
    }

    fn response(&self) -> Option<HttpResponse> {
        let response =
            HttpResponse::InternalServerError().json("Service Unavailable");
        Some(response)
    }

    fn human(&self) -> bool {
        true
    }
}

impl fmt::Display for ReadOnlyMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "Tried to write in read only mode".fmt(f)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TooManyRequests {
    pub retry_after: NaiveDateTime,
}

impl SrvError for TooManyRequests {
    fn description(&self) -> &str {
        "too many requests"
    }

    fn response(&self) -> Option<HttpResponse> {
        const HTTP_DATE_FORMAT: &str = "%a, %d %b %Y %H:%M:%S GMT";
        let retry_after = self.retry_after.format(HTTP_DATE_FORMAT);

        let mut response =
            HttpResponse::InternalServerError().json("TOO MANY REQUESTS");
        // response
        //     .headers()
        //     .insert("Retry-After".into(), vec![retry_after.to_string()]);
        Some(response)
    }

    fn human(&self) -> bool {
        true
    }
}

impl fmt::Display for TooManyRequests {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "Too many requests".fmt(f)
    }
}
