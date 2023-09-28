use crate::{
    crafting::CraftingBook,
    drag_and_drop::{Draggable, Hoverable},
    graphics::Graphics,
    inventory::Inventory,
    world_object::WorldObject,
};
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (spawn_inventory_box_system, spawn_crafting_books_system),
        )
        .add_systems(
            PostUpdate,
            (
                update_inventory_box_system,
                update_crafting_book_button_status_system,
            ),
        );
    }
}

#[derive(Component)]
pub struct InventoryBox(pub usize);

#[derive(Component)]
pub struct CraftingButton(pub usize);

const INVENTORY_NUM: usize = 8;

pub fn spawn_inventory_box_system(mut commands: Commands, graphics: Res<Graphics>) {
    let (index, size) = *graphics
        .item_index_map
        .get(&WorldObject::InventoryBox)
        .expect("inventory box index not found");

    let node_bundle = (
        NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::Center,
                column_gap: Val::Px(-2.0),
                ..Default::default()
            },
            ..default()
        },
        Name::new("Inventory Box"),
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
                        display: Display::Flex,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
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

pub fn spawn_crafting_books_system(
    mut commands: Commands,
    graphics: Res<Graphics>,
    crafting_book: Res<CraftingBook>,
) {
    let (index, size) = *graphics
        .item_index_map
        .get(&WorldObject::InventoryBox)
        .expect("inventory box index not found");

    let node_bundle = (
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexEnd,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(-2.0),
                ..Default::default()
            },
            ..default()
        },
        Name::new("Crafting Book"),
    );

    let crafting_books = crafting_book
        .craftable
        .iter()
        .enumerate()
        .map(|(recipe_index, recipe)| {
            commands
                .spawn(AtlasImageBundle {
                    texture_atlas: graphics.texture_altas.clone(),
                    texture_atlas_image: UiTextureAtlasImage {
                        index,
                        ..Default::default()
                    },
                    style: Style {
                        display: Display::Flex,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Px(size.x),
                        height: Val::Px(size.y),
                        ..default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    let (index, _size) = *graphics
                        .item_index_map
                        .get(&WorldObject::Item(recipe.preducts))
                        .unwrap_or_else(|| {
                            panic!("graphics [{:?}] index not found", &recipe.preducts)
                        });

                    parent
                        .spawn(AtlasImageBundle {
                            texture_atlas: graphics.texture_altas.clone(),
                            texture_atlas_image: UiTextureAtlasImage {
                                index,
                                ..Default::default()
                            },
                            style: Style {
                                width: Val::Percent(90.0),
                                height: Val::Percent(90.0),
                                ..default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn((
                                ButtonBundle {
                                    style: Style {
                                        width: Val::Percent(100.0),
                                        height: Val::Percent(100.0),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    ..default()
                                },
                                CraftingButton(recipe_index),
                            ));
                        });
                })
                .id()
        })
        .collect::<Vec<_>>();

    commands.spawn(node_bundle).push_children(&crafting_books);
}

pub fn update_inventory_box_system(
    mut commands: Commands,
    graphics: Res<Graphics>,
    inventory_query: Query<&Inventory, Changed<Inventory>>,
    inventory_boxes_query: Query<(Entity, &InventoryBox, Option<&Children>), With<InventoryBox>>,
) {
    if let Ok(inventory) = inventory_query.get_single() {
        let inventory_vec = inventory.to_vec();

        // TODO: fix the inventory box order
        for (inventory_box_ent, inventory_box, children) in inventory_boxes_query.iter() {
            if let Some(children) = children {
                for &child_ent in children {
                    commands.entity(child_ent).despawn_recursive();
                }
            }

            if inventory_box.0 >= inventory_vec.len() {
                continue;
            }

            let (item_type, count) = inventory_vec[inventory_box.0];

            let (index, _size) = *graphics
                .item_index_map
                .get(&WorldObject::Item(item_type))
                .unwrap_or_else(|| panic!("inventory box [{:?}] index not found", item_type));

            let aib = AtlasImageBundle {
                texture_atlas: graphics.texture_altas.clone(),
                texture_atlas_image: UiTextureAtlasImage {
                    index,
                    ..Default::default()
                },
                style: Style {
                    display: Display::Flex,
                    align_items: AlignItems::FlexEnd,
                    justify_content: JustifyContent::FlexEnd,
                    width: Val::Percent(90.0),
                    height: Val::Percent(90.0),
                    ..default()
                },
                ..Default::default()
            };
            let mut ent = commands.spawn(aib);

            if item_type.is_draggable() {
                ent.insert((Hoverable, Draggable { item_type }));
            }

            let ent = ent
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            alignment: TextAlignment::Right,
                            sections: vec![TextSection {
                                value: count.to_string(),
                                ..Default::default()
                            }],
                            ..Default::default()
                        },
                        style: Style {
                            margin: UiRect::px(0.0, 3.0, 0.0, 3.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                })
                .id();

            commands.entity(inventory_box_ent).add_child(ent);
        }
    }
}

pub fn update_crafting_book_button_status_system(
    inventory_query: Query<&Inventory, Changed<Inventory>>,
    mut crafting_button_query: Query<(&CraftingButton, &mut BackgroundColor), With<CraftingButton>>,
    crafting_book: Res<CraftingBook>,
) {
    for (crafting_btn, mut bgc) in crafting_button_query.iter_mut() {
        let crafting_index = crafting_btn.0;
        let crafting_recipe = &crafting_book.craftable[crafting_index];
        if let Ok(inventory) = inventory_query.get_single() {
            if crafting_recipe.can_craft(inventory) {
                *bgc = BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.0));
            } else {
                *bgc = BackgroundColor(Color::rgba_u8(0, 0, 0, 200));
            }
        }
    }
}
