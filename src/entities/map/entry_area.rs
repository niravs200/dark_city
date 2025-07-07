use crate::{
    constants::map::{GROUND_MATERIAL_COLOR, WALL_MATERIAL_COLOR},
    entities::map::{WallOrientation, map::MapEntity, spawn_wall_with_hole},
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn setup_entry_area(
    mut commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let ground_size = 30.0;
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

    let wall_thickness = 1.0;
    let wall_height = 10.0;

    let mut wall_material = materials.add(WALL_MATERIAL_COLOR);

    let north_wall_mesh = meshes.add(Cuboid::new(ground_size * 2.0, wall_height, wall_thickness));
    commands.spawn((
        Mesh3d(north_wall_mesh),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_xyz(0.0, wall_height / 2.0, ground_size),
        GlobalTransform::default(),
        Collider::cuboid(ground_size, wall_height / 2.0, wall_thickness / 2.0),
        MapEntity,
    ));

    let south_wall_mesh = meshes.add(Cuboid::new(ground_size * 2.0, wall_height, wall_thickness));
    commands.spawn((
        Mesh3d(south_wall_mesh),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_xyz(0.0, wall_height / 2.0, -ground_size),
        GlobalTransform::default(),
        Collider::cuboid(ground_size, wall_height / 2.0, wall_thickness / 2.0),
        MapEntity,
    ));

    spawn_wall_with_hole(
        &mut commands,
        meshes,
        &mut wall_material,
        wall_thickness,
        wall_height,
        3.0,
        ground_size,
        Vec3::new(0.0, 0.0, 0.0),
        10.0,
        WallOrientation::AlongZ,
    );

    let west_wall_mesh = meshes.add(Cuboid::new(wall_thickness, wall_height, ground_size * 2.0));
    commands.spawn((
        Mesh3d(west_wall_mesh),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_xyz(-ground_size, wall_height / 2.0, 0.0),
        GlobalTransform::default(),
        Collider::cuboid(wall_thickness / 2.0, wall_height / 2.0, ground_size),
        MapEntity,
    ));

    commands.spawn((
        DirectionalLight { ..default() },
        Transform::from_xyz(10.0, 20.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
        MapEntity,
    ));
}
