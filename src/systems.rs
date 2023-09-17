use crate::{components::Pickupable, resources::Graphics, utils::index_to_rect};
use bevy::prelude::*;

pub fn load_graphics(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut texture_assets: ResMut<Assets<TextureAtlas>>,
) {
    let image_handle = assets_server.load("player.png");

    let mut altas = TextureAtlas::from_grid(image_handle, Vec2::splat(24.0), 7, 1, None, None);

    let player_index = altas.add_texture(index_to_rect(0, 0, 24.0));

    let player_texture_altas = texture_assets.add(altas);

    let image_handle = assets_server.load("tilemap_packed.png");

    let mut altas = TextureAtlas::from_grid(image_handle, Vec2::splat(16.0), 12, 11, None, None);

    let flint_index = altas.add_texture(index_to_rect(7, 9, 16.0));

    let atlas_handle = texture_assets.add(altas);

    let graphics = Graphics {
        texture_altas: atlas_handle,
        player_texture_altas,
        player_index,
        flint_index,
    };

    commands.insert_resource(graphics);
}

pub fn spawn_flint_system(mut commands: Commands, graphics: Res<Graphics>) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: graphics.texture_altas.clone(),
            sprite: TextureAtlasSprite {
                index: graphics.flint_index,
                custom_size: Some(Vec2::splat(32.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        Pickupable,
    ));
}
