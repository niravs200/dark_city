use bevy::prelude::*;

use crate::entities::map::{setup_entry_area, setup_tutorial_area};

#[derive(Component)]
pub struct MapEntity;

pub fn setup_map(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    setup_entry_area(commands, meshes, materials);
    setup_tutorial_area(commands, meshes, materials, Vec3::new(70.0, 0.0, 0.0));
}

pub fn despawn_map(commands: &mut Commands, query: Query<Entity, With<MapEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
