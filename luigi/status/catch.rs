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

pub fn install() {
    Agent::new("luigi")
    .set_costume([50, 51, 52, 53, 54, 55, 56, 57].to_vec())

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
}