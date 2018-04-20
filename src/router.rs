//! # Router
//!
//! Access service according to RESTFul api.

use resp::judge_submit::JudgeSubmit;
use resp::judge_state::{JudgeState, TestCaseState::*, Stage::*};
use resp::judge_result::{JudgeResult, TestCaseTrace, JudgeErrorType::*};
use resp::test_code::TestCode;
use resp::ResponseResult;
use resp::ResponseResult::Success;


/// Get code for simple test.
/* TODO
 * #[get("/simple_test/<problem_id>")]
 * fn test_code(cookies: Cookies, problem_id: u32) -> ResponseResult<TestCode>
 */
#[get("/test_code")]
fn test_code() -> ResponseResult<TestCode> {
//    Exception(SomeException::build()
//        .code(0)
//        .message("simple test SomeException test".to_string()))
    Success(TestCode::build()
        .simple_test("code for simple test".to_string())
        .func_code("code for function that will be impl".to_string()))
}


/// Submit code.
/* TODO
 * #[post("/submit", format = "application/json", data = "<solution>")]
 * fn submit(cookies: Cookies, solution: LenientForm<Solution>) -> ResponseResult<JudgeSubmit>
 */
#[get("/submit")]
fn submit() -> ResponseResult<JudgeSubmit> {
    Success(JudgeSubmit::build()
        .id(1024))
}


/// Get judge state.
/* TODO
 * #[get("/judge_state/<submission_id>")]
 * fn judge_state(cookies: Cookies, submission_id: u32) -> ResponseResult<JudgeState>
 */
#[get("/judge_state")]
fn judge_state() -> ResponseResult<JudgeState> {
    Success(JudgeState::build()
        .stage(Exit {
            case_idx:   6,
            state:      Error { error: TimeLimitExceeded { time_used: 120000000 } },
        })
        .message("judge state test".to_string()))
}


/// Get final result.
/* TODO
 * #[get("/result/<submission_id>")]
 * fn result(cookies: Cookies, submission_id: u32) -> ResponseResult<JudgeResult>
 */
#[get("/result")]
fn result() -> ResponseResult<JudgeResult> {
    Success(JudgeResult::WrongAnswer(TestCaseTrace::new(box [
            (0, Pass { time_used: 100, mem_used: 10240 }),
            (1, Pass { time_used: 120, mem_used: 23333 }),
            (2, WrongAnswer { expect: "1 + 1 = 2".to_string(), got: "1 + 1 = 3".to_string() }),
    ])))
}