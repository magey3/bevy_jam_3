use bevy::prelude::*;

use crate::init::MainCamera;

pub struct MousePositionPlugin;

impl Plugin for MousePositionPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MousePosition>()
            .init_resource::<MousePosition>()
            .add_system(my_cursor_system.in_base_set(CoreSet::PreUpdate));
    }
}

#[derive(Resource, Clone, Default, Debug, Deref, DerefMut, Reflect, FromReflect)]
#[reflect(Resource, Default, Debug)]
pub struct MousePosition(pub Vec2);

fn my_cursor_system(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut mouse_position: ResMut<MousePosition>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let Ok((camera, camera_transform)) = camera_q.get_single() else { return; };

    // get the window that the camera is displaying to (or the primary window)
    let window = windows.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        **mouse_position = world_position;
    }
}
