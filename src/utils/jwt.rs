use actix_web::HttpResponse;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
   sub: String,
   name: String,
   exp: i64,
}

pub struct SlimUser {
   pub email: String,
   pub name: String,
}

impl From<Claims> for SlimUser {
   fn from(claims: Claims) -> Self {
      SlimUser {
         email: claims.sub,
         name: claims.name,
      }
   }
}

impl Claims {
   fn with_email(email: &str, name: &str) -> Self {
      Claims {
         sub: email.into(),
         name: name.into(),
         exp: (Utc::now() + Duration::hours(24)).timestamp(),
      }
   }
}

pub fn create_token(email: &str, name: &str) -> Result<String, HttpResponse> {
   let claims = Claims::with_email(email, name);
   encode(
      &Header::default(),
      &claims,
      &EncodingKey::from_secret(get_secret().as_ref()),
   )
   .map_err(|e| HttpResponse::InternalServerError().json(e.to_string()))
}

pub fn decode_token(token: &str) -> Result<SlimUser, HttpResponse> {
   decode::<Claims>(
      token,
      &DecodingKey::from_secret(get_secret().as_ref()),
      &Validation::default(),
   )
   .map(|data| data.claims.into())
   .map_err(|e| HttpResponse::Unauthorized().json(e.to_string()))
}

fn get_secret() -> String {
   env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into())
}
