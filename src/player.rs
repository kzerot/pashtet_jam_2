use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy::window::WindowResized;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, (check_resolution, move_player, move_camera, animate_sprite).run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
    actions: Res<Actions>,
) {
    
    for (indices, mut timer, mut sprite) in &mut query {
   
        timer.tick(time.delta());
        if timer.just_finished() {                    
            let mut target_index = 4;
            if let Some(movement) = actions.player_movement{
                if movement.length_squared() > 0.01{
                    target_index = if sprite.index == indices.last {
                        indices.first
                    } else {
                        sprite.index + 1
                    };
                }
            }
            sprite.index = target_index;
        }
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
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
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
    commands.spawn(Camera2dBundle::default());
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
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
        parent.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_torso_handle,
                sprite: TextureAtlasSprite::new(animation_indices_torso.first),
                transform:Transform::from_translation(Vec3 { x: 0., y: 25.0, z: 0.3 }),
                ..default()
            },
            animation_indices_torso,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ));
    })
  
        
    .insert(Player);

}

fn move_camera(
    
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
){
    
    let trans = player_query.single().translation;
    let mut cam = camera_query.single_mut();
    cam.translation.x = trans.x;
    cam.translation.y = trans.y;

}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let movement;
    if actions.player_movement.is_some() {

        let speed = 150.;
        movement = Vec3::new(
            actions.player_movement.unwrap().x * speed * time.delta_seconds(),
            actions.player_movement.unwrap().y * speed * time.delta_seconds(),
            0.,
        );
    } else {
        movement = Vec3::ZERO;
    }
    for mut player_transform in &mut player_query {
        player_transform.translation += movement;
        player_transform.rotation =  Quat::from_rotation_z(actions.mouse_angle);
    }
}
