use bevy::prelude::*;

use crate::health::DeathEvent;

use self::bomb::BombPlugin;

pub struct EnemyPlugin;

pub mod bomb;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Enemy>()
            .register_type::<Target>()
            .add_event::<SpawnEnemyEvent>()
            .add_plugin(BombPlugin)
            .add_system(on_enemy_death.in_set(EnemySet::Die));
    }
}

#[derive(Component, Clone, Default, Debug, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
pub struct Target;

#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, Reflect, FromReflect)]
pub enum Enemy {
    Bomb,
}

#[derive(Clone, Debug)]
pub struct SpawnEnemyEvent {
    pub enemy: Enemy,
    pub translation: Vec2,
}

#[derive(SystemSet, Clone, Debug, Hash, PartialEq, Eq)]
pub enum EnemySet {
    SpawnEnemies,
    AI,
    Attack,
    Die,
}

fn on_enemy_death(
    mut commands: Commands,
    enemies: Query<(), With<Enemy>>,
    mut death_events: EventReader<DeathEvent>,
) {
    for death in death_events.iter() {
        if enemies.get(death.died_id).is_ok() {
            commands.entity(death.died_id).despawn_recursive();
        }
    }
}
