// api.auth view handler

use futures::{future::result, Future};
use actix::{Handler, Message};
use actix_web::{
    dev::Payload,
    web::{Data, Json, Path},
    Error, HttpResponse, ResponseError,
    FromRequest, HttpRequest,
};
use base64::decode as base64_decode;
use diesel::prelude::*;
use diesel::{self, ExpressionMethods, QueryDsl, RunQueryDsl};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Local, NaiveDateTime, Utc};
use std::convert::From;
use jsonwebtoken::{decode, encode, Header, Validation};

use crate::errors::{ServiceError, ServiceResult};
use crate::api::{Msg, AuthMsg, UserMsg};
use crate::util::helper::gen_slug;
use crate::util::email::{try_send_confirm_email, try_send_reset_email};
use crate::schema::{users};
use crate::api::{
    re_test_email, re_test_name, re_test_psw, re_test_url, test_len_limit,
    MID_LEN,
};
use crate::{Dba, DbAddr, PooledConn};

pub const LIMIT_PERMIT: i16 = 0x01; // follow,star...
pub const BASIC_PERMIT: i16 = 0x02; // create, edit self created...
pub const EIDT_PERMIT: i16 = 0x04; // edit/del others' creats
pub const MOD_PERMIT: i16 = 0x10; // mod role
pub const ADMIN_PERMIT: i16 = 0x80; // admin

// POST: api/signup
//
pub fn signup(
    reg_user: Json<RegUser>,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let reg_usr = reg_user.into_inner();

    // for decode password
    let pswd = String::from_utf8(
        base64_decode(&reg_usr.password).unwrap_or(Vec::new())
    )
    .unwrap_or("".into());

    let reg = RegUser {
        password: pswd,
        ..reg_usr
    };

    result(reg.validate())
        .from_err()
        .and_then(move |_| db.send(reg).from_err())
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}

impl Handler<RegUser> for Dba {
    type Result = ServiceResult<Msg>;

    fn handle(&mut self, reg: RegUser, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;
        reg.register(conn)
    }
}


// POST: api/signin
//
pub fn signin(
    auth: Json<AuthUser>,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let auth_usr = auth.into_inner();

    // for decode password
    let pswd = String::from_utf8(
        base64_decode(&auth_usr.password).unwrap_or(Vec::new())
    )
    .unwrap_or("".into());

    let auth_user = AuthUser {
        password: pswd,
        ..auth_usr
    };

    result(auth_user.validate())
        .from_err()
        .and_then(move |_| db.send(auth_user).from_err())
        .and_then(|res| match res {
            Ok(user) => {
                let token = encode_token(&user)?;
                let admin = dotenv::var("ADMIN").unwrap_or("".to_string());
                let check_omg = user.uname == admin || user.can(EIDT_PERMIT);
                let auth_msg = AuthMsg {
                    status: 200,
                    message: String::from("Success"),
                    token: token,
                    exp: 5, // unit: day
                    user: user,
                    omg: check_omg,
                };
                Ok(HttpResponse::Ok().json(auth_msg))
            }
            Err(e) => Ok(e.error_response()),
        })
}

impl Handler<AuthUser> for Dba {
    type Result = ServiceResult<CheckUser>;

    fn handle(&mut self, au: AuthUser, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;
        au.auth(conn)
    }
}

// GET: api/users/{uname}
//
pub fn get(
    path_uname: Path<String>,
    db: Data<DbAddr>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let uname = path_uname.into_inner();
    db.send(QueryUser { uname })
        .from_err()
        .and_then(|res| match res {
            Ok(user) => {
                let user_msg = UserMsg {
                    status: 200,
                    message: String::from("Success"),
                    user: user,
                };
                Ok(HttpResponse::Ok().json(user_msg))
            }
            Err(er) => Ok(er.error_response()),
        })
}

impl Handler<QueryUser> for Dba {
    type Result = ServiceResult<CheckUser>;

    fn handle(&mut self, uid: QueryUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        let conn = &self.0.get()?;

        let query_user = users
            .filter(&uname.eq(&uid.uname))
            .get_result::<User>(conn)?;

        Ok(query_user.into())
    }
}

