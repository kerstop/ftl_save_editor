use std::io::Read;
use std::fs::File;
use std::io::{Error, ErrorKind};

pub struct SaveFileParser {
    file: File
}

impl SaveFileParser{
    pub fn new(file: File) -> SaveFileParser{
        SaveFileParser{file}
    }
    
    #[allow(unused)]
    pub fn discard_bytes(&mut self, bytes: u32)-> Result<(), Error> {
        let mut buff: [u8;1] = [0;1];
        for _ in 0..bytes{
            self.file.read_exact(&mut buff)?;
        }
        Ok(())

    }

    #[allow(unused)]
    pub fn read_raw_bytes(&mut self, bytes: u32) -> Result<Vec<u8>, Error> {
        let mut buff: Vec<u8> = vec![0;bytes as usize];
        self.file.read_exact(&mut buff.as_mut_slice())?;
        Ok(buff)
    }

    #[allow(unused)]
    pub fn read_i32(&mut self) -> Result<i32, Error> {
        let mut buff: [u8;4] = [0;4];
        self.file.read_exact(&mut buff)?;
        Ok(i32::from_le_bytes(buff))
    }

    #[allow(unused)]
    pub fn read_string(&mut self) -> Result<String, Error> {
        let size: i32 = self.read_i32()?;
        let mut buf: Vec<u8> = vec![0;size as usize];
        self.file.read_exact(buf.as_mut_slice());
        match String::from_utf8(buf){
            Ok(s) => Ok(s),
            Err(_) => Err(Error::new(ErrorKind::InvalidData, "Attempted to read invalid utf-8 characters"))
        }
    }

    #[allow(unused)]
    pub fn read_bool(&mut self) -> Result<bool,Error> {
        return Ok(self.read_i32()? % 2 == 1);
    }

    #[allow(unused)]
    pub fn read_until_eof(&mut self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();
        self.file.read_to_end(&mut buf);
        return buf;
    }
}