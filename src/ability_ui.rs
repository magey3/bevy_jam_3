use bevy::prelude::*;

use crate::{
    abilities::{cooldown::AbilityCooldown, Loadout},
    assets::GameAssets,
    player::{CurrentAbility, Player},
    state::GameState,
};

pub struct AbilityUiPlugin;

impl Plugin for AbilityUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_ability_ui.in_schedule(OnEnter(GameState::Playing)))
            .add_systems((move_highlight, set_highlight_width).in_set(OnUpdate(GameState::Playing)))
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

const SELECTOR_WIDTH: f32 = 64.0;

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

    commands
        .spawn((
            AbilityUiHighlight,
            NodeBundle {
                style: Style {
                    position: UiRect::new(Val::Px(0.0), Val::Px(0.0), Val::Px(68.0), Val::Auto),
                    size: Size::new(Val::Px(SELECTOR_WIDTH), Val::Px(4.0)),
                    overflow: Overflow::Hidden,
                    ..Default::default()
                },

                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(SELECTOR_WIDTH), Val::Px(4.0)),
                    ..Default::default()
                },
                background_color: Color::WHITE.into(),
                ..Default::default()
            });
        });
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

fn set_highlight_width(
    loadout: Query<&Loadout, With<Player>>,
    mut highlight: Query<&mut Style, With<AbilityUiHighlight>>,
    selected_ability: Query<&CurrentAbility>,
    cooldown: Query<Option<&AbilityCooldown>>,
) {
    for ability in &selected_ability {
        let mut style = highlight.single_mut();

        let Ok(loadout) = loadout.get_single() else { continue; };
        let ability_id = loadout.abilities[ability.0];
        let cooldown = cooldown.get(ability_id).unwrap();

        if let Some(cooldown) = cooldown {
            info!("YEAH");
            style.size.width = Val::Px(SELECTOR_WIDTH * cooldown.percent());
        } else {
            style.size.width = Val::Px(SELECTOR_WIDTH);
        }
    }
}
