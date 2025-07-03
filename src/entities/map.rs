use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::constants::map::{GROUND_MATERIAL_COLOR, STAIRS_MATERIAL_COLOR};

#[derive(Component)]
pub struct MapEntity;

pub fn setup_map(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ground_size = 50.0;
    let ground_height = 0.1;

    let ground_mesh = meshes.add(Cuboid::new(
        ground_size * 2.0,
        ground_height * 2.0,
        ground_size * 2.0,
    ));
    let ground_material = materials.add(GROUND_MATERIAL_COLOR);

    commands.spawn((
        Mesh3d(ground_mesh.clone()),
        MeshMaterial3d(ground_material.clone()),
        Transform::from_xyz(0.0, -ground_height, 0.0),
        GlobalTransform::default(),
        Collider::cuboid(ground_size, ground_height, ground_size),
        MapEntity,
    ));

    let stair_len = 30;
    let stair_step = 0.2;

    let stair_size_xz = 2.0;

    for i in 1..=stair_len {
        let step = i as f32;
        let height = step * stair_step;

        let stair_mesh = meshes.add(Cuboid::new(stair_size_xz, height, stair_size_xz));
        let stair_material = materials.add(STAIRS_MATERIAL_COLOR);

        let collider = Collider::cuboid(stair_size_xz / 2.0, height, stair_size_xz / 2.0);

        let y = height;
        let z = step * 2.0 - 20.0;
        let z_neg = step * -2.0 + 20.0;
        let x = step * 2.0 - 20.0;
        let x_neg = step * -2.0 + 20.0;

        for (x_pos, z_pos) in [(40.0, z), (-40.0, z_neg), (x, 40.0), (x_neg, -40.0)] {
            commands.spawn((
                Mesh3d(stair_mesh.clone()),
                MeshMaterial3d(stair_material.clone()),
                Transform::from_xyz(x_pos, y, z_pos),
                GlobalTransform::default(),
                collider.clone(),
                MapEntity,
            ));
        }
    }

    commands.spawn((
        DirectionalLight { ..default() },
        Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
        MapEntity,
    ));
}

pub fn despawn_map(mut commands: Commands, query: Query<Entity, With<MapEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
