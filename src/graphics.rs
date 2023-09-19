use crate::world_object::WorldObject;
use bevy::{prelude::*, utils::HashMap};
use serde::Deserialize;
use std::fs;

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
    // TODO: add more npc texture
    pub npc_texture_altas: Handle<TextureAtlas>,
    pub standard_texture_altas: Handle<TextureAtlas>,
    pub item_index_map: HashMap<WorldObject, (usize, Vec2)>,
}

#[derive(Debug, Deserialize)]
struct GraphicsDescription {
    map: HashMap<WorldObject, (Rect, Vec2)>,
}

impl GraphicsDescription {
    pub fn from_path(path: &str) -> Self {
        let desc_str = fs::read_to_string(path).unwrap();
        ron::de::from_str(&desc_str).unwrap()
    }
}

pub fn load_graphics(
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut texture_assets: ResMut<Assets<TextureAtlas>>,
) {
    let mut player_altas = TextureAtlas::from_grid(
        assets_server.load("player.png"),
        Vec2::splat(24.0),
        44,
        1,
        None,
        None,
    );
    player_altas.add_texture(Rect {
        min: Vec2::new(0.0, 0.0),
        max: Vec2::splat(24.0),
    });
    let player_texture_altas = texture_assets.add(player_altas);

    let mut npc_altas = TextureAtlas::from_grid(
        assets_server.load("npc.png"),
        Vec2::new(48.0, 64.0),
        8,
        1,
        None,
        None,
    );
    npc_altas.add_texture(Rect {
        min: Vec2::new(0.0, 0.0),
        max: Vec2::new(48.0, 64.0),
    });
    let npc_texture_altas = texture_assets.add(npc_altas);

    let mut standard_altas = TextureAtlas::from_grid(
        assets_server.load("standard.png"),
        Vec2::new(224.0, 288.0),
        39,
        1,
        None,
        None,
    );
    standard_altas.add_texture(Rect {
        min: Vec2::new(0.0, 0.0),
        max: Vec2::new(48.0, 64.0),
    });
    let standard_texture_altas = texture_assets.add(standard_altas);

    let mut texture_altas =
        TextureAtlas::new_empty(assets_server.load("texture.png"), Vec2::splat(384.0));

    let mut item_index_map = HashMap::default();

    let desc = GraphicsDescription::from_path("assets/graphics_desc.ron");

    for (&world_object, &(rect, size)) in desc.map.iter() {
        let index = texture_altas.add_texture(rect);
        item_index_map.insert(world_object, (index, size));
    }

    let atlas_handle = texture_assets.add(texture_altas);

    let graphics = Graphics {
        texture_altas: atlas_handle,
        player_texture_altas,
        npc_texture_altas,
        standard_texture_altas,
        item_index_map,
    };

    commands.insert_resource(graphics);
}
