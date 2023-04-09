use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};
use bevy_rapier2d::prelude::ExternalImpulse;
use bevy_turborand::{DelegatedRng, GlobalRng};

use crate::{
    assets::GameAssets,
    health::{DamageEvent, Health},
    lifetime::Lifetime,
};

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ExplosionEvent>()
            .add_event::<ExplosionEvent>()
            .add_systems(
                (
                    apply_explosion_forces,
                    apply_explostion_damage,
                    play_audio,
                    spawn_particles,
                    move_particles,
                )
                    .in_set(HandleExplosionSet),
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
    mut damage_events: EventWriter<DamageEvent>,
    mut objects: Query<(Entity, &Transform), With<Health>>,
) {
    for explosion in explosion_events.iter() {
        for (object_id, object_transform) in &mut objects {
            let object_position = object_transform.translation.truncate();

            let object_distance = object_position.distance(explosion.position);
            if object_distance > explosion.range {
                continue;
            }

            let object_distance_normalized = 1.0 - (object_distance / explosion.range);

            damage_events.send(DamageEvent {
                damage: explosion.damage * object_distance_normalized,
                damaged_id: object_id,
            });
        }
    }
}

#[derive(Clone, Default, Debug, Reflect, FromReflect, Component)]
struct ExplosionParticle {
    pub velocity: Vec2,
}

fn spawn_particles(
    mut commands: Commands,
    mut explosion_events: EventReader<ExplosionEvent>,
    mut global_rng: ResMut<GlobalRng>,
) {
    for explosion in explosion_events.iter() {
        const STEPS: u32 = 64;
        const RANGE: f32 = 2.0;
        const VELOCITY: f32 = 256.0;
        const MAX_SIZE: f32 = 4.0;
        for i in 0..STEPS {
            let angle = (360.0 / STEPS as f32 * i as f32 + global_rng.f32_normalized() * RANGE)
                .to_radians();
            let velocity =
                Vec2::from_angle(angle) * (VELOCITY + 64.0 * global_rng.f32_normalized());

            commands.spawn((
                ExplosionParticle { velocity },
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::splat(global_rng.f32() * MAX_SIZE)),
                        color: Color::ORANGE,
                        ..Default::default()
                    },
                    transform: Transform::from_translation(explosion.position.extend(2.0)),
                    ..Default::default()
                },
                Lifetime::new(Duration::from_millis(200)),
            ));
        }
    }
}

fn move_particles(mut particles: Query<(&mut Transform, &ExplosionParticle)>, time: Res<Time>) {
    for (mut particle_transform, particle) in &mut particles {
        particle_transform.translation += (particle.velocity * time.delta_seconds()).extend(0.0);
    }
}

fn play_audio(
    assets: Res<GameAssets>,
    audio: Res<Audio>,
    mut explosion_events: EventReader<ExplosionEvent>,
) {
    for _ in explosion_events.iter() {
        audio.play(assets.explosion.clone());
    }
}
