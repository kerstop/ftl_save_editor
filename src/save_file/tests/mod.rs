#![cfg(test)]
use super::*;
use std::io::Read;
use std::path::PathBuf;
use std::path::Path;
use std::fs::File;


#[test]
fn read_header_v11() -> Result<(), String> {
    let in_file = PathBuf::from("src\\save_file\\tests\\test_saves\\example-1.sav");

    let save = SaveFile::read_from_file(in_file.as_path())?;

    assert_eq!(save.version, 11);
    assert_eq!(save.ae_content, 1);
    assert_eq!(save.difficulty, 2);
    assert_eq!(save.ships_defeated, 0);
    assert_eq!(save.jumps_in_sector, 1);
    assert_eq!(save.scrap_collected, 0);
    assert_eq!(save.crew_recruited, 3);

    return Ok(())
}

#[test]
fn read_ship_designation_v11() -> Result<(), String> {
    let in_file = PathBuf::from("src\\save_file\\tests\\test_saves\\example-1.sav");

    let save = SaveFile::read_from_file(in_file.as_path())?;

    assert_eq!(save.ship_name, "The Kestrel");
    assert_eq!(save.ship_id, "PLAYER_SHIP_HARD");
    assert_eq!(save.sector, 1);

    return Ok(())
}


fn compare_read_and_write(in_file_path:&Path) -> Result<(), String> {

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
        return Err("Files are of different length".to_string());
    }

    for (b1, b2) in buf1.into_iter().zip(buf2.into_iter()) {
        if b1 != b2 {
            return Err("Save file was not read and writen to properly".to_string());
        }
    }

    match std::fs::remove_file(out_file_path){
        Ok(_) => (),
        Err(_) => println!("[WARNING] Unable to clean up temporary files")
    }

    Ok(())
}

#[test]
fn read_and_write() -> Result<(), String> {
    
    let example_1 = Path::new("src\\save_file\\tests\\test_saves\\example-1.sav");
    match compare_read_and_write(example_1) {
        Ok(_) => (),
        Err(e) => return Err(e)
    }

    let example_2 = Path::new("src\\save_file\\tests\\test_saves\\example-2.sav");
    match compare_read_and_write(example_2) {
        Ok(_) => (),
        Err(e) => return Err(e)
    }

    let example_3 = Path::new("src\\save_file\\tests\\test_saves\\example-3.sav");
    match compare_read_and_write(example_3) {
        Ok(_) => (),
        Err(e) => return Err(e)
    }

    Ok(())
}