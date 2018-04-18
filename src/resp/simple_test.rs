//! # Simple Test
//!
//! Simple test code.

use std::io::Cursor;

use rocket::Request;
use rocket::response;
use rocket::response::Responder;
use rocket::Response;
use rocket::http::ContentType;
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleTest {
    code:           String,
}

impl SimpleTest {
    pub fn build() -> Self {
        SimpleTest {
            code:   "".to_string(),
        }
    }

    pub fn code(mut self, code: String) -> Self {
        self.code = code;
        self
    }
}

impl<'r> Responder<'r> for SimpleTest {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .header(ContentType::new("application", "json"))
            .sized_body(Cursor::new(serde_json::to_string(&self).unwrap()))
            .ok()
    }
}