use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;

use crate::graphics::Graphics;

pub struct WorldObjectPlugin;

impl Plugin for WorldObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_world_objects_system);
    }
}

impl WorldObject {
    fn spawn(
        self,
        commands: &mut Commands,
        graphics: &Graphics,
        custom_size: Option<Vec2>,
        position: Option<Vec2>,
    ) {
        let mut bundle = SpriteSheetBundle {
            texture_atlas: graphics.texture_altas.clone(),
            sprite: TextureAtlasSprite {
                index: *graphics
                    .item_index_map
                    .get(&self)
                    .expect("item index not found"),
                ..Default::default()
            },
            ..Default::default()
        };

        if let Some(pos) = position {
            bundle.transform = Transform::from_translation(pos.extend(0.0));
        }

        bundle.sprite.custom_size = custom_size.or(Some(Vec2::splat(32.0)));

        if let WorldObject::Item(item_type) = self {
            commands.spawn((bundle, Pickupable { item: item_type }));
        } else {
            commands.spawn(bundle);
        }
    }
}

#[derive(
    Component, Debug, Default, Clone, Copy, Hash, PartialEq, Eq, InspectorOptions, Reflect,
)]
pub enum WorldObject {
    #[default]
    None,
    Item(ItemType),
    Tree,
}

#[derive(Component)]
pub struct Pickupable {
    pub item: ItemType,
}

#[derive(
    Component, Debug, Default, Clone, Copy, Hash, PartialEq, Eq, InspectorOptions, Reflect,
)]
pub enum ItemType {
    #[default]
    None,
    Arrow,
    Axe,
    Twig,
}

pub fn spawn_world_objects_system(mut commands: Commands, graphics: Res<Graphics>) {
    WorldObject::Item(ItemType::Twig).spawn(
        &mut commands,
        &graphics,
        None,
        Some(Vec2::new(40.0, 50.0)),
    );
    WorldObject::Item(ItemType::Twig).spawn(
        &mut commands,
        &graphics,
        None,
        Some(Vec2::new(-40.0, 50.0)),
    );
    WorldObject::Item(ItemType::Arrow).spawn(
        &mut commands,
        &graphics,
        None,
        Some(Vec2::new(120.0, -50.0)),
    );
    WorldObject::Tree.spawn(
        &mut commands,
        &graphics,
        Some(Vec2::new(32.0, 64.0)),
        Some(Vec2::new(420.0, -50.0)),
    );
}
