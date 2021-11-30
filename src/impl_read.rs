mod save_file_parser;

use std::path::PathBuf;
use std::path::Path;
use std::fs::File;
use crate::animation_state::AnimationState;
use crate::crew_member::CrewMember;

use save_file_parser::SaveFileParser;
use super::SaveFile;
use super::ScoreCategory;

impl SaveFile {
    pub fn read_from_file(path: &Path) -> Result<SaveFile, String> {
        
        let file: File = match File::open(&path){
            Ok(x) => x,
            Err(_) => return Err(format!("Unable to open save file at {:?}", path)) 
        };

        let mut parser = SaveFileParser::new(file);
        let mut save : SaveFile = Default::default();

        // ////// //
        // Header //
        // ////// //
        save.version = parser.read_i32();
        save.unkown_data_block_1 = parser.read_raw_bytes(4);
        save.ae_content = parser.read_i32();
        save.difficulty = parser.read_i32();
        save.ships_defeated = parser.read_i32();
        save.jumps_in_sector = parser.read_i32();
        save.scrap_collected = parser.read_i32();
        save.crew_recruited = parser.read_i32();

        // //////////////// //
        // Ship Designation //
        // //////////////// //
        save.ship_name = parser.read_string()?;
        save.ship_id = parser.read_string()?;
        save.sector = parser.read_i32();
        save.unkown_data_block_2 = parser.read_raw_bytes(4);

        // //////////////// //
        // Score Categories //
        // //////////////// //
        let num_categories = parser.read_i32();
        for _ in 0..num_categories {
            let name = parser.read_string()?;
            let value = parser.read_i32();
            save.scores.push(
                ScoreCategory::new(name, value)
            )
        }

        // //////////// //
        // Ship Details //
        // //////////// //
        parser.read_string()?;
        parser.read_string()?;
        save.ship_graphics_base_name = parser.read_string()?;

        // ////////////////////// //
        // Starting Crew Overview //
        // ////////////////////// //
        let num_crew = parser.read_i32();
        for i in 0..num_crew {
            save.starting_crew.push(Default::default());
            save.starting_crew.get_mut(i as usize).expect("").race = parser.read_string()?;
            save.starting_crew.get_mut(i as usize).expect("").name = parser.read_string()?;
        }

        // ///////////// //
        // Opponent info //
        // ///////////// //
        save.opponent_hostility = parser.read_bool();
        save.opponent_jump_charge = parser.read_i32();
        save.opponent_is_jumping = parser.read_bool();
        save.opponent_jump_animation_ticks = parser.read_i32();

        // ////////////// //
        // Ship Resources //
        // ////////////// //
        save.hull = parser.read_i32();
        save.fuel = parser.read_i32();
        save.drones = parser.read_i32();
        save.missiles = parser.read_i32();
        save.scrap = parser.read_i32();

        // //// //
        // Crew //
        // //// //
        for _ in 0..parser.read_i32() {
            save.crew.push(SaveFile::read_crew_member(&mut parser));
        }

        // //////////////////// //
        // The rest of the file //
        // //////////////////// //
        save.unkown_data_block = parser.read_until_eof(); 

        return Ok(save);
    }

    pub fn read_from_local_save() -> Result<SaveFile, String> {
        //let path: PathBuf = [r"C:\","Users","kerstop","Documents","my games","fasterthanlight","continue.sav"].iter().collect();
        let mut path: PathBuf = match home::home_dir() {
            Some(path) => path,
            None => return Err(String::from("Unable to locate users home directory.")),
        };
        path.push(PathBuf::from("Documents/my games/fasterthanlight/continue.sav"));
        SaveFile::read_from_file(path.as_path())
    }

    fn read_crew_member(parser: &mut SaveFileParser ) -> CrewMember {
        let mut unit = CrewMember::default();
        unit.name = parser.read_string().unwrap();
        unit.race = parser.read_string().unwrap();
        unit.drone = parser.read_bool();
        unit.hp = parser.read_i32();
        unit.x_cord = parser.read_i32();
        unit.y_cord = parser.read_i32();
        unit.room_number = parser.read_i32();
        unit.room_tile = parser.read_i32();
        unit.player_controlled = parser.read_bool();
        unit.clone_ready = parser.read_i32();
        unit.death_order = parser.read_i32();
        for _ in 0..parser.read_i32() {
            unit.sprite_tint_indeces.push(parser.read_i32());
        }
        unit.mind_controlled = parser.read_bool();
        unit.saved_room_square = parser.read_i32();
        unit.saved_room_id = parser.read_i32();
        unit.pilot_skill = parser.read_i32();
        unit.engine_skill = parser.read_i32();
        unit.shield_skill = parser.read_i32();
        unit.weapon_skill = parser.read_i32();
        unit.repair_skill = parser.read_i32();
        unit.combat_skill = parser.read_i32();
        unit.male = parser.read_bool();
        unit.repairs = parser.read_i32();
        unit.kills = parser.read_i32();
        unit.evasions = parser.read_i32();
        unit.jumps_survived = parser.read_i32();
        unit.skill_masteries_earned = parser.read_i32();
        unit.stun_ticks = parser.read_i32();
        unit.health_boost = parser.read_i32();
        unit.clonebay_priority = parser.read_i32();
        unit.damage_boost = parser.read_i32();
        unit.unkown_data_block_1 = parser.read_i32();
        unit.universal_death_count = parser.read_i32();
        unit.pilot_mastery_one = parser.read_bool();
        unit.pilot_mastery_two = parser.read_bool();
        unit.engine_mastery_one = parser.read_bool();
        unit.engine_mastery_two = parser.read_bool();
        unit.shield_mastery_one = parser.read_bool();
        unit.shield_mastery_two = parser.read_bool();
        unit.weapon_mastery_one = parser.read_bool();
        unit.weapon_mastery_two = parser.read_bool();
        unit.repair_mastery_one = parser.read_bool();
        unit.repair_mastery_two = parser.read_bool();
        unit.combat_mastery_one = parser.read_bool();
        unit.combat_mastery_two = parser.read_bool();
        unit.unkown_data_block_2 = parser.read_i32();
        unit.teleport_animation = SaveFile::read_animation_state(parser);
        unit.unkown_data_block_3 = parser.read_i32();
        if unit.race == "crystal" {
            //TODO: Find default values for these fields
            unit.lockdown_recharge_ticks = parser.read_i32();
            unit.lockdown_recharge_goal = parser.read_i32();
            unit.unkown_data_block_4 = parser.read_i32();
        }
        return unit;
    }

    fn read_animation_state(parser: &mut SaveFileParser) -> AnimationState {
        let mut state = AnimationState::default();
        state.playing = parser.read_bool();
        state.looping = parser.read_bool();
        state.current_frame = parser.read_i32();
        state.progress_ticks = parser.read_i32();
        state.scale = parser.read_i32();
        state.x = parser.read_i32();
        state.y = parser.read_i32();
        return state;
    }

}