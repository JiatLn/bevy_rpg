use bevy::prelude::*;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprite_system);
    }
}

#[derive(Component)]
pub struct SpriteAnimation {
    pub start_index: usize,
    pub len: usize,
    pub frame_time: f32,
}

#[derive(Component)]
pub struct FrameTime(pub f32);

pub fn animate_sprite_system(
    mut query: Query<(&mut TextureAtlasSprite, &SpriteAnimation, &mut FrameTime)>,
    time: Res<Time>,
) {
    for (mut sprite, animation, mut frame_time) in query.iter_mut() {
        if sprite.index + animation.len < animation.start_index {
            sprite.index = animation.start_index
        }
        frame_time.0 += time.delta_seconds();
        if frame_time.0 > animation.frame_time {
            let frames = (frame_time.0 / animation.frame_time) as usize;
            sprite.index += frames;
            if sprite.index >= animation.start_index + animation.len {
                sprite.index = animation.start_index
            }

            frame_time.0 -= animation.frame_time * frames as f32;
        }
    }
}
