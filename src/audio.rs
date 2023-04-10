use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_kira_audio::AudioPlugin)
            .init_resource::<Volume>()
            .add_system(set_volume);
    }
}

#[derive(Resource)]
struct Volume(f64);

impl FromWorld for Volume {
    fn from_world(world: &mut World) -> Self {
        world.resource_mut::<Audio>().set_volume(1.0);
        Self(1.0)
    }
}

fn set_volume(keys: Res<Input<KeyCode>>, audio: Res<Audio>, mut volume: ResMut<Volume>) {
    if keys.just_pressed(KeyCode::Plus) {
        volume.0 += 0.05;
        audio.set_volume(volume.0);
    } else if keys.just_pressed(KeyCode::Minus) {
        volume.0 -= 0.05;
        audio.set_volume(volume.0);
    }
}
