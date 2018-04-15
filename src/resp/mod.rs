//! # Resp
//!
//! Kinds of response, all in json format.


/// The response trait.
/// The impl of this trait should have `#[derive(Serialize, Deserialize)]`, in json format.
pub trait Resp {
    fn build() -> Self;
}


pub mod judge_state;
pub mod judge_result;
pub mod simple_test;
pub mod some_exception;
pub mod judge_submit;