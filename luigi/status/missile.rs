use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*,
        hash40,
        app::*
	},
    smash_script::*,
    smashline::*
};

use super::*;

use crate::luigi::*;

//slope fix
unsafe extern "C" fn special_s_ram_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    return 0.into();
}

unsafe extern "C" fn special_s_ram_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_GROUND);

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE) {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_discharge"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);
    }

    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_LAST_STRANS);

    let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
    let influence_factor = 0.6; 
    let base_speed_y = 0.1; 
    let initial_y_speed = base_speed_y + (stick_y * influence_factor);

    WorkModule::set_float(fighter.module_accessor, initial_y_speed, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_INITIAL_Y_SPEED);

    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_ram_main_loop as *const () as _))
}

unsafe extern "C" fn special_s_ram_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    //Control fn
    let current_animation = MotionModule::motion_kind(fighter.module_accessor);
    if current_animation == Hash40::new("special_s").hash {
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
        let boma = fighter.module_accessor;

        let lr = PostureModule::lr(boma);
        let charge_level = WorkModule::get_float(boma, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLOAT_CHARGE);
        let charge_influence_factor = 0.005;
        let base_speed_x = 1.1;
        let new_speed_x;

        if AttackModule::is_infliction(boma, *COLLISION_KIND_MASK_ALL as i32) {
            new_speed_x = base_speed_x * lr;
        } else {
            new_speed_x = (base_speed_x + (charge_level * charge_influence_factor)) * lr;
        }

        let new_speed_y = WorkModule::get_float(boma, FIGHTER_LUIGI_INSTANCE_WORK_ID_FLOAT_SPECIAL_S_INITIAL_Y_SPEED);
        println!("{} speed y float", new_speed_y);
        let modified_new_speed_y;

        if GroundModule::is_touch(boma, *GROUND_TOUCH_FLAG_DOWN as u32) && new_speed_y < 0.0 {
            modified_new_speed_y = 0.0;
        } else {
            modified_new_speed_y = new_speed_y;
        }

        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, new_speed_x, modified_new_speed_y);
    }

    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        return 0.into();
    }
    
    //if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_GROUND) {
        //if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            //fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END.into(), false.into());
            //return 0.into();
        //}
    //}
    
    let ground_touch_flag = GroundModule::get_touch_flag(fighter.module_accessor);
    let facing_direction = PostureModule::lr(fighter.module_accessor);

    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_GROUND_CHECK) {
        if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE)
        && !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_SET_ATTACK_POWER)
        && AttackModule::is_attack(fighter.module_accessor, 0, false)
        {
            let charge_level = WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLOAT_CHARGE);
            let max_charge_level = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("charge_frame_max"));
            
            let power_mul;
            if charge_level >= max_charge_level {
                power_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("attack_power_charge"));
            } else {
                let min_power = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("attack_power"));
                let max_power = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("attack_power_charge"));
                let charge_ratio = charge_level / max_charge_level;
                power_mul = min_power + (max_power - min_power) * charge_ratio;
            }
        

            AttackModule::set_power_mul_status(fighter.module_accessor, power_mul);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_SET_ATTACK_POWER);
        }
    }

    else {
        let is_touching_left_wall = (ground_touch_flag & (*GROUND_TOUCH_FLAG_LEFT as u64)) != 0;
        let is_touching_right_wall = (ground_touch_flag & (*GROUND_TOUCH_FLAG_RIGHT as u64)) != 0;

        if (facing_direction == -1.0 && is_touching_left_wall) || (facing_direction == 1.0 && is_touching_right_wall) {
            if GroundModule::is_attachable(fighter.module_accessor, GroundTouchFlag(ground_touch_flag as i32)) {
                let x_speed = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                let wall_speed_threshold = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_wall_speed_x"));
                let rand_chance = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("special_s_bound_speed_mul")) / 100.0;
                let random_roll = sv_math::randf(Hash40::new("special_s").hash, 1.0);

                if x_speed.abs() > wall_speed_threshold && random_roll < rand_chance {
                    fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_WALL.into(), false.into());
                    return 0.into();
                }
            }
            fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END.into(), false.into());
            return 0.into();
        }
    }
    
    0.into()
}


pub fn install() {
    Agent::new("luigi")
    .set_costume([50, 51, 52, 53, 54, 55, 56, 57].to_vec())

        .status(Main, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, special_s_ram_main)
        .status(Init, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_RAM, special_s_ram_init)
        .install();
}