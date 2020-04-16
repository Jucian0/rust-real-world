use crate::api_error::ApiError;
use crate::auth::create_token;
use crate::user::{User, UserMessage};
use actix_web::{get, post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
#[derive(Deserialize)]
struct RegistrationMessage {
    email: String,
    password: String,
}
#[derive(Serialize, Deserialize)]
struct PayloadAuth {
    AUTHENTICTION: String,
}

#[post("/register")]
async fn register(body: web::Json<RegistrationMessage>) -> Result<HttpResponse, ApiError> {
    let body = body.into_inner();

    let user = User::create(UserMessage {
        email: body.email,
        password: body.password,
    })?;

    Ok(HttpResponse::Ok().json(json!({"message": "Successfully registered", "user": user})))
}

#[post("/sign-in")]
async fn sign_in(credentials: web::Json<UserMessage>) -> Result<HttpResponse, ApiError> {
    let credentials = credentials.into_inner();

    let user = User::find_by_email(credentials.email).map_err(|e| match e.status_code {
        404 => ApiError::new(401, "Credentials not valid!".to_string(), e.data.path),
        _ => e,
    })?;

    let is_valid = user.verify_password(credentials.password.as_bytes())?;

    if is_valid {
        let token = match create_token(&user.email, &user.id.to_string()) {
            Ok(tk) => tk.to_string(),
            Err(_) => "".to_string(),
        };

        if token != "".to_string() {
            let response = HttpResponse::Ok().json::<PayloadAuth>(PayloadAuth {
                AUTHENTICTION: token,
            });

            Ok(response)
        } else {
            Err(ApiError::new(
                401,
                "Credentials not valid!".to_string(),
                "user_auth_key".to_string(),
            ))
        }
    } else {
        Err(ApiError::new(
            401,
            "Credentials not valid!".to_string(),
            "user_auth_key".to_string(),
        ))
    }
}

#[post("/sign-out")]
async fn sign_out() -> Result<HttpResponse, ApiError> {
    // if let Some(_) = id {
    //     Ok(HttpResponse::Ok().json(json!({"message":"Successfully signed out"})))
    // } else {
    //     Err(ApiError::new(
    //         401,
    //         "Unauthorized".to_string(),
    //         "user_auth_key".to_string(),
    //     ))
    // }
    Ok(HttpResponse::Ok().json({}))
}

#[get("/who-am-i")]
async fn who_am_i() -> Result<HttpResponse, ApiError> {
    // let id: Option<Uuid> = session.get("user_id")?;

    // if let Some(id) = id {
    //     let user = User::find(id)?;
    //     Ok(HttpResponse::Ok().json(user))
    // } else {
    //     Err(ApiError::new(
    //         401,
    //         "Unauthorized".to_string(),
    //         "user_aith_key".to_string(),
    //     ))
    // }

    Ok(HttpResponse::Ok().json({}))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(sign_in);
    cfg.service(sign_out);
    cfg.service(who_am_i);
}
