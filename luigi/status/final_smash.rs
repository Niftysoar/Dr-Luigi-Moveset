use super::*;

unsafe extern "C" fn mariod_final_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_pre_FinalCommon().get_bool();
    aLStack120 = SITUATION_KIND_NONE;
    aLStack136 = FIGHTER_KINETIC_TYPE_UNIQ;
    aLStack152 = GROUND_CORRECT_KIND_KEEP;
    aLStack168 = GROUND_CLIFF_CHECK_KIND_NONE;
    aLStack184 = false;
    aLStack200 = FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG;
    aLStack216 = FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT;
    aLStack232 = FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT;
    aLStack248 = 0;
    SVar5 = aLStack120;
    iVar6 = aLStack136;
    uVar7 = aLStack152;
    GVar8 = aLStack168;
    bVar1 = aLStack184;
    iVar9 = aLStack200;
    iVar10 = aLStack216;
    iVar11 = aLStack232;
    aLStack248;
    StatusModule::init_settings(fighter.module_accessor, SVar5, iVar6, uVar7, GVar8, bVar1, iVar9, iVar10, iVar11, in_stack_fffffffffffffef4);
    lib::L2CValue::_L2CValue(aLStack248);
    lib::L2CValue::_L2CValue(aLStack232);
    lib::L2CValue::_L2CValue(aLStack216);
    lib::L2CValue::_L2CValue(aLStack200);
    lib::L2CValue::_L2CValue(aLStack184);
    lib::L2CValue::_L2CValue(aLStack168);
    lib::L2CValue::_L2CValue(aLStack152);
    lib::L2CValue::_L2CValue(aLStack136);
    lib::L2CValue::_L2CValue(aLStack120);
    aLStack120 = false;
    aLStack136 = FIGHTER_TREADED_KIND_NO_REAC;
    aLStack152 = false;
    aLStack168 = false;
    aLStack184 = false;
    aLStack200 = FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_FINAL | FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_ATTACK | FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON;
    aLStack216 = FIGHTER_STATUS_ATTR_DISABLE_ITEM_INTERRUPT | FIGHTER_STATUS_ATTR_DISABLE_TURN_DAMAGE | FIGHTER_STATUS_ATTR_FINAL;
    aLStack232 = FIGHTER_POWER_UP_ATTACK_BIT_FINAL;
    aLStack248 = 0;
    bVar1 = aLStack120;
    iVar6 = aLStack136;
    bVar2 = aLStack152;
    bVar3 = aLStack168;
    bVar4 = aLStack184;
    uVar13 = aLStack200;
    uVar7 = aLStack216;
    uVar12 = aLStack232;
    aLStack248;
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, (bVar1 & 1), iVar6, (bVar2 & 1), (bVar3 & 1), bVar4, uVar13, uVar7, uVar12, in_stack_fffffffffffffef4);
    lib::L2CValue::_L2CValue(aLStack248);
    lib::L2CValue::_L2CValue(aLStack232);
    lib::L2CValue::_L2CValue(aLStack216);
    lib::L2CValue::_L2CValue(aLStack200);
    lib::L2CValue::_L2CValue(aLStack184);
    lib::L2CValue::_L2CValue(aLStack168);
    lib::L2CValue::_L2CValue(aLStack152);
    lib::L2CValue::_L2CValue(aLStack136);
    lib::L2CValue::_L2CValue(aLStack120);
    return_value = 0;
    return return_value.into();
}

