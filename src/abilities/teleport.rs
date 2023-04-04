use std::time::Duration;

use bevy::prelude::*;

use crate::{mouse_position::MousePosition, player::Player};

use super::{AbilitySet, Loadout, Power, UseAbilityEvent};

pub struct TeleportPowerPlugin;

impl Plugin for TeleportPowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (remove_teleport_cooldown, handle_teleport)
                .chain()
                .in_set(AbilitySet),
        );
    }
}

const TELEPORT_DISTANCE: f32 = 64.0;

fn handle_teleport(
    mut commands: Commands,
    mut player_transforms: Query<&mut Transform, With<Player>>,
    loadouts: Query<&Loadout, Without<TeleportCooldownTimer>>,
    mut ability_events: EventReader<UseAbilityEvent>,
    mouse_position: Res<MousePosition>,
) {
    for ability in ability_events.iter() {
        let Ok(loadout) = loadouts.get(ability.loadout) else { continue; };

        if loadout.abilities[ability.ability].power == Power::Teleport {
            for mut player_transform in &mut player_transforms {
                let player_position = player_transform.translation.truncate();
                let delta =
                    (**mouse_position - player_position).clamp_length_max(TELEPORT_DISTANCE);

                player_transform.translation += delta.extend(0.0);
            }
            commands
                .entity(ability.loadout)
                .insert(TeleportCooldownTimer::default());
        }
    }
}

#[derive(Component, Clone, Deref, DerefMut, Debug, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
struct TeleportCooldownTimer(pub Timer);

impl Default for TeleportCooldownTimer {
    fn default() -> Self {
        Self(Timer::new(Duration::from_secs_f32(0.5), TimerMode::Once))
    }
}

fn remove_teleport_cooldown(
    mut commands: Commands,
    time: Res<Time>,
    mut timers: Query<(Entity, &mut TeleportCooldownTimer)>,
) {
    for (entity, mut timer) in &mut timers {
        if !timer.tick(time.delta()).just_finished() {
            continue;
        }

        commands.entity(entity).remove::<TeleportCooldownTimer>();
    }
}
