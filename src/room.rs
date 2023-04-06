use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody};

use crate::{
    enemy::{Enemy, SpawnEnemyEvent},
    state::GameState,
};

pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Room>()
            .register_type::<SpawnRoomEvent>()
            .register_type::<RoomClearedEvent>()
            .add_event::<SpawnRoomEvent>()
            .add_event::<RoomClearedEvent>()
            .add_systems((spawn_rooms, check_room_cleared).in_set(OnUpdate(GameState::Playing)))
            .add_system(test.in_schedule(OnEnter(GameState::Playing)));
    }
}

fn test(mut events: EventWriter<SpawnRoomEvent>) {
    events.send(SpawnRoomEvent {
        room: Room {
            enemies: vec![Enemy::Circle, Enemy::Circle],
        },
    });
}

#[derive(Clone, Default, Debug, Reflect, FromReflect)]
pub struct Room {
    pub enemies: Vec<Enemy>,
}

#[derive(Clone, Default, Debug, Reflect, FromReflect)]
pub struct SpawnRoomEvent {
    pub room: Room,
}

const FLOOR_COLOR: Color = Color::DARK_GRAY;

fn spawn_rooms(
    mut commands: Commands,
    mut room_spawn_events: EventReader<SpawnRoomEvent>,
    mut spawn_enemy_events: EventWriter<SpawnEnemyEvent>,
) {
    for SpawnRoomEvent { room } in room_spawn_events.iter() {
        // Spawn static elements
        commands.spawn((
            RigidBody::Fixed,
            Collider::compound(vec![
                (
                    Vec2::new(0.0, 8.0 * 16.0),
                    0.0,
                    Collider::cuboid(29.0 * 8.0, 1.0),
                ),
                (
                    Vec2::new(0.0, -8.0 * 16.0),
                    0.0,
                    Collider::cuboid(29.0 * 8.0, 1.0),
                ),
                (
                    Vec2::new(14.5 * 16.0, 0.0),
                    0.0,
                    Collider::cuboid(1.0, 16.0 * 8.0),
                ),
                (
                    Vec2::new(-14.5 * 16.0, 0.0),
                    0.0,
                    Collider::cuboid(1.0, 16.0 * 8.0),
                ),
            ]),
            SpriteBundle {
                sprite: Sprite {
                    color: FLOOR_COLOR,
                    custom_size: Some(Vec2::new(29.0 * 16.0, 16.0 * 16.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ));

        // Spawn enemies
        let positions = [
            Vec2::new(-128.0, 128.0),
            Vec2::new(128.0, 128.0),
            Vec2::new(-128.0, -128.0),
            Vec2::new(128.0, -128.0),
        ];
        for (&enemy, &translation) in room.enemies.iter().zip(&positions) {
            spawn_enemy_events.send(SpawnEnemyEvent { enemy, translation })
        }
    }
}

#[derive(Clone, Default, Debug, Reflect, FromReflect)]
pub struct RoomClearedEvent;

fn check_room_cleared(
    mut last_enemy_count: Local<usize>,
    enemies: Query<(), With<Enemy>>,
    mut room_clear_events: EventWriter<RoomClearedEvent>,
) {
    let enemy_count = enemies.iter().len();
    if enemy_count == 0 && enemy_count != *last_enemy_count {
        room_clear_events.send_default();
        info!("YOU WIN");
    }
    *last_enemy_count = enemy_count;
}
