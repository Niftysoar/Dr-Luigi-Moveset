use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*,
        hash40
	},
    smash_script::*,
    smashline::*
};


use smashline::Priority::*;
use super::*;

mod acmd;
mod status;

pub const FIGHTER_LUIGI_INSTANCE_WORK_ID_FLOAT_PILL_ROTATION: i32 = 0x200000E4;
pub const FIGHTER_LUIGI_INSTANCE_WORK_ID_FLOAT_PILL_ROTATION_DIRECTION: i32 = 0x200000E5;
pub const FIGHTER_LUIGI_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_INITIAL_Y_SPEED: i32 = 0x200000E6;

pub fn install() {

    acmd::install();
    status::install();
    smashline::update_weapon_count(*WEAPON_KIND_LUIGI_FIREBALL, 4);
}