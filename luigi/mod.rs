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

//Fighter Frame

unsafe extern "C" fn luigi_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
        
        if crate::MARKED_COLORS[color as usize] {
   

            
        }
    }
}

//Status
unsafe extern "C" fn luigi_catch_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_Catch()
}

unsafe extern "C" fn luigi_catch_dash_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchDash()
}

unsafe extern "C" fn luigi_catch_turn_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchTurn()
}

unsafe extern "C" fn luigi_catch_pull_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_CatchPull()
}

unsafe extern "C" fn luigi_catch_pull_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchPull()
}

unsafe extern "C" fn luigi_catch_dash_pull_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_CatchDashPull()
}

unsafe extern "C" fn luigi_catch_dash_pull_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchDashPull()
}

unsafe extern "C" fn luigi_catch_wait_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_CatchWait()
}

unsafe extern "C" fn luigi_catch_wait_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchWait()
}

unsafe extern "C" fn luigi_catch_cut_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchCut()
}

unsafe extern "C" fn luigi_catch_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_CatchAttack()
}

unsafe extern "C" fn luigi_throw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_Throw()
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

//ORIGINAL CODE GIVEN
unsafe extern "C" fn specials_ram_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let stick_y = ControlModule::get_stick_y(boma);

    if stick_y.abs() > 0.2 { //how far the stick is pushed
        let vertical_influence = 0.4; //should be obvious 
        let y_add = stick_y * vertical_influence;
        KineticModule::add_speed(boma, &Vector3f { x: 0.0, y: y_add, z: 0.0 });
    }
    
    smashline::original_status(Main, fighter, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM)(fighter)
}


// WHAT I TRIED TO DO, TAKING INSPIRATION FROM THE PILL SCRIPT

// unsafe extern "C" fn specials_ram_main(fighter: &mut L2CFighterCommon) -> L2CValue {
//     let boma = fighter.module_accessor;

//     // Run normal Green Missile logic first
//     let ret = smashline::original_status(Main, fighter, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM)(fighter);

//     // Grab control stick input
//     let stick_y = ControlModule::get_stick_y(boma);
//     let facing = PostureModule::lr(boma);

//     // Current velocity
//     let mut speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
//     let mut speed_y = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

//     // Constants (inspired by your pill)
//     let accel_y: f32 = -0.1;
//     let influence: f32 = 0.5;
//     let speed_max_y: f32 = 3.5;
//     let speed_max_total: f32 = 5.5;

//     // Apply stick influence on launch
//     if MotionModule::frame(boma) <= 1.0 {
//         speed_y = (stick_y * influence) * 2.0;
//     } else {
//         // Add a bit of stick bias continuously
//         speed_y += stick_y * 0.05;
//     }

//     // Apply gravity-like pull so he doesn’t stay stuck up
//     speed_y += accel_y;

//     // Clamp Y speed
//     if speed_y > speed_max_y { speed_y = speed_max_y; }
//     if speed_y < -speed_max_y { speed_y = -speed_max_y; }

//     // Clamp total vector
//     let mag = (speed_x * speed_x + speed_y * speed_y).sqrt();
//     if mag > speed_max_total {
//         speed_x *= speed_max_total / mag;
//         speed_y *= speed_max_total / mag;
//     }

//     // Apply new velocity
//     KineticModule::mul_speed(
//         boma,
//         &Vector3f { x: speed_x, y: speed_y, z: 0.0 },
//         *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN
//     );

//     ret
}

//ALSO THE STATUS DOESN'T SINGLE SLOT PROPERLY
pub fn install() {
    let mut costume = &mut Vec::new();
    unsafe{
        for i in 0..MARKED_COLORS.len() {
            if MARKED_COLORS[i] {
                costume.push(i);
            }
        }
    }

    acmd::install();

    Agent::new("luigi")
    .set_costume(costume.to_vec())

        //fighter frame
        .on_line(Main, luigi_frame)

        //Misc.
        .status(End, *FIGHTER_STATUS_KIND_CATCH, luigi_catch_end)

        .status(End, *FIGHTER_STATUS_KIND_CATCH_DASH, luigi_catch_dash_end)
    
        .status(End, *FIGHTER_STATUS_KIND_CATCH_TURN, luigi_catch_turn_end)
    
        .status(Main, *FIGHTER_STATUS_KIND_CATCH_PULL, luigi_catch_pull_main)
        .status(End, *FIGHTER_STATUS_KIND_CATCH_PULL, luigi_catch_pull_end)
    
        .status(Main, *FIGHTER_STATUS_KIND_CATCH_DASH_PULL, luigi_catch_dash_pull_main)
        .status(End, *FIGHTER_STATUS_KIND_CATCH_DASH_PULL, luigi_catch_dash_pull_end)
    
        .status(Main, *FIGHTER_STATUS_KIND_CATCH_WAIT, luigi_catch_wait_main)
        .status(End, *FIGHTER_STATUS_KIND_CATCH_WAIT, luigi_catch_wait_end)
    
        .status(End, *FIGHTER_STATUS_KIND_CATCH_CUT, luigi_catch_cut_end)
    
        .status(End, *FIGHTER_STATUS_KIND_CATCH_ATTACK, luigi_catch_attack_end)
    
        .status(End, *FIGHTER_STATUS_KIND_THROW, luigi_throw_end)

        //INSTALLATION
        .status(Main, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, specials_ram_main)

        .install();

    Agent::new("luigi_fireball")
    .set_costume(costume.to_vec())

        .status(Pre, *WEAPON_LUIGI_FIREBALL_STATUS_KIND_START, luigi_fireball_start_pre)
        .status(Main, *WEAPON_LUIGI_FIREBALL_STATUS_KIND_START, luigi_fireball_start_main)
        
        .install();
}