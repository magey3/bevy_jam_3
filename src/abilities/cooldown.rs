use std::time::Duration;

use bevy::prelude::*;

use super::{AbilitySet, Loadout, UseAbilityEvent};

pub struct CooldownPlugin;

impl Plugin for CooldownPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AbilityCooldown>()
            .add_system(cooldown_timer.before(AbilitySet))
            .add_systems(
                (apply_cooldown_times, apply_system_buffers)
                    .chain()
                    .after(AbilitySet),
            );
    }
}

#[derive(Component, Clone, Deref, DerefMut, Debug, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
pub struct AbilityCooldown(pub Timer);

#[derive(Component, Clone, Default, Deref, DerefMut, Debug, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
pub struct AbilityCooldownTime(pub f32);

impl AbilityCooldown {
    pub fn new(secs: f32) -> Self {
        Self(Timer::new(Duration::from_secs_f32(secs), TimerMode::Once))
    }
}

impl Default for AbilityCooldown {
    fn default() -> Self {
        Self::new(1.0)
    }
}

fn cooldown_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut timers: Query<(Entity, &mut AbilityCooldown)>,
) {
    for (entity, mut timer) in &mut timers {
        if !timer.tick(time.delta()).just_finished() {
            continue;
        }

        commands.entity(entity).remove::<AbilityCooldown>();
    }
}

fn apply_cooldown_times(
    mut commands: Commands,
    loadouts: Query<&Loadout>,
    mut ability_events: EventReader<UseAbilityEvent>,
    ability_cooldown_times: Query<&AbilityCooldownTime, Without<AbilityCooldown>>,
) {
    for ability in ability_events.iter() {
        let Ok(loadout) = loadouts.get(ability.loadout) else {
            warn!("Invalid loadout in ability event");
            continue;
        };

        let ability_entity = loadout.abilities[ability.ability];
        let Ok(default_cooldown) = ability_cooldown_times.get(ability_entity) else { continue; };
        commands
            .entity(ability_entity)
            .insert(AbilityCooldown::new(**default_cooldown));
    }
}
