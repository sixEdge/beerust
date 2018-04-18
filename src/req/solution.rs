//! # Solution
//!
//! Solution Form.

use rocket::request::FromFormValue;
use rocket::http::RawStr;
use serde_json;


#[derive(Debug, Serialize, Deserialize, FromForm)]
pub struct Solution {
    problem_id:         usize,
    language:           SupportedLanguage,
    domain:             Domain,
    code:               String,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "lang")]
pub enum SupportedLanguage {
    #[serde(rename = "gcc")]
    Gcc,
    #[serde(rename = "g++")]
    GPlusPlus,
    #[serde(rename = "clang")]
    Clang,
    #[serde(rename = "clang++")]
    ClangPlusPlus,
    #[serde(rename = "java")]
    Java,
    #[serde(rename = "python2")]
    Python2,
    #[serde(rename = "python3")]
    Python3,
}

impl<'v> FromFormValue<'v> for SupportedLanguage {
    type Error = serde_json::Error;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, serde_json::Error> {
        serde_json::from_str(form_value.as_ref())
    }
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "domain")]
pub enum Domain {
    #[serde(rename = "problem")]
    Problem,
    #[serde(rename = "contest")]
    Contest,
}

impl<'v> FromFormValue<'v> for Domain {
    type Error = serde_json::Error;

    fn from_form_value(form_value: &'v RawStr) -> Result<Self, serde_json::Error> {
        serde_json::from_str(form_value.as_ref())
    }
}