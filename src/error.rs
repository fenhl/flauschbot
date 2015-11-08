use std::{fmt, io};
use std::error::Error;

use iron::status;
use iron::prelude::*;

use queryst;

use rustc_serialize::json;

macro_rules! errors {
    ($($name:ident($msg:expr);)*) => {
        $(
            #[derive(Debug)]
            pub struct $name(Option<&'static str>);

            impl Default for $name {
                fn default() -> $name {
                    $name(None)
                }
            }

            impl From<&'static str> for $name {
                fn from(msg: &'static str) -> $name {
                    $name(Some(msg))
                }
            }

            impl fmt::Display for $name {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    try!(fmt::Display::fmt($msg, f));
                    if let Some(ref msg) = self.0 {
                        try!(fmt::Display::fmt(": ", f));
                        try!(fmt::Display::fmt(msg, f));
                    }
                    Ok(())
                }
            }

            impl Error for $name {
                fn description(&self) -> &str {
                    if let Some(ref msg) = self.0 {
                        msg
                    } else {
                        $msg
                    }
                }
            }

            impl From<$name> for IronError {
                fn from(err: $name) -> IronError {
                    let msg = if let Some(msg) = err.0 { format!("{}: {}", $msg, msg) } else { $msg.to_owned() };
                    IronError::new(err, (status::BadRequest, msg))
                }
            }
        )*
    }
}

macro_rules! convert_errors {
    ($($from:ty :> $to:path;)*) => {
        $(
            impl From<$from> for $to {
                fn from(_: $from) -> $to { $to(Some(stringify!($from))) }
            }
        )*
    };
}

errors! {
    AuthError("authentication error");
    SlashCommandError("slash command error");
    WebhookError("error while trying to call Slack incoming webhook");
}
convert_errors! {
    queryst::ParseError :> SlashCommandError;
    io::Error :> SlashCommandError;
    json::DecoderError :> SlashCommandError;
}
