

use crate::api::auth::CheckUser;

// general response msg struct
#[derive(Deserialize, Serialize, Debug)]
pub struct Msg {
    pub status: i32,
    pub message: String,
}

// msg for login
#[derive(Deserialize, Serialize, Debug)]
pub struct AuthMsg {
    pub status: i32,
    pub message: String,
    pub token: String,
    pub exp: i32,
    pub user: CheckUser,
    pub omg: bool, // if it is the admin
}

// msg for get user info
#[derive(Deserialize, Serialize, Debug)]
pub struct UserMsg {
    pub status: i32,
    pub message: String,
    pub user: CheckUser,
}