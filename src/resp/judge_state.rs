//! # Judge State
//!
//! The state/stage of judge after submitting code,
//! is possible to change every moment.

use std::io::Cursor;

use resp::judge_state::Stage::New;
use resp::judge_result::JudgeErrorType;
use rocket::response::Responder;
use rocket::Request;
use rocket::response;
use rocket::Response;
use rocket::http::ContentType;
use serde_json;


/// Judge state.
/// Use it while judging.
#[derive(Debug, Serialize, Deserialize)]
pub struct JudgeState {
    /// stage of judge
    stage:              Stage,

    /// message
    message:            String,
}

impl JudgeState {
    pub fn build() -> Self {
        JudgeState {
            stage: New,
            message: "".to_string(),
        }
    }

    setter!(stage: Stage, message: String);
}

impl<'r> Responder<'r> for JudgeState {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .header(ContentType::new("application", "json"))
            .sized_body(Cursor::new(serde_json::to_string(&self).unwrap()))
            .ok()
    }
}


/// Judge stage
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Stage {
    /// a new submission
    New,

    /// submission is queuing...
    Queuing {
        /// the number of submissions waiting ahead of yours
        waiting_ahead:      usize,
    },

    /// compiling code
    Compiling,

    /// program is running
    Running {
        /// the index of current test case
        case_idx:           usize,
        /// state
        state:              TestCaseState,
    },

    /// be stopped (e.g. queuing out of time) or complete
    Exit {
        case_idx:           usize,
        state:              TestCaseState,
    },
}


/// State of a case test
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TestCaseState {
    Testing,
    Pass {
        /// `ms`
        time_used:  u32,
        /// `byte`
        mem_used:   u32,
    },
    WrongAnswer {
        expect:     String,
        got:        String,
    },
    Error {
        error: JudgeErrorType,
    },
}