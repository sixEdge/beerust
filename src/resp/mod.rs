//! # Resp
//!
//! Varieties of response types, in json format.

use std::fmt::Debug;
use std::io::Cursor;

use resp::some_exception::SomeException;
use rocket::Request;
use rocket::response;
use rocket::response::Responder;
use rocket::Response;
use rocket::http::ContentType;
use serde_json;
use serde::Serialize;

pub mod test_code;
pub mod judge_submit;
pub mod judge_state;
pub mod judge_result;
pub mod some_exception;


#[derive(Debug, Serialize)]
#[serde(tag = "status")]
pub enum ResponseResult<T>
    where T: Debug + Serialize
{
    Success(T),
    Exception(SomeException),
}

impl<'r, T> Responder<'r> for ResponseResult<T>
    where T: Debug + Serialize
{
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .header(ContentType::new("application", "json"))
            .sized_body(Cursor::new(serde_json::to_string(&self).unwrap()))
            .ok()
    }
}