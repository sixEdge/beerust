//! # Router
//!
//! Access service according to RESTFul api.

use rocket::response::status::NotFound;
use rocket_contrib::Json;
use resp::Resp;
use resp::some_exception::SomeException;
use resp::judge_submit::JudgeSubmit;
use resp::judge_state::{JudgeState, TestCaseState::*, Stage::*};
use resp::judge_result::JudgeResult;
use resp::judge_result::ResultTrace;


/// Submit code.
/* TODO
 * #[post("/submit", format = "application/json", data = "<solution>")]
 * fn submit(solution: LenientForm<Solution>) -> Result<Json<Submit>, SomeException>
 */
#[get("/submit")]
fn submit() -> Result<Json<JudgeSubmit>, SomeException> {
    Err(SomeException::build()
        .id(2)
        .message("submit SomeException".to_string()))
}


/// Get judge state.
#[get("/judge_state")]
fn judge_state() -> Result<Json<JudgeState>, NotFound<()>> {
    Ok(Json(JudgeState::build()
        .stage(Running {
            curr_case_idx: 6,
            state: Pass,
            duration: 1600,
        })
        .message("judge state".to_string())))
}


/// Get final result.
#[get("/result")]
fn result() -> Result<Json<JudgeResult>, NotFound<()>> {
    Ok(Json(JudgeResult::Accept(ResultTrace::new(box [
            (0, Pass),
            (1, Pass),
            (2, Fail { expect: "1 + 1 = 2".to_string(), got: "1 + 1 = 3".to_string() }),
    ]))))
}