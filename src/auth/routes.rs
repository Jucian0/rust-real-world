use crate::api_error::ApiError;
use crate::user::{User, UserMessage};
use crate::utils::create_token;
use actix_session::Session;
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
    token: String,
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
async fn sign_in(
    credentials: web::Json<UserMessage>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let credentials = credentials.into_inner();

    let user = User::find_by_email(credentials.email).map_err(|e| match e.status_code {
        404 => ApiError::new(401, "Credentials not valid!".to_string(), e.data.path),
        _ => e,
    })?;

    let is_valid = user.verify_password(credentials.password.as_bytes())?;

    if is_valid {
        session.set("user_id", user.id)?;
        session.renew();

        let token = match create_token(&user.email, &user.id.to_string()) {
            Ok(tk) => tk.to_string(),
            Err(_) => "".to_string(),
        };

        let response = HttpResponse::Ok().header("AUTHENTICTION", token).json(user);

        Ok(response)
    } else {
        Err(ApiError::new(
            401,
            "Credentials not valid!".to_string(),
            "user_auth_key".to_string(),
        ))
    }
}

#[post("/sign-out")]
async fn sign_out(session: Session) -> Result<HttpResponse, ApiError> {
    let id: Option<Uuid> = session.get("user_id")?;

    if let Some(_) = id {
        session.purge();
        Ok(HttpResponse::Ok().json(json!({"message":"Successfully signed out"})))
    } else {
        Err(ApiError::new(
            401,
            "Unauthorized".to_string(),
            "user_auth_key".to_string(),
        ))
    }
}

#[get("/who-am-i")]
async fn who_am_i(session: Session) -> Result<HttpResponse, ApiError> {
    let id: Option<Uuid> = session.get("user_id")?;

    if let Some(id) = id {
        let user = User::find(id)?;
        Ok(HttpResponse::Ok().json(user))
    } else {
        Err(ApiError::new(
            401,
            "Unauthorized".to_string(),
            "user_aith_key".to_string(),
        ))
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(sign_in);
    cfg.service(sign_out);
    cfg.service(who_am_i);
}
