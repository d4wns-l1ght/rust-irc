use crate::commands::*;
use anyhow::Result;
use std::fs::File;
use std::path::Path;

pub struct State {
    file: File,
}
impl State {
    fn new(file: File) -> Self {
        State { file }
    }
    pub fn build(path: &Path) -> Result<Self> {
        Ok(Self::new(File::open(path)?))
    }
    pub fn save_to_file(&mut self) -> Result<()> {
        Ok(())
    }
    pub fn reload_from_file(self) -> Result<Self> {
        Ok(Self::new(self.file))
    }
    pub fn apply_command(&mut self, command: Command) -> Result<()> {
        match command {
            Command::Join {
                channels: _,
                keys: _,
            } => Ok(()),
            Command::Nick { nickname: _ } => Ok(()),
            Command::User {
                username: _,
                full_name: _,
            } => Ok(()),
            Command::Ping {
                source_server: _,
                target_server: _,
            } => Ok(()),
            Command::PrivMsg {
                message_target: _,
                message_text: _,
            } => Ok(()),
            Command::Quit { quit_message: _ } => Ok(()),
        }
    }
}
