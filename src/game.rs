use bevy::input::InputSystem;
use bevy::prelude::*;

use super::game_state::GameState;
use crate::entities::map::map::MapEntity;
use crate::entities::map::{despawn_map, setup_map};
use crate::menu::load_menu_assets;
use crate::player::player::Player;
use crate::player::{
    LookInput, MovementInput, despawn_player, handle_input, player_look, player_movement,
    setup_player,
};
use crate::ui::cross_hair::Crosshair;
use crate::ui::hud::{HUD, cleanup_hud, setup_hud};
use crate::ui::{
    EscButtonState, PauseOverlay, PauseState, despawn_crosshair, despawn_pause_ui, hide_cursor,
    show_cursor, spawn_crosshair, update_esc_button_border,
};

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
        .add_systems(
            Update,
            update_esc_button_border.run_if(in_state(GameState::Game).and(paused)),
        )
        .add_systems(OnExit(GameState::Game), game_cleanup);
}

pub fn not_paused(pause_state: Res<PauseState>) -> bool {
    !pause_state.is_paused
}

pub fn paused(pause_state: Res<PauseState>) -> bool {
    pause_state.is_paused
}

fn game_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    windows: Query<&mut Window>,
) {
    load_menu_assets(&mut commands, &asset_server);

    let camera_entity = setup_player(&mut commands);
    setup_map(&mut commands, &mut meshes, &mut materials, &asset_server);
    setup_hud(&mut commands, &windows);
    hide_cursor(windows);
    spawn_crosshair(&mut commands, &asset_server, camera_entity);
}

fn game_cleanup(
    mut commands: Commands,
    query_map: Query<Entity, With<MapEntity>>,
    query_player: Query<Entity, With<Player>>,
    windows: Query<&mut Window>,
    crosshair_q: Query<Entity, With<Crosshair>>,
    pause_query: Query<Entity, With<PauseOverlay>>,
    hud_query: Query<Entity, With<HUD>>,
) {
    despawn_map(&mut commands, query_map);
    despawn_crosshair(&mut commands, crosshair_q);
    despawn_player(&mut commands, query_player);
    despawn_pause_ui(&mut commands, pause_query);
    show_cursor(windows);
    cleanup_hud(commands, hud_query);
}
