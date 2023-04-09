use bevy::{asset::LoadState, prelude::*};
use bevy_kira_audio::AudioSource;

use crate::state::GameState;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(start_loading_assets)
            .add_system(on_assets_loaded.run_if(in_state(GameState::Loading)));
    }
}

#[derive(Resource, Clone, Default, Debug, Reflect, FromReflect)]
#[reflect(Resource, Default, Debug)]
pub struct GameAssets {
    pub bomb: Handle<Image>,
    pub player: Handle<Image>,
    pub explosion: Handle<AudioSource>,
}

fn start_loading_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bomb: asset_server.load("bomb.png"),
        player: asset_server.load("rat.png"),
        explosion: asset_server.load("explosion.wav"),
    });
}

fn on_assets_loaded(
    asset_server: Res<AssetServer>,
    assets: Res<GameAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let assets = [assets.bomb.id(), assets.player.id(), assets.explosion.id()];
    if asset_server.get_group_load_state(assets) == LoadState::Loaded {
        next_state.set(GameState::Playing);
        info!("Loaded all assets");
    }
}
