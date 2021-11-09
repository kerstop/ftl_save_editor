use super::super::score_categories::ScoreCategory;
use super::super::crew_member::CrewMember;

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

    // //////////// //
    // Ship Details //
    // //////////// //
    //ship name
    //ship id
    pub ship_graphics_base_name: String,

    // ///////////// //
    // Crew Overview //
    // ///////////// //
    pub crew: Vec<CrewMember>,

    // ////////////// //
    // Ship Resources //
    // ////////////// //
    pub hull: i32,
    pub fuel: i32,
    pub drones: i32,
    pub missiles: i32,
    pub scrap: i32,

    // //////////////////// //
    // The rest of the file //
    // //////////////////// //
    pub unkown_data_block : Vec<u8>,

}

impl SaveFile {
    

    
    
}