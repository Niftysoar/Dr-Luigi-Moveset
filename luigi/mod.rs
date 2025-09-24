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
use crate::MARKED_COLORS;

mod acmd;
mod status;

//Fighter Frame

unsafe extern "C" fn luigi_frame(fighter: &mut L2CFighterCommon) {
    unsafe{
        let boma = fighter.module_accessor;
        let color = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);

        if crate::MARKED_COLORS[color as usize] {

        }
    }
}

// Pre
unsafe extern "C" fn luigi_fireball_start_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(
        weapon.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_AIR), 
        *WEAPON_KINETIC_TYPE_NORMAL, 
        GROUND_CORRECT_KIND_NONE.into(), 
        smash::app::GroundCliffCheckKind(0), 
        false, 
        0, 
        0, 
        0, 
        0
    );
    return 0.into();
}

// Main
unsafe extern "C" fn luigi_fireball_start_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("regular"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(luigi_fireball_start_main_loop as *const () as _))
}

unsafe extern "C" fn luigi_fireball_start_main_loop(weapon: &mut L2CWeaponCommon) -> L2CValue {
    // Declare owner boma
    let owner_boma = &mut *sv_battle_object::module_accessor((WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    // Declare facing
    let facing = PostureModule::lr(weapon.module_accessor);
    // Declare x and y speeds
    let energy_type = KineticModule::get_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL) as *mut smash::app::KineticEnergy;
    let mut speed_x: f32 = lua_bind::KineticEnergy::get_speed_x(energy_type);
    let mut speed_y: f32 = lua_bind::KineticEnergy::get_speed_y(energy_type);
    // Declare acceleration and max speed
    let accel_x: f32 = if facing == 1.0 { 0.04 } else { -0.04 };
    let accel_y: f32 = -0.1;
    let speed_max_x: f32 = if facing == 1.0 { 1.0 } else { -1.0 };
    let speed_max_y: f32 = 2.0;
    // Declare status_frame
    let status_frame = weapon.global_table[0xe].get_f32();
    // Get control stick y pos
    let stick_y = ControlModule::get_stick_y(owner_boma);
    
    // Add x speed until max speed is reached
    if speed_max_x > speed_x.abs() {
        speed_x += accel_x;
    }
    
    // Add y speed until max speed is reached
    if status_frame == 1.0 {
        speed_y = 2.0 + (stick_y + 1.0) / 2.0;
    }
    if speed_max_y > speed_x.abs() {
        speed_y += accel_y;
    }
    
    // Set speed
    weapon.clear_lua_stack();
    weapon.push_lua_stack(&mut L2CValue::new_int(*WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL as u64));
    weapon.push_lua_stack(&mut L2CValue::new_num(speed_x));
    weapon.push_lua_stack(&mut L2CValue::new_num(speed_y));
    sv_kinetic_energy::set_speed(weapon.lua_state_agent);
    return 0.into();
}

pub fn install() {

    acmd::install();
    status::install();

    Agent::new("luigi")
    .set_costume([50, 51, 52, 53, 54, 55, 56, 57].to_vec())

        //fighter frame
        .on_line(Main, luigi_frame)

        .install();

    Agent::new("luigi_fireball")
    .set_costume([50, 51, 52, 53, 54, 55, 56, 57].to_vec())

        .status(Pre, *WEAPON_LUIGI_FIREBALL_STATUS_KIND_START, luigi_fireball_start_pre)
        .status(Main, *WEAPON_LUIGI_FIREBALL_STATUS_KIND_START, luigi_fireball_start_main)
        
        .install();
}