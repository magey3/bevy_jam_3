use std::time::Duration;

use bevy::prelude::*;

use crate::{assets::GameAssets, state::GameState};

pub struct DeathScreenPlugin;

impl Plugin for DeathScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(show_deathscreen.in_schedule(OnEnter(GameState::DeathScreen)))
            .add_system(exit_deathscreen.in_set(OnUpdate(GameState::DeathScreen)))
            .add_system(cleanup.in_schedule(OnExit(GameState::DeathScreen)));
    }
}

#[derive(Component, Default, Debug)]
struct DeathScreen;

#[derive(Component, Default, Debug)]
struct DeathScreenTimer(pub Timer);

fn show_deathscreen(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        DeathScreen,
        DeathScreenTimer(Timer::new(Duration::from_secs(3), TimerMode::Once)),
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

fn exit_deathscreen(
    mut timers: Query<&mut DeathScreenTimer>,
    time: Res<Time>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for mut timer in &mut timers {
        if timer.0.tick(time.delta()).just_finished() {
            next_state.set(GameState::MainMenu);
        }
    }
}

fn cleanup(mut commands: Commands, death_screen: Query<Entity, With<DeathScreen>>) {
    for e in &death_screen {
        commands.entity(e).despawn_recursive();
    }
}
