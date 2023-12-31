#![allow(clippy::type_complexity)]

mod actions;
mod audio;
mod loading;
mod menu;
mod characters;
mod map;
mod interactive_items;
mod ui;
mod menu_death;
mod menu_win;
use actions::ActionsPlugin;
use audio::InternalAudioPlugin;
use bevy_egui::EguiPlugin;
use characters::bullets::BulletPlugin;

use characters::cleaner::CleanerPlugin;
use characters::turret::TurretPlugin;
use loading::LoadingPlugin;
use menu::MenuPlugin;
use characters::player::PlayerPlugin;
use characters::enemy::EnemyPlugin;
use map::MapPlugin;

use bevy::app::App;
// #[cfg(debug_assertions)]
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use menu_death::MenuDeathPlugin;
use menu_win::MenuWinPlugin;
use ui::UiPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
    MenuDeath,
    MenuWin,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins((
            LoadingPlugin,
            MenuPlugin,
            MenuDeathPlugin,
            MenuWinPlugin,
            CleanerPlugin,
            ActionsPlugin,
            InternalAudioPlugin,
            PlayerPlugin,
            EnemyPlugin,
            TurretPlugin,
            MapPlugin,
            BulletPlugin,
            EguiPlugin,
            UiPlugin
        ));

        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        // }
    }
}
