#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use abilities::AbilitiesPlugin;
use assets::AssetsPlugin;
use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};
use bevy_rapier2d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
use health::HealthPlugin;
use init::InitPlugin;
use player::PlayerPlugin;
use room::RoomPlugin;
use state::GlobalStatePlugin;

pub mod abilities;
pub mod assets;
pub mod health;
pub mod init;
pub mod player;
pub mod room;
pub mod state;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoNoVsync,
                        mode: WindowMode::BorderlessFullscreen,
                        title: "Temp Name".into(),
                        resizable: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..Default::default()
        })
        .add_plugin(GlobalStatePlugin)
        .add_plugin(InitPlugin)
        .add_plugin(AssetsPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(RoomPlugin)
        .add_plugin(AbilitiesPlugin)
        .add_plugin(HealthPlugin)
        .run();
}
