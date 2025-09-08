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
// WiN 1
// ********************
unsafe extern "C" fn game_win1(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ArticleModule::generate_article_enable(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_FIREBALL, false, -1);
        ArticleModule::generate_article_enable(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_FIREBALL, false, -1);
    }
}

unsafe extern "C" fn effect_win1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 27.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 0, 0, -2, 0, 30, 0, 0.55, 0, 0, 0, 0, 0, 0, true);
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_h_smoke_a"), Hash40::new("top"), 3, 0, -2, 0, -60, 0, 0.55, 0, 0, 0, 0, 0, 0, true);
    }
}

unsafe extern "C" fn sound_win1(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_luigi_006"));
    }
    frame(agent.lua_state_agent, 31.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_luigi_special_n01"));
    }
    frame(agent.lua_state_agent, 41.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_luigi_006"));
    }
    frame(agent.lua_state_agent, 44.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_luigi_special_n01"));
    }
    frame(agent.lua_state_agent, 56.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_luigi_damage02"));
    }
    frame(agent.lua_state_agent, 125.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_luigi_appeal03"));
    }
}

// ********************
// WiN 2
// ********************
unsafe extern "C" fn sound_win2(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 128.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_luigi_heavyget"));
    }
    frame(agent.lua_state_agent, 190.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_luigi_jump01"));
    }
}


// ********************
// WiN 3
// ********************

pub fn install() {
    Agent::new("luigi")
    .set_costume([100, 101, 102, 103, 104, 105, 106, 107].to_vec())
        .game_acmd("game_win1_luigid", game_win1, Priority::Low)
        .effect_acmd("effect_win1_luigid", effect_win1, Priority::Low) 
        .sound_acmd("sound_win1_luigid", sound_win1, Priority::Low) 

        .sound_acmd("sound_win2_luigid", sound_win2, Priority::Low)

        .install();
}