use bevy::{prelude::*, window::PrimaryWindow};

pub struct DragPlugin;

impl Plugin for DragPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (hoverable_system, draggable_system))
            .add_systems(PostUpdate, drop);
    }
}

#[derive(Component)]
pub struct Hoverable;

#[derive(Component)]
pub struct Hovered;

#[derive(Component)]
pub struct Dragged;

#[derive(Component)]
pub struct Draggable;

#[derive(Component)]
pub struct Dropped;

fn hoverable_system(
    mut commands: Commands,
    windows_query: Query<&Window, With<PrimaryWindow>>,
    hoverable_query: Query<(Entity, &GlobalTransform, &Node), (With<Hoverable>, Without<Dragged>)>,
) {
    // Games typically only have one window (the primary window)
    if let Some(position) = windows_query.single().cursor_position() {
        // println!("Cursor is inside the primary window, at {:?}", position);
        for (ent, transform, node) in hoverable_query.iter() {
            let hs = node.size();
            let half_width = hs.x / 2.0;
            let half_height = hs.y / 2.0;

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

fn drop(
    mut commands: Commands,
    mut q_dropped: Query<(Entity, &mut Transform, &GlobalTransform), Added<Dropped>>,
    windows_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Some(position) = windows_query.single().cursor_position() {
        for (entity, mut transform, global_transform) in q_dropped.iter_mut() {
            let global_pos = global_transform.translation();

            transform.translation.x = global_pos.x;
            transform.translation.y = global_pos.y;

            info!("Dropped position {:?}", position);
            commands.entity(entity).remove::<Dropped>();

            // TODO: spawn item at dropped position
        }
    }
}
