use bevy::prelude::*;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Health>();
    }
}

#[derive(Component, Clone, Default, Debug, Deref, DerefMut, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
pub struct Health(pub f32);

// TODO: Handle death
