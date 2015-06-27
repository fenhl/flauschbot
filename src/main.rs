extern crate curl;
extern crate iron;
#[macro_use] extern crate lazy_static;
extern crate router;
extern crate rustc_serialize;

use std::fmt;
use std::error::Error;
use std::io::Read;

use curl::http;

use iron::status;
use iron::prelude::*;

use router::Router;

use rustc_serialize::json::{self, Json};

macro_rules! errors {
    ($($name:ident($msg:expr);)*) => {
        $(
            #[derive(Debug)]
            pub struct $name;

            impl fmt::Display for $name {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    fmt::Display::fmt($msg, f)
                }
            }

            impl Error for $name {
                fn description(&self) -> &str {
                    $msg
                }
            }
        )*
    }
}

errors! {
    Nyi("not yet implemented");
    WebhookError("error while trying to call Slack incoming webhook");
}

fn nyi() -> IronError {
    IronError::new(Nyi, status::NotImplemented)
}

#[derive(RustcDecodable)]
struct Config {
    incoming_webhook: String
}

lazy_static! {
    static ref CONFIG: Config = json::decode(include_str!("../assets/config.json")).unwrap();
}

fn veto(req: &mut Request) -> IronResult<Response> {
    println!("DEBUG] req: {:?}", req);
    let mut body = String::default();
    try!(req.body.read_to_string(&mut body).map_err(|_| IronError::new(WebhookError, status::InternalServerError)));
    println!("DEBUG] req.body: {:?}", body);
    let request_json = Json::Object(vec![("text".to_owned(), Json::String("Veto command test".to_owned()))].into_iter().collect());
    let request_body = json::encode(&request_json).unwrap();
    try!(http::handle().post(&CONFIG.incoming_webhook[..], &request_body).exec().map_err(|_| IronError::new(WebhookError, status::InternalServerError)));
    Err(nyi()) //TODO
}

fn main() {
    // route
    let mut router = Router::new();
    router.post("/veto", veto);
    // serve
    Iron::new(router).http("0.0.0.0:18802").unwrap();
}
