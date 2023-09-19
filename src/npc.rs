use crate::{
    animation::{FrameTime, SpriteAnimation},
    graphics::Graphics,
};
use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_npc_system);
    }
}

#[derive(Component, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct NPC;

pub fn spawn_npc_system(mut commands: Commands, graphics: Res<Graphics>) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: graphics.npc_texture_altas.clone(),
            sprite: TextureAtlasSprite {
                index: graphics.npc_index,
                custom_size: Some(Vec2::new(48.0, 64.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(200.0, 200.0, 400.0),
            ..Default::default()
        },
        Name::new("Npc"),
        SpriteAnimation {
            frame_time: 1.0 / 10.0,
            start_index: 0,
            len: 8,
        },
        NPC,
        FrameTime(0.0),
    ));
}
