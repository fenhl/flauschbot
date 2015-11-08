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
            pub struct $name(Option<String>);

            impl Default for $name {
                fn default() -> $name {
                    $name(None)
                }
            }

            impl From<String> for $name {
                fn from(msg: String) -> $name {
                    $name(Some(msg))
                }
            }

            impl<'a> From<&'a str> for $name {
                fn from(msg: &str) -> $name {
                    $name(Some(msg.to_owned()))
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
                    let msg = if let Some(ref msg) = err.0 { format!("{}: {}", $msg, msg) } else { $msg.to_owned() };
                    IronError::new(err, (status::BadRequest, msg))
                }
            }
        )*
    };
}

macro_rules! convert_error {
    ($from:ty => $to:ty) => {
        impl From<$from> for $to {
            fn from(e: $from) -> $to {
                format!("{} ({})", stringify!($from), e.description()).into()
            }
        }
    };
    ($from:ty :> $to:ty) => {
        impl From<$from> for $to {
            fn from(_: $from) -> $to {
                stringify!($from).into()
            }
        }
    };
}

errors! {
    AuthError("authentication error");
    SlashCommandError("slash command error");
    WebhookError("error while trying to call Slack incoming webhook");
}

convert_error! { queryst::ParseError :> SlashCommandError }
convert_error! { io::Error => SlashCommandError }
convert_error! { json::DecoderError => SlashCommandError }
