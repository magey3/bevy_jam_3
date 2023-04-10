#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use abilities::AbilitiesPlugin;
use ability_ui::AbilityUiPlugin;
use assets::AssetsPlugin;
use audio::AudioPlugin;
use bevy::{
    prelude::*,
    window::{PresentMode, WindowMode},
};
use bevy_rapier2d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
use bevy_turborand::RngPlugin;
use death_screen::DeathScreenPlugin;
use enemy::EnemyPlugin;
use explosion::ExplosionPlugin;
use health::HealthPlugin;
use healthbar::HealthBarPlugin;
use heat_ui::HeatUiPlugin;
use init::InitPlugin;
use lifetime::LifetimePlugin;
use main_menu::MainMenuPlugin;
use mouse_position::MousePositionPlugin;
use player::PlayerPlugin;
use room::RoomPlugin;
use room_manager::RoomManagerPlugin;
use shake::ShakePlugin;
use state::GlobalStatePlugin;

pub mod abilities;
pub mod ability_ui;
pub mod assets;
pub mod audio;
pub mod death_screen;
pub mod enemy;
pub mod explosion;
pub mod health;
pub mod healthbar;
pub mod heat_ui;
pub mod init;
pub mod lifetime;
pub mod main_menu;
pub mod mouse_position;
pub mod player;
pub mod room;
pub mod room_manager;
pub mod shake;
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
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
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
        .add_plugin(DeathScreenPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(AbilityUiPlugin)
        .add_plugin(ShakePlugin)
        .add_plugin(HeatUiPlugin)
        .add_plugin(AudioPlugin)
        .run();
}
