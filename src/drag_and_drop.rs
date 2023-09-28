use crate::{
    camera::MainCamera,
    graphics::Graphics,
    inventory::Inventory,
    player::Player,
    world_object::{ItemType, WorldObject},
};
use bevy::{prelude::*, window::PrimaryWindow};

pub struct DragPlugin;

impl Plugin for DragPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (hoverable_system, draggable_system))
            .add_systems(PostUpdate, drop_system);
    }
}

#[derive(Component)]
pub struct Hoverable;

#[derive(Component)]
pub struct Hovered;

#[derive(Component)]
pub struct Dragged;

#[derive(Component)]
pub struct Draggable {
    pub item_type: ItemType,
}

#[derive(Component)]
pub struct Dropped;

fn hoverable_system(
    mut commands: Commands,
    windows_query: Query<&Window, With<PrimaryWindow>>,
    hoverable_query: Query<(Entity, &GlobalTransform, &Node), (With<Hoverable>, Without<Dragged>)>,
) {
    if let Some(position) = windows_query.single().cursor_position() {
        for (ent, transform, node) in hoverable_query.iter() {
            let node_size = node.size();
            let half_width = node_size.x / 2.0;
            let half_height = node_size.y / 2.0;

            if transform.translation().x - half_width < position.x
                && transform.translation().x + half_width > position.x
                && transform.translation().y - half_height < position.y
                && transform.translation().y + half_height > position.y
            {
                commands.entity(ent).insert(Hovered);
            } else {
                commands.entity(ent).remove::<Hovered>();
            }
        }
    }
}

fn draggable_system(
    mut commands: Commands,
    i_mouse_button: Res<Input<MouseButton>>,
    q_pressed: Query<Entity, (With<Hovered>, With<Draggable>)>,
    q_released: Query<Entity, With<Dragged>>,
) {
    if i_mouse_button.just_pressed(MouseButton::Left) {
        if let Some(entity) = q_pressed.iter().next() {
            commands.entity(entity).insert(Dragged);
        }
    } else if i_mouse_button.just_released(MouseButton::Left) {
        for entity in q_released.iter() {
            commands.entity(entity).remove::<Dragged>();
            commands.entity(entity).insert(Dropped);
        }
    }
}

fn drop_system(
    mut commands: Commands,
    dropped_query: Query<(Entity, &Draggable), Added<Dropped>>,
    windows_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut player_query: Query<&mut Inventory, With<Player>>,
    graphics: Res<Graphics>,
) {
    if let Some(position) = windows_query.single().cursor_position() {
        let (camera, camera_transform) = camera_query.single();
        let mut inventory = player_query.single_mut();

        for (entity, draggable) in dropped_query.iter() {
            commands.entity(entity).remove::<Dropped>();

            let position = camera
                .viewport_to_world(camera_transform, position)
                .map(|ray| ray.origin.truncate())
                .unwrap();

            WorldObject::Item(draggable.item_type).spawn(
                &mut commands,
                &graphics,
                None,
                Some(position),
            );
            inventory.cost(draggable.item_type, 1);
        }
    }
}
