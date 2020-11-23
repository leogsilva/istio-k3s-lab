#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::http::RawStr;
use std::sync::atomic::{AtomicUsize, Ordering};
use rocket::State;
use serde::Deserialize;
use rocket_contrib::json::{Json, JsonValue};

#[derive(Serialize, Deserialize)]
struct Check {
    api: String,
    verb: String
}

#[post("/", format = "json", data = "<input>")]
fn validate(input: Json<Check>) -> JsonValue {
    let _verb = input.0.verb;
    let _api = input.0.api;
    json!({
        "access": true,
        "message": "Acesso permitido.",
        "scopesUser": [
          10,
          23
        ]
    })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![validate])
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn hello_world() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.post("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, TEST!".into()));
    }
}