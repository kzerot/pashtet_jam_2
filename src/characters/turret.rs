use std::f32::consts::PI;

use crate::loading::TextureAssets;
use crate::GameState;
use crate::characters::base_character::{AnimationIndices, AnimationTimer, Hp};
use crate::characters::player::Player;
use crate::map::DayNight;
use bevy::math::{vec3, vec2};
use bevy::prelude::*;

use super::bullets::shot_bullet;
use super::enemy::Enemy;
use super::player::{Weapon, WeaponPattern};


pub struct TurretPlugin;

#[derive(Component, Default)]
pub struct Turret{
    target: Option<Entity>
}

/// This plugin handles Turret related stuff like movement
/// Turret logic is only active during the State `GameState::Playing`
impl Plugin for TurretPlugin {
    fn build(&self, app: &mut App) {
        app

        //.add_systems(OnEnter(GameState::Playing), spawn_enemy)
        .add_systems(Update, (reload_turrets, check_for_enemy, fire_turret).run_if(in_state(GameState::Playing)))
        ;
    }
}

fn check_for_enemy(
    
    mut query: Query<(&mut Turret, &Transform)>,
    query_enemies: Query<(Entity, &Transform), With<Enemy>>
) {
    for (mut turret, transform) in query.iter_mut() {
        if turret.target.is_none() {
            let mut nearest_dist = 100000.0;
            let mut target : Option<Entity> = None;
            for (entity, transform_enemy) in query_enemies.iter() {
                let dist = transform.translation.distance_squared(transform_enemy.translation);
                if  dist < 1000000.0 && dist < nearest_dist {
                    target = Some(entity);
                    nearest_dist = dist;
                }
            }
            turret.target = target;
            if target.is_some() {
                info!("Found an enemy for a turret");
            }
        } else if let Some(target) = &turret.target {
            if !query_enemies.get(*target).is_ok() {
                info!("Target invalid");
                turret.target = None;
            }
        }
    }
}

fn reload_turrets(
    time: Res<Time>,
    mut query: Query<&mut Weapon, With<Turret>>,
){
    for mut turret_weapon in query.iter_mut() {
        turret_weapon.from_shot += time.delta_seconds();
    }
}

fn fire_turret(
    mut commands: Commands,
    mut query: Query<(&mut Transform, &mut Weapon, &Turret), (With<Turret>, Without<Enemy>)>,
    query_enemies: Query<&Transform, With<Enemy>>,
    textures: Res<TextureAssets>,
){

    for  (mut pl_transform, mut weapon, turret) in query.iter_mut(){
        if let Some(target) = turret.target {
            if let Ok(enemy) = query_enemies.get(target){
                let mut temp_enemy_trans  = enemy.translation;
                temp_enemy_trans.z = pl_transform.translation.z;

                let dir = (temp_enemy_trans - pl_transform.translation).normalize_or_zero();
                let dir2 = vec2(dir.x, dir.y).normalize_or_zero();
                let angle = PI - dir2.angle_between(Vec2::NEG_Y);
                info!("Dir: {} Angle {}", dir2, angle.to_degrees());
                pl_transform.rotation = Quat::from_rotation_z(angle);
                if weapon.from_shot > weapon.cd*2.0 {
                    weapon.from_shot = 0.0;
                    // let velocity = (pl_transform.rotation * Vec3::Y).normalize_or_zero();
                    let velocity = dir;
                    match weapon.pattern {
                        WeaponPattern::Single => {
                            shot_bullet(&mut commands, &textures, pl_transform.translation + velocity*25.0, velocity, 1.0, 300.0);
                        },
                        WeaponPattern::TwoShot => {
                            let velocity = Quat::from_axis_angle(Vec3::Z, -PI/16.0) * velocity;
                            shot_bullet(&mut commands, &textures, pl_transform.translation + velocity*25.0, velocity, 1.0, 300.0);
                            let velocity = Quat::from_axis_angle(Vec3::Z, PI/16.0) * velocity;
                            shot_bullet(&mut commands, &textures, pl_transform.translation + velocity*25.0, velocity, 1.0, 300.0);
                        },
                        WeaponPattern::SixRay => {
                            let points = vec![
                                pl_transform.rotation*vec3(-15.0, 25.0, pl_transform.translation.z),
                                pl_transform.rotation*vec3(-10.0, 25.0, pl_transform.translation.z),
                                pl_transform.rotation*vec3(-5.0, 25.0, pl_transform.translation.z),
                                pl_transform.rotation*vec3(5.0, 25.0, pl_transform.translation.z),
                                pl_transform.rotation*vec3(10.0, 25.0, pl_transform.translation.z),
                                pl_transform.rotation*vec3(15.0, 25.0, pl_transform.translation.z),
                            ];
                            for p in points {
                                shot_bullet(&mut commands, &textures, pl_transform.translation + p, velocity, 1.0, 300.0);
                            }
                        },
                        WeaponPattern::SixAround => {
                            let points = vec![
                                Quat::from_axis_angle(Vec3::Z, 0.0) * velocity * 25.0,
                                Quat::from_axis_angle(Vec3::Z, PI/3.0) * velocity * 25.0,
                                Quat::from_axis_angle(Vec3::Z, PI/1.5) * velocity * 25.0,
                                Quat::from_axis_angle(Vec3::Z, PI) * velocity * 25.0,
                                Quat::from_axis_angle(Vec3::Z, PI + PI/1.5) * velocity * 25.0,
                                Quat::from_axis_angle(Vec3::Z, PI + PI/3.0) * velocity * 25.0,
                            ];
                            for p in points {
                                shot_bullet(&mut commands, &textures, pl_transform.translation + p, p.normalize(), 1.0, 300.0);
                            }
                        },
                        WeaponPattern::ManyAround => {
                            let points = vec![
                                Quat::from_axis_angle(Vec3::Z, 0.0) * velocity * 25.0,
                                Quat::from_axis_angle(Vec3::Z, PI/3.0) * velocity * 25.0,
                                Quat::from_axis_angle(Vec3::Z, PI/1.5) * velocity * 25.0,
                                Quat::from_axis_angle(Vec3::Z, PI) * velocity * 25.0,
                                Quat::from_axis_angle(Vec3::Z, PI + PI/1.5) * velocity * 25.0,
                                Quat::from_axis_angle(Vec3::Z, PI + PI/3.0) * velocity * 25.0,
            
                                Quat::from_axis_angle(Vec3::Z, PI/6.0) * velocity * 25.0,
                                Quat::from_axis_angle(Vec3::Z, PI/6.0+PI/3.0) * velocity * 25.0,
                                Quat::from_axis_angle(Vec3::Z, PI/6.0+PI/1.5) * velocity * 25.0,
                                Quat::from_axis_angle(Vec3::Z, PI/6.0+PI) * velocity * 25.0,
                                Quat::from_axis_angle(Vec3::Z, PI/6.0+PI + PI/1.5) * velocity * 25.0,
                                Quat::from_axis_angle(Vec3::Z, PI/6.0+PI + PI/3.0) * velocity * 25.0,
                            ];
                            for p in points {
                                shot_bullet(&mut commands, &textures, pl_transform.translation + p, p.normalize(), 1.0, 300.0);
                            }
                        },
                    }
                }
            }
        }
    
    }
}