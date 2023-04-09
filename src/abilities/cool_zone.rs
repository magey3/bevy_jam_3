use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::Damping;

use crate::{lifetime::Lifetime, player::Player};

use super::{
    cooldown::AbilityCooldown,
    heat::{Heat, Overheated},
    AbilitySet, Loadout, SideEffect, UseAbilityEvent,
};

pub struct CoolZonePlugin;

impl Plugin for CoolZonePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CoolZone>()
            .register_type::<Slowed>()
            .add_systems(
                (spawn_zone, cool_down, slow_movement, unslow_movement)
                    .chain()
                    .in_set(AbilitySet),
            );
    }
}

#[derive(Component, Clone, Debug, Default, Reflect, FromReflect)]
#[reflect(Component, Debug)]
pub struct CoolZone {
    pub cooling_speed: f32,
    pub slowdown: f32,
    pub radius: f32,
}

fn spawn_zone(
    mut commands: Commands,
    player_transform: Query<&Transform, With<Player>>,
    loadouts: Query<&Loadout, Without<Overheated>>,
    mut ability_events: EventReader<UseAbilityEvent>,
    side_effects: Query<&SideEffect, Without<AbilityCooldown>>,
) {
    for ability in ability_events.iter() {
        let Ok(loadout) = loadouts.get(ability.loadout) else {
            continue;
        };

        let ability_entity = loadout.abilities[ability.ability];
        let Ok(side_effect) = side_effects.get(ability_entity) else { continue; };
        if *side_effect != SideEffect::CoolZone {
            continue;
        }

        let player_position = player_transform.single().translation.truncate();
        commands.spawn((
            CoolZone {
                cooling_speed: 20.0,
                slowdown: 15.0,
                radius: 64.0,
            },
            SpriteBundle {
                sprite: Sprite {
                    color: Color::CYAN,
                    custom_size: Some(Vec2::splat(64.0)),
                    ..Default::default()
                },
                transform: Transform::from_translation(player_position.extend(0.5)),
                ..Default::default()
            },
            Lifetime::new(Duration::from_secs(1)),
        ));
    }
}

#[derive(Component, Clone, Debug, Default, Reflect, FromReflect)]
#[reflect(Component, Debug)]
pub struct Slowed(pub f32);

fn slow_movement(
    mut commands: Commands,
    mut objects: Query<(Entity, &mut Damping, &Transform), Without<Slowed>>,
    zones: Query<(&CoolZone, &Transform)>,
) {
    for (zone, zone_transform) in &zones {
        for (object_id, mut object_damping, object_transform) in &mut objects {
            let zone_position = zone_transform.translation.truncate();
            let object_position = object_transform.translation.truncate();

            if zone_position.distance(object_position) > zone.radius {
                continue;
            }

            object_damping.linear_damping += zone.slowdown;
            commands.entity(object_id).insert(Slowed(zone.slowdown));
        }
    }
}

fn unslow_movement(
    mut commands: Commands,
    mut objects: Query<(Entity, &Slowed, &mut Damping, &Transform)>,
    zones: Query<(&CoolZone, &Transform)>,
) {
    'next_object: for (object_id, slowdown, mut object_damping, object_transform) in &mut objects {
        for (zone, zone_transform) in &zones {
            let zone_position = zone_transform.translation.truncate();
            let object_position = object_transform.translation.truncate();

            if zone_position.distance(object_position) < zone.radius {
                continue 'next_object;
            }
        }
        object_damping.linear_damping -= slowdown.0;
        commands.entity(object_id).remove::<Slowed>();
    }
}

fn cool_down(
    mut objects: Query<(&mut Heat, &Transform)>,
    zones: Query<(&CoolZone, &Transform)>,
    time: Res<Time>,
) {
    for (zone, zone_transform) in &zones {
        for (mut object_heat, object_transform) in &mut objects {
            let zone_position = zone_transform.translation.truncate();
            let object_position = object_transform.translation.truncate();

            if zone_position.distance(object_position) > zone.radius {
                continue;
            }

            **object_heat -= zone.cooling_speed * time.delta_seconds();
        }
    }
}
