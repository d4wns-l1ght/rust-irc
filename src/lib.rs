use crate::commands::*;
use crate::state::State;
use anyhow::{Context, Result};
use std::net::TcpListener;
use std::path::PathBuf;
use std::sync::{
    Arc, RwLock,
    atomic::{AtomicBool, Ordering},
};
use std::thread::{self, JoinHandle};

mod commands;
mod server;
mod state;

pub fn run(path: String) -> Result<()> {
    let state = Arc::new(RwLock::new(State::build(PathBuf::from(&path))?));
    let exit_flag = Arc::new(AtomicBool::new(false));

    check_for_exit(&state, &exit_flag);

    let listener =
        TcpListener::bind("127.0.0.1:1667").context("Failed to bind the TcpListenter")?;

    let mut handles: Vec<JoinHandle<Result<()>>> = Vec::new();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let state = Arc::clone(&state);
                let handle = thread::spawn(move || (server::handle_client(state, stream)));
                handles.push(handle);
            }
            // Connection failed
            Err(e) => println!("Error: {e}"),
        };
    }

    match state.write() {
        Ok(mut state) => state.save(),
        Err(_e) => todo!(),
    }
}

pub fn parse_env_args(_args: Vec<String>) -> Result<String> {
    todo!()
}

fn check_for_exit(state: &Arc<RwLock<State>>, exit_flag: &Arc<AtomicBool>) {
    let state = Arc::clone(state);
    let exit_flag = Arc::clone(exit_flag);

    thread::spawn(move || {
        loop {
            if exit_flag.load(Ordering::Relaxed) {
                break;
            }
        }
        match state.write() {
            Ok(mut state) => state.save(),
            Err(_e) => todo!(),
        }
    });
}
