use std::fmt;
use std::error::Error;
use std::io::{self, Read};

use iron::status;
use iron::prelude::*;
use iron::typemap::Key;

use plugin;

use queryst;

use rustc_serialize::{json, Decodable};

#[derive(Clone, Debug)]
pub struct SlashCommandError;

impl fmt::Display for SlashCommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt("slash command error", f)
    }
}

impl Error for SlashCommandError {
    fn description(&self) -> &str {
        "slash command error"
    }
}

impl From<queryst::ParseError> for SlashCommandError {
    fn from(_: queryst::ParseError) -> SlashCommandError { SlashCommandError }
}

impl From<io::Error> for SlashCommandError {
    fn from(_: io::Error) -> SlashCommandError { SlashCommandError }
}

impl From<json::DecoderError> for SlashCommandError {
    fn from(_: json::DecoderError) -> SlashCommandError { SlashCommandError }
}

impl From<SlashCommandError> for IronError {
    fn from(err: SlashCommandError) -> IronError {
        IronError::new(err, (status::BadRequest, "slash command error"))
    }
}

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
    type Error = SlashCommandError;

    fn eval(req: &mut Request) -> Result<SlashCommand, SlashCommandError> {
        let mut body_string = String::default();
        try!(req.body.read_to_string(&mut body_string));
        let body_json = try!(queryst::parse(&body_string));
        let mut decoder = json::Decoder::new(body_json);
        let result = try!(SlashCommand::decode(&mut decoder));
        Ok(result)
    }
}
