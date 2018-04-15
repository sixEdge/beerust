#![feature(box_syntax, box_patterns)]
#![feature(const_fn)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
extern crate serde;
extern crate serde_json;

pub mod resp;
pub mod router;