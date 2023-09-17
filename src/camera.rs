use crate::player::Player;
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

pub fn spawn_camera_system(mut commands: Commands) {
    let mut camera_2d = Camera2d::default();

    camera_2d.clear_color = ClearColorConfig::Custom(Color::LIME_GREEN);

    commands.spawn(Camera2dBundle {
        camera_2d,
        ..Default::default()
    });
}

pub fn camera_follow_player_system(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player = player_query.single();
    let mut camera = camera_query.single_mut();

    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
}
