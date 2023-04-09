use std::time::Duration;

use bevy::prelude::*;

use crate::{enemy::Target, lifetime::Lifetime, player::Player};

use super::{
    cooldown::AbilityCooldown, heat::Overheated, teleport::TeleportSet, AbilitySet, Loadout,
    SideEffect, UseAbilityEvent,
};

pub struct InvisibilityWithShadowPlugin;

impl Plugin for InvisibilityWithShadowPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Invisible>().add_systems(
            (
                ability,
                on_shadow_death,
                apply_system_buffers,
                on_add_invisible,
                on_removed_invisible,
            )
                .chain()
                .in_set(AbilitySet)
                .before(TeleportSet),
        );
    }
}

#[derive(Component, Clone, Default, Debug, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
pub struct Invisible;

fn on_add_invisible(mut commands: Commands, added_invisible: Query<Entity, Added<Invisible>>) {
    for entity in &added_invisible {
        commands.entity(entity).remove::<Target>();
    }
}

fn on_removed_invisible(
    mut commands: Commands,
    mut removed_invisible: RemovedComponents<Invisible>,
) {
    for entity in removed_invisible.iter() {
        commands.entity(entity).insert(Target);
    }
}

#[derive(Component, Clone, Default, Debug, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
pub struct Shadow;

fn ability(
    mut commands: Commands,
    player: Query<(Entity, &Transform, &Handle<Image>, &Sprite), With<Player>>,
    loadouts: Query<&Loadout, Without<Overheated>>,
    mut ability_events: EventReader<UseAbilityEvent>,
    side_effects: Query<&SideEffect, Without<AbilityCooldown>>,
) {
    for ability in ability_events.iter() {
        let Ok(loadout) = loadouts.get(ability.loadout) else {
            continue;
        };

        let ability_entity = loadout.abilities[ability.ability];
        let Ok(side_effect) = side_effects.get(ability_entity) else { continue; };
        if *side_effect != SideEffect::InvisibleWithShadow {
            continue;
        }

        let (player_id, player_transform, player_texture, player_sprite) = player.single();
        commands.entity(player_id).insert(Invisible);

        commands.spawn((
            Shadow,
            Target,
            Lifetime::new(Duration::from_secs(3)),
            SpriteBundle {
                sprite: player_sprite.clone(),
                transform: *player_transform,
                texture: player_texture.clone(),
                ..Default::default()
            },
        ));
    }
}

fn on_shadow_death(
    mut commands: Commands,
    mut removed_shadows: RemovedComponents<Shadow>,
    player: Query<Entity, With<Player>>,
) {
    for _ in removed_shadows.iter() {
        commands.entity(player.single()).remove::<Invisible>();
    }
}
