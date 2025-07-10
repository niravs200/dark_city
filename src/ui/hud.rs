use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query},
    },
    prelude::*,
    text::{JustifyText, TextFont, TextLayout},
    ui::{PositionType, TextShadow, Val, widget::Text},
    utils::default,
};

use crate::constants::hud::DEFAULT_ROOM_NAME;

#[derive(Component)]
pub struct HUD;

#[derive(Component)]
pub struct RoomNameDisplay;

fn setup_room_display(commands: &mut Commands, windows: &Query<&mut Window>) {
    let window = windows.single().unwrap();
    let font_size = window.height() * 0.03;

    commands.spawn((
        Text::new(DEFAULT_ROOM_NAME),
        TextFont {
            font: default(),
            font_size: font_size,
            ..default()
        },
        TextShadow::default(),
        TextLayout::new_with_justify(JustifyText::Center),
        TextColor(Color::srgba(0.7, 0.7, 0.7, 0.6)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(80.0),
            left: Val::Percent(45.0),
            ..default()
        },
        HUD,
        RoomNameDisplay,
    ));
}

pub fn update_room_display_text(
    new_text: Option<&str>,
    mut room_name_q: Query<&mut Text, With<RoomNameDisplay>>,
) {
    if let Ok(mut text) = room_name_q.single_mut() {
        let new_value = new_text.unwrap_or("");
        if text.0 != new_value {
            text.0 = new_value.to_string();
        }
    }
}

pub fn setup_hud(commands: &mut Commands, windows: &Query<&mut Window>) {
    setup_room_display(commands, windows);
}

pub fn cleanup_hud(mut commands: Commands, hud_query: Query<Entity, With<HUD>>) {
    for entity in &hud_query {
        commands.entity(entity).despawn();
    }
}
