use crate::commands::*;
use crate::state::State;
use anyhow::{Context, Result};
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::{Arc, RwLock};
use std::thread::{self, JoinHandle};

mod commands;
mod state;

pub fn parse_env_args(_args: Vec<String>) -> Result<String> {
    Ok("".to_owned())
}

pub fn run(path: String) -> Result<()> {
    let state = Arc::new(RwLock::new(State::build(Path::new(&path))?));

    let listener =
        TcpListener::bind("127.0.0.1:1667").context("Failed to bind the TcpListenter")?;

    let mut handles: Vec<JoinHandle<Result<()>>> = Vec::new();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let state = Arc::clone(&state);
                let handle = thread::spawn(move || (handle_client(state, BufReader::new(stream))));
                handles.push(handle);
            }
            Err(e) => println!("Error: {e}"),
        };
    }

    match state.write() {
        Ok(mut state) => state.save_to_file(),
        Err(_e) => todo!(),
    }
}

fn handle_client(state: Arc<RwLock<State>>, mut stream: BufReader<TcpStream>) -> Result<()> {
    let mut buf = String::new();
    loop {
        buf.clear();

        if let Ok(0) = stream.read_line(&mut buf) {
            // EOF
            return Ok(());
        }
        let command = match parse_command(&buf)? {
            Command::Quit { quit_message: _ } => return Ok(()),
            command => command,
        };
        let mut state = match state.write() {
            Ok(state) => state,
            Err(_e) => todo!(),
        };
        if let Err(e) = state.apply_command(command) {
            println!("error: {e}");
        }
        if let Err(e) = state.save_to_file() {
            println!("error: {e}");
        }
    }
}

fn parse_command(line: &str) -> Result<Command> {
    Ok(Command::Join {
        channels: Vec::new(),
        keys: None,
    })
}
