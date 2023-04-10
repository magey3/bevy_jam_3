use bevy::prelude::*;

use crate::{abilities::heat::Heat, player::Player, state::GameState};

pub struct HeatUiPlugin;

impl Plugin for HeatUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_heat_ui.in_schedule(OnEnter(GameState::Playing)))
            .add_system(set_slider.in_set(OnUpdate(GameState::Playing)))
            .add_system(despawn_heat_ui.in_schedule(OnExit(GameState::Playing)));
    }
}

#[derive(Component, Default, Debug)]
pub struct HeatUi;

#[derive(Component, Default, Debug)]
pub struct HeatUiSlider;

fn spawn_heat_ui(mut commands: Commands) {
    commands
        .spawn((
            HeatUi,
            NodeBundle {
                style: Style {
                    position: UiRect::new(Val::Auto, Val::Px(20.0), Val::Px(20.0), Val::Auto),
                    position_type: PositionType::Absolute,
                    padding: UiRect::all(Val::Px(10.0)),
                    size: Size::new(Val::Px(400.0), Val::Px(64.0)),
                    ..Default::default()
                },
                background_color: Color::GRAY.into(),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    HeatUiSlider,
                    NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            overflow: Overflow::Hidden,
                            ..Default::default()
                        },

                        ..Default::default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Px(400.0), Val::Px(64.0)),
                            ..Default::default()
                        },
                        background_color: Color::ORANGE_RED.into(),
                        ..Default::default()
                    });
                });
        });
}

fn despawn_heat_ui(mut commands: Commands, query: Query<Entity, With<HeatUi>>) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}

fn set_slider(
    mut sliders: Query<&mut Style, With<HeatUiSlider>>,
    player_heat: Query<&Heat, With<Player>>,
) {
    for mut style in &mut sliders {
        let Ok(heat) = player_heat.get_single() else { return; };

        style.size.width = Val::Percent(heat.clamp(0.0, 100.0));
    }
}
