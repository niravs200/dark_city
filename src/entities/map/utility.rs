use std::collections::HashSet;

use crate::{
    constants::map::{
        BASE_ROOM_SIZE, GROUND_HEIGHT, ROOF_MATERIAL_COLOR, ROOF_THICKNESS, WALL_THICKNESS,
    },
    entities::map::map::MapEntity,
};
use bevy::{
    image::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    math::Affine2,
    prelude::*,
};
use bevy_rapier3d::prelude::*;

pub enum WallOrientation {
    AlongX,
    AlongZ,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum WallType {
    NORTH,
    SOUTH,
    WEST,
    EAST,
}

impl WallType {
    pub fn all() -> &'static [WallType] {
        &[
            WallType::NORTH,
            WallType::SOUTH,
            WallType::WEST,
            WallType::EAST,
        ]
    }
}

pub fn make_north_wall(
    meshes: &mut ResMut<Assets<Mesh>>,
    ground_size: f32,
    commands: &mut Commands,
    wall_material: &Handle<StandardMaterial>,
    offset: Vec3,
    wall_height: f32,
    wall_thickness: f32,
) {
    let north_wall_mesh = meshes.add(Cuboid::new(ground_size * 2.0, wall_height, wall_thickness));
    commands.spawn((
        Mesh3d(north_wall_mesh),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_xyz(
            0.0 + offset.x,
            wall_height / 2.0 + offset.y,
            ground_size + offset.z,
        ),
        GlobalTransform::default(),
        Collider::cuboid(ground_size, wall_height / 2.0, wall_thickness / 2.0),
        MapEntity,
    ));
}

pub fn make_south_wall(
    meshes: &mut ResMut<Assets<Mesh>>,
    ground_size: f32,
    commands: &mut Commands,
    wall_material: &Handle<StandardMaterial>,
    offset: Vec3,
    wall_height: f32,
    wall_thickness: f32,
) {
    let south_wall_mesh = meshes.add(Cuboid::new(ground_size * 2.0, wall_height, wall_thickness));
    commands.spawn((
        Mesh3d(south_wall_mesh),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_xyz(
            0.0 + offset.x,
            wall_height / 2.0 + offset.y,
            -ground_size + offset.z,
        ),
        GlobalTransform::default(),
        Collider::cuboid(ground_size, wall_height / 2.0, wall_thickness / 2.0),
        MapEntity,
    ));
}

pub fn make_west_wall(
    meshes: &mut ResMut<Assets<Mesh>>,
    ground_size: f32,
    commands: &mut Commands,
    wall_material: &Handle<StandardMaterial>,
    offset: Vec3,
    wall_height: f32,
    wall_thickness: f32,
) {
    let west_wall_mesh = meshes.add(Cuboid::new(wall_thickness, wall_height, ground_size * 2.0));
    commands.spawn((
        Mesh3d(west_wall_mesh),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_xyz(
            -ground_size + offset.x,
            wall_height / 2.0 + offset.y,
            0.0 + offset.z,
        ),
        GlobalTransform::default(),
        Collider::cuboid(wall_thickness / 2.0, wall_height / 2.0, ground_size),
        MapEntity,
    ));
}

pub fn make_east_wall(
    meshes: &mut ResMut<Assets<Mesh>>,
    ground_size: f32,
    commands: &mut Commands,
    wall_material: &Handle<StandardMaterial>,
    offset: Vec3,
    wall_height: f32,
    wall_thickness: f32,
) {
    let east_wall_mesh = meshes.add(Cuboid::new(wall_thickness, wall_height, ground_size * 2.0));
    commands.spawn((
        Mesh3d(east_wall_mesh),
        MeshMaterial3d(wall_material.clone()),
        Transform::from_xyz(
            ground_size + offset.x,
            wall_height / 2.0 + offset.y,
            0.0 + offset.z,
        ),
        GlobalTransform::default(),
        Collider::cuboid(wall_thickness / 2.0, wall_height / 2.0, ground_size),
        MapEntity,
    ));
}

