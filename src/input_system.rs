use std::fs::File;
use std::io::Read;
use crate::Config;

const BUFFER_SIZE: usize = 10;
const BUFFER_A_EOF: usize = BUFFER_SIZE - 1;
const BUFFER_B_EOF: usize = (2 * BUFFER_SIZE) - 1;

pub struct DoubleBuffer {
    file: File,
    buffer: [u8; 2 * BUFFER_SIZE],
    begin: usize,
    forward: usize,
}

impl DoubleBuffer {
    pub fn new(config: Config) -> Result<DoubleBuffer, &'static str> {
        let begin = 0;
        let forward = 0;

        let mut file = match File::open(config.filename) {
            Ok(file) => file,
            Err(_) => return Err("Couldn't read the file")
        };

        let mut buffer = [0u8; 2 * BUFFER_SIZE];

        match file.read(&mut buffer[0..BUFFER_A_EOF]) {
            Ok(_) => (),
            Err(_) => return Err("Couldn't read the file")
        };

        Ok(DoubleBuffer { file, buffer, begin, forward })
    }
}

impl Iterator for DoubleBuffer {
    type Item = Result<char, &'static str>;

    fn next(&mut self) -> Option<Self::Item> {
        let c = match self.buffer[self.forward] {
            /*
             * If EOF (represented by the byte 0) is found, we need to find out if it's
             * the end of file or the end of one of the buffers.
             * End of file -> Stop analysis.
             * End of buffer -> Load the other buffer and move the forward index.
             */
            0u8 => {
                if self.forward == BUFFER_A_EOF {
                    // Load second buffer reading the next characters
                    let n = match self.file.read(&mut self.buffer[BUFFER_SIZE..BUFFER_B_EOF]) {
                        Ok(n) => (n),
                        Err(_) => return Some(Err("Couldn't read the file"))
                    };

                    /* Check if we read less bytes than BUFFER_SIZE it means we reached the
                     * end of file. We need to insert 0 after the last char to represent EOF.
                    */
                    if n < BUFFER_SIZE - 1 {
                        self.buffer[BUFFER_A_EOF + n + 1] = 0u8;
                    }

                    // Move the index to the start of the second buffer
                    self.forward = self.forward + 1;
                } else if self.forward == BUFFER_B_EOF {
                    // Load first buffer reading the next characters
                    let n = match self.file.read(&mut self.buffer[0..BUFFER_A_EOF]) {
                        Ok(n) => (n),
                        Err(_) => return Some(Err("Couldn't read the file"))
                    };

                    /* Check if we read less bytes than BUFFER_SIZE it means we reached the
                    * end of file. We need to insert 0 after the last char to represent EOF.
                   */
                    if n < BUFFER_SIZE - 1 {
                        self.buffer[n + 1] = 0u8;
                    }

                    // Move the index to the start of the first buffer
                    self.forward = 0;
                } else {
                    // We reached the end of file return None to stop the loop
                    return None;
                }
                self.buffer[self.forward] as char
            }
            _ => self.buffer[self.forward] as char
        };

        self.forward = self.forward + 1;
        Some(Ok(c))
    }
}