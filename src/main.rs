use bevy::prelude::*;

mod despawn_screen;
mod game;
mod game_state;
mod menu;
mod menu_cloud;
mod splash;

use game::game_plugin;
use game_state::GameState;
use menu::menu_plugin;
use splash::splash_plugin;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((splash::splash_plugin, menu::menu_plugin, game::game_plugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}
