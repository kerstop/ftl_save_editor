mod save_file;
mod save_file_parser;
mod save_file_writer;
mod score_categories;
mod crew_member;

use save_file::SaveFile;
use std::path::Path;


fn main() {

    let mut save2 = SaveFile::read_from_local_save().expect("s");
    save2.print_ship_designation();
    save2.ship_name = "Testing123".to_string();
    save2.print_ship_designation();
    save2.write_to_local_save();
     
}
