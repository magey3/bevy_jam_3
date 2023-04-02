use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody};

use crate::state::GameState;

pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnRoomEvent>()
            .add_system(spawn_rooms.run_if(in_state(GameState::Playing)))
            .add_system(test.in_schedule(OnEnter(GameState::Playing)));
    }
}

fn test(mut events: EventWriter<SpawnRoomEvent>) {
    events.send(SpawnRoomEvent { room: Room {} });
}

pub struct Room {}

pub struct SpawnRoomEvent {
    pub room: Room,
}

const FLOOR_COLOR: Color = Color::DARK_GRAY;

fn spawn_rooms(mut commands: Commands, mut room_spawn_events: EventReader<SpawnRoomEvent>) {
    for _ in room_spawn_events.iter() {
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
    }
}
