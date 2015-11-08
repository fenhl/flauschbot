extern crate curl;
extern crate iron;
#[macro_use] extern crate lazy_static;
extern crate plugin;
extern crate queryst;
extern crate regex;
extern crate router;
extern crate rustc_serialize;

mod cmd;
mod error;

use std::error::Error;

use curl::http;

use iron::status;
use iron::prelude::*;

use router::Router;

use rustc_serialize::json::{self, Json};

use cmd::SlashCommand;

#[derive(RustcDecodable)]
struct Config {
    incoming_webhook: String,
    token: String
}

lazy_static! {
    static ref CONFIG: Config = json::decode(include_str!("../assets/config.json")).unwrap();
}

fn veto(req: &mut Request) -> IronResult<Response> {
    let cmd = try!(req.get::<SlashCommand>());
    if &cmd.token != &CONFIG.token[..] {
        return Err(IronError::new(error::AuthError::from("invalid token"), status::Unauthorized));
    }
    let request_json = Json::Object(vec![("text".to_owned(), Json::String(format!("User {:?} has been vetoed", cmd.text)))].into_iter().collect());
    let request_body = json::encode(&request_json).unwrap();
    try!(http::handle().post(&CONFIG.incoming_webhook[..], &request_body).exec().map_err(|e| IronError::new(error::WebhookError::from(e.description()), status::InternalServerError)));
    Ok(Response::with(status::Ok))
}

fn main() {
    // route
    let mut router = Router::new();
    router.post("/veto", veto);
    // serve
    Iron::new(router).http("0.0.0.0:18802").unwrap();
}
