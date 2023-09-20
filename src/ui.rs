use crate::{graphics::Graphics, inventory::Inventory, world_object::WorldObject};
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_inventory_box_system)
            .add_systems(PostUpdate, update_inventory_box_system);
    }
}

#[derive(Component)]
pub struct InventoryBox(pub usize);

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
        .map(|i| {
            (
                AtlasImageBundle {
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
                },
                InventoryBox(i),
            )
        })
        .collect::<Vec<_>>();

    commands.spawn(node_bundle).with_children(|parent| {
        inventory_boxes.into_iter().for_each(|inventoy_box| {
            parent.spawn(inventoy_box);
        });
    });
}

pub fn update_inventory_box_system(
    mut commands: Commands,
    graphics: Res<Graphics>,
    inventory_query: Query<&Inventory, Changed<Inventory>>,
    inventory_boxes_query: Query<
        (Entity, &InventoryBox, &Style, Option<&Children>),
        With<InventoryBox>,
    >,
) {
    if let Ok(inventory) = inventory_query.get_single() {
        if inventory.items.is_empty() {
            return;
        }
        let inventory_vec = inventory.to_vec();

        // TODO: fix the inventory box order
        for (inventory_box_ent, inventory_box, style, children) in inventory_boxes_query.iter() {
            if let Some(children) = children {
                for &child_ent in children {
                    commands.entity(child_ent).despawn_recursive();
                }
            }

            if inventory_box.0 >= inventory_vec.len() {
                continue;
            }

            // TODO: add the inventory count
            let (item_type, _count) = inventory_vec[inventory_box.0];

            let (index, _size) = *graphics
                .item_index_map
                .get(&WorldObject::Item(item_type))
                .expect(&format!("inventory box [{:?}] index not found", item_type));

            let aib = AtlasImageBundle {
                texture_atlas: graphics.texture_altas.clone(),
                texture_atlas_image: UiTextureAtlasImage {
                    index,
                    ..Default::default()
                },
                style: Style {
                    width: style.width,
                    height: style.height,
                    ..default()
                },
                ..Default::default()
            };
            let ent = commands.spawn(aib).id();
            commands.entity(inventory_box_ent).add_child(ent);
        }
    }
}
