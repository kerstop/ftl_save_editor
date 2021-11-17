//use super::save_file::SaveFile;

impl super::SaveFile {
    
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