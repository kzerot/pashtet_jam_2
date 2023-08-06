use bevy::prelude::*;

use crate::{map::{Map, DayNight}, ui::UiLog, GameState};

use super::{player::{Inventoty, TemporaryItems}};
pub struct CleanerPlugin;
impl Plugin for CleanerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Playing), despawn_all);
    }
}



fn despawn_all(
    mut commands: Commands,
    query_transform: Query<Entity, With<Transform>>,

) {
    query_transform.for_each(|entity|{
        commands.entity(entity).despawn_recursive();
    });
    commands.remove_resource::<Map>();
    commands.remove_resource::<UiLog>();
    commands.remove_resource::<Inventoty>();
    commands.remove_resource::<TemporaryItems>();
    commands.remove_resource::<DayNight>();

    commands.insert_resource(Map::default());
    commands.insert_resource(DayNight {
        current_day_time: 0.7,
        current_night_time: 0.3,
        full_day_time: 60.0,
        time: 0.0,
        is_night: false,
        day: 1
    });
    commands.init_resource::<UiLog>();
    commands.init_resource::<Inventoty>();
    commands.init_resource::<TemporaryItems>();
}