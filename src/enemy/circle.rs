use bevy::prelude::*;
use bevy_rapier2d::prelude::{
    Collider, Damping, ExternalForce, ExternalImpulse, LockedAxes, RigidBody, Velocity,
};

use crate::{health::Health, player::Player};

use super::{Enemy, EnemySet, SpawnEnemyEvent};

pub(super) struct CirclePlugin;

impl Plugin for CirclePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Circle>()
            .register_type::<CircleMesh>()
            .add_startup_system(init_mesh)
            .add_systems((spawn_circle, follow_player).in_set(EnemySet::SpawnEnemies));
    }
}

const CIRCLE_MOVE_FORCE: f32 = 32.0;

#[derive(Component, Clone, Default, Debug, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
pub struct Circle;

#[derive(Resource, Clone, Default, Debug, Deref, DerefMut, Reflect, FromReflect)]
#[reflect(Resource, Default, Debug)]
struct CircleMesh(pub Handle<Mesh>);

#[derive(Resource, Clone, Default, Debug, Deref, DerefMut, Reflect, FromReflect)]
#[reflect(Resource, Default, Debug)]
struct CircleMaterial(pub Handle<ColorMaterial>);

fn init_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut spawn: EventWriter<SpawnEnemyEvent>,
) {
    commands.insert_resource(CircleMesh(meshes.add(shape::Circle::new(8.0).into())));
    commands.insert_resource(CircleMaterial(materials.add(ColorMaterial {
        color: Color::RED,
        ..Default::default()
    })));
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
                    material: circle_material.0.clone(),
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
        let player_transform = player.single();

        let player_pos = player_transform.translation.truncate();
        let circle_pos = circle_transform.translation.truncate();

        let dir = (player_pos - circle_pos).normalize_or_zero();

        circle_force.force = dir * CIRCLE_MOVE_FORCE;
    }
}
