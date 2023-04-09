use bevy::prelude::*;

use crate::{assets::GameAssets, player::CurrentAbility, state::GameState};

pub struct AbilityUiPlugin;

impl Plugin for AbilityUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_ability_ui.in_schedule(OnEnter(GameState::Playing)))
            .add_system(move_highlight.in_set(OnUpdate(GameState::Playing)))
            .add_system(despawn_ability_ui.in_schedule(OnExit(GameState::Playing)));
    }
}

#[derive(Component, Default, Debug)]
pub struct AbilityUi;

#[derive(Component, Default, Debug)]
struct AbilityUiHighlight;

#[derive(Resource)]
struct AbilitySlots {
    pub slots: [Entity; 4],
}

fn spawn_ability_ui(mut commands: Commands, assets: Res<GameAssets>) {
    let style = Style {
        size: Size::new(Val::Px(64.0), Val::Px(64.0)),
        ..Default::default()
    };
    let slots = [
        commands
            .spawn(ImageBundle {
                style: style.clone(),
                image: UiImage::new(assets.teleport_slot.clone()),
                ..Default::default()
            })
            .id(),
        commands
            .spawn(ImageBundle {
                style: style.clone(),
                image: UiImage::new(assets.fireball_slot.clone()),
                ..Default::default()
            })
            .id(),
        commands
            .spawn(ImageBundle {
                style: style.clone(),
                image: UiImage::new(assets.ice_wall_slot.clone()),
                ..Default::default()
            })
            .id(),
        commands
            .spawn(ImageBundle {
                style,
                image: UiImage::new(assets.shotgun_slot.clone()),
                ..Default::default()
            })
            .id(),
    ];
    commands
        .spawn((
            AbilityUi,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    margin: UiRect::new(Val::Px(20.0), Val::Auto, Val::Px(20.0), Val::Auto),
                    gap: Size::new(Val::Px(8.0), Val::Auto),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .push_children(&slots);

    commands.insert_resource(AbilitySlots { slots });

    commands.spawn((
        AbilityUiHighlight,
        NodeBundle {
            style: Style {
                position: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(68.0), Val::Auto),
                size: Size::new(Val::Px(64.0), Val::Px(4.0)),
                ..Default::default()
            },
            background_color: Color::WHITE.into(),
            ..Default::default()
        },
    ));
}

fn despawn_ability_ui(
    mut commands: Commands,
    query: Query<Entity, Or<(With<AbilityUi>, With<AbilityUiHighlight>)>>,
) {
    for e in &query {
        commands.entity(e).despawn_recursive();
    }
}

fn move_highlight(
    mut commands: Commands,
    highlight: Query<Entity, With<AbilityUiHighlight>>,
    selected_ability: Query<&CurrentAbility, Changed<CurrentAbility>>,
    slots: Res<AbilitySlots>,
) {
    for ability in &selected_ability {
        let id = highlight.single();
        commands.entity(id).remove_parent();
        commands.entity(slots.slots[ability.0]).add_child(id);
    }
}
