use bevy::prelude::*;

use crate::{graphics::Graphics, world_object::WorldObject};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_inventory_box_system);
    }
}

#[derive(Component)]
pub struct InventoryLayout;

const BOX_GAP: f32 = 30.0;
const INVENTORY_NUM: usize = 8;

pub fn spawn_inventory_box_system(mut commands: Commands, graphics: Res<Graphics>) {
    let (index, size) = *graphics
        .item_index_map
        .get(&WorldObject::InventoryBox)
        .expect("inventory box index not found");

    let node_bundle = (
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::End,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(BOX_GAP),
                ..Default::default()
            },
            ..default()
        },
        InventoryLayout,
    );

    let inventory_boxes = (0..INVENTORY_NUM)
        .map(|_| AtlasImageBundle {
            texture_atlas: graphics.texture_altas.clone(),
            texture_atlas_image: UiTextureAtlasImage {
                index,
                ..Default::default()
            },
            style: Style {
                width: Val::Px(size.x),
                height: Val::Px(size.y),
                ..default()
            },
            ..Default::default()
        })
        .collect::<Vec<_>>();

    commands.spawn(node_bundle).with_children(|parent| {
        inventory_boxes.into_iter().for_each(|inventoy_box| {
            parent.spawn(inventoy_box);
        });
    });
}
