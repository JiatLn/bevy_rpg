use crate::resources::Graphics;
use bevy::{prelude::*, utils::HashMap};
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

#[derive(Component)]
pub struct Pickupable {
    pub item: Item,
}

#[derive(Component, Debug, InspectorOptions, Reflect)]
#[reflect(InspectorOptions)]
pub struct Inventory {
    pub items: HashMap<Item, usize>,
}

#[derive(
    Component, Debug, Default, Clone, Copy, Hash, PartialEq, Eq, InspectorOptions, Reflect,
)]
pub enum Item {
    #[default]
    None,
    Arrow,
    Axe,
}

impl Inventory {
    pub fn new() -> Self {
        Inventory {
            items: HashMap::default(),
        }
    }
    pub fn add(&mut self, item: Item) {
        *self.items.entry(item).or_insert(0) += 1;
    }
}

pub fn spawn_items_system(mut commands: Commands, graphics: Res<Graphics>) {
    spawn_item(
        &mut commands,
        Item::Arrow,
        &graphics,
        Some(Transform::from_xyz(40.0, 50.0, 0.0)),
    );
    spawn_item(
        &mut commands,
        Item::Arrow,
        &graphics,
        Some(Transform::from_xyz(-40.0, 50.0, 0.0)),
    );
    spawn_item(
        &mut commands,
        Item::Axe,
        &graphics,
        Some(Transform::from_xyz(120.0, -50.0, 0.0)),
    );
}

fn spawn_item(
    commands: &mut Commands,
    item: Item,
    graphics: &Graphics,
    transform: Option<Transform>,
) {
    let mut bundle = SpriteSheetBundle {
        texture_atlas: graphics.texture_altas.clone(),
        sprite: TextureAtlasSprite {
            index: *graphics
                .item_index_map
                .get(&item)
                .expect("item index not found"),
            custom_size: Some(Vec2::splat(32.0)),
            ..Default::default()
        },
        ..Default::default()
    };
    if let Some(tf) = transform {
        bundle.transform = tf;
    }
    commands.spawn((bundle, Pickupable { item }));
}
