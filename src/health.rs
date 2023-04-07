use bevy::prelude::*;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Health>()
            .register_type::<MaxHealth>()
            .register_type::<DamageEvent>()
            .add_event::<DamageEvent>()
            .add_event::<DeathEvent>()
            .add_systems(
                (handle_damage, handle_death)
                    .chain()
                    .in_base_set(CoreSet::PostUpdate),
            );
    }
}

#[derive(Component, Clone, Default, Debug, Deref, DerefMut, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
pub struct Health(pub f32);

#[derive(Component, Clone, Default, Debug, Deref, DerefMut, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
pub struct MaxHealth(pub f32);

#[derive(Component, Clone, Debug, Reflect, FromReflect)]
#[reflect(Debug)]
pub struct DamageEvent {
    pub damaged_id: Entity,
    pub damage: f32,
}

fn handle_damage(mut healths: Query<&mut Health>, mut damage_events: EventReader<DamageEvent>) {
    for damage in damage_events.iter() {
        let Ok(mut health) = healths.get_mut(damage.damaged_id) else { return; };
        **health -= damage.damage;
    }
}

#[derive(Component, Clone, Debug, Reflect, FromReflect)]
#[reflect(Debug)]
pub struct DeathEvent {
    pub died_id: Entity,
}

fn handle_death(
    mut death_events: EventWriter<DeathEvent>,
    changed_healths: Query<(Entity, &Health), Changed<Health>>,
) {
    for (id, health) in &changed_healths {
        if **health < 0.0 {
            death_events.send(DeathEvent { died_id: id });
        }
    }
}
