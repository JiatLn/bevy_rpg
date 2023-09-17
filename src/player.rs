use crate::{
    inventory::{Inventory, Pickupable},
    resources::Graphics,
};
use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

#[derive(Component, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Player {
    pub speed: f32,
    pub arm_len: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            speed: 80.0,
            arm_len: 50.0,
        }
    }
}

pub fn spawn_palyer_system(mut commands: Commands, graphics: Res<Graphics>) {
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: graphics.player_texture_altas.clone(),
            sprite: TextureAtlasSprite {
                index: graphics.player_index,
                custom_size: Some(Vec2::splat(48.0)),
                ..Default::default()
            },
            // TODO: FIX player z-index
            transform: Transform::from_xyz(0.0, 0.0, 900.0),
            ..Default::default()
        },
        Player::new(),
        Inventory::new(),
    ));
}

pub fn player_movement_system(
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Player, &mut TextureAtlasSprite)>,
    time: Res<Time>,
) {
    let (mut palyer_tf, player, mut sprite) = player_query.single_mut();

    let delta = player.speed * time.delta_seconds();

    if keyboard.any_pressed([KeyCode::A, KeyCode::Left]) {
        palyer_tf.translation.x -= delta;
        sprite.flip_x = true;
    }
    if keyboard.any_pressed([KeyCode::D, KeyCode::Right]) {
        palyer_tf.translation.x += delta;
        sprite.flip_x = false;
    }
    if keyboard.any_pressed([KeyCode::S, KeyCode::Down]) {
        palyer_tf.translation.y -= delta;
    }
    if keyboard.any_pressed([KeyCode::W, KeyCode::Up]) {
        palyer_tf.translation.y += delta;
    }
}

pub fn player_pickup_system(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<(&Transform, &Player, &mut Inventory), With<Player>>,
    mut pick_query: Query<(Entity, &Transform, &Pickupable), (With<Pickupable>, Without<Player>)>,
) {
    let (player_tf, player, mut inventory) = player_query.single_mut();

    if keyboard.pressed(KeyCode::Space) {
        for (ent, pickupable_tf, pickupable) in pick_query.iter_mut() {
            let distance = pickupable_tf
                .translation
                .truncate()
                .distance(player_tf.translation.truncate());
            if distance <= player.arm_len {
                commands.entity(ent).despawn_recursive();
                inventory.add(pickupable.item);
                dbg!(&inventory.items);
            }
        }
    }
}
