use crate::graphics::Graphics;
use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;

pub struct WorldObjectPlugin;

impl Plugin for WorldObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_world_objects_system)
            .add_systems(Update, update_world_objects_graphics_system)
            .add_systems(Update, regrowth_system);
    }
}

impl From<WorldObject> for String {
    fn from(val: WorldObject) -> Self {
        match val {
            WorldObject::None => "None".to_string(),
            WorldObject::Item(item_type) => format!("Item {item_type:?}"),
            WorldObject::Tree => "Tree".to_string(),
            WorldObject::Trunk => "Sapling".to_string(),
        }
    }
}

impl WorldObject {
    fn spawn(
        self,
        commands: &mut Commands,
        graphics: &Graphics,
        custom_size: Option<Vec2>,
        position: Option<Vec2>,
        pickup_item: Option<ItemType>,
        drops: Option<WorldObject>,
    ) {
        let (index, size) = *graphics
            .item_index_map
            .get(&self)
            .expect(&format!("world object index not found: {:?}", self));
        let mut sprite_sheet = SpriteSheetBundle {
            texture_atlas: graphics.texture_altas.clone(),
            sprite: TextureAtlasSprite {
                index,
                ..Default::default()
            },
            ..Default::default()
        };

        if let Some(pos) = position {
            sprite_sheet.transform = Transform::from_translation(pos.extend(0.0));
        }

        sprite_sheet.sprite.custom_size = custom_size.or(Some(size));

        if let Some(item_type) = pickup_item {
            commands.spawn((
                sprite_sheet,
                Pickupable::new(item_type, drops),
                Name::new(String::from(self)),
            ));
        } else {
            commands.spawn((sprite_sheet, Name::new(String::from(self))));
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
    Trunk,
}

#[derive(Component, InspectorOptions, Reflect)]
pub struct Pickupable {
    pub item: ItemType,
    pub drops: Option<WorldObject>,
}

impl Pickupable {
    pub fn new(item: ItemType, drops: Option<WorldObject>) -> Self {
        Pickupable { item, drops }
    }
}

#[derive(
    Component, Debug, Default, Clone, Copy, Hash, PartialEq, Eq, InspectorOptions, Reflect,
)]
pub enum ItemType {
    #[default]
    None,
    Stone,
    Grass,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct ReGrowthTimer {
    timer: Timer,
}

pub fn regrowth_system(
    mut commands: Commands,
    mut world_obj_query: Query<(Entity, &mut WorldObject, Option<&mut ReGrowthTimer>)>,
    time: Res<Time>,
) {
    for (ent, mut world_obj, timer) in world_obj_query.iter_mut() {
        if *world_obj == WorldObject::Trunk {
            match timer {
                Some(mut timer) => {
                    timer.timer.tick(time.delta());
                    if timer.timer.finished() {
                        commands.entity(ent).remove::<ReGrowthTimer>();
                        *world_obj = WorldObject::Tree;
                        commands.entity(ent).insert(Pickupable {
                            drops: Some(WorldObject::Trunk),
                            item: ItemType::Grass,
                        });
                    }
                }
                None => {
                    commands.entity(ent).insert(ReGrowthTimer {
                        timer: Timer::from_seconds(2.0, TimerMode::Once),
                    });
                }
            }
        }
    }
}

pub fn spawn_world_objects_system(mut commands: Commands, graphics: Res<Graphics>) {
    WorldObject::Item(ItemType::Grass).spawn(
        &mut commands,
        &graphics,
        None,
        Some(Vec2::new(40.0, 50.0)),
        Some(ItemType::Grass),
        None,
    );
    WorldObject::Item(ItemType::Grass).spawn(
        &mut commands,
        &graphics,
        None,
        Some(Vec2::new(-40.0, 50.0)),
        Some(ItemType::Grass),
        None,
    );
    WorldObject::Item(ItemType::Stone).spawn(
        &mut commands,
        &graphics,
        None,
        Some(Vec2::new(120.0, -50.0)),
        Some(ItemType::Stone),
        None,
    );
    WorldObject::Trunk.spawn(
        &mut commands,
        &graphics,
        None,
        Some(Vec2::new(120.0, -90.0)),
        None,
        None,
    );
    WorldObject::Tree.spawn(
        &mut commands,
        &graphics,
        Some(Vec2::new(64.0, 96.0)),
        Some(Vec2::new(420.0, -50.0)),
        Some(ItemType::Grass),
        Some(WorldObject::Trunk),
    );
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
            .expect(&format!("world object index not found: {:?}", world_object));
        sprite.index = index;
        if let Some(old_size) = sprite.custom_size {
            transform.translation.x -= (old_size.x - size.x) / 2.0;
            transform.translation.y -= (old_size.y - size.y) / 2.0;
        }
        sprite.custom_size = Some(size);
    }
}
