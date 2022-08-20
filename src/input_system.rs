use std::fs;
use crate::Config;

const _BUFFER_SIZE: u32 = 4096;

pub struct InputBuffer {
    bytes: Vec<u8>,
    _begin: usize,
    forward: usize,
}

impl InputBuffer {
    pub fn new(config: Config) -> Result<InputBuffer, &'static str> {
        let _begin = 0;
        let forward = 0;

        let bytes = match fs::read(config.filename) {
            Ok(bytes) => bytes,
            Err(_) => return Err("Couldn't read the file")
        };

        Ok(InputBuffer { bytes, _begin, forward })
    }
}

impl Iterator for InputBuffer {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.forward;

        self.forward = self.forward + 1;

        if index >= self.bytes.len() {
            return None
        }

        Some(self.bytes[index] as char)
    }
}