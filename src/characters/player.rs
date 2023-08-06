use crate::GameState;
use crate::actions::Actions;
use crate::loading::TextureAssets;

use bevy::prelude::*;
use bevy::window::{WindowResized, PrimaryWindow};
use bevy_easings::Lerp;
use crate::characters::base_character::*;

use super::bullets::shot_bullet;
pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Weapon {
    cd: f32,
    from_shot: f32
}


/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, (check_resolution, move_player, move_camera, animate_sprite, fire).run_if(in_state(GameState::Playing)));
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
    .insert(Weapon {
        cd: 0.1,
        from_shot: 0.0
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
    mut query: Query<(&Transform, &mut Weapon), With<Player>>, 
    buttons: Res<Input<MouseButton>>,
    textures: Res<TextureAssets>,
){
    if buttons.pressed(MouseButton::Left) {
        let (pl_transform, mut weapon) = query.single_mut();
        if weapon.from_shot > weapon.cd {
            weapon.from_shot = 0.0;
            let velocity = (pl_transform.rotation * Vec3::Y).normalize_or_zero();
            shot_bullet(&mut commands, &textures, pl_transform.translation + velocity*25.0, velocity, 1.0, 300.0)
        }
    }
}