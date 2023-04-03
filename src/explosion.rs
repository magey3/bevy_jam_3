use bevy::prelude::*;
use bevy_rapier2d::prelude::ExternalImpulse;

use crate::health::Health;

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ExplosionEvent>()
            .add_event::<ExplosionEvent>()
            .add_systems(
                (apply_explosion_forces, apply_explostion_damage).in_set(HandleExplosionSet),
            );
    }
}

#[derive(SystemSet, Clone, Default, Debug, PartialEq, Eq, Hash)]
pub struct HandleExplosionSet;

#[derive(Clone, Default, Debug, Reflect, FromReflect)]
pub struct ExplosionEvent {
    pub position: Vec2,
    pub range: f32,
    pub force: f32,
    pub damage: f32,
}

fn apply_explosion_forces(
    mut explosion_events: EventReader<ExplosionEvent>,
    mut objects: Query<(&mut ExternalImpulse, &Transform)>,
) {
    for explosion in explosion_events.iter() {
        for (mut impulse, object_transform) in &mut objects {
            let object_position = object_transform.translation.truncate();

            let object_distance = object_position.distance(explosion.position);
            if object_distance > explosion.range {
                continue;
            }

            let object_distance_normalized = object_distance / explosion.range;

            let dir = (object_position - explosion.position).normalize_or_zero();
            let force = explosion.force * object_distance_normalized * dir;

            impulse.impulse = force;
        }
    }
}

fn apply_explostion_damage(
    mut explosion_events: EventReader<ExplosionEvent>,
    mut objects: Query<(&mut Health, &Transform)>,
) {
    for explosion in explosion_events.iter() {
        for (mut health, object_transform) in &mut objects {
            let object_position = object_transform.translation.truncate();

            let object_distance = object_position.distance(explosion.position);
            if object_distance > explosion.range {
                continue;
            }

            let object_distance_normalized = object_distance / explosion.range;

            **health -= explosion.damage * object_distance_normalized;
        }
    }
}
