use bevy::prelude::*;

mod camera;
mod crafting;
mod debug;
mod inventory;
mod player;
mod resources;
mod systems;
mod utils;

fn main() {
    let default_plugins = DefaultPlugins
        .set(ImagePlugin::default_nearest())
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Logic Farming Rougelike".into(),
                resolution: (1600.0 / 2.0, 900.0 / 2.0).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        });

    App::new()
        .add_plugins(default_plugins)
        .add_systems(PreStartup, systems::load_graphics)
        .insert_resource(crafting::CraftingBook::new())
        .add_systems(Startup, camera::spawn_camera_system)
        .add_systems(Startup, player::spawn_palyer_system)
        .add_systems(Startup, inventory::spawn_items_system)
        .add_systems(Update, player::player_movement_system)
        .add_systems(Update, camera::camera_follow_player_system)
        .add_systems(Update, player::player_pickup_system)
        .add_systems(Update, crafting::test_crafting_system)
        .add_plugins(debug::DebugPlugin)
        .run();
}
