//! # Submit
//!
//! Response when submit successfully.

use std::io::Cursor;

use rocket::response::Responder;
use rocket::Request;
use rocket::response;
use rocket::Response;
use rocket::http::ContentType;
use serde_json;


#[derive(Debug, Serialize, Deserialize)]
pub struct JudgeSubmit {
    id:             usize,

    // TODO add submit timestamp
}

impl JudgeSubmit {
    pub fn build() -> Self {
        JudgeSubmit {
            id: 0,
        }
    }

    setter!(id|usize);
}

impl<'r> Responder<'r> for JudgeSubmit {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .header(ContentType::new("application", "json"))
            .sized_body(Cursor::new(serde_json::to_string(&self).unwrap()))
            .ok()
    }
}