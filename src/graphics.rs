use crate::{inventory::ItemType, utils::index_to_rect};
use bevy::{prelude::*, utils::HashMap};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_graphics);
    }
}

#[derive(Resource)]
pub struct Graphics {
    pub texture_altas: Handle<TextureAtlas>,
    pub player_texture_altas: Handle<TextureAtlas>,
    pub player_index: usize,
    pub item_index_map: HashMap<ItemType, usize>,
}

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

    let arrow_index = altas.add_texture(index_to_rect(9, 11, 16.0));
    let axe_index = altas.add_texture(index_to_rect(10, 7, 16.0));
    let twig_index = altas.add_texture(index_to_rect(1, 5, 16.0));

    let atlas_handle = texture_assets.add(altas);

    let mut item_index_map = HashMap::default();
    item_index_map.insert(ItemType::Arrow, arrow_index);
    item_index_map.insert(ItemType::Axe, axe_index);
    item_index_map.insert(ItemType::Twig, twig_index);

    let graphics = Graphics {
        texture_altas: atlas_handle,
        player_texture_altas,
        player_index,
        item_index_map,
    };

    commands.insert_resource(graphics);
}
