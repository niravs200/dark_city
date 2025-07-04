use bevy::prelude::*;

#[derive(Component)]
pub struct PauseOverlay;

#[derive(Resource, Default)]
pub struct PauseState {
    pub is_paused: bool,
}

pub fn spawn_pause_ui(mut commands: Commands) {
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
        });
}

pub fn despawn_pause_ui(mut commands: Commands, query: Query<Entity, With<PauseOverlay>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
