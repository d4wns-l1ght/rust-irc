use std::str::Split;

use anyhow::{Result, bail};
use thiserror::Error;

use crate::{Command, CommandKind};

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unrecognised Command: {0}")]
    UnrecognisedCommand(String),
    #[error("Malformed/Empty Command: {0}")]
    MalformedCommand(String),
}

// [ ":" prefix SPACE ] command [ params ] crlf
/// Parses an IRC command according to RFC 2812
/// Errors when the command is malformed or unrecognised
pub fn try_parse_from_line(line: &mut str) -> Result<Command> {
    match line.chars().next() {
        Some(':') => {
            let mut iter = line.split(" ");
            Ok(Command {
                prefix: parse_prefix(iter.next())?,
                kind: parse_command(iter)?,
            })
        }
        Some(_c) => Ok(Command {
            prefix: None,
            kind: parse_command(line.split(" "))?,
        }),
        None => bail!(ParseError::MalformedCommand(line.to_owned())),
    }
}

// servername / ( nickname [ [ "!" user ] "@" host ] )
// (preceded by a ':' that we know is present from try_parse_from_line)
fn parse_prefix(line: Option<&str>) -> Result<Option<String>> {
    todo!()
}

// 1*letter / 3digit
// 1 or more letters / 3 digits
fn parse_command(mut line: Split<'_, &str>) -> Result<CommandKind> {
    let word = match line.next() {
        Some(word) => word,
        None => bail!(ParseError::MalformedCommand(line.collect())),
    }
    .to_lowercase();

    match word.as_str() {
        "join" => parse_join(line),
        "nick" => todo!(),
        "user" => todo!(),
        "ping" => todo!(),
        "privmsg" => todo!(),
        "quit" => todo!(),
        &_ => bail!(ParseError::UnrecognisedCommand(line.collect())),
    }
}

fn parse_join(mut line: Split<'_, &str>) -> Result<CommandKind> {
    let channels = match line.next() {
        Some("0") => {
            return Ok(CommandKind::Join {
                channels: vec!["0".to_owned()],
                keys: None,
            });
        }
        Some(channels) => channels.split(",").map(|s| s.to_owned()).collect(),
        None => bail!(ParseError::UnrecognisedCommand(line.collect())),
    };

    let keys = line
        .next()
        .map(|keys| keys.split(",").map(|s| s.to_owned()).collect());

    match line.next() {
        Some(_p) => Err(ParseError::MalformedCommand(line.collect()).into()),
        None => Ok(CommandKind::Join { channels, keys }),
    }
}
