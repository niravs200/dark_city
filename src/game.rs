use bevy::input::InputSystem;
use bevy::prelude::*;

use crate::entities::{despawn_map, setup_map};
use crate::menu::{MenuAssets, are_menu_assets_loaded, load_menu_assets};
use crate::player::{
    LookInput, MovementInput, despawn_player, handle_input, player_look, player_movement,
    setup_player,
};

use super::game_state::GameState;

pub fn game_plugin(app: &mut App) {
    app.init_resource::<MovementInput>()
        .init_resource::<LookInput>()
        .add_systems(OnEnter(GameState::Game), game_setup)
        .add_systems(
            PreUpdate,
            handle_input
                .after(InputSystem)
                .run_if(in_state(GameState::Game)),
        )
        .add_systems(Update, player_look.run_if(in_state(GameState::Game)))
        .add_systems(Update, player_movement.run_if(in_state(GameState::Game)))
        .add_systems(Update, game.run_if(in_state(GameState::Game)))
        .add_systems(OnExit(GameState::Game), despawn_map)
        .add_systems(OnExit(GameState::Game), despawn_player);
}

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

fn game_setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    load_menu_assets(&mut commands, asset_server);

    setup_player(&mut commands);
    setup_map(&mut commands, meshes, materials);

    commands.insert_resource(GameTimer(Timer::from_seconds(120.0, TimerMode::Once)));
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
