use bevy::{prelude::*, utils::HashMap};

use crate::inventory::Item;

#[derive(Resource)]
pub struct Graphics {
    pub texture_altas: Handle<TextureAtlas>,
    pub player_texture_altas: Handle<TextureAtlas>,
    pub player_index: usize,
    pub item_index_map: HashMap<Item, usize>,
}