// POST: api/users/{uname}
//
pub fn update(
    db: Data<DbAddr>,
    user: Json<UpdateUser>,
    auth: CheckUser,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let up_user = user.into_inner();

    // auth.uname == user.uname
    if auth.uname != up_user.uname {
        panic!("No Permission"); // to have a better way!!
    }

    result(up_user.validate())
        .from_err()
        .and_then(move |_| db.send(up_user).from_err())
        .and_then(|res| match res {
            Ok(user) => {
                let token = encode_token(&user)?;
                let admin = dotenv::var("ADMIN").unwrap_or("".to_string());
                let check_omg = user.uname == admin || user.can(EIDT_PERMIT);
                let auth_msg = AuthMsg {
                    status: 200,
                    message: String::from("Success"),
                    token: token,
                    exp: 5, // unit: day
                    user: user,
                    omg: check_omg,
                };
                Ok(HttpResponse::Ok().json(auth_msg))
            }
            Err(e) => Ok(e.error_response()),
        })
}

impl Handler<UpdateUser> for Dba {
    type Result = ServiceResult<CheckUser>;

    fn handle(&mut self, up: UpdateUser, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;
        up.update(conn)
    }
}

// PUT: api/users/{uname}
//
pub fn change_psw(
    db: Data<DbAddr>,
    psw: Json<ChangePsw>,
    auth: CheckUser,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let usr_psw = psw.into_inner();

    // auth.uname == user.uname
    if auth.uname != usr_psw.uname {
        panic!("No Permission"); // to have a better way!!
    }

    // for decode password
    let new_psw = String::from_utf8(
        base64_decode(&usr_psw.new_psw).unwrap_or(Vec::new())
    )
    .unwrap_or("".into());

    let old_psw = String::from_utf8(
        base64_decode(&usr_psw.old_psw).unwrap_or(Vec::new())
    )
    .unwrap_or("".into());

    let user_psw = ChangePsw {
        old_psw,
        new_psw,
        ..usr_psw
    };

    result(user_psw.validate())
        .from_err()
        .and_then(move |_| db.send(user_psw).from_err())
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)),
            Err(e) => Ok(e.error_response()),
        })
}

impl Handler<ChangePsw> for Dba {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, psw: ChangePsw, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        let conn = &self.0.get()?;

        let check_user = users
            .filter(&uname.eq(&psw.uname))
            .load::<User>(conn)?
            .pop();

        if let Some(old) = check_user {
            match verify(&psw.old_psw, &old.psw_hash) {
                Ok(valid) if valid => {
                    // hash psw then update
                    let new_password: String = hash_password(&psw.new_psw)?;
                    diesel::update(&old)
                        .set(psw_hash.eq(new_password))
                        .execute(conn)?;

                    Ok(Msg {
                        status: 200,
                        message: String::from("Success"),
                    })
                }
                _ => Ok(Msg {
                    status: 401,
                    message: String::from("Somehing Wrong"),
                }),
            }
        } else {
            Ok(Msg {
                status: 404,
                message: String::from("No Existing"),
            })
        }
    }
}

// POST api/reset
//
// 1-request reset, send mail  '/reset'
pub fn reset_psw_req(
    db: Data<DbAddr>,
    re_req: Json<ResetReq>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let req = re_req.into_inner(); // need uname and email
    result(req.validate())
        .from_err()
        .and_then(move |_| db.send(req).from_err())
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)), // 200 or 401 or 404
            Err(e) => Ok(e.error_response()),
        })
}

// POST api/reset/{token}
//
// 2- using token in mail to verify
// reset user password  '/reset/{token}'
pub fn reset_psw(
    db: Data<DbAddr>,
    p_info: Path<String>,
    newpsw: Json<ResetPsw>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    use base64::decode;

    let reset_psw = newpsw.into_inner().re_psw;
    let re_psw = String::from_utf8(decode(&reset_psw).unwrap_or(Vec::new()))
        .unwrap_or("".into());

    let tok = p_info.into_inner();
    let de_tok =
        String::from_utf8(decode(&tok).unwrap_or(Vec::new())).unwrap_or("".into());

    let tc = verify_token(&de_tok);
    let uname = tc.uname;
    let email = tc.email;
    let exp = tc.exp;
    let reset = ResetPsw {
        re_psw,
        uname,
        email,
        exp,
    };
    result(reset.validate())
        .from_err()
        .and_then(move |_| db.send(reset).from_err())
        .and_then(|res| match res {
            Ok(msg) => Ok(HttpResponse::Ok().json(msg)), // 200 or 404
            Err(e) => Ok(e.error_response()),
        })
}

