use crate::api_error::ApiError;
use crate::auth::decode_token;
use crate::user::{User, UserMessage};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use serde_json::json;
use uuid::Uuid;

#[get("/users")]
async fn find_all(req: HttpRequest) -> Result<HttpResponse, ApiError> {
    let users = User::find_all()?;

    let auth = req.headers().get("Authorization");
    let split: Vec<&str> = auth.unwrap().to_str().unwrap().split("Bearer").collect();
    let token = split[1].trim();

    let decoded = match decode_token(token) {
        Ok(_) => true,
        Err(_) => false,
    };

    if decoded {
        Ok(HttpResponse::Ok().json(users))
    } else {
        return Err(ApiError::new(
            401,
            "Credentials not valid!".to_string(),
            "user_auth_key".to_string(),
        ));
    }
}

#[get("/users/{id}")]
async fn find(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let user = User::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/users")]
async fn create(user: web::Json<UserMessage>) -> Result<HttpResponse, ApiError> {
    let user = User::create(user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[put("/users/{id}")]
async fn update(
    id: web::Path<Uuid>,
    user: web::Json<UserMessage>,
) -> Result<HttpResponse, ApiError> {
    let user = User::update(id.into_inner(), user.into_inner())?;
    Ok(HttpResponse::Ok().json(user))
}

#[delete("/users/{id}")]
async fn delete(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let num_deleted = User::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": num_deleted })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}
