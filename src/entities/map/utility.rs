use crate::entities::map::map::MapEntity;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub enum WallOrientation {
    AlongX,
    AlongZ,
}

pub fn spawn_wall_with_hole(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    wall_material: &Handle<StandardMaterial>,
    wall_thickness: f32,
    wall_height: f32,
    hole_radius: f32,
    ground_size: f32,
    offset: Vec3,
    extension: f32,
    orientation: WallOrientation,
) {
    let extended_size = ground_size + extension;

    let hole_h = hole_radius * 2.0;
    let top_h = wall_height - hole_h;
    let top_y = hole_h + top_h / 2.0;

    match orientation {
        WallOrientation::AlongZ => {
            let top_mesh = meshes.add(Cuboid::new(wall_thickness, top_h, extended_size * 2.0));
            commands.spawn((
                Mesh3d(top_mesh),
                MeshMaterial3d(wall_material.clone()),
                Transform::from_xyz(ground_size + offset.x, top_y + offset.y, offset.z),
                GlobalTransform::default(),
                Collider::cuboid(wall_thickness / 2.0, top_h / 2.0, extended_size),
                MapEntity,
            ));

            let side_w = extended_size - hole_radius;
            let side_z = (extended_size + hole_radius) / 2.0;

            let mut spawn_side = |z_center: f32| {
                let mesh = meshes.add(Cuboid::new(wall_thickness, wall_height, side_w));
                commands.spawn((
                    Mesh3d(mesh),
                    MeshMaterial3d(wall_material.clone()),
                    Transform::from_xyz(
                        ground_size + offset.x,
                        wall_height / 2.0 + offset.y,
                        z_center + offset.z,
                    ),
                    GlobalTransform::default(),
                    Collider::cuboid(wall_thickness / 2.0, wall_height / 2.0, side_w / 2.0),
                    MapEntity,
                ));
            };

            spawn_side(-side_z);
            spawn_side(side_z);
        }

        WallOrientation::AlongX => {
            let top_mesh = meshes.add(Cuboid::new(extended_size * 2.0, top_h, wall_thickness));
            commands.spawn((
                Mesh3d(top_mesh),
                MeshMaterial3d(wall_material.clone()),
                Transform::from_xyz(offset.x, top_y + offset.y, ground_size + offset.z),
                GlobalTransform::default(),
                Collider::cuboid(extended_size, top_h / 2.0, wall_thickness / 2.0),
                MapEntity,
            ));

            let side_w = extended_size - hole_radius;
            let side_x = (extended_size + hole_radius) / 2.0;

            let mut spawn_side = |x_center: f32| {
                let mesh = meshes.add(Cuboid::new(side_w, wall_height, wall_thickness));
                commands.spawn((
                    Mesh3d(mesh),
                    MeshMaterial3d(wall_material.clone()),
                    Transform::from_xyz(
                        x_center + offset.x,
                        wall_height / 2.0 + offset.y,
                        ground_size + offset.z,
                    ),
                    GlobalTransform::default(),
                    Collider::cuboid(side_w / 2.0, wall_height / 2.0, wall_thickness / 2.0),
                    MapEntity,
                ));
            };

            spawn_side(-side_x);
            spawn_side(side_x);
        }
    }
}
