
extern crate ftl_save_editor;

use ftl_save_editor::SaveFile;

fn main() {
    
    let mut save = SaveFile::read_from_local_save().unwrap();
    save.ship_name = "Testing123".to_string();
    save.scrap = 500;
    for i in &mut save.crew {
        i.name = String::from("DerpBot9000")
    }
    save.write_to_local_save().unwrap();
     
}