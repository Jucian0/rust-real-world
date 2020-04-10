use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

use crate::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
   pub id: Uuid,
   pub exp: i64,
}

pub trait CanGenerateJwt {
   fn generate_jwt(&self) -> Result<String, String>;
}

impl CanGenerateJwt for User {
   fn generate_jwt(&self) -> Result<String, String> {
      let exp = (Utc::now() + Duration::days(21)).timestamp();
      let claims = Claims { id: self.id, exp };

      let header = Header::default();
      let token = encode(
         &header,
         &claims,
         &EncodingKey::from_secret(get_secret().as_ref()),
      );

      Ok(token)
   }
}

pub trait CanDecodeJwt {
   fn decode_jwt(&self) -> Result<TokenData<Claims>, String>;
}

impl CanDecodeJwt for String {
   fn decode_jwt(&self) -> Result<TokenData<Claims>, String> {
      match decode::<Claims>(
         &self,
         &DecodingKey::from_secret(get_secret().as_ref()),
         &Validation::default(),
      ) {
         Ok(res) => Ok(res),
         Err(e) => Err(e.into(),
      }
   }
}

fn get_secret() -> String {
   env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into())
}
