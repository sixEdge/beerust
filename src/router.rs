//! # Router
//!
//! Access service according to RESTFul api.

use rocket::response::status::NotFound;
use resp::some_exception::SomeException;
use resp::judge_submit::JudgeSubmit;
use resp::judge_state::{JudgeState, TestCaseState::*, Stage::*};
use resp::judge_result::{JudgeResult, TestCaseTrace, JudgeErrorType::*};


/// Get code for simple test.
/* TODO
 * #[get("/simple_test/<sid>")]
 * fn simple_test(cookies: Cookies) -> Result<JudgeState, NotFound<()>>
 */
#[get("/simple_test")]
fn simple_test() -> Result<JudgeState, NotFound<()>> {
    Err(NotFound(()))
}


/// Submit code.
/* TODO
 * #[post("/submit", format = "application/json", data = "<solution>")]
 * fn submit(cookies: Cookies, solution: LenientForm<Solution>) -> Result<Submit, SomeException>
 */
#[get("/submit")]
fn submit() -> Result<JudgeSubmit, SomeException> {
    Err(SomeException::build()
        .code(2)
        .message("submit SomeException test".to_string()))
}


/// Get judge state.
/* TODO
 * #[get("/judge_state/<sid>")]
 * fn judge_state(cookies: Cookies, sid: u32) -> Result<JudgeState, NotFound<()>>
 */
#[get("/judge_state")]
fn judge_state() -> Result<JudgeState, NotFound<()>> {
    Ok(JudgeState::build()
        .stage(Exit {
            case_idx:   6,
            state:      Error { error: TimeLimitExceeded { time_used: 120000000 } },
        })
        .message("judge state test".to_string()))
}


/// Get final result.
/* TODO
 * #[get("/result/<sid>")]
 * fn result(cookies: Cookies, sid: u32) -> Result<JudgeResult, NotFound<()>>
 */
#[get("/result")]
fn result() -> Result<JudgeResult, NotFound<()>> {
    Ok(JudgeResult::WrongAnswer(TestCaseTrace::new(box [
            (0, Pass { time_used: 100, mem_used: 10240 }),
            (1, Pass { time_used: 120, mem_used: 23333 }),
            (2, WrongAnswer { expect: "1 + 1 = 2".to_string(), got: "1 + 1 = 3".to_string() }),
    ])))
}