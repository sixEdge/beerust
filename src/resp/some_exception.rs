//! # Some Exception
//!
//! Trow an exception when something went wrong.

use std::io::Cursor;

use rocket::Request;
use rocket::response::{self, Response, Responder};
use rocket::http::ContentType;
use serde_json;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SomeException {
    code:               ExceptionCode,
    message:            String,
}

impl SomeException {
    pub fn build() -> Self {
        SomeException {
            code:       UNDEFINED_EXCEPTION,
            message:    "".to_string(),
        }
    }

    setter!(code|ExceptionCode, message|String);
}

impl<'r> Responder<'r> for SomeException {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .header(ContentType::new("application", "json"))
            .sized_body(Cursor::new(serde_json::to_string(&self).unwrap()))
            .ok()
    }
}


type ExceptionCode = i32;
const UNDEFINED_EXCEPTION: ExceptionCode = 0;
