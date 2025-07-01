use super::{GameState, despawn_screen::despawn_screen};
use crate::menu::{MenuAssets, are_menu_assets_loaded, load_menu_assets};
use bevy::prelude::*;

pub fn splash_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Splash), splash_setup)
        .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
        .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
}

#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let icon = asset_server.load("branding/icon.png");

    load_menu_assets(&mut commands, asset_server);

    commands.spawn((
        Node {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        OnSplashScreen,
        children![(
            ImageNode::new(icon),
            Node {
                width: Val::Percent(60.0),
                height: Val::Percent(60.0),
                ..default()
            }
        )],
    ));

    commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
    menu_assets: Res<MenuAssets>,
    asset_server: Res<AssetServer>,
) {
    let all_loaded = are_menu_assets_loaded(menu_assets, asset_server);
    if all_loaded && timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }
}
