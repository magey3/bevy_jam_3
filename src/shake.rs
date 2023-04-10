use bevy::prelude::*;
use bevy_turborand::{DelegatedRng, GlobalRng};

pub struct ShakePlugin;

impl Plugin for ShakePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Shake>().add_system(shake);
    }
}

#[derive(Component, Clone, Default, Debug, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
pub struct Shake {
    pub amount: f32,
    pub max_translation: f32,
    pub max_rotation: f32,
}

fn shake(
    mut object: Query<(&mut Transform, &mut Shake)>,
    mut rng: ResMut<GlobalRng>,
    time: Res<Time>,
) {
    for (mut transform, mut shake) in &mut object {
        if shake.amount <= 0.0 {
            transform.translation.x = 0.0;
            transform.translation.y = 0.0;
            transform.rotation = Quat::default();
            continue;
        }

        let offset = (Vec2::new(rng.f32_normalized(), rng.f32_normalized()) * shake.amount.powi(2))
            .clamp_length_max(shake.max_translation);

        transform.translation.x = offset.x;
        transform.translation.y = offset.y;

        transform.rotation = Quat::from_euler(
            EulerRot::XYZ,
            0.0,
            0.0,
            (rng.f32_normalized() * shake.amount.powi(2))
                .clamp(-shake.max_rotation, shake.max_rotation),
        );

        shake.amount -= 4.0 * time.delta_seconds();
    }
}
