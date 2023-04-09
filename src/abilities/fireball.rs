use std::time::Duration;

use bevy::prelude::*;

use crate::{explosion::ExplosionEvent, mouse_position::MousePosition};

use super::{
    cooldown::AbilityCooldown, heat::Overheated, AbilitySet, Loadout, Power, UseAbilityEvent,
};

pub struct FireballPlugin;

impl Plugin for FireballPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Fireball>()
            .add_systems((spawn_fireball, handle_fireball_explosion).in_set(AbilitySet));
    }
}

#[derive(Component, Clone, Debug, Default, Reflect, FromReflect)]
#[reflect(Component, Debug)]
pub struct Fireball {
    pub timer: Timer,
}

fn spawn_fireball(
    mut commands: Commands,
    loadouts: Query<&Loadout, Without<Overheated>>,
    mut ability_events: EventReader<UseAbilityEvent>,
    mouse_position: Res<MousePosition>,
    powers: Query<&Power, Without<AbilityCooldown>>,
) {
    for ability in ability_events.iter() {
        let Ok(loadout) = loadouts.get(ability.loadout) else {
            continue;
        };

        let ability_entity = loadout.abilities[ability.ability];
        let Ok(power) = powers.get(ability_entity) else { continue; };
        if *power != Power::Fireball {
            continue;
        }

        commands.spawn((
            Fireball {
                timer: Timer::new(Duration::from_millis(250), TimerMode::Once),
            },
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED.with_a(0.0),
                    custom_size: Some(Vec2::splat(16.0)),
                    ..Default::default()
                },
                transform: Transform::from_translation(mouse_position.extend(1.0)),
                ..Default::default()
            },
        ));
    }
}

fn handle_fireball_explosion(
    mut commands: Commands,
    time: Res<Time>,
    mut fireballs: Query<(Entity, &Transform, &mut Sprite, &mut Fireball)>,
    mut explosion_events: EventWriter<ExplosionEvent>,
) {
    for (fireball_id, fireball_transform, mut fireball_sprite, mut fireball) in &mut fireballs {
        if !fireball.timer.tick(time.delta()).just_finished() {
            fireball_sprite.color = fireball_sprite
                .color
                .with_a(fireball.timer.elapsed_secs() * 0.7);
            continue;
        }

        explosion_events.send(ExplosionEvent {
            position: fireball_transform.translation.truncate(),
            range: 32.0,
            force: 100.0,
            damage: 200.0,
        });

        commands.entity(fireball_id).despawn_recursive();
    }
}
