use std::time::Duration;

use bevy::prelude::*;

pub struct LifetimePlugin;

impl Plugin for LifetimePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Lifetime>()
            .add_system(lifetime_system.in_base_set(CoreSet::PostUpdate));
    }
}

#[derive(Component, Clone, Default, Debug, Deref, DerefMut, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
pub struct Lifetime(pub Timer);

impl Lifetime {
    pub fn new(duration: Duration) -> Self {
        Self(Timer::new(duration, TimerMode::Once))
    }
}

fn lifetime_system(
    mut commands: Commands,
    mut lifetimes: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut lifetimes {
        if lifetime.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
