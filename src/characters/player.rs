use std::f32::consts::PI;

use crate::GameState;
use crate::actions::Actions;
use crate::interactive_items::chest::WEAPONS;
use crate::loading::TextureAssets;
use crate::ui::UiLog;

use bevy::math::vec3;
use bevy::prelude::*;
use bevy::window::{WindowResized, PrimaryWindow};
use bevy_easings::Lerp;
use crate::characters::base_character::*;

use super::bullets::shot_bullet;
use super::turret::Turret;
pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Clone, Copy)]
pub enum WeaponPattern {
    Single,
    TwoShot,
    SixRay,
    SixAround,
    ManyAround
}
#[derive(Component, Clone)]
pub struct Weapon {
    pub cd: f32,
    pub from_shot: f32,
    pub pattern: WeaponPattern,
    pub name: String
}
#[derive(Component)]
pub struct Energy(pub i32);

#[derive(Resource, Default)]
pub struct TemporaryItems {
    pub weapon: Option<Weapon>,
    pub turret: Option<Weapon>,
    pub timestamp: f64
}

#[derive(Resource)]
pub struct Inventoty {
    pub turret: Option<Weapon>
}

impl Default for Inventoty {
    fn default() -> Self {        
        Self { turret: Some(WEAPONS[0].clone()) }
    }
}
/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TemporaryItems>()
            .init_resource::<Inventoty>()
            .add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, (check_resolution, move_player, move_camera, animate_sprite, fire, change_weapon, place_turret).run_if(in_state(GameState::Playing)));
    }
}


fn set_aspect_for_camera(projection: &mut OrthographicProjection, w: f32){
    projection.scale = 800.0 / w;
}

fn check_resolution(mut query: Query<&mut OrthographicProjection, With<Camera>>, resize_event: Res<Events<WindowResized>>){
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        println!("width = {} height = {}", e.width, e.height);
        set_aspect_for_camera(&mut query.single_mut(), e.width);
    }
}


fn spawn_player(mut commands: Commands, 
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    primary_window: Query<&Window, With<PrimaryWindow>>
) {

    let texture_atlas =
        TextureAtlas::from_grid(textures.texture_legs.clone(), Vec2::new(200.0, 200.0), 4, 4, None, None);
    let texture_atlas_legs_handle = texture_atlases.add(texture_atlas);
    let texture_atlas =
        TextureAtlas::from_grid(textures.texture_torso.clone(), Vec2::new(200.0, 200.0), 4, 4, None, None);
    let texture_atlas_torso_handle = texture_atlases.add(texture_atlas);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 15 };
    let animation_indices_torso = AnimationIndices { first: 0, last: 15 };

    let mut scale = 1.0;
    if let Ok(window) = primary_window.get_single() {
           scale = 800.0 / window.width();
    }

    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection { 
             scale: scale,
            ..Default::default() 
        },
        ..Default::default()
    });
    commands.spawn( SpriteBundle {
        texture: textures.texture_shadow.clone(),
        transform: Transform {
            translation: Vec3 { x: 0., y: 25.0, z: 0.1 },
            scale: Vec3::splat(0.25),
            ..Default::default()
        },
        ..Default::default()
    }).with_children(|parent|{
        parent.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_legs_handle,
                sprite: TextureAtlasSprite::new(animation_indices.first),
                transform:Transform::from_translation(Vec3 { x: 0., y: 25.0, z: 0.2 }),
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
        ));
        parent.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_torso_handle,
                sprite: TextureAtlasSprite::new(animation_indices_torso.first),
                transform:Transform::from_translation(Vec3 { x: 0., y: 25.0, z: 0.3 }),
                ..default()
            },
            animation_indices_torso,
            AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
        ));
    }) 
    
    .insert(Player)
    .insert(Hp(100.0))
    .insert(Energy(100))
    .insert(Weapon {
        cd: 0.2,
        from_shot: 0.0,
        pattern: WeaponPattern::Single,
        name: "Base Eradicator".into()
    });

}

