use std::io::Read;
use std::fs::File;

pub struct SaveFileParser {
    file: File
}

impl SaveFileParser{
    pub fn new(file: File) -> SaveFileParser{
        SaveFileParser{file}
    }
    
    #[allow(unused)]
    pub fn discard_bytes(&mut self, bytes: u32) {
        let mut buff: [u8;1] = [0;1];
        for _ in 0..bytes{
            self.file.read(&mut buff);
        }
    }

    #[allow(unused)]
    pub fn read_raw_bytes(&mut self, bytes: u32) -> Vec<u8> {
        let mut buff: Vec<u8> = vec![0;bytes as usize];
        self.file.read(&mut buff.as_mut_slice());
        buff
    }

    #[allow(unused)]
    pub fn read_i32(&mut self) -> i32 {
        let mut buff: [u8;4] = [0;4];
        self.file.read_exact(&mut buff).expect("Unexpected end of file encountered");
        i32::from_le_bytes(buff)
    }

    #[allow(unused)]
    pub fn read_string(&mut self) -> Result<String, String> {
        let size: i32 = self.read_i32();
        let mut buf: Vec<u8> = vec![0;size as usize];
        self.file.read_exact(buf.as_mut_slice());
        match String::from_utf8(buf){
            Ok(s) => Ok(s),
            Err(_) => Err(String::from("Error parsing string"))
        }
    }

    #[allow(unused)]
    pub fn read_until_eof(&mut self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        self.file.read_to_end(&mut buf);
        return buf;
    }
}