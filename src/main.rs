#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use abilities::AbilitiesPlugin;
use assets::AssetsPlugin;
use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};
use bevy_kira_audio::AudioPlugin;
use bevy_rapier2d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
use bevy_turborand::RngPlugin;
use enemy::EnemyPlugin;
use explosion::ExplosionPlugin;
use health::HealthPlugin;
use healthbar::HealthBarPlugin;
use init::InitPlugin;
use lifetime::LifetimePlugin;
use mouse_position::MousePositionPlugin;
use player::PlayerPlugin;
use room::RoomPlugin;
use room_manager::RoomManagerPlugin;
use state::GlobalStatePlugin;

pub mod abilities;
pub mod assets;
pub mod enemy;
pub mod explosion;
pub mod health;
pub mod healthbar;
pub mod init;
pub mod lifetime;
pub mod mouse_position;
pub mod player;
pub mod room;
pub mod room_manager;
pub mod state;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        #[cfg(not(target_family = "wasm"))]
                        present_mode: PresentMode::AutoNoVsync,
                        #[cfg(not(target_family = "wasm"))]
                        mode: WindowMode::BorderlessFullscreen,
                        title: "Temp Name".into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_linear()),
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .add_plugin(GlobalStatePlugin)
        .add_plugin(RngPlugin::default())
        .add_plugin(AudioPlugin)
        .add_plugin(InitPlugin)
        .add_plugin(AssetsPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(RoomPlugin)
        .add_plugin(AbilitiesPlugin)
        .add_plugin(HealthPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(ExplosionPlugin)
        .add_plugin(MousePositionPlugin)
        .add_plugin(LifetimePlugin)
        .add_plugin(HealthBarPlugin)
        .add_plugin(RoomManagerPlugin)
        .run();
}
