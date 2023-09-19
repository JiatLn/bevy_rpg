use crate::{
    animation::{FrameTime, SpriteAnimation},
    graphics::Graphics,
    inventory::Inventory,
    world_object::Pickupable,
};
use bevy::prelude::*;
use bevy_inspector_egui::{prelude::ReflectInspectorOptions, InspectorOptions};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_palyer_system)
            .add_systems(Update, player_movement_system)
            .add_systems(Update, player_pickup_system);
    }
}

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
            transform: Transform::from_xyz(0.0, 0.0, 900.0),
            ..Default::default()
        },
        Player::new(),
        Inventory::new(),
        Name::new("Player"),
        SpriteAnimation {
            start_index: 0,
            len: 4,
            frame_time: 1.0 / 5.0,
        },
        FrameTime(0.0),
    ));
}

pub fn player_movement_system(
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Player, &mut SpriteAnimation)>,
    time: Res<Time>,
) {
    let (mut palyer_tf, player, mut animation) = player_query.single_mut();

    let delta = player.speed * time.delta_seconds();

    if keyboard.any_pressed([KeyCode::A, KeyCode::Left]) {
        palyer_tf.translation.x -= delta;
        animation.start_index = 4;
    } else if keyboard.any_pressed([KeyCode::D, KeyCode::Right]) {
        palyer_tf.translation.x += delta;
        animation.start_index = 8;
    } else if keyboard.any_pressed([KeyCode::S, KeyCode::Down]) {
        palyer_tf.translation.y -= delta;
        animation.start_index = 0;
    } else if keyboard.any_pressed([KeyCode::W, KeyCode::Up]) {
        palyer_tf.translation.y += delta;
        animation.start_index = 12;
    }
}

pub fn player_pickup_system(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    mut player_query: Query<(&Transform, &Player, &mut Inventory), With<Player>>,
    pick_query: Query<(Entity, &Transform, &Pickupable), With<Pickupable>>,
) {
    let (player_tf, player, mut inventory) = player_query.single_mut();

    if keyboard.just_pressed(KeyCode::Space) {
        let closest_item = pick_query
            .iter()
            .map(|(ent, pickupable_tf, pickupable)| {
                let distance = pickupable_tf
                    .translation
                    .truncate()
                    .distance(player_tf.translation.truncate());
                (ent, pickupable, distance)
            })
            .filter(|item| item.2 <= player.arm_len)
            .min_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

        match closest_item {
            Some((ent, pickupable, _)) => {
                if let Some(drops) = pickupable.drops {
                    commands.entity(ent).remove::<Pickupable>().insert(drops);
                } else {
                    commands.entity(ent).despawn_recursive();
                }
                inventory.add(pickupable.item, 1);
            }
            None => (),
        }
    }
}
