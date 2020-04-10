use crate::utils::jwt::{decode_token, SlimUser};
use actix_identity::Identity;
use actix_web::*;
use actix_web::{dev, FromRequest, HttpRequest, HttpResponse};
use api_error::ApiError;

impl FromRequest for LoggedUser {
   fn from_request(
      req: &HttpRequest,
      payload: &mut dev::Payload,
   ) -> Result<HttpResponse, ApiError> {
      let generator = req
         .app_data::<CsrfTokenGenerator>()
         .ok_or(HttpResponse::InternalServerError())?;

      let csrf_token = req
         .headers()
         .get("x-csrf-token")
         .ok_or(HttpResponse::Unauthorized())?;

      let decoded_token = hex::decode(&csrf_token)
         .map_err(|error| HttpResponse::InternalServerError().json(error.to_string()))?;

      generator
         .verify(&decoded_token)
         .map_err(|_| HttpResponse::Unauthorized())?;

      // We're using the CookieIdentityPolicy middleware
      // to handle cookies, with this implementation this
      // will validate the cookie according to the secret
      // provided in main function
      if let Some(identity) = Identity::from_request(req, payload).identity() {
         let user: SlimUser = decode_token(&identity)?;
         return Ok(user as LoggedUser);
      }
      Err(HttpResponse::Unauthorized().into())
   }
}
