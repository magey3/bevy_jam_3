use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::{ActiveEvents, Collider, CollisionEvent, RigidBody, Velocity};

use crate::{
    health::DamageEvent, lifetime::Lifetime, mouse_position::MousePosition, player::Player,
};

use super::{
    cooldown::AbilityCooldown, heat::Overheated, AbilitySet, Loadout, Power, UseAbilityEvent,
};

pub struct ShotgunPlugin;

impl Plugin for ShotgunPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Bullet>()
            .add_systems((shoot, hurt).chain().in_set(AbilitySet));
    }
}

const BULLET_VELOCITY: f32 = 512.0;
const BULLET_DAMAGE: f32 = 15.0;
const SPREAD: f32 = 5.0;

#[derive(Component, Clone, Debug, Default, Reflect, FromReflect)]
#[reflect(Component, Debug)]
pub struct Bullet;

fn shoot(
    mut commands: Commands,
    player_transforms: Query<&Transform, With<Player>>,
    loadouts: Query<&Loadout, Without<Overheated>>,
    mut ability_events: EventReader<UseAbilityEvent>,
    mouse_position: Res<MousePosition>,
    powers: Query<&Power, Without<AbilityCooldown>>,
) {
    for ability in ability_events.iter() {
        let Ok(loadout) = loadouts.get(ability.loadout) else {
            continue;
        };

        let ability_entity = loadout.abilities[ability.ability];
        let Ok(power) = powers.get(ability_entity) else { continue; };
        if *power != Power::Shotgun {
            continue;
        }

        let player_position = player_transforms.single().translation.truncate();
        let dir = (**mouse_position - player_position).normalize_or_zero();

        for i in -1..=1 {
            let dir = dir.rotate(Vec2::from_angle(SPREAD.to_radians() * i as f32));
            commands.spawn((
                Bullet,
                RigidBody::Dynamic,
                Velocity {
                    linvel: dir * BULLET_VELOCITY,
                    ..Default::default()
                },
                Collider::ball(1.0),
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::ORANGE,
                        custom_size: Some(Vec2::splat(2.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(
                        (player_position + dir * 16.0).extend(1.0),
                    ),
                    ..Default::default()
                },
                ActiveEvents::COLLISION_EVENTS,
                Lifetime::new(Duration::from_secs(2)),
            ));
        }
    }
}

fn hurt(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    bullets: Query<(), With<Bullet>>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for collision in collision_events.iter() {
        let CollisionEvent::Started(e1, e2, _) = collision else { return };
        if bullets.get(*e1).is_ok() && bullets.get(*e2).is_err() {
            commands.entity(*e1).despawn_recursive();
            damage_events.send(DamageEvent {
                damaged_id: *e2,
                damage: BULLET_DAMAGE,
            });
        } else if bullets.get(*e2).is_ok() && bullets.get(*e1).is_err() {
            commands.entity(*e2).despawn_recursive();
            damage_events.send(DamageEvent {
                damaged_id: *e1,
                damage: BULLET_DAMAGE,
            });
        } else if bullets.get(*e2).is_ok() && bullets.get(*e1).is_ok() {
            // do not despawn on bullet collisions
            //commands.entity(*e1).despawn_recursive();
            //commands.entity(*e2).despawn_recursive();
        }
    }
}
