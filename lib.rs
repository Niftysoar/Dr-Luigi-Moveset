#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

use std::collections::HashMap;
use smash::hash40;
use csk::*;
use param_config::*;
use smash::lib::lua_const::*;

mod luigi;

pub static mut MARKED_COLORS: [bool; 256] = [false; 256];

extern "C" fn mods_mounted(_ev: arcropolis_api::Event) {
    let lowest_color: i32 = 50;
    let color_num: i32 = 8;
    let marked_slots: Vec<i32> = (50..=57).collect();
    //let doctor_luigi: (Vec<i32> = (50..=57).collect()) && *FIGHTER_KIND_LUIGI;

    unsafe {
        for slot in &marked_slots {
            MARKED_COLORS[*slot as usize] = true;
        }
    }

    // Param Edits

    update_int_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("air_lasso_type"), 0, 0));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("walk_speed_max"), 0, 0.915));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("ground_brake"), 0, 0.05));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("dash_speed"), 0, 1.685));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("run_accel_mul"), 0, 0.06));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("run_speed_max"), 0, 1.405));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("jump_speed_x_mul"), 0, 0.9));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("jump_speed_x_max"), 0, 0.95));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("jump_aerial_speed_x_mul"), 0, 0.915));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("air_accel_x_mul"), 0, 0.02));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("air_accel_x_add"), 0, 0.08));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("jump_y"), 0, 38.25));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("jump_aerial_y"), 0, 37.65));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("air_speed_x_stable"), 0, 0.728));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("air_accel_y"), 0, 0.09));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("air_speed_y_stable"), 0, 1.32));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("dive_speed_y"), 0, 2.315));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("weight"), 0, 100.0));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("damage_fly_top_air_accel_y"), 0, 0.07559));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("shield_radius"), 0, 10.6));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("wall_jump_type"), 0, 0.0));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("landing_attack_air_frame_f"), 0, 15.0));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("landing_attack_air_frame_lw"), 0, 13.0));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("landing_attack_air_frame_hi"), 0, 10.0));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("param_special_hi"), (hash40("pass_mul")), 0.99));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("param_special_hi"), (hash40("air_pass_mul")), 1.0));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("param_special_hi"), (hash40("fall_max_x")), 0.712));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("param_special_lw"), (hash40("accel_x")), 0.8)); //test #s
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("param_special_lw"), (hash40("air_accel_x")), 0.09));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("param_special_lw"), (hash40("limit_x")), 1.20)); //test #s
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("param_special_lw"), (hash40("air_limit_x")), 0.9));
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("param_special_lw"), (hash40("buoyancy_max")), 0.75)); //test #s
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("param_special_lw"), (hash40("air_y_spd_mul")), 3.2)); //TEST USUALLY 1.1
    update_float_2(*FIGHTER_KIND_LUIGI, marked_slots.clone(), (hash40("param_special_lw"), (hash40("air_limit_y")), 2.2));
    // CSK Stuff

    add_narration_characall_entry("vc_narration_characall_luigid");

    add_chara_db_entry_info(CharacterDatabaseEntry{
        ui_chara_id: hash40("ui_chara_luigid"),
        clone_from_ui_chara_id: Some(hash40("ui_chara_luigi")),
        name_id: StringType::Overwrite(CStrCSK::new("luigid")),
        is_dlc:BoolType::Overwrite(false),
        is_patch: BoolType::Overwrite(false),
        color_num: UnsignedByteType::Overwrite(color_num as u8),
        extra_index_maps: UnsignedByteMap::Overwrite(HashMap::from([
            (hash40("color_start_index"), UnsignedByteType::Overwrite(lowest_color as u8))
        ])),
        extra_hash_maps: Hash40Map::Overwrite(HashMap::from([
            (hash40("characall_label_c00"), Hash40Type::Overwrite(hash40("vc_narration_characall_luigid"))),
            (hash40("original_ui_chara_hash"), Hash40Type::Overwrite(hash40("ui_chara_luigi")))
        ])),
        ..Default::default()
    });

    add_chara_layout_db_entry_info(CharacterLayoutDatabaseEntry {
        ui_layout_id: hash40("ui_chara_luigid_00"),
        clone_from_ui_layout_id: Some(hash40("ui_chara_luigi_00")),
        ui_chara_id: Hash40Type::Overwrite(hash40("ui_chara_luigid")),
        ..Default::default()
    });

}


#[skyline::main(name = "smashline_test")]
pub fn main() {

    unsafe {
        extern "C" {
            fn arcrop_register_event_callback(
                ty: arcropolis_api::Event,
                callback: arcropolis_api::EventCallbackFn,
            );
        }
        arcrop_register_event_callback(arcropolis_api::Event::ModFilesystemMounted, mods_mounted);
    }

    luigi::install();
}