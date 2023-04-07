use bevy::prelude::*;
use bevy_rapier2d::prelude::{
    Collider, Damping, ExternalForce, ExternalImpulse, LockedAxes, RigidBody, Velocity,
};
use leafwing_input_manager::{
    prelude::{ActionState, InputManagerPlugin, InputMap},
    Actionlike, InputManagerBundle,
};

use crate::{
    abilities::{cooldown::AbilityCooldownTime, Loadout, Power, SideEffect, UseAbilityEvent},
    health::{Health, MaxHealth},
    state::GameState,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .add_plugin(InputManagerPlugin::<PlayerActions>::default())
            .add_systems(
                (select_ability, use_ability, move_player)
                    .chain()
                    .in_set(OnUpdate(GameState::Playing)),
            )
            .add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)));
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum PlayerActions {
    Left,
    Right,
    Up,
    Down,
    Ability1,
    Ability2,
    Ability3,
    Ability4,
    UseAbility,
}

#[derive(Component, Clone, Default, Debug, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
pub struct Player;

fn spawn_player(mut commands: Commands) {
    let ability = commands
        .spawn((
            Power::Teleport,
            SideEffect::TakeDamage,
            AbilityCooldownTime(1.0),
        ))
        .id();
    let ability2 = commands
        .spawn((
            Power::Fireball,
            SideEffect::TakeDamage,
            AbilityCooldownTime(1.0),
        ))
        .id();
    let ability3 = commands
        .spawn((
            Power::IceWall,
            SideEffect::TakeDamage,
            AbilityCooldownTime(1.0),
        ))
        .id();
    let mut input_map = InputMap::new([
        (KeyCode::A, PlayerActions::Left),
        (KeyCode::S, PlayerActions::Down),
        (KeyCode::D, PlayerActions::Right),
        (KeyCode::W, PlayerActions::Up),
        (KeyCode::Key1, PlayerActions::Ability1),
        (KeyCode::Key2, PlayerActions::Ability2),
        (KeyCode::Key3, PlayerActions::Ability3),
        (KeyCode::Key4, PlayerActions::Ability4),
    ]);

    input_map.insert(MouseButton::Left, PlayerActions::UseAbility);

    commands.spawn((
        Player,
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(16.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..Default::default()
        },
        InputManagerBundle::<PlayerActions> {
            input_map,
            ..Default::default()
        },
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED_Z,
        Velocity::default(),
        Damping {
            linear_damping: 5.0,
            ..Default::default()
        },
        Loadout {
            abilities: vec![ability, ability2, ability3],
        },
        Collider::cuboid(8.0, 8.0),
        ExternalForce::default(),
        ExternalImpulse::default(),
        Health(100.0),
        MaxHealth(100.0),
        CurrentAbility(0),
    ));
}

fn move_player(
    mut players: Query<(&mut ExternalForce, &ActionState<PlayerActions>), With<Player>>,
) {
    const PLAYER_MOVE_FORCE: f32 = 64.0;
    for (mut force, action) in &mut players {
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

#[derive(Component, Clone, Default, Debug, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
pub struct CurrentAbility(pub usize);

fn select_ability(
    mut players: Query<(&mut CurrentAbility, &ActionState<PlayerActions>), With<Player>>,
) {
    for (mut current_ability, action) in &mut players {
        if action.just_pressed(PlayerActions::Ability1) {
            current_ability.0 = 0;
        } else if action.just_pressed(PlayerActions::Ability2) {
            current_ability.0 = 1;
        } else if action.just_pressed(PlayerActions::Ability3) {
            current_ability.0 = 2;
        } else if action.just_pressed(PlayerActions::Ability4) {
            current_ability.0 = 3;
        }
    }
}

fn use_ability(
    players: Query<
        (Entity, &CurrentAbility, &ActionState<PlayerActions>),
        (With<Player>, With<Loadout>),
    >,
    mut use_ability: EventWriter<UseAbilityEvent>,
) {
    for (player_entity, current_ability, action) in &players {
        if action.just_pressed(PlayerActions::UseAbility) {
            use_ability.send(UseAbilityEvent {
                loadout: player_entity,
                ability: current_ability.0,
            });
        }
    }
}
