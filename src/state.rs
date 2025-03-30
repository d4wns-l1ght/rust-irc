use anyhow::Result;
use std::fs::File;
use std::path::PathBuf;

pub struct State {
    _file_path: PathBuf,
    _data: File,
}
impl State {
    fn new(file_path: PathBuf, data: File) -> Self {
        State {
            _file_path: file_path,
            _data: data,
        }
    }
    pub fn build(path: PathBuf) -> Result<Self> {
        // TODO: this should create a new file probably
        let data = File::open(&path)?;
        Ok(Self::new(path, data))
    }
    // mut to prevent multiple threads from writing to the file at the same time
    pub fn save(&mut self) -> Result<()> {
        todo!()
    }
    pub fn _reload_from_file(self) -> Result<Self> {
        todo!()
    }
}
