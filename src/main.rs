use bevy::prelude::*;

mod constants;
mod despawn_screen;
mod entities;
mod game;
mod game_state;
mod menu;
mod player;

use bevy_rapier3d::plugin::{NoUserData, RapierPhysicsPlugin};
use game::game_plugin;
use game_state::GameState;
use menu::{menu_plugin, splash_plugin};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, RapierPhysicsPlugin::<NoUserData>::default()))
        .init_state::<GameState>()
        .add_plugins((splash_plugin, menu_plugin, game_plugin))
        .run();
}
