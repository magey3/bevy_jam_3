use bevy::prelude::*;
use bevy_turborand::{DelegatedRng, GlobalRng};

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
            enemies: vec![Enemy::Bomb, Enemy::Bomb],
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
    mut rng: ResMut<GlobalRng>,
) {
    for _ in room_cleared_events.iter() {
        current_room.0 += 1;

        let enemy_options = [(Enemy::Bomb, 1.0)];

        let mut room_difficulty = current_room.0 as f32 + 3.0 + rng.f32_normalized() * 2.0;

        let mut enemies = Vec::new();
        while room_difficulty > 0.0 {
            let option = rng.sample(&enemy_options).unwrap();
            enemies.push(option.0);
            room_difficulty -= option.1;
        }

        events.send(SpawnRoomEvent {
            room: Room { enemies },
        });
        info!("Switched to room {}", current_room.0);
    }
}
