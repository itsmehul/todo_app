pub mod create_user;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseDataUser {
    data: ResponseUser,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseUser {
    id: i32,
    username: String,
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestCreateUser {
    username: String,
    password: String,
}
