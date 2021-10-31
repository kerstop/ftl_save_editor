#![cfg(test)]
use super::*;
use std::path::PathBuf;


#[test]
fn read_header_v11() -> Result<(), String> {
    let in_file = PathBuf::from("src\\save_file\\tests\\test_saves\\example-1.sav");

    let save = SaveFile::read_from_file(in_file)?;

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

    let save = SaveFile::read_from_file(in_file)?;

    assert_eq!(save.ship_name, "The Kestrel");
    assert_eq!(save.ship_id, "PLAYER_SHIP_HARD");
    assert_eq!(save.sector, 1);

    return Ok(())
}