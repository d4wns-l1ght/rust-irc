use std::cell::LazyCell;
use std::str::Split;

use anyhow::{Context, Result, anyhow, bail};
use regex::Regex;
use thiserror::Error;

use crate::{Command, CommandKind};

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unrecognised Command: [{0}]")]
    UnrecognisedCommand(String),
    #[error("Malformed/Empty Command: [{0}]")]
    MalformedCommand(String),
}

fn join_str_iter<'a>(iter: impl Iterator<Item = &'a str>) -> String {
    iter.fold(String::new(), |a, b| a + b + " ")
}

// [ ":" prefix SPACE ] command [ params ] crlf
/// Parses an IRC command according to RFC 2812
/// Errors when the command is malformed or unrecognised
pub fn try_parse_from_line(line: &mut str) -> Result<Command> {
    match line.chars().next() {
        Some(':') => {
            let mut iter = line.split(" ");
            Ok(Command {
                prefix: Some(
                    parse_prefix(iter.next().ok_or(anyhow!("Impossibly bad prefix"))?)
                        .context("Bad Command: {line}")?,
                ),
                kind: parse_command(iter).context(format!("\nWhole Line: {line}"))?,
            })
        }
        Some(_c) => Ok(Command {
            prefix: None,
            kind: parse_command(line.split(" ")).context(format!("\nWhole Line: {line}"))?,
        }),
        None => bail!(ParseError::MalformedCommand(line.to_owned())),
    }
}

pub fn regex_match(
    haystack: &str,
    regex_generation: fn() -> Result<Regex, regex::Error>,
) -> Result<String> {
    let re: LazyCell<Result<Regex, regex::Error>> = LazyCell::new(regex_generation);

    match re.as_ref() {
        Ok(r) => {
            if r.is_match(haystack) {
                Ok(haystack.to_owned())
            } else {
                Err(anyhow!("Bad match: {haystack}"))
            }
        }
        Err(e) => Err(e.clone().into()),
    }
}

// servername / ( nickname [ [ "!" user ] "@" host ] )
// (preceded by a ':' that we know is present from try_parse_from_line)
fn parse_prefix(prefix: &str) -> Result<String> {
    let prefix = &prefix[1..];
    if let Ok(pre) = parse_servername(prefix) {
        Ok(pre)
    } else if let Ok(pre) = parse_nickname_etc_for_prefix(prefix) {
        Ok(pre)
    } else {
        Err(anyhow!("Bad prefix: {prefix}"))
    }
}

fn parse_nickname_etc_for_prefix(word: &str) -> Result<String> {
    regex_match(word, || {
        Regex::new(
            r"^(?x)
            (?:[A-Za-z\x5B-\x60\x7B-\x7D][\-A-Za-z0-9\x5B-\x60\x7B-\x7D]{0,8}) # nickname
            (?: # [ [ ! user ] @ host ]
            (?:![\x01-\x07\x08-\x09\x0B-\x0C\x0E-\x1F\x21-\x2B\x2D-\x39\x3B-\xFF]+)? # # [ ! user ]
            (?:@ # @ host
            (?:[A-Za-z0-9][\-A-Za-z0-9]*[A-Za-z0-9]*(?:[\.A-Za-z0-9][\-A-Za-z0-9]*[A-Za-z0-9]*)*) # hostname
            |(?: #hostaddr
            (?:(?:[0-9]{1,3}\.){3}[0-9]) | # ip4addr
            (?:(?:[0-9A-F]+[:0-9A-F]+) | (?:0:0:0:0:0:(?:0F{4}):(?:(?:[0-9]{1,3}\.){3}[0-9]))) # ip6addr
            )
            ))?$",
        )
    })
}

// servername =  hostname
// hostname   =  shortname *( "." shortname )
// shortname  =  ( letter / digit ) *( letter / digit / "-" ) *( letter / digit )
// letter     =  %x41-5A / %x61-7A       ; A-Z / a-z
// digit      =  %x30-39                 ; 0-9
fn parse_servername(word: &str) -> Result<String> {
    regex_match(word, || {
        Regex::new(
            r"^(?:[A-Za-z0-9](?:\-|[A-Za-z0-9])*[A-Za-z0-9]*)(?:\.[A-Za-z0-9](?:\-|[A-Za-z0-9])*[A-Za-z0-9]*)*$",
        )
    })
}

// 1*letter / 3digit
// 1 or more letters / 3 digits
fn parse_command(mut line: Split<'_, &str>) -> Result<CommandKind> {
    let word = match line.next() {
        Some(word) => word,
        None => bail!(ParseError::MalformedCommand(join_str_iter(line))),
    }
    .to_lowercase();

    match word.as_str() {
        "join" => parse_join(line),
        "nick" => todo!(),
        "user" => todo!(),
        "ping" => todo!(),
        "privmsg" => todo!(),
        "quit" => todo!(),
        &_ => bail!(ParseError::UnrecognisedCommand(join_str_iter(line))),
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
        Some(channels) => channels
            .split(",")
            .map(parse_channel)
            .collect::<Result<Vec<String>>>()
            .context("Bad command: {line}")?,
        None => bail!(ParseError::UnrecognisedCommand(join_str_iter(line))),
    };

    let keys = line
        .next()
        .map(|keys| keys.split(",").map(|s| s.to_owned()).collect());

    match line.next() {
        Some(_p) => Err(ParseError::MalformedCommand(join_str_iter(line)).into()),
        None => Ok(CommandKind::Join { channels, keys }),
    }
}

fn parse_channel(channel: &str) -> Result<String> {
    regex_match(channel, || {
        Regex::new(
            r"^(#|\+|!\b[A-Z0-9]{5}\b|&)[\x01-\x07\x08-\x09\x0B-\x0C\x0E-\x1F\x21-\x2B\x2D-\x39\x3B-\xFF]+(?::[\x01-\x07\x08-\x09\x0B-\x0C\x0E-\x1F\x21-\x2B\x2D-\x39\x3B-\xFF]+)?$",
        )
    })
}
