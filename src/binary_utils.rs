use crate::error::*;
use crate::models::*;
use binary_rw::{BinaryReader, BinaryWriter, Endian, FileStream};
use std::fs::File;
use std::path::Path;

pub fn read_progress_binary<T: AsRef<Path>>(path: T) -> Result<ProgressData> {
    let file = File::open(path)?;
    let mut stream = FileStream::new(file);
    let mut reader = BinaryReader::new(&mut stream, Endian::Little);
    let mut progress = ProgressData::default();
    let num = reader.read_i32()?;
    if num != -9854 {
        return Err(Error("Error in file magic!".to_string()));
    }

    progress.visited_sector_type_max_entry_id = reader.read_i32()?;
    let j = reader.read_i32()?;

    for _ in 0..j {
        let val = reader.read_i32()?;
        progress.visited_sector_types_datasets.push(val)
    }
    let num2 = reader.read_i32()?;

    if num2 >= 1 {
        progress.completed_fb_campaign = reader.read_bool()?;
    }
    if num2 >= 2 {
        progress.jumps_completed = reader.read_i32()?;
        for i in 0..5 {
            progress.jumps_completed_per_class[i] = reader.read_i32()?;
        }
        progress.blind_jumps_completed = reader.read_i32()?;
        progress.levels_earned = reader.read_i32()?;
        for i in 0..5 {
            progress.levels_earned_per_class[i] = reader.read_i32()?;
        }
        progress.pawn_deaths_total = reader.read_i32()?;
        progress.crew_deaths_total = reader.read_i32()?;
        progress.biscuits_sold_individual_total = reader.read_i32()?;
        progress.max_crew_level_reached = reader.read_i32()?;
        progress.missions_completed = reader.read_i32()?;
        progress.vulrog_discovered = reader.read_bool()?;
    }
    if num2 >= 3 {
        for i in 0..6 {
            progress.levels_earned_per_faction[i] = reader.read_i32()?;
        }
        progress.enemy_ships_destroyed = reader.read_i32()?;
        progress.talents_unlocked = reader.read_i32()?;
        progress.credits_earned = reader.read_i32()?;
        progress.credits_spent = reader.read_i32()?;
        progress.damage_dealt_pawn = reader.read_i32()?;
        progress.damage_taken_pawn = reader.read_i32()?;
        progress.damage_dealt_ship = reader.read_i32()?;
        progress.damage_taken_ship = reader.read_i32()?;
        progress.shots_fired = reader.read_i32()?;
        progress.system_health_healed = reader.read_f32()?;
        progress.footsteps_taken = reader.read_i32()?;
        progress.max_chaos_level_reached = reader.read_f32()?;
    }
    if num2 >= 4 {
        let num3 = reader.read_i32()?;
        for _ in 0..num3 {
            progress.missions_ids_completed.push(reader.read_i32()?)
        }
        let num4 = reader.read_i32()?;
        for _ in 0..num4 {
            let item = UnlockID {
                pawn_type: reader.read_i32()?,
                cosmetic_type: reader.read_i32()?,
                cosmetic_id: reader.read_i32()?,
            };
            progress.unlocks_opened.push(item)
        }
        progress.turret_shots_fired = reader.read_i32()?;
    }

    if num2 >= 5 {
        progress.races_completed_bit_field = reader.read_i32()?;
    }

    if num2 >= 6 {
        progress.extorts_attempted = reader.read_i32()?;
        progress.diplomacy_attempted = reader.read_i32()?;
        progress.pawn_hp_healed = reader.read_f32()?;
    }

    if num2 >= 7 {
        progress.any_ending_completed = reader.read_bool()?;
        progress.cu_ending_completed = reader.read_bool()?;
        progress.aog_ending_completed = reader.read_bool()?;
        progress.wd_ending_completed = reader.read_bool()?;
        progress.fb_ending_completed = reader.read_bool()?;
        progress.keep_it_ending_completed = reader.read_bool()?;
        progress.free_it_ending_completed = reader.read_bool()?;
        progress.destroy_it_ending_completed = reader.read_bool()?;
    }

    if num2 >= 8 {
        let num = reader.read_i32()?;

        for _ in 0..num {
            let item = ShipUnlockID {
                ship_type: reader.read_i32()?,
                variant: reader.read_i32()?,
            };
            progress.opened_ship_unlocks.push(item)
        }
    }

    if num2 >= 9 {
        progress.abyss_ending_completed = reader.read_bool()?;
    }

    Ok(progress)
}

