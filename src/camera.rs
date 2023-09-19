use crate::player::Player;
use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, spawn_camera_system)
            .add_systems(Update, camera_follow_player_system);
    }
}

#[derive(Component)]
pub struct MainCamera;

pub fn spawn_camera_system(mut commands: Commands) {
    let mut camera_2d = Camera2d::default();

    camera_2d.clear_color = ClearColorConfig::Custom(Color::LIME_GREEN);

    commands.spawn((
        Camera2dBundle {
            camera_2d,
            ..Default::default()
        },
        MainCamera,
        Name::new("Main Camera"),
    ));
}

pub fn camera_follow_player_system(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok(player) = player_query.get_single() {
        let mut camera = camera_query.single_mut();

        camera.translation.x = player.translation.x;
        camera.translation.y = player.translation.y;
    }
}
