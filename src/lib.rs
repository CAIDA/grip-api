#![plugin(rocket_codegen)]
#![feature(plugin)]
#![feature(proc_macro_hygiene)]

extern crate elastic;
extern crate elastic_derive;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

pub mod backend;
