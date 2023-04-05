use bevy::prelude::*;

use crate::{mouse_position::MousePosition, player::Player};

use super::{cooldown::AbilityCooldown, AbilitySet, Loadout, Power, UseAbilityEvent};

pub struct TeleportPowerPlugin;

impl Plugin for TeleportPowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_teleport.in_set(AbilitySet));
    }
}

const TELEPORT_DISTANCE: f32 = 64.0;

fn handle_teleport(
    mut player_transforms: Query<&mut Transform, With<Player>>,
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
        if *power != Power::Teleport {
            continue;
        }

        for mut player_transform in &mut player_transforms {
            let player_position = player_transform.translation.truncate();
            let delta = (**mouse_position - player_position).clamp_length_max(TELEPORT_DISTANCE);

            player_transform.translation += delta.extend(0.0);
        }
    }
}