impl Handler<ResetReq> for Dba {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, req: ResetReq, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        let conn = &self.0.get()?;

        let check_user = users
            .filter(&uname.eq(&req.uname))
            .get_result::<User>(conn)?;

        if req.email == check_user.email {
            let rq_uname = req.uname;
            let rq_email = req.email;
            let tok = generate_token(&rq_uname, &rq_email, 60 * 2)
                .unwrap_or("".to_owned());

            try_send_reset_email(&rq_email, &rq_uname, &tok)?;

            Ok(Msg {
                status: 200,
                message: String::from("The token has been sent to you via email"),
            })
        } else {
            Ok(Msg {
                status: 404,
                message: String::from("No Existing User or Email"),
            })
        }
    }
}

// handle msg from .reset_psw
impl Handler<ResetPsw> for Dba {
    type Result = Result<Msg, ServiceError>;

    fn handle(&mut self, psw: ResetPsw, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        let conn = &self.0.get()?;

        let check_user = users
            .filter(&uname.eq(&psw.uname))
            .load::<User>(conn)?
            .pop();

        if let Some(old) = check_user {
            if old.email == psw.email {
                let new_password: String = hash_password(&psw.re_psw)?;
                diesel::update(&old)
                    .set(psw_hash.eq(new_password))
                    .execute(conn)?;

                return Ok(Msg {
                    status: 200,
                    message: String::from("Success"),
                });
            }
            Ok(Msg {
                status: 401,
                message: String::from("Something Wrong"),
            })
        } else {
            Ok(Msg {
                status: 404,
                message: String::from("No Existing User"),
            })
        }
    }
}

// handle msg from tmpl.confirm_email
// only signed up user need to confirm email
impl Handler<TokClaim> for Dba {
    type Result = Result<bool, ServiceError>;

    fn handle(&mut self, tok: TokClaim, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        let conn = &self.0.get()?;

        let check_user = users
            .filter(&uname.eq(&tok.uname))
            .load::<User>(conn)?
            .pop();

        let now = chrono::Utc::now().timestamp();
        let check: bool = tok.exp >= now;

        if let Some(old) = check_user {
            if check && old.email == tok.email {
                diesel::update(&old)
                    .set(email_confirmed.eq(true))
                    .execute(conn)?;
                return Ok(true);
            }
            Ok(false)
        } else {
            Ok(false)
        }
    }
}


// ============================================================================
// ============================================================================
// Model
// ============================================================================

#[derive(Clone, Debug, Serialize, Deserialize, Identifiable, Queryable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub uname: String, // unique
    pub psw_hash: String,
    pub join_at: NaiveDateTime,
    pub last_seen: NaiveDateTime,
    pub avatar: String,
    pub email: String, // unique but can be ""
    pub link: String,
    pub intro: String,
    pub location: String,
    pub nickname: String,
    pub permission: i16,
    pub auth_from: String,     // for OAuth
    pub email_confirmed: bool, // for email confirm
    pub karma: i32,
    pub is_pro: bool,
    pub can_push: bool,
    pub push_email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable, Default)]
#[table_name = "users"]
pub struct BuildUser {
    pub uname: String, // unique
    pub psw_hash: String,
    pub avatar: String,
    pub email: String, // unique but can be ""
    pub link: String,
    pub intro: String,
    pub location: String,
    pub nickname: String,
    pub permission: i16,
    pub auth_from: String,     // for OAuth
    pub email_confirmed: bool, // for email confirm
}

impl User {
    // User's constructor
    pub fn new(uname: &str, psw_hash: &str) -> BuildUser {
        BuildUser {
            uname: uname.to_owned(),
            psw_hash: psw_hash.to_owned(),
            permission: LIMIT_PERMIT | BASIC_PERMIT,
            ..BuildUser::default()
        }
    }
    // check permission
    pub fn can(&self, permission: i16) -> bool {
        (self.permission & permission) == permission
    }
}

// return as user info w/o password
#[derive(Clone, Debug, Serialize, Deserialize, Identifiable, Queryable)]
#[table_name = "users"]
pub struct CheckUser {
    pub id: i32,
    pub uname: String,
    pub join_at: NaiveDateTime,
    pub avatar: String,
    pub email: String,
    pub intro: String,
    pub location: String,
    pub nickname: String,
    pub permission: i16,
    pub link: String,
    pub auth_from: String,
    pub email_confirmed: bool,
}

