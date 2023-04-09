use bevy::prelude::*;

pub struct InitPlugin;

impl Plugin for InitPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MainCamera>()
            .add_startup_system(spawn_camera);
    }
}

#[derive(Component, Clone, Default, Debug, Reflect, FromReflect)]
#[reflect(Component, Default, Debug)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Camera2dBundle {
            projection: OrthographicProjection {
                #[cfg(target_family = "wasm")]
                scale: 1.0 / 2.6,
                #[cfg(not(target_family = "wasm"))]
                scale: 1.0 / 4.0,
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}
