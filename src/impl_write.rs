mod save_file_writer;

use crate::animation_state::AnimationState;
use crate::crew_member::CrewMember;

use super::SaveFile;
use save_file_writer::SaveFileWriter;
use std::path::PathBuf;
use std::path::Path;
use std::fs::OpenOptions;

impl SaveFile {
    pub fn write_to_file(&self, path: &Path) -> Result<(), String> {
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

        // ////////////////////// //
        // Starting Crew Overview //
        // ////////////////////// //
        writer.write_i32(self.starting_crew.len() as i32);
        for i in &self.starting_crew {
            writer.write_string(&i.race);
            writer.write_string(&i.name);
        }

        // ///////////// //
        // Opponent info //
        // ///////////// //
        writer.write_bool(self.opponent_hostility);
        writer.write_i32(self.opponent_jump_charge);
        writer.write_bool(self.opponent_is_jumping);
        writer.write_i32(self.opponent_jump_animation_ticks);

        // ////////////// //
        // Ship Resources //
        // ////////////// //
        writer.write_i32(self.hull);
        writer.write_i32(self.fuel);
        writer.write_i32(self.drones);
        writer.write_i32(self.missiles);
        writer.write_i32(self.scrap);

        // //// //
        // Crew //
        // //// //
        writer.write_i32(self.crew.len() as i32);
        for member in &self.crew {
            SaveFile::write_crew_member(member, &mut writer)
        }

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
        self.write_to_file(path.as_path()).expect("Unable to save data to local save game location.");
        Ok(())
    }

    fn write_crew_member(u: &CrewMember, w: &mut SaveFileWriter) {
        w.write_string(&u.name);
        w.write_string(&u.race);
        w.write_bool(u.drone);
        w.write_i32(u.hp);
        w.write_i32(u.x_cord);
        w.write_i32(u.y_cord);
        w.write_i32(u.room_number);
        w.write_i32(u.room_tile);
        w.write_bool(u.player_controlled);
        w.write_i32(u.clone_ready);
        w.write_i32(u.death_order);
        w.write_i32(u.sprite_tint_indeces.len() as i32);
        for t in &u.sprite_tint_indeces {
            w.write_i32(*t);
        }
        w.write_bool(u.mind_controlled);
        w.write_i32(u.saved_room_square);
        w.write_i32(u.saved_room_id);
        w.write_i32(u.pilot_skill);
        w.write_i32(u.engine_skill);
        w.write_i32(u.shield_skill);
        w.write_i32(u.weapon_skill);
        w.write_i32(u.repair_skill);
        w.write_i32(u.combat_skill);
        w.write_bool(u.male);
        w.write_i32(u.repairs);
        w.write_i32(u.kills);
        w.write_i32(u.evasions);
        w.write_i32(u.jumps_survived);
        w.write_i32(u.skill_masteries_earned);
        w.write_i32(u.stun_ticks);
        w.write_i32(u.health_boost);
        w.write_i32(u.clonebay_priority);
        w.write_i32(u.damage_boost);
        w.write_i32(u.unkown_data_block_1);
        w.write_i32(u.universal_death_count);
        w.write_bool(u.pilot_mastery_one);
        w.write_bool(u.pilot_mastery_two);
        w.write_bool(u.engine_mastery_one);
        w.write_bool(u.engine_mastery_two);
        w.write_bool(u.shield_mastery_one);
        w.write_bool(u.shield_mastery_two);
        w.write_bool(u.weapon_mastery_one);
        w.write_bool(u.weapon_mastery_two);
        w.write_bool(u.repair_mastery_one);
        w.write_bool(u.repair_mastery_two);
        w.write_bool(u.combat_mastery_one);
        w.write_bool(u.combat_mastery_two);
        w.write_i32(u.unkown_data_block_2);
        SaveFile::write_animation_state(&u.teleport_animation, w);
        w.write_i32(u.unkown_data_block_3);
        if u.race == "crystal" {
            w.write_i32(u.lockdown_recharge_ticks);
            w.write_i32(u.lockdown_recharge_goal);
            w.write_i32(u.unkown_data_block_4);
        }
    }fn write_animation_state(a: &AnimationState, w: &mut SaveFileWriter) {
        w.write_bool(a.playing);
        w.write_bool(a.looping);
        w.write_i32(a.current_frame);
        w.write_i32(a.progress_ticks);
        w.write_i32(a.scale);
        w.write_i32(a.x);
        w.write_i32(a.y);
    }
}