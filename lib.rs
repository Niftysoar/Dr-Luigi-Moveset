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
use smash::lib::lua_const::{FIGHTER_KIND_LUIGI};

mod luigi;

pub static mut MARKED_COLORS: [bool; 256] = [false; 256];

extern "C" fn mods_mounted(_ev: arcropolis_api::Event) {
    let lowest_color: i32 = 50;
    let color_num: i32 = 8;
    let marked_slots: Vec<i32> = (50..=57).collect();

    unsafe {
        for slot in &marked_slots {
            MARKED_COLORS[*slot as usize] = true;
        }
    }

    // Param Edits

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