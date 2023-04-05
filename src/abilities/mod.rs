use bevy::prelude::*;

use crate::player::Player;

use self::{
    cooldown::CooldownPlugin, take_damage::TakeDamageSideEffectPlugin,
    teleport::TeleportPowerPlugin,
};

pub struct AbilitiesPlugin;

pub mod cooldown;
mod take_damage;
mod teleport;

#[derive(SystemSet, Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct AbilitySet;

impl Plugin for AbilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Power>()
            .register_type::<SideEffect>()
            .register_type::<Loadout>()
            .add_event::<UseAbilityEvent>()
            .add_system(test)
            .add_plugin(CooldownPlugin)
            .add_plugin(TeleportPowerPlugin)
            .add_plugin(TakeDamageSideEffectPlugin);
    }
}

fn test(
    keys: Res<Input<KeyCode>>,
    mut use_ability: EventWriter<UseAbilityEvent>,
    player: Query<Entity, With<Player>>,
) {
    if keys.just_pressed(KeyCode::R) {
        use_ability.send(UseAbilityEvent {
            loadout: player.single(),
            ability: 0,
        });
    }
}

#[derive(Component, Clone, Debug, Default, PartialEq, Eq, Hash, Reflect, FromReflect)]
#[reflect(Component, Debug)]
pub enum Power {
    #[default]
    Teleport,
}

#[derive(Component, Clone, Debug, Default, PartialEq, Eq, Hash, Reflect, FromReflect)]
#[reflect(Component, Debug)]
pub enum SideEffect {
    #[default]
    TakeDamage,
}

#[derive(Component, Clone, Default, Debug, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
pub struct Loadout {
    pub abilities: Vec<Entity>,
}

#[derive(Clone, Debug)]
pub struct UseAbilityEvent {
    pub loadout: Entity,
    pub ability: usize,
}