unsafe extern "C" fn mariod_final_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    aLStack96 = 0x201bc9217c;
    fighter.clear_lua_stack();
    fighter.push_lua_stack(&mut L2CValue::new_num(aLStack96));
    sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
    lib::L2CAgent::pop_lua_stack(fighter, 1);
    lib::L2CValue::_L2CValue(aLStack160);
    lib::L2CValue::_L2CValue(aLStack96);
    aLStack96 = false;
    bVar1 = aLStack96;
    AreaModule::set_whole(fighter.module_accessor, bVar1);
    lib::L2CValue::_L2CValue(aLStack96);
    aLStack96 = FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT;
    iVar3 = aLStack96;
    WorkModule::enable_transition_term(fighter.module_accessor, iVar3);
    lib::L2CValue::_L2CValue(aLStack96);
    aLStack96 = FIGHTER_STATUS_TRANSITION_TERM_ID_FALL;
    iVar3 = aLStack96;
    WorkModule::enable_transition_term(fighter.module_accessor, iVar3);
    lib::L2CValue::_L2CValue(aLStack96);
    this_00 = fighter.global_table :: 0x16;
    aLStack96 = SITUATION_KIND_GROUND;
    uVar5 = this_00 :: aLStack96;
    lib::L2CValue::_L2CValue(aLStack96);
    if !uVar5 {
        aLStack96 = FIGHTER_KINETIC_TYPE_MOTION_AIR;
        iVar3 = aLStack96;
        KineticModule::change_kinetic(fighter.module_accessor, iVar3);
        lib::L2CValue::_L2CValue(aLStack96);
        aLStack96 = GROUND_CORRECT_KIND_AIR;
        GVar4 = aLStack96;
        GroundModule::correct(fighter.module_accessor, GVar4);
        lib::L2CValue::_L2CValue(aLStack96);
        aLStack112 = FIGHTER_MARIOD_STATUS_FINAL_FLAG_FIRST;
        iVar3 = aLStack112;
        bVar1 = WorkModule::is_flag(fighter.module_accessor, iVar3);
        aLStack96 = bVar1;
        bVar2 = aLStack96;
        lib::L2CValue::_L2CValue(aLStack96);
        lib::L2CValue::_L2CValue(aLStack112);
        if !bVar2 {
            aLStack96 = Hash40::new("final_air");
            aLStack112 = 0.0;
            aLStack128 = 1.0;
            aLStack144 = false;
            HVar6 = aLStack96;
            fVar7 = aLStack112;
            fVar8 = aLStack128;
            bVar1 = aLStack144;
            MotionModule::change_motion(fighter.module_accessor, HVar6, fVar7, fVar8, bVar1, 0.0, false, false);
            lib::L2CValue::_L2CValue(aLStack144);
            lib::L2CValue::_L2CValue(aLStack128);
            lib::L2CValue::_L2CValue(aLStack112);
            lib::L2CValue::_L2CValue(aLStack96);
            aLStack96 = FIGHTER_MARIOD_STATUS_FINAL_FLAG_FIRST;
            iVar3 = aLStack96;
            WorkModule::on_flag(fighter.module_accessor, iVar3);
        } else {
            aLStack96 = Hash40::new("final_air");
            HVar6 = aLStack96;
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, HVar6, -1.0, 1.0, 0.0, false, false);
        }
    } else {
        aLStack96 = FIGHTER_KINETIC_TYPE_MOTION;
        iVar3 = aLStack96;
        KineticModule::change_kinetic(fighter.module_accessor, iVar3);
        lib::L2CValue::_L2CValue(aLStack96);
        aLStack96 = GROUND_CORRECT_KIND_GROUND;
        GVar4 = aLStack96;
        GroundModule::correct(fighter.module_accessor, GVar4);
        lib::L2CValue::_L2CValue(aLStack96);
        aLStack112 = FIGHTER_MARIOD_STATUS_FINAL_FLAG_FIRST;
        iVar3 = aLStack112;
        bVar1 = WorkModule::is_flag(fighter.module_accessor, iVar3);
        aLStack96 = bVar1;
        bVar2 = aLStack96;
        lib::L2CValue::_L2CValue(aLStack96);
        lib::L2CValue::_L2CValue(aLStack112);
        if !bVar2 {
            aLStack96 = Hash40::new("final_start");
            aLStack112 = 0.0;
            aLStack128 = 1.0;
            aLStack144 = false;
            HVar6 = aLStack96;
            fVar7 = aLStack112;
            fVar8 = aLStack128;
            bVar1 = aLStack144;
            MotionModule::change_motion(fighter.module_accessor, HVar6, fVar7, fVar8, bVar1, 0.0, false, false);
            lib::L2CValue::_L2CValue(aLStack144);
            lib::L2CValue::_L2CValue(aLStack128);
            lib::L2CValue::_L2CValue(aLStack112);
            lib::L2CValue::_L2CValue(aLStack96);
            aLStack96 = FIGHTER_MARIOD_STATUS_FINAL_FLAG_FIRST;
            iVar3 = aLStack96;
            WorkModule::on_flag(fighter.module_accessor, iVar3);
        } else {
            aLStack96 = Hash40::new("final_start");
            HVar6 = aLStack96;
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, HVar6, -1.0, 1.0, 0.0, false, false);
        }
    }
    lib::L2CValue::_L2CValue(aLStack96);
    aLStack96 = mariod_final_main_loop;
    fighter.sub_shift_status_main(0xa0).get_bool();
    lib::L2CValue::_L2CValue(aLStack96);
    return return_value.into();
}

