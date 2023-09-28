use crate::graphics::Graphics;
use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use serde::Deserialize;

pub struct WorldObjectPlugin;

impl Plugin for WorldObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_world_objects_system)
            .add_systems(Update, update_world_objects_graphics_system)
            .add_systems(Update, regrowth_system);
    }
}

#[derive(
    Deserialize,
    Component,
    Debug,
    Default,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    InspectorOptions,
    Reflect,
)]
pub enum WorldObject {
    #[default]
    None,
    Item(ItemType),
    Tree,
    Trunk,
    GrassWithFlower,
    Grass,
    InventoryBox,
}

#[derive(Component, InspectorOptions, Reflect)]
pub struct Pickupable {
    pub item: ItemType,
    pub drops: Option<WorldObject>,
}

#[derive(
    Deserialize,
    Component,
    Debug,
    Default,
    Clone,
    Copy,
    Hash,
    PartialEq,
    Eq,
    InspectorOptions,
    Reflect,
)]
pub enum ItemType {
    #[default]
    None,
    Stone,
    Stones,
    Flower,
    Axe,
    Wood,
    Fire,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct ReGrowthTimer(pub Timer);

impl From<WorldObject> for String {
    fn from(val: WorldObject) -> Self {
        match val {
            WorldObject::None => "None".to_string(),
            WorldObject::Item(item_type) => format!("Item {item_type:?}"),
            WorldObject::Tree => "Tree".to_string(),
            WorldObject::Trunk => "Sapling".to_string(),
            WorldObject::GrassWithFlower => "grass with flower".to_string(),
            WorldObject::Grass => "grass without flower".to_string(),
            WorldObject::InventoryBox => "inventory box".to_string(),
        }
    }
}

impl ItemType {
    pub fn is_draggable(&self) -> bool {
        matches!(self, ItemType::Fire)
    }
}

impl WorldObject {
    pub fn spawn(
        self,
        commands: &mut Commands,
        graphics: &Graphics,
        custom_size: Option<Vec2>,
        position: Option<Vec2>,
    ) -> Entity {
        let (index, size) = *graphics
            .item_index_map
            .get(&self)
            .unwrap_or_else(|| panic!("world object index not found: {:?}", self));
        let mut sprite_sheet = SpriteSheetBundle {
            texture_atlas: graphics.texture_altas.clone(),
            sprite: TextureAtlasSprite {
                index,
                ..Default::default()
            },
            ..Default::default()
        };

        sprite_sheet.transform =
            Transform::from_translation(position.unwrap_or(Vec2::ZERO).extend(0.0));
        sprite_sheet.sprite.custom_size = custom_size.or(Some(size));

        let mut ent = commands.spawn((sprite_sheet, Name::new(String::from(self))));

        if let Some(pickable) = self.pickupable_into() {
            ent.insert(pickable);
        }
        ent.id()
    }
    pub fn growth_into(&self) -> Option<(WorldObject, f32)> {
        match self {
            WorldObject::Trunk => Some((WorldObject::Tree, 4.0)),
            WorldObject::Grass => Some((WorldObject::GrassWithFlower, 2.0)),
            _ => None,
        }
    }
    pub fn pickupable_into(&self) -> Option<Pickupable> {
        match self {
            WorldObject::Item(item_type) => Some(Pickupable {
                item: *item_type,
                drops: None,
            }),
            WorldObject::Tree => Some(Pickupable {
                item: ItemType::Wood,
                drops: Some(WorldObject::Trunk),
            }),
            WorldObject::GrassWithFlower => Some(Pickupable {
                item: ItemType::Flower,
                drops: Some(WorldObject::Grass),
            }),
            _ => None,
        }
    }
}

pub fn regrowth_system(
    mut commands: Commands,
    mut world_obj_query: Query<(Entity, &mut WorldObject, Option<&mut ReGrowthTimer>)>,
    time: Res<Time>,
) {
    for (ent, mut world_obj, growth_timer) in world_obj_query.iter_mut() {
        if let Some((growth_into, growth_time)) = world_obj.growth_into() {
            match growth_timer {
                Some(mut timer) => {
                    timer.0.tick(time.delta());
                    if timer.0.finished() {
                        commands.entity(ent).remove::<ReGrowthTimer>();
                        if let Some(pickupable) = growth_into.pickupable_into() {
                            commands.entity(ent).insert(pickupable);
                        }
                        *world_obj = growth_into;
                    }
                }
                None => {
                    commands
                        .entity(ent)
                        .insert(ReGrowthTimer(Timer::from_seconds(
                            growth_time,
                            TimerMode::Once,
                        )));
                }
            }
        }
    }
}

pub fn spawn_world_objects_system(mut commands: Commands, graphics: Res<Graphics>) {
    let world_objects = vec![
        WorldObject::Item(ItemType::Stone).spawn(
            &mut commands,
            &graphics,
            None,
            Some(Vec2::new(40.0, 50.0)),
        ),
        WorldObject::Item(ItemType::Stone).spawn(
            &mut commands,
            &graphics,
            None,
            Some(Vec2::new(-40.0, 30.0)),
        ),
        WorldObject::Item(ItemType::Stone).spawn(
            &mut commands,
            &graphics,
            None,
            Some(Vec2::new(120.0, -50.0)),
        ),
        WorldObject::Trunk.spawn(
            &mut commands,
            &graphics,
            None,
            Some(Vec2::new(120.0, -90.0)),
        ),
        WorldObject::Tree.spawn(
            &mut commands,
            &graphics,
            Some(Vec2::new(64.0, 96.0)),
            Some(Vec2::new(420.0, -50.0)),
        ),
        WorldObject::Tree.spawn(
            &mut commands,
            &graphics,
            Some(Vec2::new(64.0, 96.0)),
            Some(Vec2::new(280.0, -60.0)),
        ),
        WorldObject::GrassWithFlower.spawn(
            &mut commands,
            &graphics,
            None,
            Some(Vec2::new(180.0, -60.0)),
        ),
    ];

    commands
        .spawn(SpatialBundle::default())
        .push_children(&world_objects);
}

pub fn update_world_objects_graphics_system(
    mut world_obj_query: Query<
        (&WorldObject, &mut TextureAtlasSprite, &mut Transform),
        Changed<WorldObject>,
    >,
    graphics: Res<Graphics>,
) {
    for (world_object, mut sprite, mut transform) in world_obj_query.iter_mut() {
        let (index, size) = *graphics
            .item_index_map
            .get(world_object)
            .unwrap_or_else(|| panic!("world object index not found: {:?}", world_object));
        sprite.index = index;
        if let Some(old_size) = sprite.custom_size {
            transform.translation.y -= (old_size.y - size.y) / 2.0;
        }
        sprite.custom_size = Some(size);
    }
}
