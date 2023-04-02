use bevy::prelude::*;

use crate::player::Player;

use super::{AbilitySet, Loadout, Power, UseAbilityEvent};

pub struct TeleportPowerPlugin;

impl Plugin for TeleportPowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_teleport.in_set(AbilitySet));
    }
}

fn handle_teleport(
    mut player_transforms: Query<&mut Transform, With<Player>>,
    loadouts: Query<&Loadout>,
    mut ability_events: EventReader<UseAbilityEvent>,
) {
    for ability in ability_events.iter() {
        let loadout = loadouts.get(ability.loadout).unwrap();

        if loadout.abilities[ability.ability].power == Power::Teleport {
            for mut player_transform in &mut player_transforms {
                // TODO: Make this follow the mouse
                player_transform.translation += Vec3::new(16.0, 16.0, 0.0);
            }
        }
    }
}
