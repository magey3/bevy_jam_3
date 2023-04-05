use bevy::prelude::*;

use crate::{health::Health, player::Player};

use super::{cooldown::AbilityCooldown, AbilitySet, Loadout, SideEffect, UseAbilityEvent};

pub struct TakeDamageSideEffectPlugin;

impl Plugin for TakeDamageSideEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(take_damage.in_set(AbilitySet));
    }
}

fn take_damage(
    mut player_healths: Query<&mut Health, With<Player>>,
    loadouts: Query<&Loadout>,
    mut ability_events: EventReader<UseAbilityEvent>,
    side_effects: Query<&SideEffect, Without<AbilityCooldown>>,
) {
    for ability in ability_events.iter() {
        let Ok(loadout) = loadouts.get(ability.loadout) else {
            warn!("Invalid loadout in ability event");
            continue;
        };

        let ability_entity = loadout.abilities[ability.ability];
        let Ok(side_effect) = side_effects.get(ability_entity) else { continue; };
        if *side_effect != SideEffect::TakeDamage {
            continue;
        }

        for mut health in &mut player_healths {
            health.0 -= 10.0;
        }
    }
}