fn move_camera(
    
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
){
    
    let trans = player_query.single().translation;
    let mut cam = camera_query.single_mut();
    cam.translation.x = cam.translation.x.lerp(&trans.x, &0.2);
    cam.translation.y = cam.translation.y.lerp(&trans.y, &0.2);

}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&mut Transform, &mut Weapon, &Children), With<Player>>,
    mut sprites: Query<&mut AnimationIndices>
) {
    let movement;
    let mut anim_indices = (4,4);
    if actions.player_movement.is_some() {

        let speed = 150.;
        movement = Vec3::new(
            actions.player_movement.unwrap().x * speed * time.delta_seconds(),
            actions.player_movement.unwrap().y * speed * time.delta_seconds(),
            0.,
        );
        anim_indices = (0, 15);
    } else {
        movement = Vec3::ZERO;
    }
    for (mut player_transform, mut weapon, children) in &mut player_query {
        weapon.from_shot += time.delta_seconds();
        player_transform.translation += movement;
        player_transform.rotation =  Quat::from_rotation_z(actions.mouse_angle);
        for child in children.iter() {
            let sprite = sprites.get_mut(*child);
            if let Ok(mut sprite) = sprite {
                sprite.first = anim_indices.0;
                sprite.last = anim_indices.1;
            }
        }
    }
}

fn fire(
    mut commands: Commands,
    mut query: Query<(&Transform, &mut Weapon, &mut Energy), With<Player>>, 
    buttons: Res<Input<MouseButton>>,
    textures: Res<TextureAssets>,
){
    if buttons.pressed(MouseButton::Left) {
        let (pl_transform, mut weapon, mut energy) = query.single_mut();
        if weapon.from_shot > weapon.cd && energy.0 > 0 {
            weapon.from_shot = 0.0;
            let velocity = (pl_transform.rotation * Vec3::Y).normalize_or_zero();
            match weapon.pattern {
                WeaponPattern::Single => {
                    shot_bullet(&mut commands, &textures, pl_transform.translation + velocity*25.0, velocity, 1.0, 300.0);
                    energy.0 -= 1;
                },
                WeaponPattern::TwoShot => {
                    let velocity = Quat::from_axis_angle(Vec3::Z, -PI/16.0) * velocity;
                    shot_bullet(&mut commands, &textures, pl_transform.translation + velocity*25.0, velocity, 1.0, 300.0);
                    let velocity = Quat::from_axis_angle(Vec3::Z, PI/16.0) * velocity;
                    shot_bullet(&mut commands, &textures, pl_transform.translation + velocity*25.0, velocity, 1.0, 300.0);
                    energy.0 -= 2;
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
     
                    energy.0 -= 6;
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
                    energy.0 -= 6;
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
                    energy.0 -= 12;
                },
            }
            energy.0 = energy.0.clamp(0, 500);
        }
    }
}


fn change_weapon(
    mut inventory: ResMut<Inventoty>,
    mut temporary: ResMut<TemporaryItems>,
    time: Res<Time>,
    mut query: Query<&mut Weapon, With<Player>>,
    keys: Res<Input<KeyCode>>,
    mut log: ResMut<UiLog>
) {
    if keys.just_pressed(KeyCode::R){
        if temporary.weapon.is_some() && time.elapsed_seconds_f64() - temporary.timestamp <= 5.0 {
            let mut weapon = query.single_mut();
            if let Some(random_weapon) = &temporary.weapon {
                weapon.name = random_weapon.name.clone();
                weapon.cd = random_weapon.cd;
                weapon.pattern = random_weapon.pattern;
                log.message_time_stamp = time.elapsed_seconds_f64();
                log.last_message = format!("Weapon changed to {}", weapon.name);
                temporary.weapon = None;
            }
        }
        else if temporary.turret.is_some()  && time.elapsed_seconds_f64() - temporary.timestamp <= 5.0 {
            if let Some(random_weapon) = &temporary.turret {
                let new_turret = Weapon {
                    cd: random_weapon.cd,
                    from_shot: 0.0,
                    pattern: random_weapon.pattern,
                    name: random_weapon.name.clone(),
                };
                inventory.turret = Some(new_turret);
                log.message_time_stamp = time.elapsed_seconds_f64();
                log.last_message = format!("Turret changed to {}", random_weapon.name);
                temporary.turret = None;
            }
        }
    }

}

fn place_turret(
    mut inventory: ResMut<Inventoty>,
    mut commands: Commands,
    query: Query<&Transform, With<Player>>,
    keys: Res<Input<KeyCode>>,
    textures: Res<TextureAssets>,
) {
    if keys.just_pressed(KeyCode::E) {
        if let Some(turret) = &inventory.turret {
            let transform = query.single().translation;
            commands.spawn(
                SpriteBundle {
                    texture: textures.texture_turret.clone(),
                    transform: Transform::from_translation(transform).with_scale(Vec3::splat(0.25)),
                    ..Default::default()
                }
            ).insert(Turret::default()).insert(turret.clone());
        }
        inventory.turret = None;
    }
}