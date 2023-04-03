use bevy::prelude::*;

use self::circle::CirclePlugin;

pub struct EnemyPlugin;

pub mod circle;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEnemyEvent>().add_plugin(CirclePlugin);
    }
}

#[derive(Component, Clone, Debug, PartialEq, Eq)]
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
