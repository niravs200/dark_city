use bevy::input::InputSystem;
use bevy::prelude::*;

use crate::entities::map::MapEntity;
use crate::entities::{despawn_map, setup_map};
use crate::menu::{MenuAssets, are_menu_assets_loaded, load_menu_assets};
use crate::player::player::Player;
use crate::player::{
    LookInput, MovementInput, despawn_player, handle_input, player_look, player_movement,
    setup_player,
};
use crate::ui::cross_hair::Crosshair;
use crate::ui::{
    EscButtonState, PauseState, despawn_crosshair, hide_cursor, show_cursor, spawn_crosshair,
    update_esc_button_border,
};

use super::game_state::GameState;

pub fn game_plugin(app: &mut App) {
    app.init_resource::<MovementInput>()
        .init_resource::<LookInput>()
        .init_resource::<PauseState>()
        .init_resource::<EscButtonState>()
        .add_systems(OnEnter(GameState::Game), game_setup)
        .add_systems(
            PreUpdate,
            handle_input
                .after(InputSystem)
                .run_if(in_state(GameState::Game)),
        )
        .add_systems(Update, player_look.run_if(in_state(GameState::Game)))
        .add_systems(
            Update,
            player_movement.run_if(in_state(GameState::Game).and(not_paused)),
        )
        .add_systems(Update, game.run_if(in_state(GameState::Game)))
        .add_systems(
            Update,
            update_esc_button_border.run_if(in_state(GameState::Game).and(paused)),
        )
        .add_systems(OnExit(GameState::Game), game_cleanup);
}

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

pub fn not_paused(pause_state: Res<PauseState>) -> bool {
    !pause_state.is_paused
}

pub fn paused(pause_state: Res<PauseState>) -> bool {
    pause_state.is_paused
}

fn game_setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    windows: Query<&mut Window>,
) {
    load_menu_assets(&mut commands, &asset_server);

    let camera_entity = setup_player(&mut commands);
    setup_map(&mut commands, meshes, materials);
    hide_cursor(windows);
    spawn_crosshair(&mut commands, &asset_server, camera_entity);

    commands.insert_resource(GameTimer(Timer::from_seconds(10.0, TimerMode::Once)));
}

fn game_cleanup(
    mut commands: Commands,
    query_map: Query<Entity, With<MapEntity>>,
    query_player: Query<Entity, With<Player>>,
    windows: Query<&mut Window>,
    crosshair_q: Query<Entity, With<Crosshair>>,
) {
    despawn_map(&mut commands, query_map);
    despawn_crosshair(&mut commands, crosshair_q);
    despawn_player(&mut commands, query_player);
    show_cursor(windows);
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
