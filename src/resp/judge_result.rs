//! # Result
//!
//! The final result of judgement, several scenes are as below:
//! + `Accept`: which means your code are correct
//! + `WrongAnswer`: failed to pass all the test cases
//! + `Error`: such as `CompilationError`, `TimeLimitExceeded`...
//! + `Panic`: when server arises some mistakes

use std::io::Cursor;

use resp::judge_state::TestCaseState;
use rocket::response::Responder;
use rocket::Request;
use rocket::response;
use rocket::Response;
use rocket::http::ContentType;
use serde_json;


#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum JudgeResult {
    Accept(TestCaseTrace),
    WrongAnswer(TestCaseTrace),
    Error { message: String, error_type: JudgeErrorType },
    ServerPanic { message: String },
}

impl<'r> Responder<'r> for JudgeResult {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .header(ContentType::new("application", "json"))
            .sized_body(Cursor::new(serde_json::to_string(&self).unwrap()))
            .ok()
    }
}


/// [(test case index * state)]
type _TestCaseTrace = [(usize, TestCaseState)];


/// Trace every test case that had been judged.
#[derive(Debug, Serialize, Deserialize)]
pub struct TestCaseTrace {
    cases_result:           Box<_TestCaseTrace>,
}

impl TestCaseTrace {
    pub fn new(cases_result: Box<_TestCaseTrace>) -> Self {
        TestCaseTrace {
            cases_result,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum JudgeErrorType {
    CompilationError,

    MemoryLimitExceeded { mem_used: u32 },

    TimeLimitExceeded   { time_used: u32 },

    PresentationError,

    RestrictedFunction,

    RuntimeError,
}