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
pub struct TestCode {
    func_code:      String,
    simple_test:    String,
}


impl TestCode {
    pub fn build() -> Self {
        TestCode {
            func_code:      "".to_string(),
            simple_test:    "".to_string(),
        }
    }

    setter!(func_code: String, simple_test: String);
}

impl<'r> Responder<'r> for TestCode {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .header(ContentType::new("application", "json"))
            .sized_body(Cursor::new(serde_json::to_string(&self).unwrap()))
            .ok()
    }
}