mod save_file;
mod save_file_parser;
mod save_file_writer;
mod score_categories;
mod crew_member;

use save_file::SaveFile;
use std::path::Path;


fn main() {

    let mut save = SaveFile::read_from_local_save().unwrap();
    save.ship_name = "Testing123".to_string();
    save.scrap = 500;
    save.write_to_local_save();
     
}
