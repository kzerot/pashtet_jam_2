

use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::actions::game_control::{get_movement, GameControl};
use crate::characters::player::Player;
use crate::GameState;

mod game_control;

pub const FOLLOW_EPSILON: f32 = 5.;

pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Actions>().add_systems(
            Update,
            (set_movement_actions, mouse_control).run_if(in_state(GameState::Playing)),
        );
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
    pub mouse_angle: f32
}



pub fn mouse_control(
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut actions: ResMut<Actions>,
) {
    if let Ok(window) = primary_window.get_single() {
        if let Some(position) = window.cursor_position() {
            let win_size = Vec2 {x: window.width(), y: window.height() } / 2.0;
            let angle = (position - win_size).angle_between(Vec2::NEG_Y);
            actions.mouse_angle = angle;
        }
    }

}


pub fn set_movement_actions(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<Input<KeyCode>>,
    touch_input: Res<Touches>,
    player: Query<&Transform, With<Player>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let mut player_movement = Vec2::new(
        get_movement(GameControl::Right, &keyboard_input)
            - get_movement(GameControl::Left, &keyboard_input),
        get_movement(GameControl::Up, &keyboard_input)
            - get_movement(GameControl::Down, &keyboard_input),
    );

    if let Some(touch_position) = touch_input.first_pressed_position() {
        let (camera, camera_transform) = camera.single();
        if let Some(touch_position) = camera.viewport_to_world_2d(camera_transform, touch_position)
        {
            let diff = touch_position - player.single().translation.xy();
            if diff.length() > FOLLOW_EPSILON {
                player_movement = diff.normalize();
            }
        }
    }

    if player_movement != Vec2::ZERO {
        actions.player_movement = Some(player_movement.normalize());
    } else {
        actions.player_movement = None;
    }
}
