use std::f32::consts::PI;

use bevy::{prelude::{Plugin, Component, OnEnter, Commands, Res, Transform, Resource, Entity, ResMut, Update, Query, With, Camera2d, info, IntoSystemConfigs, in_state, DespawnRecursiveExt, Color, PointLightBundle, Without, EventWriter, Event, Vec3, Quat}, sprite::{SpriteBundle, Sprite, TextureAtlasSprite}, math::vec3, utils::HashMap, time::Time};

use crate::{loading::TextureAssets, GameState, characters::{enemy::Enemy, bullets::Bullet}, interactive_items::chest::{Chest, open_chest}};

pub struct MapPlugin;

#[derive(Resource)]
pub struct DayNight {
    pub current_day_time: f32,
    pub current_night_time: f32,
    pub full_day_time: f32,
    pub time: f32,
    pub is_night: bool,
    pub day: i32
}

#[derive(Component)]
struct Ground;

#[derive(Resource, Default)]
pub struct Map {
    pub tiles: HashMap<(i32, i32), Entity>,
    pub last_position: (i32, i32),
    pub chestes_spawned: Vec<(i32, i32)>
}

// Event
#[derive(Event)]
pub struct DayNightEvent(pub bool);


impl Plugin for MapPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_event::<DayNightEvent>()
        .insert_resource(Map::default())
        .insert_resource(DayNight {
            current_day_time: 0.7,
            current_night_time: 0.3,
            full_day_time: 60.0,
            time: 0.0,
            is_night: false,
            day: 1
        })
        .add_systems(OnEnter(GameState::Playing), spawn_map)
        .add_systems(Update, (open_chest, (check_map, day_night_cycle, day_night_coloring).chain()).run_if(in_state(GameState::Playing)))
        ;
    }
}


fn day_night_cycle(
    mut day_night: ResMut<DayNight>,
    time: Res<Time>,
    mut ev_daynight: EventWriter<DayNightEvent>,
) {
    day_night.time += time.delta_seconds();
    if day_night.time >= day_night.full_day_time {
        info!("New day");
        day_night.day += 1;
        day_night.time = 0.0;
        day_night.current_day_time = (day_night.current_day_time - 0.05).clamp(0.1, 0.9);
        day_night.current_night_time = (day_night.current_night_time + 0.05).clamp(0.1, 0.9);
        day_night.is_night = false;
        ev_daynight.send(DayNightEvent(false));
    }
    else if !day_night.is_night && day_night.time >= day_night.current_day_time * day_night.full_day_time{
        
        day_night.is_night = true;
        ev_daynight.send(DayNightEvent(true));
        info!("New night");
    }
}

fn day_night_coloring(
    day_night: Res<DayNight>,
    mut query_animated: Query<&mut TextureAtlasSprite, Without<Enemy>>,
    mut query_simple: Query<&mut Sprite, Without<Bullet>>
) {
    let light_intency;
    let need_change;
    let time_percent = day_night.time/day_night.full_day_time;
    
    if time_percent > day_night.current_day_time - 0.1 && time_percent <= day_night.current_day_time {
        light_intency = (day_night.current_day_time - time_percent) * 10.0;
        need_change = true;
    }
    else if time_percent > 0.0 && time_percent <= 0.1 {
        light_intency = time_percent * 10.0;
        need_change = true;
    } else {
        light_intency = 0.0;
        need_change = false;
    }
    if need_change {
        let target_color_vec = vec3(0.4, 0.4, 0.7);
        let current_color_vec = target_color_vec.lerp(vec3(1., 1., 1.), light_intency);
        for mut sprite in query_animated.iter_mut() {
            sprite.color = Color::rgb(current_color_vec.x, current_color_vec.y, current_color_vec.z);
        }
        for mut sprite in query_simple.iter_mut() {
            sprite.color = Color::rgb(current_color_vec.x, current_color_vec.y, current_color_vec.z);    
        }
    }
}

