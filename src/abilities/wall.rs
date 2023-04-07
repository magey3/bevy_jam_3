use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::{Collider, RigidBody};

use crate::{lifetime::Lifetime, mouse_position::MousePosition, player::Player};

use super::{cooldown::AbilityCooldown, AbilitySet, Loadout, Power, UseAbilityEvent};

pub struct WallPowerPlugin;

impl Plugin for WallPowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<IceWall>()
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
    loadouts: Query<&Loadout>,
    mut ability_events: EventReader<UseAbilityEvent>,
    mouse_position: Res<MousePosition>,
    powers: Query<&Power, Without<AbilityCooldown>>,
) {
    for ability in ability_events.iter() {
        let Ok(loadout) = loadouts.get(ability.loadout) else {
            warn!("Invalid loadout in ability event");
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
