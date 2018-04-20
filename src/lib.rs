#![feature(box_syntax, box_patterns)]
#![feature(const_fn)]
#![feature(custom_derive)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
//#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
//#[macro_use] extern crate lazy_static;
extern crate serde;
extern crate serde_json;

#[macro_use] pub mod macros;
pub mod req;
pub mod resp;
pub mod router;