pub fn spawn_map(
    mut command: Commands,
    mut map: ResMut<Map>,
    textures: Res<TextureAssets>
) {
    for x in -2..3 {
        for y in -2..3 {
            let position = vec3(
                x as f32 * 512.0,
                y as f32 * 512.0,
                0.0
            );
            let id = command.spawn(
                SpriteBundle{
                    texture: textures.texture_ground.clone(),
                    transform: Transform::from_translation(position),
                    ..Default::default()
                }
            ).id();
            map.tiles.insert((x,y), id);                
            if !map.chestes_spawned.contains(&(x,y)){
                map.chestes_spawned.push((x,y));
                spawn_chests(&mut command, &textures, position, Color::WHITE);
            }
        }
    }
    command.spawn(PointLightBundle {
        transform: Transform { translation: vec3(0.0, 0.0, -0.5), ..Default::default() },
        ..Default::default()
    });
}

pub fn check_map(
    day_night: Res<DayNight>,
    mut command: Commands,
    mut map: ResMut<Map>,
    textures: Res<TextureAssets>,
    query: Query<&Transform, With<Camera2d>>
) {
    let camera_position = query.single().translation;
    let true_position = ((camera_position.x / 512.0) as i32, (camera_position.y / 512.0) as i32);
    if true_position != map.last_position {
        map.last_position = true_position;
        // clean old
        let mut new_positions: Vec<(i32, i32)> = Vec::new();
        for x in -2..3 {
            for y in -2..3 {
                new_positions.push((true_position.0 + x, true_position.1 + y));
            }
        }

        let keys: Vec<(i32, i32)> = map.tiles.keys().cloned().collect();
        for pos in keys {
            if !new_positions.contains(&pos) {
                let ent = map.tiles.get(&pos);
                if let Some(ent) = ent {
                    command.entity(*ent).despawn_recursive();
                }
                map.tiles.remove(&pos);
            }
        }


        // Spawn tile with needed color
        let light_intency;
        let time_percent = day_night.time/day_night.full_day_time;
        
        if time_percent > day_night.current_day_time - 0.1 && time_percent <= day_night.current_day_time {
            light_intency = (day_night.current_day_time - time_percent) * 10.0;
        }
        else if time_percent > 0.0 && time_percent <= 0.1 {
            light_intency = time_percent * 10.0;
        } else if day_night.is_night {
            light_intency = 0.0;
        } else {
            light_intency = 1.0;
        }
        let target_color_vec = vec3(0.4, 0.4, 0.7);
        let current_color_vec = target_color_vec.lerp(vec3(1., 1., 1.), light_intency);
        let color = Color::rgb(current_color_vec.x, current_color_vec.y, current_color_vec.z);
        for pos in new_positions.iter() {
            if !map.tiles.contains_key(pos) {
                let position = vec3(
                    pos.0 as f32 * 512.0,
                    pos.1 as f32 * 512.0,
                    0.0
                );
                let id = command.spawn(
                    SpriteBundle{
                        texture: textures.texture_ground.clone(),
                        transform: Transform::from_translation(position),
                        sprite: Sprite {
                            color: color,
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                ).id();
                map.tiles.insert(*pos, id);
                if !map.chestes_spawned.contains(pos){
                    map.chestes_spawned.push(*pos);
                    spawn_chests(&mut command, &textures, position, color)
                }

            }
        }
    }
}

fn spawn_chests(
    commands: &mut Commands,
    textures: &Res<TextureAssets>,
    pos: Vec3,
    color: Color,
) {
    let chest_num = 2 + (rand::random::<f32>() * 6.0) as i32;
    for _ in 0..chest_num {
        let new_pos = pos + vec3(
            pos.x - 128.0 + rand::random::<f32>() * 256.0,
            pos.y - 128.0 + rand::random::<f32>() * 256.0,
            0.1
        );
        let mut chest = Chest::default();
        chest.generate();
        commands.spawn(
            SpriteBundle {
                texture: textures.texture_chest_closed.clone(),
                transform: Transform::from_translation(new_pos)
                    .with_scale(Vec3::splat(0.25))
                    .with_rotation(Quat::from_axis_angle(Vec3::Z, PI*2.0* rand::random::<f32>())),
                sprite: Sprite {
                    color: color,
                    ..Default::default()
                },
                ..Default::default()
            }
        ).insert(chest);
    }
}