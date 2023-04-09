use bevy::{app::AppExit, prelude::*};
use leafwing_input_manager::{
    prelude::{ActionState, ActionStateDriver, InputManagerPlugin, InputMap},
    Actionlike, InputManagerBundle,
};

use crate::{assets::GameAssets, state::GameState};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<MainMenuActions>::default())
            .add_system(spawn_menu.in_schedule(OnEnter(GameState::MainMenu)))
            .add_systems((handle_actions, highlight).in_set(OnUpdate(GameState::MainMenu)))
            .add_system(despawn_menu.in_schedule(OnExit(GameState::MainMenu)));
    }
}

#[derive(Component, Default, Debug)]
struct MainMenuRoot;

#[derive(Actionlike, Component, PartialEq, Eq, Clone, Copy, Hash, Debug)]
enum MainMenuActions {
    Play,
    Exit,
}

fn spawn_menu(mut commands: Commands, assets: Res<GameAssets>) {
    commands
        .spawn((
            MainMenuRoot,
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    gap: Size::new(Val::Auto, Val::Px(20.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
            InputManagerBundle::<MainMenuActions> {
                input_map: InputMap::default(),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            let parent_id = parent.parent_entity();
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            size: Size::new(Val::Px(200.0), Val::Px(50.0)),
                            ..Default::default()
                        },
                        background_color: Color::DARK_GRAY.into(),
                        ..Default::default()
                    },
                    ActionStateDriver {
                        action: MainMenuActions::Play,
                        entity: parent_id,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Play",
                            TextStyle {
                                font: assets.font_normal.clone(),
                                font_size: 48.0,
                                color: Color::WHITE,
                            },
                        ),
                        ..Default::default()
                    });
                });
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            size: Size::new(Val::Px(200.0), Val::Px(50.0)),
                            ..Default::default()
                        },
                        background_color: Color::DARK_GRAY.into(),
                        ..Default::default()
                    },
                    ActionStateDriver {
                        action: MainMenuActions::Exit,
                        entity: parent_id,
                    },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Exit",
                            TextStyle {
                                font: assets.font_normal.clone(),
                                font_size: 48.0,
                                color: Color::WHITE,
                            },
                        ),
                        ..Default::default()
                    });
                });
        });
}

fn handle_actions(
    input: Query<&ActionState<MainMenuActions>>,
    mut exit_events: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for input in &input {
        if input.just_pressed(MainMenuActions::Play) {
            next_state.set(GameState::Playing);
        }
        if input.just_pressed(MainMenuActions::Exit) {
            exit_events.send_default();
        }
    }
}

fn despawn_menu(mut commands: Commands, menus: Query<Entity, With<MainMenuRoot>>) {
    for e in &menus {
        commands.entity(e).despawn_recursive();
    }
}

fn highlight(
    mut buttons: Query<(&mut BackgroundColor, &Interaction), (With<Button>, Changed<Interaction>)>,
) {
    for (mut color, interaction) in &mut buttons {
        if let Interaction::Hovered = *interaction {
            *color = BackgroundColor(Color::GRAY);
        } else {
            *color = BackgroundColor(Color::DARK_GRAY);
        }
    }
}
