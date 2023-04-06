use bevy::{prelude::*, sprite::Anchor};

use crate::health::{Health, MaxHealth};

pub struct HealthBarPlugin;

impl Plugin for HealthBarPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<HealthBar>()
            .add_systems((despawn_bar, spawn_bar, render_bar));
    }
}

#[derive(Component, Clone, Debug, Reflect, FromReflect)]
#[reflect(Debug)]
pub struct HealthBar {
    pub value: Entity,
}

fn spawn_bar(mut commands: Commands, added_healths: Query<(Entity, &Transform), Added<Health>>) {
    for (value, transform) in &added_healths {
        let pos = transform.translation.truncate();
        commands.spawn((
            HealthBar { value },
            SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: Some(Vec2::new(16.0, 4.0)),
                    anchor: Anchor::BottomCenter,
                    ..Default::default()
                },
                transform: Transform::from_translation((pos + Vec2::new(0.0, 9.0)).extend(900.0)),
                ..Default::default()
            },
        ));
    }
}

fn despawn_bar(
    mut commands: Commands,
    bars: Query<(Entity, &HealthBar)>,
    healths: Query<(), With<Health>>,
) {
    for (bar_entity, bar) in &bars {
        if healths.get(bar.value).is_err() {
            commands.entity(bar_entity).despawn_recursive();
        }
    }
}

fn render_bar(
    mut bars: Query<(&mut Transform, &HealthBar)>,
    healths: Query<
        (&Transform, &Health, &MaxHealth),
        (
            Without<HealthBar>,
            Or<(Changed<Health>, Changed<Transform>)>,
        ),
    >,
) {
    for (mut bar_transform, bar) in &mut bars {
        let Ok((health_transform, health, max_health)) = healths.get(bar.value) else { continue; };

        let health_pos = health_transform.translation.truncate();

        bar_transform.translation =
            (health_pos + Vec2::new(0.0, 9.0)).extend(bar_transform.translation.z);
        let value = (**health / **max_health).max(0.0);

        bar_transform.scale.x = value;
    }
}
