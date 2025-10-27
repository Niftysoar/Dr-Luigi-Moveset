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

unsafe extern "C" fn game_final(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
        macros::CHECK_VALID_FINAL_START_CAMERA(agent, 0, 1, 20, 0, 0, 0);
        macros::SLOW_OPPONENT(agent, 20.0, 85.0);
    }

    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, false, -1);
    }

    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
        frame(agent.lua_state_agent, 10.0);

        if macros::is_excute(agent) {
            macros::FT_SET_FINAL_FEAR_FACE(agent, 60);
            macros::REQ_FINAL_START_CAMERA_arg3(agent, Hash40::new("d04finalstart.nuanmb"), false, false);
            macros::FT_START_CUTIN(agent);
        } else {
            if macros::is_excute(agent) {
                PostureModule::scale(agent.module_accessor);
                // methodlib::L2CValue::operator*(lib::L2CValueconst&)const(-444723117, 2.05);
                CAM_ZOOM_IN_arg5(0);
                macros::FT_START_CUTIN(agent);
            }

            // if get_value_float(agent, *SO_VAR_FLOAT_LR) < 0.0 {
            //     if macros::is_excute(agent) {
            //         camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
            //     }
            // } else {
            //     if macros::is_excute(agent) {
            //         camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
            //     }
            // }
        }
    }

    frame(agent.lua_state_agent, 25.0);
    WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA);
    // 0xda370(false, false);

    if macros::is_excute(agent) {
        macros::CAM_ZOOM_OUT(agent);
    }
}

unsafe extern "C" fn effect_final(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EffectModule::req_screen(agent.module_accessor, Hash40::new("bg_mariod_final"), false, false, false);
    }
    frame(agent.lua_state_agent, 56.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("mariod_final_smoke"), Hash40::new("top"), 0, 10, 13.5, 0, 0, 0, 0.55, true);
        macros::EFFECT_FLW_POS(agent, Hash40::new("mariod_final_light"), Hash40::new("top"), 0, 10, 15, 0, 0, 0, 1.1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_final(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_luigi_009"));
        macros::PLAY_SE(agent, Hash40::new("se_luigi_final01"));
    }
    wait(agent.lua_state_agent, 54.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_luigi_win02"));
        macros::PLAY_SE(agent, Hash40::new("se_luigi_final02"));
    }
}

unsafe extern "C" fn expression_final(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        sv_animcmd::START_INFO_FLASH_EYE(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 56.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_beamm"), 18, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 75.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_18_fireballsp"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 126.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_18_fireballsp2"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 156.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

unsafe extern "C" fn game_finalair(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::WHOLE_HIT(agent, *HIT_STATUS_XLU);
        macros::CHECK_VALID_FINAL_START_CAMERA(agent, 0, 1, 20, 0, 0, 0);
        macros::SLOW_OPPONENT(agent, 20.0, 85.0);
    }

    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        ArticleModule::generate_article(agent.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, false, -1);
    }

    if !WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA) {
        frame(agent.lua_state_agent, 10.0);

        if macros::is_excute(agent) {
            macros::FT_SET_FINAL_FEAR_FACE(agent, 60);
            macros::REQ_FINAL_START_CAMERA_arg3(agent, Hash40::new("d04finalstart.nuanmb"), false, false);
            macros::FT_START_CUTIN(agent);
        } else {
            if macros::is_excute(agent) {
                PostureModule::scale(agent.module_accessor);
                // methodlib::L2CValue::operator*(lib::L2CValueconst&)const(-444723117, 2.05);
                CAM_ZOOM_IN_arg5(0);
                macros::FT_START_CUTIN(agent);
            }

            // if get_value_float(agent, *SO_VAR_FLOAT_LR) < 0.0 {
            //     if macros::is_excute(agent) {
            //         camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
            //     }
            // } else {
            //     if macros::is_excute(agent) {
            //         camera!(agent, *MA_MSC_CMD_CAMERA_CAM_OFFSET, 0, 0);
            //     }
            // }
        }
    }

    frame(agent.lua_state_agent, 25.0);
    WorkModule::is_flag(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_FINAL_START_CAMERA);
    // 0xda370(false, false);

    if macros::is_excute(agent) {
        macros::CAM_ZOOM_OUT(agent);
    }
}

unsafe extern "C" fn effect_finalair(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        EffectModule::req_screen(agent.module_accessor, Hash40::new("bg_mariod_final"), false, false, false);
    }
    frame(agent.lua_state_agent, 56.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FLW_POS(agent, Hash40::new("mariod_final_smoke"), Hash40::new("top"), 0, 10, 13.5, 0, 0, 0, 0.55, true);
        macros::EFFECT_FLW_POS(agent, Hash40::new("mariod_final_light"), Hash40::new("top"), 0, 10, 15, 0, 0, 0, 1.1, true);
        macros::LAST_EFFECT_SET_RATE(agent, 1.4);
    }
    frame(agent.lua_state_agent, 60.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -6, 0, 0, 0, 0, 0, 1.3, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn sound_finalair(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_luigi_009"));
        macros::PLAY_SE(agent, Hash40::new("se_luigi_final01"));
    }
    wait(agent.lua_state_agent, 54.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("vc_luigi_win02"));
        macros::PLAY_SE(agent, Hash40::new("se_luigi_final02"));
    }
}

unsafe extern "C" fn expression_finalair(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        ItemModule::set_have_item_visibility(agent.module_accessor, false, 0);
        sv_animcmd::START_INFO_FLASH_EYE(agent.lua_state_agent);
    }
    frame(agent.lua_state_agent, 56.0);
    if macros::is_excute(agent) {
        macros::QUAKE(agent, *CAMERA_QUAKE_KIND_L);
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_beamm"), 18, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 75.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_18_fireballsp"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 126.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_18_fireballsp2"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 156.0);
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_LR, 3);
    }
}

// pub fn install() {
//     Agent::new("luigi")
//     .set_costume([50, 51, 52, 53, 54, 55, 56, 57].to_vec())
//         // .game_acmd("game_finalstart", game_final, Priority::Low) 
//         // .sound_acmd("sound_finalstart", sound_final, Priority::Low)  
//         // .effect_acmd("effect_finalstart", effect_final, Priority::Low)  
//         // .expression_acmd("expression_finalstart", expression_final, Priority::Low)

//         // .game_acmd("game_finalairstart", game_finalair, Priority::Low) 
//         // .sound_acmd("sound_finalairstart", sound_finalair, Priority::Low)  
//         // .effect_acmd("effect_finalairstart", effect_finalair, Priority::Low)  
//         // .expression_acmd("expression_finalairstart", expression_finalair, Priority::Low)

//         .install();
// }