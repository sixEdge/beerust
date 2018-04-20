#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate beerust;

use rocket::config::{Config, Environment};
use beerust::router::*;


const PATH_TO_BASE: &'static str = "/j";


fn rocket() -> rocket::Rocket {
    let config = Config::build(Environment::Development /* Production */)
        .address("localhost")
        .port(8080)
        .workers(4).unwrap();

    rocket::custom(config, true)
        .mount(PATH_TO_BASE,    routes![test_code])
        .mount(PATH_TO_BASE,    routes![submit])
        .mount(PATH_TO_BASE,    routes![judge_state])
        .mount(PATH_TO_BASE,    routes![result])
}


fn main() {
    rocket().launch();
}