use super::super::score_categories::ScoreCategory;

#[derive(Default, Debug)]
pub struct SaveFile {
    // ////// //
    // Header //
    // ////// //
    pub version:i32,
    pub unkown_data_block_1 : Vec<u8>,
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
    pub unkown_data_block_2 : Vec<u8>,
    // //////////////// //
    // Score Categories //
    // //////////////// //
    pub scores: Vec<ScoreCategory>,

    // //////////////////// //
    // The rest of the file //
    // //////////////////// //
    pub unkown_data_block : Vec<u8>,

}

impl SaveFile {
    

    
    
}