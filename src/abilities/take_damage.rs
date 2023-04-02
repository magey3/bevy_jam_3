use bevy::prelude::*;

use crate::{health::Health, player::Player};

use super::{AbilitySet, Loadout, SideEffect, UseAbilityEvent};

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
) {
    for ability in ability_events.iter() {
        let loadout = loadouts.get(ability.loadout).unwrap();

        if loadout.abilities[ability.ability].side_effect == SideEffect::TakeDamage {
            for mut health in &mut player_healths {
                health.0 -= 10.0;
                info!("TOOK DAMAGE {health:?}");
            }
        }
    }
}
