use bevy::prelude::*;

#[derive(States, Clone, Copy, Default, Debug, Reflect, FromReflect, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    Playing,
    DeathScreen,
    MainMenu,
}

pub struct GlobalStatePlugin;

impl Plugin for GlobalStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().register_type::<GameState>();
    }
}
