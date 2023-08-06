use std::f32::consts::PI;

use crate::loading::TextureAssets;
use crate::GameState;
use crate::characters::base_character::{AnimationIndices, AnimationTimer, Hp};
use crate::characters::player::Player;
use crate::map::DayNight;
use bevy::math::vec3;
use bevy::prelude::*;


pub struct EnemyPlugin;

#[derive(Resource)]
struct SpawnTimer {
    /// How often to spawn a new bomb? (repeating timer)
    timer: Timer,
}

#[derive(Component)]
pub struct Enemy {
    speed: f32
}

/// This plugin handles Enemy related stuff like movement
/// Enemy logic is only active during the State `GameState::Playing`
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(SpawnTimer {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating)
        })
        //.add_systems(OnEnter(GameState::Playing), spawn_enemy)
        .add_systems(Update, (move_enemy, spawn_enemy_timeout, check_death, check_collisions).run_if(in_state(GameState::Playing)))
        ;
    }
}


fn spawn_enemy(commands: &mut Commands, 
    textures: &Res<TextureAssets>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>, 
    pos: Vec3
) {

    let texture_atlas =
        TextureAtlas::from_grid(textures.texture_enemy.clone(), Vec2::new(200.0, 200.0), 4, 4, None, None);
    let texture_atlas_enemy_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices { first: 0, last: 15 };

    commands.spawn( (
        SpriteSheetBundle {
            texture_atlas: texture_atlas_enemy_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform:Transform{
                translation: pos,
                scale: vec3(0.25, 0.25, 0.25),
                ..Default::default()
            },
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ))
    .insert(Enemy { speed : 120.0 + rand::random::<f32>()*20.0 })
    .insert(Hp(2.0));

}

fn move_enemy(
    mut commands: Commands,
    day_night: Res<DayNight>,
    time: Res<Time>,
    mut enemy_query: Query<(&mut Transform, &Enemy, Entity, &ComputedVisibility), Without<Player>>,
    player_query: Query<&Transform, (Without<Enemy>, With<Player>)>,

) {
    let player = player_query.single();
    
    for (mut tr, enemy, entity, visibility) in enemy_query.iter_mut(){
        
        let mut direction = player.translation - tr.translation;
        if !day_night.is_night {
            direction = -direction;

        }
        // let near = direction.length_squared() <= 6000.0;
        direction.z = 0.0;
        direction = direction.normalize();

        let speed = enemy.speed;
        let movement = direction * speed * time.delta_seconds();
        tr.translation += movement;
        
        
        if !day_night.is_night {
            if !visibility.is_visible_in_view() {
                if let Some(ent) = commands.get_entity(entity) {
                    ent.despawn_recursive();
                    info!("Despawn fleed enemy");
                }
            }
        }


    }
}


fn spawn_enemy_timeout(
    mut timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    mut commands: Commands,
    day_night: Res<DayNight>,
    player_query: Query<&Transform, (Without<Enemy>, With<Player>)>,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>

){
    if day_night.is_night {
        timer.timer.tick(time.delta());
        if timer.timer.finished() {
            let base_pos = player_query.single().translation;
            for _ in 0..3 {
                let distance = 600.0 + rand::random::<f32>() * 600.0;
                let angle = PI * 2.0 * rand::random::<f32>();
                let pos = vec3(angle.cos() * distance, angle.sin() * distance, 0.05) + base_pos;
                spawn_enemy(&mut commands, &textures, &mut texture_atlases, pos);
            }
        }
    }
}

fn check_death(
    mut commands: Commands,
    query: Query<(&Hp, Entity), (Changed<Hp>, With<Enemy>)>
) {
    for (hp, entity) in query.iter() {
        if hp.0 <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn check_collisions(
    time: Res<Time>,
    query: Query<&Transform, With<Enemy>>,
    mut query_player: Query<(&mut Hp, &Transform), With<Player>>
) {
    let (mut hp, player_tr) = query_player.single_mut();

    for transform in query.iter() {
            if transform.translation.distance_squared(player_tr.translation) <= 900.0 {
                hp.0 -= 2.0 * time.delta_seconds();
            }
    }
}