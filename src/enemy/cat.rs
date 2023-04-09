use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::{
    Collider, Damping, ExternalForce, ExternalImpulse, LockedAxes, RigidBody, Velocity,
};

use crate::health::{DamageEvent, Health, MaxHealth};

use super::{Enemy, EnemySet, SpawnEnemyEvent, Target};

pub struct CatPlugin;

impl Plugin for CatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_cat.in_set(EnemySet::SpawnEnemies))
            .add_systems(
                (
                    cat_tracking,
                    tracking_to_jump_transition,
                    cat_jumping_to_target,
                    cat_attacking,
                    cat_jumping_from_target,
                )
                    .chain()
                    .in_set(EnemySet::AI),
            );
    }
}

#[derive(Component, Clone, Default, Debug, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
pub struct Cat;

#[derive(Component, Copy, Clone, Default, Debug, PartialEq, Eq, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
pub enum CatState {
    #[default]
    Tracking,
    JumpingToTarget,
    Attacking,
    JumpingFromTarget,
}

fn spawn_cat(mut commands: Commands, mut spawn_enemy_events: EventReader<SpawnEnemyEvent>) {
    for SpawnEnemyEvent { enemy, translation } in spawn_enemy_events.iter() {
        if *enemy == Enemy::Bomb {
            commands.spawn((
                Enemy::Cat,
                Cat,
                CatState::default(),
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::RED,
                        custom_size: Some(Vec2::splat(16.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(translation.extend(1.0)),
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
                MaxHealth(100.0),
            ));
        }
    }
}

const LEAD_ANGLE: f32 = 36.0;
const MOVE_FORCE: f32 = 64.0;
const TRACKING_DISTANCE: f32 = 128.0;

#[derive(Component, Clone, Default, Debug, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
pub struct TrackingTimer(pub Timer);

fn cat_tracking(
    player: Query<&Transform, With<Target>>,
    mut cats: Query<(&Transform, &mut ExternalForce, &CatState)>,
) {
    for (cat_transform, mut cat_force, cat_state) in &mut cats {
        if *cat_state != CatState::Tracking {
            continue;
        }

        let Ok(player_transform) = player.get_single() else { return; };
        let player_position = player_transform.translation.truncate();
        let cat_position = cat_transform.translation.truncate();

        let delta = cat_position - player_position;

        let target_position = delta
            .rotate(Vec2::from_angle(LEAD_ANGLE.to_radians()))
            .clamp_length_max(TRACKING_DISTANCE)
            + player_position;

        let delta = target_position - cat_position;

        let move_dir = delta.normalize_or_zero();

        cat_force.force = MOVE_FORCE * move_dir;
    }
}

fn tracking_to_jump_transition(
    mut commands: Commands,
    player: Query<&Transform, With<Target>>,
    mut cats: Query<(
        Entity,
        &Transform,
        &mut CatState,
        Option<&mut TrackingTimer>,
    )>,
    time: Res<Time>,
) {
    for (cat_id, cat_transform, mut cat_state, tracking_timer) in &mut cats {
        if *cat_state != CatState::Tracking {
            continue;
        }
        let Ok(player_transform) = player.get_single() else { return; };
        let player_position = player_transform.translation.truncate();
        let cat_position = cat_transform.translation.truncate();

        if tracking_timer.is_none() && player_position.distance(cat_position) <= TRACKING_DISTANCE {
            commands.entity(cat_id).insert(TrackingTimer(Timer::new(
                Duration::from_secs(2),
                TimerMode::Once,
            )));
        }

        if let Some(mut timer) = tracking_timer {
            if timer.0.tick(time.delta()).just_finished() {
                *cat_state = CatState::JumpingToTarget;
                commands.entity(cat_id).remove::<TrackingTimer>();
            }
        }
    }
}

const JUMP_IMPULSE: f32 = 128.0;

fn cat_jumping_to_target(
    mut commands: Commands,
    player: Query<&Transform, With<Target>>,
    mut cats: Query<(Entity, &Transform, &mut ExternalImpulse, &mut CatState), Changed<CatState>>,
) {
    for (cat_id, cat_transform, mut cat_force, mut cat_state) in &mut cats {
        if *cat_state != CatState::JumpingToTarget {
            continue;
        }
        let Ok(player_transform) = player.get_single() else { return; };
        let player_position = player_transform.translation.truncate();
        let cat_position = cat_transform.translation.truncate();

        let move_dir = (player_position - cat_position).normalize_or_zero();

        cat_force.impulse = JUMP_IMPULSE * move_dir;
        *cat_state = CatState::Attacking;
        commands.entity(cat_id).insert(AttackTimeout(Timer::new(
            Duration::from_secs(1),
            TimerMode::Once,
        )));
    }
}

fn cat_jumping_from_target(
    player: Query<&Transform, With<Target>>,
    mut cats: Query<(&Transform, &mut ExternalImpulse, &mut CatState), Changed<CatState>>,
) {
    for (cat_transform, mut cat_force, mut cat_state) in &mut cats {
        if *cat_state != CatState::JumpingFromTarget {
            continue;
        }
        let Ok(player_transform) = player.get_single() else { return; };
        let player_position = player_transform.translation.truncate();
        let cat_position = cat_transform.translation.truncate();

        let move_dir = (cat_position - player_position).normalize_or_zero();

        cat_force.impulse = JUMP_IMPULSE * move_dir;
        *cat_state = CatState::Tracking;
    }
}

const ATTACK_RANGE: f32 = 16.0;
const ATTACK_DAMAGE: f32 = 50.0;

#[derive(Component, Clone, Default, Debug, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
pub struct AttackTimeout(pub Timer);

fn cat_attacking(
    mut commands: Commands,
    player: Query<(Entity, &Transform), With<Target>>,
    mut cats: Query<(Entity, &Transform, &mut CatState, &mut AttackTimeout)>,
    mut damage_events: EventWriter<DamageEvent>,
    time: Res<Time>,
) {
    for (cat_id, cat_transform, mut cat_state, mut attack_timeout) in &mut cats {
        if *cat_state != CatState::Attacking {
            continue;
        }
        let Ok((player_id, player_transform)) = player.get_single() else { return; };

        if attack_timeout.0.tick(time.delta()).just_finished() {
            *cat_state = CatState::JumpingFromTarget;
            commands.entity(cat_id).remove::<AttackTimeout>();
            continue;
        }

        let player_position = player_transform.translation.truncate();
        let cat_position = cat_transform.translation.truncate();

        if player_position.distance(cat_position) <= ATTACK_RANGE {
            damage_events.send(DamageEvent {
                damaged_id: player_id,
                damage: ATTACK_DAMAGE,
            });
            *cat_state = CatState::JumpingFromTarget;
            commands.entity(cat_id).remove::<AttackTimeout>();
        }
    }
}
