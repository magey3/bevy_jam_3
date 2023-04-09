use bevy::prelude::*;

use crate::{assets::GameAssets, state::GameState};

pub struct DeathScreenPlugin;

impl Plugin for DeathScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(show_deathscreen.in_schedule(OnEnter(GameState::DeathScreen)));
    }
}

#[derive(Component, Default, Debug)]
struct DeathScreen;

fn show_deathscreen(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        DeathScreen,
        TextBundle {
            text: Text::from_section(
                "You have died!",
                TextStyle {
                    font: assets.font_italic.clone(),
                    font_size: 120.0,
                    color: Color::WHITE,
                },
            ),
            style: Style {
                position_type: PositionType::Absolute,
                margin: UiRect::all(Val::Auto),
                ..Default::default()
            },
            z_index: ZIndex::Global(1),
            ..Default::default()
        },
    ));
}