unsafe extern "C" fn mariod_final_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    bVar1 = CancelModule::is_enable_cancel(fighter.module_accessor);
    aLStack80 = bVar1;
    aLStack64 = true;
    uVar4 = aLStack80 :: aLStack64;
    lib::L2CValue::_L2CValue(aLStack64);
    if !uVar4 {
        lib::L2CValue::_L2CValue(aLStack80);
// LAB_710000574c:
        aLStack80 = FIGHTER_STATUS_TRANSITION_TERM_ID_WAIT;
        iVar3 = aLStack80;
        bVar1 = WorkModule::is_enable_transition_term(fighter.module_accessor, iVar3);
        aLStack64 = bVar1;
        bVar2 = aLStack64;
        lib::L2CValue::_L2CValue(aLStack64);
        lib::L2CValue::_L2CValue(aLStack80);
        if !bVar2 {
// LAB_7100005830:
            aLStack80 = FIGHTER_STATUS_TRANSITION_TERM_ID_FALL;
            iVar3 = aLStack80;
            bVar1 = WorkModule::is_enable_transition_term(fighter.module_accessor, iVar3);
            aLStack64 = bVar1;
            bVar2 = aLStack64;
            lib::L2CValue::_L2CValue(aLStack64);
            lib::L2CValue::_L2CValue(aLStack80);
            if !bVar2 // goto LAB_7100005930;
            bVar1 = MotionModule::is_end(fighter.module_accessor);
            aLStack64 = bVar1;
            bVar2 = aLStack64;
            lib::L2CValue::_L2CValue(aLStack64);
            if !bVar2 // goto LAB_7100005930;
            pLVar5 = fighter.global_table :: 0x16;
            aLStack64 = SITUATION_KIND_AIR;
            uVar4 = pLVar5 :: aLStack64;
            lib::L2CValue::_L2CValue(aLStack64);
            if !uVar4 // goto LAB_7100005930;
            aLStack64 = FIGHTER_STATUS_KIND_FINAL_JUMP_END;
            aLStack80 = false;
            fighter.change_status(0xc0.into(), 0xb0.into());
        } else {
            bVar1 = MotionModule::is_end(fighter.module_accessor);
            aLStack64 = bVar1;
            bVar2 = aLStack64;
            lib::L2CValue::_L2CValue(aLStack64);
            if !bVar2 // goto LAB_7100005830;
            pLVar5 = fighter.global_table :: 0x16;
            aLStack64 = SITUATION_KIND_GROUND;
            uVar4 = pLVar5 :: aLStack64;
            lib::L2CValue::_L2CValue(aLStack64);
            if !uVar4 // goto LAB_7100005830;
            aLStack64 = FIGHTER_STATUS_KIND_WAIT;
            aLStack80 = false;
            fighter.change_status(0xc0.into(), 0xb0.into());
        }
        lib::L2CValue::_L2CValue(aLStack80);
        pLVar5 = aLStack64;
    } else {
        aLStack112 = false;
        fighter.sub_wait_ground_check_common(0x90).get_bool();
        aLStack64 = false;
        uVar4 = aLStack96 :: aLStack64;
        lib::L2CValue::_L2CValue(aLStack64);
        if uVar4 {
            fighter);
            aLStack64 = false;
            uVar4 = aLStack128 :: aLStack64;
            lib::L2CValue::_L2CValue(aLStack64);
            lib::L2CValue::_L2CValue(aLStack128);
            lib::L2CValue::_L2CValue(aLStack96);
            lib::L2CValue::_L2CValue(aLStack112);
            lib::L2CValue::_L2CValue(aLStack80);
            if !uVar4 // goto LAB_7100005930;
            // goto LAB_710000574c;
        }
        lib::L2CValue::_L2CValue(aLStack96);
        lib::L2CValue::_L2CValue(aLStack112);
        pLVar5 = aLStack80;
    }
    lib::L2CValue::_L2CValue(pLVar5);
// LAB_7100005930:
    return_value = 0;
    return return_value.into(.sub_air_check_fall_common().get_bool();
}

unsafe extern "C" fn mariod_final_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    aLStack64 = FIGHTER_MARIOD_GENERATE_ARTICLE_HUGECAPSULE;
    aLStack80 = ARTICLE_OPE_TARGET_ALL;
    aLStack96 = false;
    iVar2 = aLStack64;
    AVar3 = aLStack80;
    bVar1 = aLStack96;
    ArticleModule::shoot(fighter.module_accessor, iVar2, AVar3, bVar1);
    lib::L2CValue::_L2CValue(aLStack96);
    lib::L2CValue::_L2CValue(aLStack80);
    lib::L2CValue::_L2CValue(aLStack64);
    aLStack80 = 0x1e0aba2d68;
    fighter.clear_lua_stack();
    fighter.push_lua_stack(&mut L2CValue::new_num(aLStack80));
    sv_battle_object::notify_event_msc_cmd(fighter.lua_state_agent);
    lib::L2CAgent::pop_lua_stack(fighter, 1);
    lib::L2CValue::_L2CValue(aLStack64);
    lib::L2CValue::_L2CValue(aLStack80);
    return_value = 0;
    return return_value.into();
}