use bevy::prelude::*;

use crate::{game_state::GameState, menu::menu::MenuState};

#[derive(Component)]
pub struct PauseOverlay;

#[derive(Resource, Default)]
pub struct PauseState {
    pub is_paused: bool,
}

#[derive(Resource, Default)]
pub struct EscButtonState {
    pub progress: u8,
}

#[derive(Component)]
pub struct EscButton;

fn get_border_for_state(state: u8) -> UiRect {
    match state {
        0 => UiRect::all(Val::Px(0.0)),
        1 => UiRect {
            top: Val::Px(3.0),
            right: Val::Px(0.0),
            bottom: Val::Px(0.0),
            left: Val::Px(0.0),
        },
        2 => UiRect {
            top: Val::Px(3.0),
            right: Val::Px(3.0),
            bottom: Val::Px(0.0),
            left: Val::Px(0.0),
        },
        3 => UiRect {
            top: Val::Px(3.0),
            right: Val::Px(3.0),
            bottom: Val::Px(3.0),
            left: Val::Px(0.0),
        },
        4 => UiRect {
            top: Val::Px(3.0),
            right: Val::Px(3.0),
            bottom: Val::Px(3.0),
            left: Val::Px(3.0),
        },
        5 => UiRect::all(Val::Px(4.0)),
        _ => UiRect::all(Val::Px(0.0)),
    }
}

pub fn spawn_pause_ui(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            PauseOverlay,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("PAUSED"),
                TextFont {
                    font_size: 100.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            parent
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        bottom: Val::Px(30.0),
                        right: Val::Px(30.0),
                        width: Val::Px(80.0),
                        height: Val::Px(50.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: get_border_for_state(0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 0.9)),
                    BorderColor(Color::WHITE),
                    BorderRadius::all(Val::Px(8.0)),
                    EscButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("ESC"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

pub fn update_esc_button_border(
    esc_state: Res<EscButtonState>,
    mut query: Query<&mut Node, With<EscButton>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut pause_state: ResMut<PauseState>,
) {
    if esc_state.is_changed() {
        if esc_state.progress == 6 {
            game_state.set(GameState::Menu);
            menu_state.set(MenuState::Main);
            pause_state.is_paused = false;
            return;
        }
        for mut node in query.iter_mut() {
            node.border = get_border_for_state(esc_state.progress);
        }
    }
}

pub fn despawn_pause_ui(commands: &mut Commands, query: Query<Entity, With<PauseOverlay>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
