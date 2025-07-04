use bevy::{
    core_pipeline::core_2d::Camera2d,
    ecs::{
        entity::Entity,
        query::With,
        system::{Commands, Query},
    },
};

pub fn camera2d_spawn(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn camera2d_despawn(mut commands: Commands, query: Query<Entity, With<Camera2d>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