pub fn make_room(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    wall_height: f32,
    door_radius: f32,
    offset: Vec3,
    extension: f32,
    empty_side: HashSet<WallType>,
    door_side: HashSet<WallType>,
    asset_server: &Res<AssetServer>,
) {
    let ground_size = BASE_ROOM_SIZE + extension;
    let ground_height = GROUND_HEIGHT;
    let ground_mesh = meshes.add(Cuboid::new(
        ground_size * 2.0,
        ground_height * 2.0,
        ground_size * 2.0,
    ));

    let ground_color_texture = Some(asset_server.load_with_settings(
        "map/ground.png",
        |s: &mut _| {
            *s = ImageLoaderSettings {
                sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                    address_mode_u: ImageAddressMode::Repeat,
                    address_mode_v: ImageAddressMode::Repeat,
                    ..default()
                }),
                ..default()
            }
        },
    ));

    let ground_material = materials.add(StandardMaterial {
        base_color_texture: ground_color_texture,
        perceptual_roughness: 0.8,
        reflectance: 0.4,
        uv_transform: Affine2::from_scale(Vec2::new(ground_size / 6., ground_size / 6.)),
        ..default()
    });

    commands.spawn((
        Mesh3d(ground_mesh.clone()),
        MeshMaterial3d(ground_material.clone()),
        Transform::from_xyz(0.0 + offset.x, -ground_height + offset.y, 0.0 + offset.z),
        GlobalTransform::default(),
        Collider::cuboid(ground_size, ground_height, ground_size),
        MapEntity,
    ));

    let wall_thickness = WALL_THICKNESS;
    let wall_color_texture = Some(asset_server.load_with_settings(
        "map/bricks_wall.png",
        |s: &mut _| {
            *s = ImageLoaderSettings {
                sampler: ImageSampler::Descriptor(ImageSamplerDescriptor {
                    address_mode_u: ImageAddressMode::Repeat,
                    address_mode_v: ImageAddressMode::Repeat,
                    ..default()
                }),
                ..default()
            }
        },
    ));
    let wall_material_ew = materials.add(StandardMaterial {
        base_color_texture: wall_color_texture.clone(),
        uv_transform: Affine2::from_scale(Vec2::new(6., 1.))
            * Affine2::from_angle(std::f32::consts::PI / 2.0),
        ..default()
    });

    let wall_material_ns = materials.add(StandardMaterial {
        base_color_texture: wall_color_texture.clone(),
        uv_transform: Affine2::from_scale(Vec2::new(6., 1.)),
        ..default()
    });

    for wall_type in WallType::all() {
        if !empty_side.contains(&wall_type) {
            if door_side.contains(&wall_type) {
                match wall_type {
                    WallType::NORTH => spawn_wall_with_hole(
                        commands,
                        meshes,
                        &wall_material_ns,
                        wall_color_texture.clone(),
                        materials,
                        wall_thickness,
                        wall_height,
                        door_radius,
                        ground_size,
                        Vec3::new(offset.x, offset.y, ground_size + offset.z),
                        0.0,
                        WallOrientation::AlongX,
                    ),
                    WallType::SOUTH => spawn_wall_with_hole(
                        commands,
                        meshes,
                        &wall_material_ns,
                        wall_color_texture.clone(),
                        materials,
                        wall_thickness,
                        wall_height,
                        door_radius,
                        ground_size,
                        Vec3::new(offset.x, offset.y, -ground_size + offset.z),
                        0.0,
                        WallOrientation::AlongX,
                    ),
                    WallType::WEST => spawn_wall_with_hole(
                        commands,
                        meshes,
                        &wall_material_ew,
                        wall_color_texture.clone(),
                        materials,
                        wall_thickness,
                        wall_height,
                        door_radius,
                        ground_size,
                        Vec3::new(-ground_size + offset.x, offset.y, offset.z),
                        0.0,
                        WallOrientation::AlongZ,
                    ),
                    WallType::EAST => spawn_wall_with_hole(
                        commands,
                        meshes,
                        &wall_material_ew,
                        wall_color_texture.clone(),
                        materials,
                        wall_thickness,
                        wall_height,
                        door_radius,
                        ground_size,
                        Vec3::new(ground_size + offset.x, offset.y, offset.z),
                        0.0,
                        WallOrientation::AlongZ,
                    ),
                }
            } else {
                match wall_type {
                    WallType::NORTH => make_north_wall(
                        meshes,
                        ground_size,
                        commands,
                        &wall_material_ns,
                        offset,
                        wall_height,
                        wall_thickness,
                    ),
                    WallType::SOUTH => make_south_wall(
                        meshes,
                        ground_size,
                        commands,
                        &wall_material_ns,
                        offset,
                        wall_height,
                        wall_thickness,
                    ),
                    WallType::WEST => make_west_wall(
                        meshes,
                        ground_size,
                        commands,
                        &wall_material_ew,
                        offset,
                        wall_height,
                        wall_thickness,
                    ),
                    WallType::EAST => make_east_wall(
                        meshes,
                        ground_size,
                        commands,
                        &wall_material_ew,
                        offset,
                        wall_height,
                        wall_thickness,
                    ),
                }
            }
        }

        make_roof(commands, meshes, materials, wall_height, offset, extension);
    }
}

