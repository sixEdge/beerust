//! # Some Exception
//!
//! Trow an exception when something went wrong.

use std::io::Cursor;

use rocket::Request;
use rocket::response::{self, Response, Responder};
use rocket::http::ContentType;
use resp::Resp;
use serde_json;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SomeException {
    id:                 ExceptionId,
    message:            String,
}

impl SomeException {
    pub fn id(mut self, id: usize) -> Self {
        self.id = id;
        self
    }

    pub fn message(mut self, message: String) -> Self {
        self.message = message;
        self
    }
}

impl Resp for SomeException {
    fn build() -> Self {
        SomeException {
            id:         UNDEFINED_EXCEPTION,
            message:    "".to_string(),
        }
    }
}

impl<'r> Responder<'r> for SomeException {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .header(ContentType::new("application", "json"))
            .sized_body(Cursor::new(serde_json::to_string(&self).unwrap()))
            .ok()
    }
}


type ExceptionId = usize;
const UNDEFINED_EXCEPTION: ExceptionId = 0;
