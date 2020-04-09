use crate::api_error::ApiError;
use serde::Serialize;
use std::collections::HashMap;

lazy_static::lazy_static{
   static ref SENDINBLUE_API_KEY:String = std::env::var("SENDINBLUE_API_KEY").unwrap_or("".to_string());
}

#[derive(Debug, Serialize)]
pub struct Contact{
   email:String,
   name:Option<String>
}

impl Contact{
   pub fn new<T: Into<String>>(email:T, name:T)->Self{
      Contact{email:email.into(), name:Some(name.into())}
   }
}

impl<T:Into<String>>From<T> for Contact{
   fn from(email:T)->Self{
      Contact{email:email.into(), name:Nome}
   }
}



