//! # Judge State
//!
//! The state/stage of judge after submitting code,
//! is possible to change every moment.

use resp::Resp;
use resp::judge_state::Stage::New;


/// Judge state.
/// Use it while judging.
#[derive(Debug, Serialize, Deserialize)]
pub struct JudgeState {
    /// Stage of judge
    stage:              Stage,
    message:            String,
}

impl JudgeState {
    pub fn stage(mut self, stage: Stage) -> Self {
        self.stage = stage;
        self
    }

    pub fn message(mut self, message: String) -> Self {
        self.message = message;
        self
    }
}

impl Resp for JudgeState {
    fn build() -> Self {
        JudgeState { stage: New, message: "".to_string() }
    }
}


/// Judge stage
#[derive(Debug, Serialize, Deserialize)]
pub enum Stage {
    /// a new submission
    New,

    /// program is queuing...
    Queuing {
        /// the number of submissions waiting ahead of yours
        waiting_ahead:      usize,
    },

    /// program is compiling...
    Compiling,

    /// program is running...
    Running {
        /// the index of current test case
        curr_case_idx:      usize,
        /// state
        state:              TestCaseState,
        /// running time in `ms`
        duration:           u32,
    },

    /// be stopped (e.g. queuing out of time) or complete
    EXIT,
}


/// State of a case test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestCaseState {
    Testing,
    Pass,
    Fail {
        expect: String,
        got:    String,
    },
}