pub fn write_data_to_file(file: File, data: ProgressData) -> Result<()> {
    let mut stream = FileStream::new(file);
    let mut writer = BinaryWriter::new(&mut stream, Endian::Little);

    writer.write_i32(-9854)?;
    writer.write_i32(data.visited_sector_type_max_entry_id)?;
    writer.write_i32(data.visited_sector_types_datasets.len() as i32)?;
    for v in data.visited_sector_types_datasets {
        writer.write_i32(v)?;
    }
    writer.write_i32(9i32)?;
    writer.write_bool(data.completed_fb_campaign)?;
    writer.write_i32(data.jumps_completed)?;
    for v in data.jumps_completed_per_class {
        writer.write_i32(v)?;
    }
    writer.write_i32(data.blind_jumps_completed)?;
    writer.write_i32(data.levels_earned)?;
    for v in data.levels_earned_per_class {
        writer.write_i32(v)?;
    }
    writer.write_i32(data.pawn_deaths_total)?;
    writer.write_i32(data.crew_deaths_total)?;
    writer.write_i32(data.biscuits_sold_individual_total)?;
    writer.write_i32(data.max_crew_level_reached)?;
    writer.write_i32(data.missions_completed)?;
    writer.write_bool(data.vulrog_discovered)?;

    for v in data.levels_earned_per_faction {
        writer.write_i32(v)?;
    }

    writer.write_i32(data.enemy_ships_destroyed)?;
    writer.write_i32(data.talents_unlocked)?;

    writer.write_i32(data.credits_earned)?;

    writer.write_i32(data.credits_spent)?;

    writer.write_i32(data.damage_dealt_pawn)?;

    writer.write_i32(data.damage_taken_pawn)?;

    writer.write_i32(data.damage_dealt_ship)?;

    writer.write_i32(data.damage_taken_ship)?;

    writer.write_i32(data.shots_fired)?;
    writer.write_f32(data.system_health_healed)?;
    writer.write_i32(data.footsteps_taken)?;
    writer.write_f32(data.max_chaos_level_reached)?;
    writer.write_i32(data.missions_ids_completed.len() as i32)?;

    for v in data.missions_ids_completed {
        writer.write_i32(v)?;
    }

    writer.write_i32(data.unlocks_opened.len() as i32)?;

    for v in data.unlocks_opened {
        writer.write_i32(v.pawn_type)?;
        writer.write_i32(v.cosmetic_type)?;
        writer.write_i32(v.cosmetic_id)?;
    }

    writer.write_i32(data.turret_shots_fired)?;
    writer.write_i32(data.races_completed_bit_field)?;
    writer.write_i32(data.extorts_attempted)?;
    writer.write_i32(data.diplomacy_attempted)?;
    writer.write_f32(data.pawn_hp_healed)?;
    writer.write_bool(data.any_ending_completed)?;
    writer.write_bool(data.cu_ending_completed)?;
    writer.write_bool(data.aog_ending_completed)?;
    writer.write_bool(data.wd_ending_completed)?;
    writer.write_bool(data.fb_ending_completed)?;
    writer.write_bool(data.keep_it_ending_completed)?;
    writer.write_bool(data.free_it_ending_completed)?;
    writer.write_bool(data.destroy_it_ending_completed)?;
    writer.write_i32(data.opened_ship_unlocks.len() as i32)?;
    for v in data.opened_ship_unlocks {
        writer.write_i32(v.ship_type)?;
        writer.write_i32(v.variant)?;
    }
    writer.write_bool(data.abyss_ending_completed)?;

    Ok(())
}
