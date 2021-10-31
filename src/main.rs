


mod save_file;
mod save_file_parser;
mod score_categories;

use save_file::SaveFile;
use std::path::Path;


fn main() {
    
    let save1 = match SaveFile::read_from_local_save(){
        Ok(save) => save,
        Err(msg) => panic!("{}", msg)
    };
    save1.print_all_save_info();
    println!("///////////");

    let save2 = SaveFile::read_from_file(Path::new("src/save_file/tests/test_saves/example-1.sav")
    .to_path_buf()).expect("s");
    save2.print_all_save_info();
     
}
