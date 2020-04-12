use crate::api_error::ApiError;
use crate::user::{User, UserMessage};
use actix_session::Session;
use actix_web::{get, post, web, HttpResponse};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

#[derive(Deserialize)]
struct RegistrationMessage {
   email: String,
   password: String,
}

#[post("/register")]
async fn register(body: web::Json<RegistrationMessage>) -> Result<HttpResponse, ApiError> {
   let body = body.into_inner();
   let email = body.email.clone();
   let test = User::find_by_email(body.email).is_ok();

   if test {
      return Err(ApiError::new(409, "There is a user with that e-mail"));
   }

   let user = User::create(UserMessage {
      email: email,
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
      404 => ApiError::new(401, "Credentials not valid!".to_string()),
      _ => e,
   })?;

   let is_valid = user.verify_password(credentials.password.as_bytes())?;

   if is_valid == true {
      session.set("user_id", user.id)?;
      session.renew();

      Ok(HttpResponse::Ok().json(user))
   } else {
      Err(ApiError::new(401, "Credentials not valid!".to_string()))
   }
}

#[post("/sign-out")]
async fn sign_out(session: Session) -> Result<HttpResponse, ApiError> {
   let id: Option<Uuid> = session.get("user_id")?;

   if let Some(_) = id {
      session.purge();
      Ok(HttpResponse::Ok().json(json!({"message":"Successfully signed out"})))
   } else {
      Err(ApiError::new(401, "Unauthorized".to_string()))
   }
}

#[get("/who-am-i")]
async fn who_am_i(session: Session) -> Result<HttpResponse, ApiError> {
   let id: Option<Uuid> = session.get("user_id")?;

   if let Some(id) = id {
      let user = User::find(id)?;
      Ok(HttpResponse::Ok().json(user))
   } else {
      Err(ApiError::new(401, "Unauthorized".to_string()))
   }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
   cfg.service(register);
   cfg.service(sign_in);
   cfg.service(sign_out);
   cfg.service(who_am_i);
}
