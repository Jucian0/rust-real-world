use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginResponse {
    pub message: String,
    pub status: bool,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub message: String,
    pub status: bool,
}

pub struct SlimUser {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Login {
    pub email: String,
    pub password: String,
    #[serde(default)]
    pub remember_me: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Register {
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password: String,
}