pub fn make_roof(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    wall_height: f32,
    offset: Vec3,
    extension: f32,
) {
    let ground_size = BASE_ROOM_SIZE + extension;
    let roof_thickness = ROOF_THICKNESS;

    let roof_mesh = meshes.add(Cuboid::new(
        ground_size * 2.0,
        roof_thickness,
        ground_size * 2.0,
    ));

    let roof_material = materials.add(StandardMaterial {
        base_color: ROOF_MATERIAL_COLOR,
        perceptual_roughness: 1.0,
        ..default()
    });

    commands.spawn((
        Mesh3d(roof_mesh),
        MeshMaterial3d(roof_material.clone()),
        Transform::from_xyz(
            0.0 + offset.x,
            wall_height + roof_thickness / 2.0 + offset.y,
            0.0 + offset.z,
        ),
        GlobalTransform::default(),
        Collider::cuboid(ground_size, roof_thickness / 2.0, ground_size),
        MapEntity,
    ));
}

pub fn spawn_wall_with_hole(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    wall_material: &Handle<StandardMaterial>,
    wall_color_texture: Option<Handle<Image>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,

    wall_thickness: f32,
    wall_height: f32,
    door_radius: f32,
    ground_size: f32,
    offset: Vec3,
    extension: f32,
    orientation: WallOrientation,
) {
    let extended_size = ground_size + extension;

    let hole_h = door_radius * 2.0;
    let top_h = wall_height - hole_h;
    let top_y = hole_h + top_h / 2.0;

    match orientation {
        WallOrientation::AlongZ => {
            let wall_material_top = materials.add(StandardMaterial {
                base_color_texture: wall_color_texture.clone(),
                uv_transform: Affine2::from_scale(Vec2::new(6., 0.5))
                    * Affine2::from_angle(std::f32::consts::PI / 2.0),
                ..default()
            });
            let top_mesh = meshes.add(Cuboid::new(wall_thickness, top_h, extended_size * 2.0));
            commands.spawn((
                Mesh3d(top_mesh),
                MeshMaterial3d(wall_material_top.clone()),
                Transform::from_xyz(offset.x, top_y + offset.y, offset.z).with_scale(Vec3::ONE),
                GlobalTransform::default(),
                Collider::cuboid(wall_thickness / 2.0, top_h / 2.0, extended_size),
                MapEntity,
            ));

            let side_w = extended_size - door_radius;
            let side_z = (extended_size + door_radius) / 2.0;
            //TODO: Have to fix this, subtracting by 4 is only temperory measure. It just pushed the wall bellow surface.
            let side_h = wall_height - top_h - 4.;

            let mut spawn_side = |z_center: f32| {
                let mesh = meshes.add(Cuboid::new(wall_thickness, wall_height, side_w));
                commands.spawn((
                    Mesh3d(mesh),
                    MeshMaterial3d(wall_material.clone()),
                    Transform::from_xyz(offset.x, side_h / 2.0 + offset.y, z_center + offset.z),
                    GlobalTransform::default(),
                    Collider::cuboid(wall_thickness / 2.0, side_h / 2.0, side_w / 2.0),
                    MapEntity,
                ));
            };

            spawn_side(-side_z);
            spawn_side(side_z);
        }

        WallOrientation::AlongX => {
            let wall_material_top = materials.add(StandardMaterial {
                base_color_texture: wall_color_texture.clone(),
                uv_transform: Affine2::from_scale(Vec2::new(6., 0.5)),
                ..default()
            });

            let top_mesh = meshes.add(Cuboid::new(extended_size * 2.0, top_h, wall_thickness));
            commands.spawn((
                Mesh3d(top_mesh),
                MeshMaterial3d(wall_material_top),
                Transform::from_xyz(offset.x, top_y + offset.y, offset.z),
                GlobalTransform::default(),
                Collider::cuboid(extended_size, top_h / 2.0, wall_thickness / 2.0),
                MapEntity,
            ));

            let side_w = extended_size - door_radius;
            let side_x = (extended_size + door_radius) / 2.0;
            //TODO: Have to fix this, subtracting by 4 is only temperory measure. It just pushed the wall bellow surface.
            let side_h = wall_height - top_h - 4.;

            let mut spawn_side = |x_center: f32| {
                let mesh = meshes.add(Cuboid::new(side_w, wall_height, wall_thickness));
                commands.spawn((
                    Mesh3d(mesh),
                    MeshMaterial3d(wall_material.clone()),
                    Transform::from_xyz(x_center + offset.x, side_h / 2.0 + offset.y, offset.z),
                    GlobalTransform::default(),
                    Collider::cuboid(side_w / 2.0, side_h / 2.0, wall_thickness / 2.0),
                    MapEntity,
                ));
            };

            spawn_side(-side_x);
            spawn_side(side_x);
        }
    }
}