impl CheckUser {
    // check permission
    pub fn can(&self, permission: i16) -> bool {
        (self.permission & permission) == permission
    }
}

impl From<User> for CheckUser {
    fn from(user: User) -> Self {
        CheckUser {
            id: user.id,
            uname: user.uname,
            join_at: user.join_at,
            avatar: user.avatar,
            email: user.email,
            intro: user.intro,
            location: user.location,
            nickname: user.nickname,
            permission: user.permission,
            link: user.link,
            auth_from: user.auth_from,
            email_confirmed: user.email_confirmed,
        }
    }
}

impl From<BuildUser> for CheckUser {
    fn from(user: BuildUser) -> Self {
        CheckUser {
            id: 0,
            uname: user.uname,
            join_at: Utc::now().naive_utc(),
            avatar: user.avatar,
            email: user.email,
            intro: user.intro,
            location: user.location,
            nickname: user.nickname,
            permission: user.permission,
            link: user.link,
            auth_from: user.auth_from,
            email_confirmed: user.email_confirmed,
        }
    }
}

impl Message for CheckUser {
    type Result = Result<Msg, ServiceError>;
}

// auth via token
impl FromRequest for CheckUser {
    type Config = ();
    type Error = ServiceError;
    type Future = Result<CheckUser, ServiceError>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(auth_token) = req.headers().get("authorization") {
            if let Ok(auth) = auth_token.to_str() {
                let user: CheckUser = decode_token(auth)?;
                return Ok(user);
            }
        }
        Err(ServiceError::Unauthorized.into())
    }
}

// jwt Token auth: Claim, token
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub iss: String, // issuer
    pub sub: String, // subject
    pub iat: i64,    // issued at
    pub exp: i64,    // expiry
    pub uid: i32, // user id
    pub uname: String,
    pub permission: i16,
}

// claims's constructor
impl Claims {
    pub fn new(uid: i32, uname: &str, permit: i16) -> Self {
        Claims {
            iss: "Newdin".into(),
            sub: "auth".into(),
            iat: Utc::now().timestamp(),
            exp: (Utc::now() + Duration::hours(24 * 5)).timestamp(),
            uid: uid,
            uname: uname.to_owned(),
            permission: permit,
        }
    }
}

impl From<Claims> for CheckUser {
    fn from(claims: Claims) -> Self {
        CheckUser {
            id: claims.uid,
            uname: claims.uname,
            join_at: Utc::now().naive_utc(),
            avatar: "".to_owned(),
            email: "".to_owned(),
            intro: "".to_owned(),
            location: "".to_owned(),
            nickname: "".to_owned(),
            permission: claims.permission,
            link: "".to_owned(),
            auth_from: "".to_owned(),
            email_confirmed: false,
        }
    }
}

// # for jwt auth
// # encode authed user info as token w/ secret key,
// # then send to client as cookie;
// # request w/ such token to server,
// # decode token to get authed user info w/ secret key

fn get_secret() -> String {
    dotenv::var("SECRET_KEY").unwrap_or_else(|_| "AHaR9uyS3s5SeCREkY".into())
}

pub fn encode_token(data: &CheckUser) -> Result<String, ServiceError> {
    let claims = Claims::new(data.id, data.uname.as_str(), data.permission);
    encode(&Header::default(), &claims, get_secret().as_ref())
        .map_err(|_err| ServiceError::InternalServerError("encode".into()))
}

pub fn decode_token(token: &str) -> Result<CheckUser, ServiceError> {
    decode::<Claims>(token, get_secret().as_ref(), &Validation::default())
        .map(|data| Ok(data.claims.into()))
        .map_err(|_err| ServiceError::Unauthorized)?
}

pub fn hash_password(plain: &str) -> Result<String, ServiceError> {
    // get the hashing cost from the env variable or use default
    let hashing_cost: u32 = match dotenv::var("HASH_ROUNDS") {
        Ok(cost) => cost.parse().unwrap_or(DEFAULT_COST),
        _ => DEFAULT_COST,
    };
    hash(plain, hashing_cost)
        .map_err(|_| ServiceError::InternalServerError("hash".into()))
}

// # modle for api/handler

// message to sign up user
#[derive(Deserialize, Serialize, Debug)]
pub struct RegUser {
    pub uname: String,
    pub email: String,
    pub password: String,
    pub confirm: String,
}

