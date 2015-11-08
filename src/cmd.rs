use std::error::Error;
use std::io::Read;

use iron::prelude::*;
use iron::typemap::Key;

use plugin;

use queryst;

use rustc_serialize::{json, Decodable};

use error;

#[derive(RustcDecodable, Clone, Debug)]
pub struct SlashCommand {
    pub token: String,
    pub team_id: String,
    pub team_domain: String,
    pub channel_id: String,
    pub channel_name: String,
    pub user_id: String,
    pub user_name: String,
    pub command: String,
    pub text: String
}

impl Key for SlashCommand {
    type Value = SlashCommand;
}

impl<'a, 'b> plugin::Plugin<Request<'a, 'b>> for SlashCommand {
    type Error = error::SlashCommandError;

    fn eval(req: &mut Request) -> Result<SlashCommand, error::SlashCommandError> {
        let mut body_string = String::default();
        try!(req.body.read_to_string(&mut body_string));
        let body_json = try!(queryst::parse(&body_string));
        let mut decoder = json::Decoder::new(body_json);
        let result = try!(SlashCommand::decode(&mut decoder));
        Ok(result)
    }
}
