use std::path::PathBuf;
use std::path::Path;
use std::fs::File;
use super::super::save_file_parser::SaveFileParser;
use super::super::score_categories::ScoreCategory;
use super::super::crew_member::StartingCrewMember;
use super::head::SaveFile;

impl SaveFile {
    pub fn read_from_file(path: &Path) -> Result<SaveFile, String> {
        
        let file: File = match File::open(&path){
            Ok(x) => x,
            Err(_) => return Err(format!("Unable to open save file at {:?}", path)) 
        };

        let mut parser = SaveFileParser::new(file);
        let mut save : SaveFile = Default::default();

        // ////// //
        // Header //
        // ////// //
        save.version = parser.read_i32();
        save.unkown_data_block_1 = parser.read_raw_bytes(4);
        save.ae_content = parser.read_i32();
        save.difficulty = parser.read_i32();
        save.ships_defeated = parser.read_i32();
        save.jumps_in_sector = parser.read_i32();
        save.scrap_collected = parser.read_i32();
        save.crew_recruited = parser.read_i32();

        // //////////////// //
        // Ship Designation //
        // //////////////// //
        save.ship_name = parser.read_string()?;
        save.ship_id = parser.read_string()?;
        save.sector = parser.read_i32();
        save.unkown_data_block_2 = parser.read_raw_bytes(4);

        // //////////////// //
        // Score Categories //
        // //////////////// //
        let num_categories = parser.read_i32();
        for _ in 0..num_categories {
            let name = parser.read_string()?;
            let value = parser.read_i32();
            save.scores.push(
                ScoreCategory::new(name, value)
            )
        }

        // //////////// //
        // Ship Details //
        // //////////// //
        parser.read_string()?;
        parser.read_string()?;
        save.ship_graphics_base_name = parser.read_string()?;

        // ////////////////////// //
        // Starting Crew Overview //
        // ////////////////////// //
        let num_crew = parser.read_i32();
        for i in 0..num_crew {
            save.starting_crew.push(Default::default());
            save.starting_crew.get_mut(i as usize).expect("").race = parser.read_string()?;
            save.starting_crew.get_mut(i as usize).expect("").name = parser.read_string()?;
        }

        // ////////////// //
        // Ship Resources //
        // ////////////// //
        save.hull = parser.read_i32();
        save.fuel = parser.read_i32();
        save.drones = parser.read_i32();
        save.missiles = parser.read_i32();
        save.scrap = parser.read_i32();

        // //////////////////// //
        // The rest of the file //
        // //////////////////// //
        save.unkown_data_block = parser.read_until_eof(); 

        return Ok(save);
    }

    pub fn read_from_local_save() -> Result<SaveFile, String> {
        //let path: PathBuf = [r"C:\","Users","kerstop","Documents","my games","fasterthanlight","continue.sav"].iter().collect();
        let mut path: PathBuf = match home::home_dir() {
            Some(path) => path,
            None => return Err(String::from("Unable to locate users home directory.")),
        };
        path.push(PathBuf::from("Documents/my games/fasterthanlight/continue.sav"));
        SaveFile::read_from_file(path.as_path())
    }

}