impl RegUser {
    fn validate(&self) -> ServiceResult<()> {
        let uname = &self.uname.trim();
        let psw = &self.password;
        let check = re_test_name(uname) && re_test_psw(psw);

        if check {
            Ok(())
        } else {
            Err(ServiceError::BadRequest("Invalid username or password".into()))
        }
    }

    fn register(
        &self,
        conn: &PooledConn
    ) -> ServiceResult<Msg> {
        use crate::schema::users::dsl::*;
        let check_user = users
            .filter(&uname.eq(&self.uname))
            .load::<User>(conn)?
            .pop();
        match check_user {
            Some(_) => Ok(Msg {
                status: 409,
                message: String::from("Duplicated"),
            }),
            None => {
                // hash password
                let pswd: String = hash_password(&self.password)?;
                let unm: &str = &self.uname.trim();
                let new_user = User::new(unm, &pswd);
                let mut newUser = new_user.clone();

                let user_email: &str = &self.email.trim();
                if re_test_email(user_email) {
                    // check user-email duplication
                    let check_email_user = users
                        .filter(&email.eq(user_email))
                        .load::<User>(conn)?
                        .pop();
                    let check_dup_email = match check_email_user {
                        Some(_) => true,
                        None => false,
                    };
                    if !check_dup_email {
                        newUser = BuildUser {
                            email: user_email.to_owned(),
                            ..new_user
                        };
                        let tok = generate_token(unm, user_email, 60 * 24 * 2)?;
                        try_send_confirm_email(user_email, unm, &tok)?;
                    }
                }

                diesel::insert_into(users)
                    .values(&newUser)
                    .get_result::<User>(conn)?;

                Ok(Msg {
                    status: 201,
                    message: String::from("Success"),
                })
            }
        }
    }
}

impl Message for RegUser {
    type Result = Result<Msg, ServiceError>;
}

// message to login user
#[derive(Deserialize, Serialize, Debug)]
pub struct AuthUser {
    pub uname: String,
    pub password: String,
}

impl AuthUser {
    fn validate(&self) -> ServiceResult<()> {
        let uname = &self.uname.trim();
        let psw = &self.password;
        let check = test_len_limit(uname, 3, 42) && test_len_limit(psw, 8, 18);

        if check {
            Ok(())
        } else {
            Err(ServiceError::BadRequest("Invalid username or password".into()))
        }
    }

    fn auth(
       &self,
       conn: &PooledConn
    ) -> ServiceResult<CheckUser> {
        use crate::schema::users::dsl::*;

        let query_user = users
            .filter(&uname.eq(self.uname.trim()))
            .load::<User>(conn)?
            .pop();

        if let Some(check_user) = query_user {
            match verify(&self.password, &check_user.psw_hash) {
                Ok(valid) if valid => {
                    // update last_seen
                    let logged = diesel::update(&check_user)
                        .set(last_seen.eq(Utc::now().naive_utc()))
                        .get_result::<User>(conn)?;
                    return Ok(logged.into());
                }
                _ => (),
            }
        }
        Err(ServiceError::BadRequest("Auth Failed".into()))
    }
}

impl Message for AuthUser {
    type Result = Result<CheckUser, ServiceError>;
}

// as msg in get user by uname
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct QueryUser {
    pub uname: String,
}

impl Message for QueryUser {
    type Result = Result<CheckUser, ServiceError>;
}

// message to update user
#[derive(Deserialize, Serialize, Debug, Clone, AsChangeset)]
#[table_name = "users"]
pub struct UpdateUser {
    pub uname: String,
    pub avatar: String,
    pub email: String,
    pub intro: String,
    pub location: String,
    pub nickname: String, 
}

impl UpdateUser {
    fn validate(&self) -> ServiceResult<()> {
        let nickname = &self.nickname.trim();
        let nickname_test = if nickname.len() == 0 {
            true
        } else {
            re_test_name(nickname)
        };
        let avatar = &self.avatar.trim();
        let avatar_test = if avatar.len() == 0 {
            true
        } else {
            re_test_url(avatar)
        };
        let check_len = test_len_limit(&self.location, 0, MID_LEN);
        let check = nickname_test && avatar_test && check_len;

        if check {
            Ok(())
        } else {
            Err(ServiceError::BadRequest("Invalid Input".into()))
        }
    }

