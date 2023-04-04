use bevy::prelude::*;

use crate::{mouse_position::MousePosition, player::Player};

use super::{AbilitySet, Loadout, Power, UseAbilityEvent};

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
) {
    for ability in ability_events.iter() {
        let loadout = loadouts.get(ability.loadout).unwrap();

        if loadout.abilities[ability.ability].power == Power::Teleport {
            for mut player_transform in &mut player_transforms {
                let player_position = player_transform.translation.truncate();
                let delta =
                    (**mouse_position - player_position).clamp_length_max(TELEPORT_DISTANCE);

                player_transform.translation += delta.extend(0.0);
            }
        }
    }
}
