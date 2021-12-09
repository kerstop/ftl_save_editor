#![cfg(test)]
use super::*;
use std::io::Read;
use std::path::PathBuf;
use std::path::Path;
use std::fs::File;
use std::io::{Error, ErrorKind};




fn compare_read_and_write(in_file_path:&Path) -> Result<(), std::io::Error> {

    let save = SaveFile::read_from_file(in_file_path)?;

    let mut tmp_buf = in_file_path.to_path_buf();
    tmp_buf.set_extension("sav-tmptest");
    let out_file_path = tmp_buf.as_path();

    save.write_to_file(out_file_path)?;

    let mut file1 = File::open(in_file_path).expect("Unable to open specified save file");
    let mut file2 = File::open(out_file_path).expect("Unable to open temporary save file");

    let mut buf1 : Vec<u8> = Vec::new();
    let mut buf2 : Vec<u8> = Vec::new();
    
    let len_file1 = file1.read_to_end(&mut buf1).expect("Error while reading file");
    let len_file2 = file2.read_to_end(&mut buf2).expect("Error while reading file");

    if len_file1 != len_file2 {
        return Err(Error::new(ErrorKind::InvalidData, "Files are of different length"));
    }

    for (b1, b2) in buf1.into_iter().zip(buf2.into_iter()) {
        if b1 != b2 {
            return Err(Error::new(ErrorKind::InvalidData, "Save file was not read and writen to properly"));
        }
    }

    match std::fs::remove_file(out_file_path){
        Ok(_) => (),
        Err(_) => println!("[WARNING] Unable to clean up temporary files")
    }

    Ok(())
}

#[test]
fn read_and_write() -> Result<(), std::io::Error> {
    
    let mut example_1 = PathBuf::from(file!());
    example_1.pop();
    example_1.push("test_saves/example-1.sav");
    compare_read_and_write(example_1.as_path())?;

    let mut example_2 = PathBuf::from(file!());
    example_2.pop();
    example_2.push("test_saves/example-2.sav");
    compare_read_and_write(example_2.as_path())?;

    let mut example_3 = PathBuf::from(file!());
    example_3.pop();
    example_3.push("test_saves/example-3.sav");
    compare_read_and_write(example_3.as_path())?;

    Ok(())
}