use bevy::{asset::LoadState, prelude::*};

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
    pub bevy_icon: Handle<Image>,
}

fn start_loading_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        bevy_icon: asset_server.load("icon.png"),
    });
}

fn on_assets_loaded(
    asset_server: Res<AssetServer>,
    assets: Res<GameAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if asset_server.get_load_state(assets.bevy_icon.clone()) == LoadState::Loaded {
        next_state.set(GameState::Playing);
        info!("Loaded all assets");
    }
}