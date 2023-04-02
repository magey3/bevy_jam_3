use bevy::prelude::*;

use crate::player::Player;

use self::teleport::TeleportPowerPlugin;

pub struct AbilitiesPlugin;

mod teleport;

#[derive(SystemSet, Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct AbilitySet;

impl Plugin for AbilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Power>()
            .register_type::<SideEffect>()
            .register_type::<Ability>()
            .register_type::<Loadout>()
            .add_event::<UseAbilityEvent>()
            .add_system(test)
            .add_plugin(TeleportPowerPlugin);
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

#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect, FromReflect)]
#[reflect(Debug)]
pub enum Power {
    Teleport,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect, FromReflect)]
#[reflect(Debug)]
pub enum SideEffect {
    Woo,
}

#[derive(Clone, Debug, Reflect, FromReflect)]
#[reflect(Debug)]
pub struct Ability {
    pub power: Power,
    pub side_effect: SideEffect,
}

#[derive(Component, Clone, Default, Debug, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
pub struct Loadout {
    pub abilities: Vec<Ability>,
}

#[derive(Clone, Debug)]
pub struct UseAbilityEvent {
    pub loadout: Entity,
    pub ability: usize,
}
