use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody};

use crate::{
    lifetime::Lifetime,
    mouse_position::MousePosition,
    player::{CurrentAbility, Player},
};

use super::{
    cooldown::AbilityCooldown, heat::Overheated, AbilitySet, Loadout, Power, UseAbilityEvent,
};

pub struct WallPowerPlugin;

impl Plugin for WallPowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<IceWall>()
            .add_systems((show_ghost, move_ghost).chain())
            .add_system(spawn_icewall.in_set(AbilitySet));
    }
}

#[derive(Component, Clone, Debug, Default, Reflect, FromReflect)]
#[reflect(Component, Debug)]
pub struct IceWall;

const MAX_ICEWALL_CAST_DISTANCE: f32 = 64.0;

fn spawn_icewall(
    mut commands: Commands,
    player: Query<&Transform, With<Player>>,
    loadouts: Query<&Loadout, Without<Overheated>>,
    mut ability_events: EventReader<UseAbilityEvent>,
    mouse_position: Res<MousePosition>,
    powers: Query<&Power, Without<AbilityCooldown>>,
) {
    for ability in ability_events.iter() {
        let Ok(loadout) = loadouts.get(ability.loadout) else {
            continue;
        };

        let ability_entity = loadout.abilities[ability.ability];
        let Ok(power) = powers.get(ability_entity) else { continue; };
        if *power != Power::IceWall {
            continue;
        }

        let player_position = player.single().translation.truncate();

        let delta =
            (**mouse_position - player_position).clamp_length_max(MAX_ICEWALL_CAST_DISTANCE);
        let rotation = delta.perp().angle_between(Vec2::X);

        let transform = Transform::from_translation((player_position + delta).extend(0.0))
            .with_rotation(Quat::from_rotation_z(-rotation));

        commands.spawn((
            IceWall,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::CYAN,
                    custom_size: Some(Vec2::new(64.0, 8.0)),
                    ..Default::default()
                },
                transform,
                ..Default::default()
            },
            Collider::cuboid(32.0, 4.0),
            RigidBody::Fixed,
            Lifetime::new(Duration::from_secs(2)),
        ));
    }
}

#[derive(Component, Clone, Debug, Default, Reflect, FromReflect)]
#[reflect(Component, Debug)]
pub struct IceWallGhost;

fn show_ghost(
    mut commands: Commands,
    player: Query<(&CurrentAbility, &Loadout), (With<Player>, Changed<CurrentAbility>)>,
    powers: Query<&Power, Without<AbilityCooldown>>,
    ghosts: Query<Entity, With<IceWallGhost>>,
) {
    let Ok((current_ability, loadout)) = player.get_single() else { return;};

    let Ok(power) = powers.get(loadout.abilities[current_ability.0]) else { return; };
    if *power == Power::IceWall {
        commands.spawn((
            IceWallGhost,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::CYAN.with_a(0.5),
                    custom_size: Some(Vec2::new(64.0, 8.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ));
    } else {
        for ghost_id in &ghosts {
            commands.entity(ghost_id).despawn_recursive();
        }
    }
}

fn move_ghost(
    player: Query<&Transform, With<Player>>,
    mouse_position: Res<MousePosition>,
    mut ghosts: Query<&mut Transform, (With<IceWallGhost>, Without<Player>)>,
) {
    for mut ghost_transform in &mut ghosts {
        let player_position = player.single().translation.truncate();

        let delta =
            (**mouse_position - player_position).clamp_length_max(MAX_ICEWALL_CAST_DISTANCE);
        let rotation = delta.perp().angle_between(Vec2::X);

        let transform = Transform::from_translation((player_position + delta).extend(0.0))
            .with_rotation(Quat::from_rotation_z(-rotation));

        *ghost_transform = transform;
    }
}
