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

use crate::luigi::*;

// Pre
unsafe extern "C" fn luigi_fireball_start_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    StatusModule::init_settings(weapon.module_accessor, smash::app::SituationKind(*SITUATION_KIND_AIR), *WEAPON_KINETIC_TYPE_NORMAL, *GROUND_CORRECT_KIND_AIR as u32, smash::app::GroundCliffCheckKind(0), false, 0, 0, 0, 0);
    return 0.into();
}

// Main
unsafe extern "C" fn luigi_fireball_start_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    MotionModule::change_motion(weapon.module_accessor, Hash40::new("regular"), 0.0, 1.0, false, 0.0, false, false);
    weapon.fastshift(L2CValue::Ptr(luigi_fireball_start_main_loop as *const () as _))
}

unsafe extern "C" fn luigi_fireball_start_init(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let lr = PostureModule::lr(weapon.module_accessor);
    let owner_id = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER) as u32;
    let owner_boma = sv_battle_object::module_accessor(owner_id);

    // Spawn position
    let owner_x = PostureModule::pos_x(&mut *owner_boma);
    let owner_y = PostureModule::pos_y(&mut *owner_boma);
    let owner_z = PostureModule::pos_z(&mut *owner_boma);
    PostureModule::set_pos(
        weapon.module_accessor,
        &Vector3f { x: owner_x + (8.0 * lr), y: owner_y + 7.0, z: owner_z }
    );

    // Initial speed + gravity
    let speed_x = 1.3 * lr;
    let speed_y = 2.0;
    let gravity = 0.08;

    weapon.clear_lua_stack();
    sv_kinetic_energy!(set_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y);
    sv_kinetic_energy!(set_stable_speed, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, speed_x, speed_y);
    sv_kinetic_energy!(set_accel, weapon, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, 0.0, -gravity);
    KineticModule::enable_energy(weapon.module_accessor, *WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);

    // Lifespan
    WorkModule::set_int(weapon.module_accessor, 160, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);

    0.into()
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

    WorkModule::dec_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LIFE);
    
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

unsafe extern "C" fn luigi_fireball_start_exec(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let boma = weapon.module_accessor;
    
    let mut current_rotation_y = WorkModule::get_float(boma, ARTICLE_INSTANCE_WORK_ID_FLOAT_ROTATION);
    current_rotation_y += 5.0 ;
    WorkModule::set_float(boma, current_rotation_y, ARTICLE_INSTANCE_WORK_ID_FLOAT_ROTATION);

    let new_rotation = smash::phx::Vector3f {
        x: 0.0,
        y: current_rotation_y,
        z: 0.0,
    };

    ModelModule::set_joint_rotate(
    boma,
    Hash40::new("Drcapsule"),
    &new_rotation,
    MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},
    MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8}
    );
    
    let is_touching_ground = GroundModule::ray_check(
        weapon.module_accessor,
        &Vector2f { x: PostureModule::pos_x(weapon.module_accessor), y: PostureModule::pos_y(weapon.module_accessor) },
        &Vector2f { x: 0.0, y: -3.5 },
        true
    ) == 1;

    if is_touching_ground {
    let bounce_count = WorkModule::get_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_CUSTOMIZE_NO);

        // 1. Changed the limit to 3
        if bounce_count >= 3 {
            notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
            macros::EFFECT(weapon, Hash40::new("sys_erace_smoke"), Hash40::new("top"), 0, -3.5, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);
            weapon.pop_lua_stack(1);
            return 0.into();
        } else {
            let lr = PostureModule::lr(weapon.module_accessor);
            let bounce_x = 1.2 * lr;
            let bounce_y = 1.5;

            weapon.clear_lua_stack();
            sv_kinetic_energy!(set_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, bounce_x, bounce_y);
            sv_kinetic_energy!(set_stable_speed, weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL, bounce_x, bounce_y);
            
            // 2. Add this line to increase the bounce count
            WorkModule::inc_int(weapon.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_CUSTOMIZE_NO);

            // Bounce effect
            macros::PLAY_SE(weapon, Hash40::new("se_luigi_special_n02"));
            macros::FOOT_EFFECT(weapon, Hash40::new("sys_run_smoke"), Hash40::new("top"), 0, -3.5, 0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, false);

            WorkModule::set_int(weapon.module_accessor, bounce_count + 1, *WEAPON_INSTANCE_WORK_ID_INT_CUSTOMIZE_NO);
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("luigi_fireball")
    .set_costume([50, 51, 52, 53, 54, 55, 56, 57].to_vec())

        .status(Pre, *WEAPON_LUIGI_FIREBALL_STATUS_KIND_START, luigi_fireball_start_pre)
        .status(Init, *WEAPON_LUIGI_FIREBALL_STATUS_KIND_START, luigi_fireball_start_init)
        .status(Main, *WEAPON_LUIGI_FIREBALL_STATUS_KIND_START, luigi_fireball_start_main)
        .status(Exec, *WEAPON_LUIGI_FIREBALL_STATUS_KIND_START, luigi_fireball_start_exec)
        
        .install();
}