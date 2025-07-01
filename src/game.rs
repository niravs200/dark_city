use bevy::prelude::*;

use crate::menu::{MenuAssets, are_menu_assets_loaded, load_menu_assets};

use super::{TEXT_COLOR, despawn_screen::despawn_screen, game_state::GameState};

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), game_setup)
        .add_systems(Update, game.run_if(in_state(GameState::Game)))
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
}

#[derive(Component)]
struct OnGameScreen;

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    load_menu_assets(&mut commands, asset_server);

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        OnGameScreen,
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::BLACK),
            children![(
                Text::new("Game Started. Will finish in 5 sec"),
                TextFont {
                    font_size: 67.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
                Node {
                    margin: UiRect::all(Val::Px(50.0)),
                    ..default()
                },
            )]
        )],
    ));

    commands.insert_resource(GameTimer(Timer::from_seconds(5.0, TimerMode::Once)));
}

fn game(
    time: Res<Time>,
    mut game_state: ResMut<NextState<GameState>>,
    mut timer: ResMut<GameTimer>,
    menu_assets: Res<MenuAssets>,
    asset_server: Res<AssetServer>,
) {
    let all_loaded = are_menu_assets_loaded(menu_assets, asset_server);
    if all_loaded && timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu)
    }
}
