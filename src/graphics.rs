use crate::{
    utils::index_to_rect,
    world_object::{ItemType, WorldObject},
};
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
    pub item_index_map: HashMap<WorldObject, (usize, Vec2)>,
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

    let image_handle = assets_server.load("texture.png");

    let mut altas = TextureAtlas::new_empty(image_handle, Vec2::splat(384.0));

    let stone_index = altas.add_texture(Rect {
        min: Vec2::new(32.0, 160.0),
        max: Vec2::new(64.0, 176.0),
    });
    let grass_index = altas.add_texture(Rect {
        min: Vec2::new(0.0, 96.0),
        max: Vec2::new(32.0, 128.0),
    });
    let tree_index = altas.add_texture(Rect {
        min: Vec2::new(0.0, 0.0),
        max: Vec2::new(64.0, 96.0),
    });
    let trunk_index = altas.add_texture(Rect {
        min: Vec2::new(64.0, 144.0),
        max: Vec2::new(80.0, 160.0),
    });

    let atlas_handle = texture_assets.add(altas);

    let mut item_index_map = HashMap::default();
    item_index_map.insert(
        WorldObject::Item(ItemType::Stone),
        (stone_index, Vec2::new(32.0, 16.0)),
    );
    item_index_map.insert(
        WorldObject::Item(ItemType::Grass),
        (grass_index, Vec2::splat(16.0)),
    );
    item_index_map.insert(WorldObject::Tree, (tree_index, Vec2::new(64.0, 96.0)));
    item_index_map.insert(WorldObject::Trunk, (trunk_index, Vec2::splat(16.0)));

    let graphics = Graphics {
        texture_altas: atlas_handle,
        player_texture_altas,
        player_index,
        item_index_map,
    };

    commands.insert_resource(graphics);
}
