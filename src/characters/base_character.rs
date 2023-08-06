
use bevy::prelude::*;

#[derive(Component)]
pub struct Hp(pub f32);

// Animation section
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
    // actions: Res<Actions>,
) {
    
    for (indices, mut timer, mut sprite) in &mut query {
   
        timer.tick(time.delta());
        if timer.just_finished() {                    

            sprite.index = if sprite.index >= indices.last {
                indices.first
            } 
            else if sprite.index < indices.first {
                indices.first
            }
            else {
                sprite.index + 1
            };

        }
    }

}