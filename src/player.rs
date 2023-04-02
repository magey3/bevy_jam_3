use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, Damping, ExternalForce, LockedAxes, RigidBody, Velocity};
use leafwing_input_manager::{
    prelude::{ActionState, InputManagerPlugin, InputMap},
    Actionlike, InputManagerBundle,
};

use crate::state::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .add_plugin(InputManagerPlugin::<PlayerActions>::default())
            .add_system(move_player)
            .add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)));
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum PlayerActions {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component, Clone, Default, Debug, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
pub struct Player;

fn spawn_player(mut commands: Commands) {
    info!("SPAWN");
    commands.spawn((
        Player,
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(16.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        InputManagerBundle::<PlayerActions> {
            input_map: InputMap::new([
                (KeyCode::A, PlayerActions::Left),
                (KeyCode::S, PlayerActions::Down),
                (KeyCode::D, PlayerActions::Right),
                (KeyCode::W, PlayerActions::Up),
            ]),
            ..Default::default()
        },
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED_Z,
        Velocity::default(),
        Damping {
            linear_damping: 5.0,
            ..Default::default()
        },
        Collider::cuboid(8.0, 8.0),
        ExternalForce::default(),
    ));
}

fn move_player(
    mut players: Query<
        (
            &mut Damping,
            &mut ExternalForce,
            &ActionState<PlayerActions>,
        ),
        With<Player>,
    >,
) {
    const PLAYER_MOVE_FORCE: f32 = 64.0;
    const PLAYER_DAMPING_FORCE: f32 = 5.0;
    for (mut damping, mut force, action) in &mut players {
        let mut new_force = Vec2::ZERO;
        if action.pressed(PlayerActions::Left) {
            new_force.x = -1.0;
        } else if action.pressed(PlayerActions::Right) {
            new_force.x = 1.0;
        }
        if action.pressed(PlayerActions::Down) {
            new_force.y = -1.0;
        } else if action.pressed(PlayerActions::Up) {
            new_force.y = 1.0;
        }

        force.force = new_force.normalize_or_zero() * PLAYER_MOVE_FORCE;
    }
}
