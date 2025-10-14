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

// ********************
// JAB 2
// ********************
unsafe extern "C" fn game_attack12(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 180, 25, 0, 20, 1.5, 0.0, 5.2, 5.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 180, 20, 0, 20, 2.8, 0.0, 5.8, 9.5, None, None, None, 1.23, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 2, 0, Hash40::new("top"), 3.0, 180, 15, 0, 15, 4.0, 0.0, 6.2, 14.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

// ********************
// JAB 3
// ********************
unsafe extern "C" fn game_attack13(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 6.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 4.0, 58, 70, 0, 60, 5.0, 0.0, 8.5, 9.2, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 4.0, 58, 70, 0, 60, 3.5, 0.0, 7.0, 4.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
    }
    wait(agent.lua_state_agent, 12.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
}

unsafe extern "C" fn effect_attack13(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 7.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_FLIP(agent, Hash40::new("mariod_smash_impact"), Hash40::new("mariod_smash_impact"), Hash40::new("handl"), 1.0, 0, 0, 0, 0, 0, 0.8, true, *EF_FLIP_YZ);
    }
}

pub fn install() {
    Agent::new("luigi")
    .set_costume([50, 51, 52, 53, 54, 55, 56, 57].to_vec())
        .game_acmd("game_attack12_luigid", game_attack12, Priority::Low)

        .game_acmd("game_attack13_luigid", game_attack13, Priority::Low) 
        .effect_acmd("effect_attack13_luigid", effect_attack13, Priority::Low)

        .install();
}