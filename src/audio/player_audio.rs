use bevy::{
    asset::{AssetServer, Handle},
    ecs::{
        resource::Resource,
        system::{Commands, Res},
    },
};
use bevy_kira_audio::prelude::*;

#[derive(Resource, Debug)]
pub struct PlayerSoundLibrary {
    pub walk: Handle<AudioSource>,
    pub run: Handle<AudioSource>,
    pub sword_strike: Handle<AudioSource>,
}

pub fn load_player_audio(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let walk_audio_asset: Handle<AudioSource> = asset_server.load("audio/walking.wav");
    let run_audio_asset: Handle<AudioSource> = asset_server.load("audio/running.wav");
    let sword_strike_audio_asset: Handle<AudioSource> = asset_server.load("audio/sword_strike.wav");
    commands.insert_resource(PlayerSoundLibrary {
        walk: walk_audio_asset,
        run: run_audio_asset,
        sword_strike: sword_strike_audio_asset,
    });
}
