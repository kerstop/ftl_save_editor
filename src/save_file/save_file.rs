
use std::path::PathBuf;
use std::fs::File;
use super::save_file_parser::SaveFileParser;
use super::score_categories::ScoreCategory;

#[derive(Default, Debug)]
pub struct SaveFile {
    // ////// //
    // Header //
    // ////// //
    pub version:i32,
    pub ae_content: i32,
    pub difficulty: i32,
    pub ships_defeated: i32,
    pub jumps_in_sector: i32,
    pub scrap_collected: i32,
    pub crew_recruited: i32,

    // //////////////// //
    // Ship Designation //
    // //////////////// //
    pub ship_name: String,
    pub ship_id: String,
    pub sector : i32,

    // //////////////// //
    // Score Categories //
    // //////////////// //
    pub scores: Vec<ScoreCategory>,

}

impl SaveFile {
    pub fn read_from_file(path: PathBuf) -> Result<SaveFile, String> {
        
        let file: File = match File::open(&path){
            Ok(x) => x,
            Err(_) => return Err(format!("Unable to open save file at {:?}", path)) 
        };

        let mut parser = SaveFileParser::new(file);
        let mut save : SaveFile = Default::default();

        ////////////
        // Header //
        ////////////
        save.version = parser.read_i32();
        parser.discard_bytes(4);
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
        parser.discard_bytes(4);

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

        return Ok(save);
    }

    pub fn read_from_local_save() -> Result<SaveFile, String> {
        //let path: PathBuf = [r"C:\","Users","kerstop","Documents","my games","fasterthanlight","continue.sav"].iter().collect();
        let mut path: PathBuf = match home::home_dir() {
            Some(path) => path,
            None => return Err(String::from("Unable to locate users home directory.")),
        };
        path.push(PathBuf::from("Documents/my games/fasterthanlight/continue.sav"));
        SaveFile::read_from_file(path)
    }

    #[allow(unused)]
    pub fn print_all_save_info(&self){
        self.print_header();
        println!();
        self.print_ship_designation();
        println!();
        self.print_scores();
    }

    pub fn print_header(&self){
        println!("###Save Header");
        println!("version: {}", self.version);
        println!("ae_content: {}", self.ae_content);
        println!("difficulty: {}", self.difficulty);
        println!("ships_defeated: {}", self.ships_defeated);
        println!("jumps_in_sector: {}", self.jumps_in_sector);
        println!("scrap_collected: {}", self.scrap_collected);
        println!("crew_recruited: {}", self.crew_recruited);
    }

    pub fn print_ship_designation(&self) {
        println!("###Ship designation");
        println!("ship name: {}", self.ship_name);
        println!("ship id: {}", self.ship_id);
        println!("sector: {}", self.sector);
    }
    
    pub fn print_scores(&self) {
        println!("###Scores");
        for score in self.scores.as_slice() {
            println!("{}: {}", score.name, score.value)
        }
    }
}