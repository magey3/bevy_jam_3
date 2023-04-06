use bevy::prelude::*;

use self::circle::CirclePlugin;

pub struct EnemyPlugin;

pub mod circle;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Enemy>()
            .add_event::<SpawnEnemyEvent>()
            .add_plugin(CirclePlugin);
    }
}

#[derive(Component, Copy, Clone, Debug, PartialEq, Eq, Reflect, FromReflect)]
pub enum Enemy {
    Circle,
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
}