    fn update(
       &self,
       conn: &PooledConn
    ) -> ServiceResult<CheckUser> {
        use crate::schema::users::dsl::*;

        let user_ = self.clone(); // get a copy for later use
        let new_user_email: &str = self.email.trim();
        let unm = &self.uname.trim();

        let old_user = users.filter(&uname.eq(unm)).get_result::<User>(conn)?;
        let old_user_email: &str = old_user.email.trim();

        // default using old email
        let mut up_user = UpdateUser {
            email: old_user_email.to_owned(),
            ..self.clone()
        };

        // if update w/ another email
        if re_test_email(new_user_email) && new_user_email != old_user_email {
            // check user-email duplication
            let check_email_user = users
                .filter(&email.eq(new_user_email))
                .load::<User>(conn)?
                .pop();
            let check_dup_email = match check_email_user {
                Some(_) => true,
                None => false,
            };
            // sen confirm email if new unique email added
            if !check_dup_email {
                // if not dup and valid new email, using new email
                up_user = user_;
                let tok = generate_token(unm, new_user_email, 60 * 24 * 2)?;
                try_send_confirm_email(new_user_email, unm, &tok)?;
            }
        }

        let update_user = diesel::update(&old_user)
            .set(&up_user)
            .get_result::<User>(conn)?;

        Ok(update_user.into())
    } 
}

impl Message for UpdateUser {
    type Result = Result<CheckUser, ServiceError>;
}

// msg to change psw
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChangePsw {
    pub old_psw: String,
    pub new_psw: String,
    pub uname: String,
}

impl ChangePsw {
    fn validate(&self) -> ServiceResult<()> {
        let check = re_test_psw(&self.new_psw);

        if check {
            Ok(())
        } else {
            Err(ServiceError::BadRequest("Invalid password".into()))
        }
    }
}

impl Message for ChangePsw {
    type Result = Result<Msg, ServiceError>;
}

// msg to request reset psw
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResetReq {
    pub uname: String,
    pub email: String,
}

impl ResetReq {
    fn validate(&self) -> ServiceResult<()> {
        let check = re_test_name(&self.uname) && re_test_email(&self.email);
        if check {
            Ok(())
        } else {
            Err(ServiceError::BadRequest("Invalid".into()))
        }
    }
}

impl Message for ResetReq {
    type Result = Result<Msg, ServiceError>;
}

// msg to reset psw
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResetPsw {
    pub re_psw: String,
    pub uname: String,
    pub email: String,
    pub exp: i64,
}

impl Message for ResetPsw {
    type Result = Result<Msg, ServiceError>;
}

impl ResetPsw {
    fn validate(&self) -> ServiceResult<()> {
        let check = re_test_psw(&self.re_psw)
            && re_test_name(&self.uname)
            && Utc::now().timestamp() <= self.exp;
        if check {
            Ok(())
        } else {
            Err(ServiceError::BadRequest("Invalid password".into()))
        }
    }
}

// confirm token
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokClaim {
    pub exp: i64,
    pub uname: String,
    pub email: String,
}

impl Message for TokClaim {
    type Result = Result<bool, ServiceError>;
}

pub fn generate_token(
    uname: &str,
    email: &str,
    expiration: i64,
) -> Result<String, ServiceError> {
    let claim = TokClaim {
        exp: (Utc::now() + Duration::minutes(expiration)).timestamp(),
        uname: uname.to_string(),
        email: email.to_string(),
    };
    encode(&Header::default(), &claim, get_secret().as_ref())
        .map_err(|_err| ServiceError::InternalServerError("encode".into()))
}

pub fn verify_token(token: &str) -> TokClaim {
    let res =
        decode::<TokClaim>(token, get_secret().as_ref(), &Validation::default());
    //let now = Utc::now().timestamp();
    let (exp, uname, email) = match res {
        Ok(t) => {
            let c = t.claims;
            (c.exp, c.uname, c.email)
        }
        _ => (0, "".to_string(), "".to_string()),
    };
    TokClaim { exp, uname, email }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GUser {
    pub sub: Option<String>,  // required
    pub name: Option<String>, // required
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub picture: Option<String>,
    pub email: Option<String>, // required
    pub email_verified: Option<bool>,
    pub locale: Option<String>,
}

impl Message for GUser {
    type Result = Result<CheckUser, ServiceError>;
}
