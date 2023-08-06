use bevy::{prelude::*, math::vec3};
use bevy::sprite::SpriteBundle;

use crate::{loading::TextureAssets, GameState};

use super::{enemy::Enemy, base_character::Hp};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, ((move_bullets, check_collisions).chain()).run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Bullet {
    damage: f32,
    speed: f32, 
    time_alive: f32
}

#[derive(Component)]
pub struct BulletVelocity(Vec3);


pub fn shot_bullet(
    commands: &mut Commands,
    textures: &Res<TextureAssets>,
    pos: Vec3,
    velocity: Vec3,
    damage: f32,
    speed: f32
) {
    commands.spawn( SpriteBundle {
        texture: textures.texture_bullet.clone(),
        transform: Transform {
            translation: pos + vec3(0.,0.,0.3),
            scale: Vec3::splat(0.25),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Bullet {
        damage,
        speed,
        time_alive: 0.0
    })
    .insert(BulletVelocity(velocity));
}


fn move_bullets(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Bullet, &BulletVelocity)>,
) {

    for (mut tr, mut bullet, velocity) in query.iter_mut() {
        tr.translation += velocity.0 * time.delta_seconds() * bullet.speed;
        bullet.time_alive += time.delta_seconds();
    }
}

fn check_collisions(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Bullet)>,
    mut query_enemies: Query<(&mut Hp, &Transform), With<Enemy>>
) {
    for (entity, bullet_transform, bullet) in query.iter() {
        if bullet.time_alive >= 5.0 {
            if let Some(ent) = commands.get_entity(entity){
                ent.despawn_recursive();
            }
            continue;
        }
        for (mut enemy_hp, enemy_transform) in query_enemies.iter_mut() {
            if bullet_transform.translation.distance_squared(enemy_transform.translation) <= 25.0*25.0 {
                enemy_hp.0 -= bullet.damage;
                if let Some(ent) = commands.get_entity(entity){
                    ent.despawn_recursive();
                }
            }
        }
    }
}

