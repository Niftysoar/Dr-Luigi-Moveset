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

// Final
// unsafe extern "C" fn final_end(fighter: &mut L2CAgentBase) -> L2CValue {
//     // Fire Mario's huge capsule article (e.g., giant fireball or cinematic projectile)
//     let article_id = *FIGHTER_MARIOD_GENERATE_ARTICLE_HUGECAPSULE;
//     let target = *ARTICLE_OPE_TARGET_ALL;
//     let is_shoot = false;

//     ArticleModule::shoot(fighter.module_accessor, article_id, target, is_shoot);

//     // Notify event to signal end of Final Smash
//     notify_event_msc_cmd!(fighter, Hash40::new("final_end"));

//     // Return 0 to signal no special state change
//     0.into()
// }

// unsafe extern "C" fn final_main(fighter: &mut L2CAgentBase) -> L2CValue {
//     // Notify Final Smash activation event
//     notify_event_msc_cmd!(fighter, Hash40::new("final_start"));

//     // Disable area collisions (probably for cinematic behavior)
//     AreaModule::set_whole(fighter.module_accessor, false);

//     // Enable status transitions
//     WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT);
//     WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL);

//     let situation = StatusModule::situation_kind(fighter.module_accessor);

//     // Handle air/ground behavior separately
//     if situation == *SITUATION_KIND_AIR {
//         // Set to air kinetic and correct position
//         KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_AIR);
//         GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));

//         if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIOD_STATUS_FINAL_FLAG_FIRST) {
//             MotionModule::change_motion(
//                 fighter.module_accessor,
//                 Hash40::new("final_air_start"),
//                 0.0,
//                 1.0,
//                 false,
//                 0.0,
//                 false,
//                 false
//             );
//             WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIOD_STATUS_FINAL_FLAG_FIRST);
//         } else {
//             MotionModule::change_motion_inherit_frame(
//                 fighter.module_accessor,
//                 Hash40::new("final_air_start"),
//                 -1.0,
//                 1.0,
//                 0.0,
//                 false,
//                 false
//             );
//         }
//     } else {
//         // Ground kinetic and correction
//         KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION);
//         GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));

//         if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_MARIOD_STATUS_FINAL_FLAG_FIRST) {
//             MotionModule::change_motion(
//                 fighter.module_accessor,
//                 Hash40::new("final_start"),
//                 0.0,
//                 1.0,
//                 false,
//                 0.0,
//                 false,
//                 false
//             );
//             WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARIOD_STATUS_FINAL_FLAG_FIRST);
//         } else {
//             MotionModule::change_motion_inherit_frame(
//                 fighter.module_accessor,
//                 Hash40::new("final_start"),
//                 -1.0,
//                 1.0,
//                 0.0,
//                 false,
//                 false
//             );
//         }
//     }

//     // Set up main loop for Final Smash state
//     fighter.global_table[0x1C] = L2CValue::Ptr(final_main_loop as *const ());

//     0.into()
// }

// unsafe extern "C" fn final_main_loop(fighter: &mut L2CAgentBase) -> L2CValue {
//     // If cancel is NOT enabled, continue through final sequence logic
//     if !CancelModule::is_enable_cancel(fighter.module_accessor) {
//         // Check WAIT transition
//         if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT) {
//             // If animation is finished and Mario is on the ground, switch to WAIT
//             if MotionModule::is_end(fighter.module_accessor)
//                 && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
//             {
//                 fighter.change_status(*FIGHTER_STATUS_KIND_WAIT, false.into());
//             }
//         }
//         // Else: check FALL transition
//         else if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_FALL) {
//             // If animation is finished and Mario is in the air, switch to FINAL_JUMP_END
//             if MotionModule::is_end(fighter.module_accessor)
//                 && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR
//             {
//                 fighter.change_status(*FIGHTER_STATUS_KIND_FINAL_JUMP_END, false.into());
//             }
//         }
//     } else {
//         // Ground wait check
//         if L2CFighterCommon::sub_wait_ground_check_common(fighter).get_bool() {
//             // Air fall check
//             if !L2CFighterCommon::sub_air_check_fall_common(fighter).get_bool() {
//                 // Re-enter main transition logic
//                 return final_main_loop(fighter);
//             }
//         }
//     }

//     0.into()
// }

// unsafe extern "C" fn final_pre(fighter: &mut L2CAgentBase) -> L2CValue {
//     // Run shared Final Smash setup (like common cancel disabling)
//     L2CFighterCommon::sub_status_pre_FinalCommon(fighter);

//     // --- Status Initialization ---
//     StatusModule::init_settings(
//         fighter.module_accessor,
//         *SITUATION_KIND_NONE,
//         *FIGHTER_KINETIC_TYPE_UNIQ,
//         *GROUND_CORRECT_KIND_KEEP,
//         *GROUND_CLIFF_CHECK_KIND_NONE,
//         false, // cliff check
//         *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
//         *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
//         *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
//         0
//     );

//     // --- Status Properties & Flags ---
//     FighterStatusModuleImpl::set_fighter_status_data(
//         fighter.module_accessor,
//         false,                               // no treaded
//         *FIGHTER_TREADED_KIND_NO_REAC,       // no reaction
//         false, false, false,                 // no turn, damage, or cancel
//         _FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_FINAL
//             | FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK
//             | FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON, // logging flags
//         _FIGHTER_STATUS_ATTR_DISABLE_ITEM_INTERRUPT
//             | FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE
//             | _FIGHTER_STATUS_ATTR_FINAL,              // status attributes
//         _FIGHTER_POWER_UP_ATTACK_BIT_FINAL,            // power-up flags
//         0
//     );

//     0.into()
// }

// Fireball Special N
unsafe extern "C" fn luigi_fireball_game_regular(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.33, 361, 84, 0, 40, 5.4, 0.0, 0.0, 0.0, Some(0.0), Some(-1.7), Some(0.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_NONE);
        AttackModule::enable_safe_pos(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 8.33, 361, 84, 0, 40, 5.4, 0.0, 0.0, 0.0, Some(0.0), Some(-1.7), Some(0.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_NONE);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 7.33, 361, 84, 0, 40, 5.4, 0.0, 0.0, 0.0, Some(0.0), Some(-1.7), Some(0.0), 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_NONE);
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

    Agent::new("luigi")
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

        .install();

    Agent::new("luigi_fireball")
        .game_acmd("game_regular_luigid", luigi_fireball_game_regular, Default)
        .status(Pre, *WEAPON_LUIGI_FIREBALL_STATUS_KIND_START, luigi_fireball_start_pre)
        .status(Main, *WEAPON_LUIGI_FIREBALL_STATUS_KIND_START, luigi_fireball_start_main)
        .install();
}