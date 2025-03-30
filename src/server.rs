use anyhow::Result;
use std::{
    io::{BufRead, BufReader, BufWriter},
    net::{Shutdown, TcpStream},
    sync::{Arc, RwLock},
};

#[cfg(test)]
mod tests;

mod parser;
pub use crate::server::parser::try_parse_from_line;

use crate::{Command, CommandKind, state::State};

pub fn handle_client(state: Arc<RwLock<State>>, stream: TcpStream) -> Result<()> {
    let mut stream_writer = BufWriter::new(stream.try_clone()?);
    let mut stream_reader = BufReader::new(stream);

    let mut buf = String::new();
    loop {
        buf.clear();

        match stream_reader.read_line(&mut buf) {
            Ok(0) => {
                // If, for some other reason, a client connection is closed without  the
                // client  issuing  a  QUIT  command  (e.g.  client  dies and EOF occurs
                // on socket), the server is required to fill in the quit  message  with
                // some sort  of  message  reflecting the nature of the event which
                // caused it to happen.
                let mut state = match state.write() {
                    Ok(state) => state,
                    Err(_e) => todo!(),
                };
                apply_command(
                    &mut state,
                    Command {
                        prefix: None,
                        kind: CommandKind::Quit {
                            quit_message: Some("Socket disconnected".to_owned()),
                        },
                    },
                    stream_writer,
                );
                return Ok(());
            }
            Ok(i) if i >= 513 => todo!(), // max len is 512 bytes
            Ok(_) => (),
            // Happens if a non UTF-8 byte is read. try to recover by attempting to parse
            // leftover bytes in buf?
            Err(_e) => todo!(),
        }
        let command = try_parse_from_line(&mut buf)?;
        let mut state = match state.write() {
            Ok(state) => state,
            Err(_e) => todo!(),
        };
        stream_writer = match apply_command(&mut state, command, stream_writer) {
            Some(stream) => stream,
            None => return Ok(()),
        };

        if let Err(e) = state.save() {
            println!("error: {e}");
        }
    }
}

fn apply_command(
    state: &mut State,
    command: Command,
    stream: BufWriter<TcpStream>,
) -> Option<BufWriter<TcpStream>> {
    match command.kind {
        CommandKind::Join {
            channels: _,
            keys: _,
        } => Some(stream),
        CommandKind::Nick { nickname: _ } => todo!(),
        CommandKind::User {
            username: _,
            full_name: _,
        } => Some(stream),
        CommandKind::Ping {
            source_server: _,
            target_server: _,
        } => Some(stream),
        CommandKind::PrivMsg {
            message_target: _,
            message_text: _,
        } => Some(stream),
        CommandKind::Quit { quit_message } => {
            quit(state, quit_message, stream);
            None
        }
    }
}

fn quit(state: &mut State, comment: Option<String>, connection: BufWriter<TcpStream>) {
    let connection = connection.into_inner().unwrap();
    if let Err(_e) = connection.shutdown(Shutdown::Both) {
        todo!();
    };
    todo!()
}
