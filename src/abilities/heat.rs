use bevy::prelude::*;

use crate::player::Player;

use super::{AbilitySet, Loadout, UseAbilityEvent};

pub struct HeatPlugin;

impl Plugin for HeatPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Heat>()
            .register_type::<Overheated>()
            .add_system(add_heat_on_ability.in_set(AbilitySet))
            .add_systems(
                (add_overheated, remove_heat, remove_overheated)
                    .chain()
                    .after(AbilitySet),
            );
    }
}

const OVERHEAT_THRESHOLD: f32 = 100.0;
const HEAT_REMOVAL_RATE: f32 = 20.0;

#[derive(Component, Clone, Default, Deref, DerefMut, Debug, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
pub struct Heat(pub f32);

#[derive(Component, Clone, Default, Debug, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
pub struct Overheated;

fn add_overheated(
    mut commands: Commands,
    heats: Query<(Entity, &Heat), (Changed<Heat>, Without<Overheated>)>,
) {
    for (entity, heat) in &heats {
        if **heat > OVERHEAT_THRESHOLD {
            commands.entity(entity).insert(Overheated);
            info!("OVERHEATED");
        }
    }
}

fn remove_overheated(
    mut commands: Commands,
    heats: Query<(Entity, &Heat), (Changed<Heat>, With<Overheated>)>,
) {
    for (entity, heat) in &heats {
        if **heat <= 0.0 {
            commands.entity(entity).remove::<Overheated>();
            info!("REMOVED OVERHEAT");
        }
    }
}

#[derive(Component, Clone, Default, Deref, DerefMut, Debug, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
pub struct AddHeatOnUse(pub f32);

fn add_heat_on_ability(
    mut ability_events: EventReader<UseAbilityEvent>,
    mut players: Query<(&mut Heat, &Loadout), (With<Player>, Without<Overheated>)>,
    ability: Query<&AddHeatOnUse>,
) {
    for ability_event in ability_events.iter() {
        let Ok((mut heat, loadout)) = players.get_mut(ability_event.loadout) else {
            continue;
        };
        let ability_entity = loadout.abilities[ability_event.ability];
        let Ok(heat_to_add) = ability.get(ability_entity) else { continue; };

        **heat += **heat_to_add;
    }
}

fn remove_heat(mut heats: Query<&mut Heat>, time: Res<Time>) {
    for mut heat in &mut heats {
        **heat = (**heat - HEAT_REMOVAL_RATE * time.delta_seconds()).max(0.0);
    }
}
