use std::fs::File;
use std::io::Write;

pub struct SaveFileWriter {
    file : File,
}

impl SaveFileWriter {
    pub fn new(file : File) -> SaveFileWriter {
        SaveFileWriter { file }
    }

    #[allow(unused)]
    pub fn write_raw_bytes(&mut self, bytes: &[u8]) {
        self.file.write(bytes);
    }

    #[allow(unused)]
    pub fn write_bool(&mut self, b: bool) {
        if b {
            self.write_i32(1);
        } else {
            self.write_i32(0);
        }
    }

    #[allow(unused)]
    pub fn write_i32(&mut self, n: i32) {
        let bytes = n.to_le_bytes();
        self.file.write(&bytes);
    }

    #[allow(unused)]
    pub fn write_string(&mut self, s: &str) {
        self.write_i32(s.len() as i32);
        self.file.write(s.as_bytes());
    }
}