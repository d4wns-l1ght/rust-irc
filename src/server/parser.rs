use std::cell::LazyCell;
use std::str::Split;

use anyhow::{Context, Result, anyhow, bail};
use regex::{Captures, Regex};
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
        .trim_end()
        .to_owned()
}

// [ ":" prefix SPACE ] command [ params ] crlf
/// Parses an IRC command according to RFC 2812
/// Errors when the command is malformed or unrecognised
/// You can assume that any text-based limitations (allowed chars, length, etc) are assured by this function
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
                Err(anyhow!("Bad match: [{haystack}]"))
            }
        }
        Err(e) => Err(e.clone().into()),
    }
}

pub fn regex_capture(
    haystack: &str,
    regex_generation: fn() -> Result<Regex, regex::Error>,
) -> Result<Captures<'_>> {
    let re: LazyCell<Result<Regex, regex::Error>> = LazyCell::new(regex_generation);

    match re.as_ref() {
        Ok(r) => {
            if let Some(caps) = r.captures(haystack) {
                Ok(caps)
            } else {
                Err(anyhow!("Bad match: [{haystack}]"))
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
        None => bail!(ParseError::MalformedCommand(join_str_iter(&mut line))),
    }
    .to_lowercase();

    match word.as_str() {
        "join" => parse_join(line),
        "nick" => parse_nick(line),
        "user" => parse_user(&join_str_iter(line)),
        "ping" => parse_ping(&join_str_iter(line)),
        "privmsg" => parse_privmsg(&join_str_iter(line)),
        "quit" => parse_quit(&join_str_iter(line)),
        x => bail!(ParseError::UnrecognisedCommand(format!(
            "{} {}",
            x,
            join_str_iter(&mut line)
        ))),
    }
}

fn parse_channel(channel: &str) -> Result<String> {
    regex_match(channel, || {
        Regex::new(
            r"^[#&][^\x00\x07\x0A\x0D,\x20]{1,200}$",
        )
    })
}

// <channel>{,<channel>} [<key>{,<key>}]
// channel = ('#' | '&') <chstring>
// chstring = <any 8bit code except SPACE, BELL, NUL, CR, LF and comma (',')>
fn parse_join(mut line: Split<'_, &str>) -> Result<CommandKind> {
    let channels = match line.next() {
        Some("0") => match line.next() {
            Some(_p) => return Err(ParseError::MalformedCommand(join_str_iter(&mut line)).into()),
            None => {
                return Ok(CommandKind::Join {
                    channels: vec!["0".to_owned()],
                    keys: None,
                });
            }
        },
        Some(channels) => channels
            .split(",")
            .map(parse_channel)
            .collect::<Result<Vec<String>>>()
            .context("Bad command: {line}")?,
        None => bail!(ParseError::UnrecognisedCommand(join_str_iter(&mut line))),
    };

    let keys = line
        .next()
        .map(|keys| keys.split(",").map(|s| s.to_owned()).collect());

    match line.next() {
        Some(_p) => Err(ParseError::MalformedCommand(join_str_iter(&mut line)).into()),
        None => Ok(CommandKind::Join { channels, keys }),
    }
}

// Parameters: <nickname>
fn parse_nick(mut line: Split<'_, &str>) -> Result<CommandKind> {
    let caps = regex_capture(
        line.next()
            .ok_or(ParseError::UnrecognisedCommand(join_str_iter(&mut line)))?,
        || {
            Regex::new(
                r"^(?<nickname>[A-Za-z\x5B-\x60\x7B-\x7D][\-A-Za-z0-9\x5B-\x60\x7B-\x7D]{0,8})$",
            )
        },
    )?;

    let nickname = caps
        .name("nickname")
        .ok_or(ParseError::MalformedCommand(join_str_iter(&mut line)))?
        .as_str();

    match line.next() {
        Some(_p) => Err(ParseError::MalformedCommand(join_str_iter(&mut line)).into()),
        None => Ok(CommandKind::Nick {
            nickname: nickname.to_owned(),
        }),
    }
}

// Parameters: <user> <mode> <unused> <realname>
fn parse_user(line: &str) -> Result<CommandKind> {
    let caps = regex_capture(line, || {
        Regex::new(
            r"^(?x)
        (?<user>[\x01-\x07\x08-\x09\x0B-\x0C\x0E-\x1F\x21-\x2B\x2D-\x39\x3B-\xFF]+)
        \s(?<mode>[0-9])\s\*\s:
        (?<realname>[\s\x01-\x07\x08-\x09\x0B-\x0C\x0E-\x1F\x21-\x2B\x2D-\x39\x3B-\xFF]+)$",
        )
    })?;
    let user_name = caps
        .name("user")
        .ok_or(ParseError::MalformedCommand(line.to_owned()))?
        .as_str();
    let mode = caps
        .name("mode")
        .ok_or(ParseError::MalformedCommand(line.to_owned()))?
        .as_str()
        .parse::<u8>()?;
    let real_name = caps
        .name("realname")
        .ok_or(ParseError::MalformedCommand(line.to_owned()))?
        .as_str();

    Ok(CommandKind::User {
        user_name: user_name.to_owned(),
        mode,
        real_name: real_name.to_owned(),
    })
}

fn parse_ping(line: &str) -> Result<CommandKind> {
    todo!()
}

fn parse_privmsg(line: &str) -> Result<CommandKind> {
    todo!()
}

fn parse_quit(line: &str) -> Result<CommandKind> {
    todo!()
}
