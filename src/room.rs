use std::iter::repeat_with;

use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody};
use bevy_turborand::{DelegatedRng, GlobalRng};

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
            .add_systems((spawn_rooms, check_room_cleared).in_set(OnUpdate(GameState::Playing)));
    }
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

#[derive(Component, Clone, Default, Debug)]
struct Arena;

fn spawn_rooms(
    mut commands: Commands,
    mut room_spawn_events: EventReader<SpawnRoomEvent>,
    mut spawn_enemy_events: EventWriter<SpawnEnemyEvent>,
    mut rand: ResMut<GlobalRng>,
    arenas: Query<(), With<Arena>>,
) {
    for SpawnRoomEvent { room } in room_spawn_events.iter() {
        // Spawn static elements
        if arenas.iter().len() == 0 {
            commands.spawn((
                Arena,
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
        }

        const SPAWN_AREA_SIZE: f32 = 64.0;
        const SPAWN_AREA_OFFSET: Vec2 =
            Vec2::new(14.5 * 16.0 - SPAWN_AREA_SIZE, 8.0 * 16.0 - SPAWN_AREA_SIZE);

        let random_positions = repeat_with(|| (rand.f32_normalized(), rand.f32_normalized()))
            .map(|(x, y)| {
                (
                    if x < 0.0 {
                        x * SPAWN_AREA_SIZE - SPAWN_AREA_OFFSET.x
                    } else {
                        x * SPAWN_AREA_SIZE + SPAWN_AREA_OFFSET.x
                    },
                    if y < 0.0 {
                        y * SPAWN_AREA_SIZE - SPAWN_AREA_OFFSET.y
                    } else {
                        y * SPAWN_AREA_SIZE + SPAWN_AREA_OFFSET.y
                    },
                )
            })
            .map(|(x, y)| Vec2::new(x, y));

        for (&enemy, translation) in room.enemies.iter().zip(random_positions) {
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
    }
    *last_enemy_count = enemy_count;
}
