#![feature(proc_macro_hygiene)]

extern crate rocket_contrib;
extern crate elastic_derive;
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate elastic;
extern crate maud;

pub mod backend;
