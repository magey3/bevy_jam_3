use bevy::prelude::*;

use crate::{health::DamageEvent, player::Player};

use super::{
    cooldown::AbilityCooldown, heat::Overheated, AbilitySet, Loadout, SideEffect, UseAbilityEvent,
};

pub struct TakeDamageSideEffectPlugin;

impl Plugin for TakeDamageSideEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(take_damage.in_set(AbilitySet));
    }
}

const DAMAGE: f32 = 5.0;

fn take_damage(
    player: Query<Entity, With<Player>>,
    loadouts: Query<&Loadout, Without<Overheated>>,
    mut ability_events: EventReader<UseAbilityEvent>,
    side_effects: Query<&SideEffect, Without<AbilityCooldown>>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for ability in ability_events.iter() {
        let Ok(loadout) = loadouts.get(ability.loadout) else {
            continue;
        };

        let ability_entity = loadout.abilities[ability.ability];
        let Ok(side_effect) = side_effects.get(ability_entity) else { continue; };
        if *side_effect != SideEffect::TakeDamage {
            continue;
        }

        damage_events.send(DamageEvent {
            damaged_id: player.single(),
            damage: DAMAGE,
        })
    }
}
