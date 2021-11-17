mod save_file;



use save_file::SaveFile;
use std::path::Path;


fn main() {
    let x = save_file::crew_member::StartingCrewMember{name:"s".to_string(),race:"s".to_string()};
    let mut save = SaveFile::read_from_local_save().unwrap();
    save.ship_name = "Testing123".to_string();
    save.scrap = 500;
    save.write_to_local_save();
     
}
