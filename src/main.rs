use bevy::prelude::*;

mod animation;
mod camera;
mod crafting;
mod debug;
mod graphics;
mod inventory;
mod npc;
mod player;
mod ui;
mod world_object;

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
        .add_plugins(graphics::GraphicsPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(npc::NpcPlugin)
        .add_plugins(world_object::WorldObjectPlugin)
        .add_plugins(crafting::CraftingPlugin)
        .add_plugins(animation::AnimationPlugin)
        .add_plugins(ui::UiPlugin)
        .add_plugins(debug::DebugPlugin)
        .run();
}
