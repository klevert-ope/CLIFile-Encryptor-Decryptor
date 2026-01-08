use std::fs;
use std::path::PathBuf;

pub fn read_file(path: &PathBuf) -> std::io::Result<Vec<u8>> {
    fs::read(path)
}

pub fn write_file(path: &PathBuf, data: &Vec<u8>) -> std::io::Result<()> {
    fs::write(path, data)
}
