use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};

use crate::{assets::GameAssets, mouse_position::MousePosition, player::Player};

use super::{
    cooldown::AbilityCooldown, heat::Overheated, AbilitySet, Loadout, Power, UseAbilityEvent,
};

pub struct TeleportPowerPlugin;

impl Plugin for TeleportPowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_teleport.in_set(AbilitySet).in_set(TeleportSet));
    }
}

#[derive(SystemSet, Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct TeleportSet;

const TELEPORT_DISTANCE: f32 = 64.0;

fn handle_teleport(
    mut player_transforms: Query<&mut Transform, With<Player>>,
    loadouts: Query<&Loadout, Without<Overheated>>,
    mut ability_events: EventReader<UseAbilityEvent>,
    mouse_position: Res<MousePosition>,
    powers: Query<&Power, Without<AbilityCooldown>>,
    audio: Res<Audio>,
    assets: Res<GameAssets>,
) {
    for ability in ability_events.iter() {
        let Ok(loadout) = loadouts.get(ability.loadout) else {
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

        audio.play(assets.teleport_sound.clone());
    }
}
