#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use reqwest; 
use rocket::{
   catch, catchers, 
   fairing::{Fairing, Info, Kind},
   get, 
   http::{Cookie, CookieJar},
   launch, post, 
   response::status::Created,
   routes, uri, Date, Reqwest, State,
};
use rocket_contrib::{
    json, 
    json::{Json, JsonValue, JsonError},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::RwLock;

#[get("/")]
fn hello_world() -> &'static str {
    "Hello World!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

