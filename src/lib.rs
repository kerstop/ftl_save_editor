



mod tests;

//function definitions
mod impl_print;
mod impl_read;
mod impl_write;

//data types
mod score_categories;
mod crew_member;
mod animation_state;

mod save_file_parser;
mod save_file_writer;

use crew_member::{CrewMember, StartingCrewMember};
use score_categories::ScoreCategory;




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

    // ////////////////////// //
    // Starting Crew Overview //
    // ////////////////////// //
    pub starting_crew: Vec<StartingCrewMember>,

    // ///////////// //
    // Opponent info //
    // ///////////// //
    pub opponent_hostility: bool,
    pub opponent_jump_charge: i32,
    pub opponent_is_jumping: bool,
    pub opponent_jump_animation_ticks: i32,

    // ////////////// //
    // Ship Resources //
    // ////////////// //
    pub hull: i32,
    pub fuel: i32,
    pub drones: i32,
    pub missiles: i32,
    pub scrap: i32,

    // //// //
    // Crew //
    // //// //
    pub crew: Vec<CrewMember>,

    // //////////////////// //
    // The rest of the file //
    // //////////////////// //
    pub unkown_data_block : Vec<u8>,

}

impl SaveFile {
    

    
    
}