//! # Result
//!
//! The final result of judgement, several scenes are as below:
//! + `Accept`: which means your code are correct
//! + `Fail`: failed to pass all the test cases
//! + `Error`: such as `Compile Error`, `Out Of Memory`...
//! + `Panic`: when server arises some mistakes

use resp::judge_state::TestCaseState;


#[derive(Debug, Serialize, Deserialize)]
pub enum JudgeResult {
    Accept(ResultTrace),
    Fail(ResultTrace),
    Error(JudgeErrorType),
    ServerPanic(String),
}


type _ResultTrace = [(usize, TestCaseState)];


/// Trace every test case that had been judged.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResultTrace {
    /// [(test case id * TestCaseState)]
    cases_result:           Box<_ResultTrace>,

    /// judged-cases number
    size:                   usize,
}

impl ResultTrace {
    pub fn new(cases_result: Box<_ResultTrace>) -> Self {
        let size = cases_result.len();
        ResultTrace {
            cases_result,
            size,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JudgeErrorType {
    /// (message)
    CompileError(String),

    /// (message, memory usage)
    OutOfMemory(String, u32),




    // TODO ...




    /// (message, error code)
    RuntimeError(String, i32),
}