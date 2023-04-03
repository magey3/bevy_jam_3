use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::{
    Collider, Damping, ExternalForce, ExternalImpulse, LockedAxes, RigidBody, Velocity,
};

use crate::{
    explosion::{ExplosionEvent, HandleExplosionSet},
    health::Health,
    player::Player,
};

use super::{Enemy, EnemySet, SpawnEnemyEvent};

pub(super) struct CirclePlugin;

impl Plugin for CirclePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Circle>()
            .register_type::<CircleMesh>()
            .add_startup_system(init_mesh)
            .add_systems(
                (
                    insert_explosion_timers,
                    increment_explosion_timer,
                    animate_explosion,
                    explode.before(HandleExplosionSet),
                )
                    .chain()
                    .in_set(EnemySet::Attack),
            )
            .add_systems((
                spawn_circle.in_set(EnemySet::SpawnEnemies),
                follow_player.in_set(EnemySet::AI),
            ));
    }
}

const CIRCLE_MOVE_FORCE: f32 = 32.0;
const CIRCLE_EXPLODE_DISTANCE: f32 = 32.0;

#[derive(Component, Clone, Default, Debug, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
pub struct Circle;

#[derive(Resource, Clone, Default, Debug, Deref, DerefMut, Reflect, FromReflect)]
#[reflect(Resource, Default, Debug)]
struct CircleMesh(pub Handle<Mesh>);

#[derive(Resource, Clone, Default, Debug, Reflect, FromReflect)]
#[reflect(Resource, Default, Debug)]
struct CircleMaterial {
    pub default: Handle<ColorMaterial>,
    pub highlight: Handle<ColorMaterial>,
}

fn init_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut spawn: EventWriter<SpawnEnemyEvent>,
) {
    commands.insert_resource(CircleMesh(meshes.add(shape::Circle::new(8.0).into())));
    commands.insert_resource(CircleMaterial {
        default: materials.add(ColorMaterial {
            color: Color::RED,
            ..Default::default()
        }),
        highlight: materials.add(ColorMaterial {
            color: Color::WHITE,
            ..Default::default()
        }),
    });
    for y in -3..=3 {
        spawn.send(SpawnEnemyEvent {
            enemy: Enemy::Circle,
            translation: Vec2::new(64.0, 16.0 * y as f32),
        });
    }
}

fn spawn_circle(
    mut commands: Commands,
    mut spawn_enemy_events: EventReader<SpawnEnemyEvent>,
    circle_mesh: Res<CircleMesh>,
    circle_material: Res<CircleMaterial>,
) {
    for SpawnEnemyEvent { enemy, translation } in spawn_enemy_events.iter() {
        if *enemy == Enemy::Circle {
            commands.spawn((
                Enemy::Circle,
                Circle,
                ColorMesh2dBundle {
                    transform: Transform::from_translation(translation.extend(1.0)),
                    mesh: circle_mesh.0.clone().into(),
                    material: circle_material.default.clone(),
                    ..Default::default()
                },
                RigidBody::Dynamic,
                LockedAxes::ROTATION_LOCKED,
                Velocity::default(),
                Collider::ball(8.0),
                ExternalForce::default(),
                ExternalImpulse::default(),
                Damping {
                    linear_damping: 5.0,
                    ..Default::default()
                },
                Health(100.0),
            ));
        }
    }
}

fn follow_player(
    mut circles: Query<(&mut ExternalForce, &Transform), With<Circle>>,
    player: Query<&Transform, With<Player>>,
) {
    for (mut circle_force, circle_transform) in &mut circles {
        let Ok(player_transform) = player.get_single() else { return; };

        let player_pos = player_transform.translation.truncate();
        let circle_pos = circle_transform.translation.truncate();

        let dir = (player_pos - circle_pos).normalize_or_zero();

        circle_force.force = dir * CIRCLE_MOVE_FORCE;
    }
}

#[derive(Component, Clone, Default, Debug, Deref, DerefMut, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
struct CircleExplosionTimer(pub Timer);

fn insert_explosion_timers(
    mut commands: Commands,
    circles: Query<(Entity, &Transform), (Without<CircleExplosionTimer>, With<Circle>)>,
    player: Query<&Transform, With<Player>>,
) {
    for (circle_entity, circle_transform) in &circles {
        let Ok(player_transform) = player.get_single() else { return; };

        let player_pos = player_transform.translation.truncate();
        let circle_pos = circle_transform.translation.truncate();

        if player_pos.distance(circle_pos) < CIRCLE_EXPLODE_DISTANCE {
            commands.entity(circle_entity).insert((
                CircleExplosionTimer(Timer::new(Duration::from_secs_f64(1.0), TimerMode::Once)),
                CircleAnimationTimer(Timer::new(
                    Duration::from_secs_f64(0.1),
                    TimerMode::Repeating,
                )),
            ));
        }
    }
}

fn increment_explosion_timer(mut timers: Query<&mut CircleExplosionTimer>, time: Res<Time>) {
    for mut timer in &mut timers {
        timer.tick(time.delta());
    }
}

fn explode(
    mut commands: Commands,
    circles: Query<(Entity, &Transform, &CircleExplosionTimer)>,
    mut explosion_events: EventWriter<ExplosionEvent>,
) {
    for (circle_entity, circle_transform, circle_timer) in &circles {
        if !circle_timer.finished() {
            continue;
        }

        explosion_events.send(ExplosionEvent {
            position: circle_transform.translation.truncate(),
            range: 100.0,
            force: 200.0,
            damage: 50.0,
        });
        commands.entity(circle_entity).despawn_recursive();
    }
}

#[derive(Component, Clone, Default, Deref, DerefMut, Debug, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
struct CircleAnimationTimer(pub Timer);

fn animate_explosion(
    mut circles: Query<(&mut Handle<ColorMaterial>, &mut CircleAnimationTimer)>,
    materials: Res<CircleMaterial>,
    time: Res<Time>,
) {
    for (mut material, mut timer) in &mut circles {
        if !timer.tick(time.delta()).just_finished() {
            continue;
        }
        if material.id() == materials.default.id() {
            *material = materials.highlight.clone()
        } else {
            *material = materials.default.clone()
        }
    }
}
