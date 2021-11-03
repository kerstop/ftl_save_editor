use super::head::SaveFile;
use super::super::save_file_writer::SaveFileWriter;
use std::path::PathBuf;
use std::fs::File;
use std::fs::OpenOptions;

impl SaveFile {
    pub fn write_to_file(&self, path: PathBuf) -> Result<(), String> {
        let file = match OpenOptions::new().write(true).create(true).open(&path) {
            Ok(x) => x,
            Err(_) => return Err(format!("Unable to write to file at {:?}", path))
        };

        let mut writer = SaveFileWriter::new(file);

        // ////// //
        // Header //
        // ////// //
        writer.write_i32(self.version);
        writer.write_raw_bytes(self.unkown_data_block_1.as_slice());
        writer.write_i32(self.ae_content);
        writer.write_i32(self.difficulty);
        writer.write_i32(self.ships_defeated);
        writer.write_i32(self.jumps_in_sector);
        writer.write_i32(self.scrap_collected);
        writer.write_i32(self.crew_recruited);

        // //////////////// //
        // Ship Designation //
        // //////////////// //
        writer.write_string(&self.ship_name);
        writer.write_string(&self.ship_id);
        writer.write_i32(self.sector);
        writer.write_raw_bytes(&self.unkown_data_block_2);

        // //////////////// //
        // Score Categories //
        // //////////////// //
        writer.write_i32(self.scores.len() as i32);
        for i in &self.scores {
            writer.write_string(&i.name);
            writer.write_i32(i.value);
        }

        // //////////// //
        // Ship Details //
        // //////////// //
        writer.write_string(&self.ship_id);
        writer.write_string(&self.ship_name);
        writer.write_string(&self.ship_graphics_base_name);

        // //////////////////// //
        // The rest of the file //
        // //////////////////// //
        writer.write_raw_bytes(self.unkown_data_block.as_slice());


        Ok(())
    }

    pub fn write_to_local_save(&self) -> Result<(), String> {
        let mut path: PathBuf = match home::home_dir() {
            Some(path) => path,
            None => return Err(String::from("Unable to locate users home directory.")),
        };
        path.push(PathBuf::from("Documents/my games/fasterthanlight/continue.sav"));
        self.write_to_file(path);
        Ok(())
    }
}