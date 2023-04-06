use bevy::prelude::*;

use crate::{
    enemy::Enemy,
    room::{Room, RoomClearedEvent, SpawnRoomEvent},
    state::GameState,
};

pub struct RoomManagerPlugin;

impl Plugin for RoomManagerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CurrentRoom>()
            .init_resource::<CurrentRoom>()
            .add_system(init.in_schedule(OnEnter(GameState::Playing)))
            .add_system(room_loop);
    }
}

fn init(mut events: EventWriter<SpawnRoomEvent>) {
    events.send(SpawnRoomEvent {
        room: Room {
            enemies: vec![Enemy::Circle, Enemy::Circle],
        },
    });
}

#[derive(Resource, Clone, Default, Debug, Reflect, FromReflect)]
#[reflect(Resource, Debug)]
pub struct CurrentRoom(u32);

fn room_loop(
    mut events: EventWriter<SpawnRoomEvent>,
    mut room_cleared_events: EventReader<RoomClearedEvent>,
    mut current_room: ResMut<CurrentRoom>,
) {
    for _ in room_cleared_events.iter() {
        current_room.0 += 1;
        events.send(SpawnRoomEvent {
            room: Room {
                enemies: vec![Enemy::Circle, Enemy::Circle],
            },
        });
        info!("Switched to room {}", current_room.0);
    }
}
