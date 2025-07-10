use std::collections::HashSet;

use bevy::prelude::*;

use crate::{
    constants::map::BASE_ROOM_SIZE,
    entities::map::utility::{WallType, make_room},
};

#[derive(Component)]
pub struct MapEntity;

struct Room {
    name: String,
    wall_height: f32,
    door_radius: f32,
    extension: f32,
    empty_side: HashSet<WallType>,
    door_side: HashSet<WallType>,
    offset: Vec3,
}

#[derive(Clone, Debug)]
pub struct RoomBounds {
    pub name: String,
    pub min: Vec3,
    pub max: Vec3,
}

#[derive(Resource)]
pub struct RoomBoundsData {
    pub bounds: Vec<RoomBounds>,
}

impl Room {
    pub fn get_bounds(&self) -> (Vec3, Vec3) {
        let map_size = BASE_ROOM_SIZE + self.extension;

        let min = Vec3::new(
            self.offset.x - map_size / 2.0,
            self.offset.y - 5.0,
            self.offset.z - map_size / 2.0,
        );

        let max = Vec3::new(
            self.offset.x + map_size / 2.0,
            self.offset.y + self.wall_height,
            self.offset.z + map_size / 2.0,
        );

        (min, max)
    }
}

fn extract_room_bounds(commands: &mut Commands, rooms: &Vec<Room>) {
    let bounds_vec: Vec<RoomBounds> = rooms
        .iter()
        .map(|room| {
            let (min, max) = room.get_bounds();

            RoomBounds {
                name: room.name.clone(),
                min,
                max,
            }
        })
        .collect();

    commands.insert_resource(RoomBoundsData { bounds: bounds_vec });
}

pub fn setup_map(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    asset_server: &Res<AssetServer>,
) {
    let mut rooms: Vec<Room> = Vec::new();

    rooms.push(Room {
        name: String::from("Entry Area"),
        wall_height: 10.0,
        door_radius: 3.0,
        extension: 0.0,
        empty_side: [WallType::EAST].into_iter().collect(),
        door_side: HashSet::new(),
        offset: Vec3::new(0.0, 0.0, 0.0),
    });

    rooms.push(Room {
        name: String::from("Tutorial Area"),
        wall_height: 10.0,
        door_radius: 3.0,
        extension: 10.0,
        empty_side: HashSet::new(),
        door_side: [WallType::SOUTH, WallType::WEST].into_iter().collect(),
        offset: Vec3::new(70.0, 0.0, 0.0),
    });

    rooms.push(Room {
        name: String::from("Enemy Area 1"),
        wall_height: 10.0,
        door_radius: 3.0,
        extension: 0.0,
        empty_side: [WallType::NORTH, WallType::SOUTH].into_iter().collect(),
        door_side: HashSet::new(),
        offset: Vec3::new(70.0, 0.0, -70.0),
    });

    rooms.push(Room {
        name: String::from("Enemy Area 2"),
        wall_height: 10.0,
        door_radius: 3.0,
        extension: 10.0,
        empty_side: HashSet::new(),
        door_side: [WallType::NORTH, WallType::SOUTH].into_iter().collect(),
        offset: Vec3::new(70.0, 0.0, -140.0),
    });

    rooms.push(Room {
        name: String::from("Enemy Area 3"),
        wall_height: 10.0,
        door_radius: 3.0,
        extension: 10.0,
        empty_side: [WallType::NORTH].into_iter().collect(),
        door_side: [WallType::EAST, WallType::WEST].into_iter().collect(),
        offset: Vec3::new(70.0, 0.0, -210.0),
    });

    rooms.push(Room {
        name: String::from("Puzzle Room 1"),
        wall_height: 10.0,
        door_radius: 3.0,
        extension: 0.0,
        empty_side: [WallType::EAST].into_iter().collect(),
        door_side: [WallType::WEST].into_iter().collect(),
        offset: Vec3::new(0.0, 0.0, -210.0),
    });

    rooms.push(Room {
        name: String::from("Reward Room 1"),
        wall_height: 10.0,
        door_radius: 3.0,
        extension: -10.0,
        empty_side: [WallType::EAST].into_iter().collect(),
        door_side: HashSet::new(),
        offset: Vec3::new(-50.0, 0.0, -210.0),
    });

    rooms.push(Room {
        name: String::from("Enemy Room 4"),
        wall_height: 10.0,
        door_radius: 3.0,
        extension: 0.0,
        empty_side: [WallType::WEST].into_iter().collect(),
        door_side: [WallType::EAST].into_iter().collect(),
        offset: Vec3::new(140.0, 0.0, -210.0),
    });

    rooms.push(Room {
        name: String::from("Puzzle Room 2"),
        wall_height: 10.0,
        door_radius: 3.0,
        extension: 0.0,
        empty_side: [WallType::WEST].into_iter().collect(),
        door_side: [WallType::EAST, WallType::NORTH].into_iter().collect(),
        offset: Vec3::new(200.0, 0.0, -210.0),
    });

    rooms.push(Room {
        name: String::from("Hidden Room 1"),
        wall_height: 10.0,
        door_radius: 3.0,
        extension: 0.0,
        empty_side: [WallType::WEST].into_iter().collect(),
        door_side: HashSet::new(),
        offset: Vec3::new(260.0, 0.0, -210.0),
    });

    rooms.push(Room {
        name: String::from("Enemy Room 5"),
        wall_height: 10.0,
        door_radius: 3.0,
        extension: 10.0,
        empty_side: [WallType::SOUTH, WallType::NORTH].into_iter().collect(),
        door_side: HashSet::new(),
        offset: Vec3::new(200.0, 0.0, -140.0),
    });

    rooms.push(Room {
        name: String::from("Boss Room"),
        wall_height: 10.0,
        door_radius: 3.0,
        extension: 20.0,
        empty_side: HashSet::new(),
        door_side: [WallType::SOUTH, WallType::NORTH].into_iter().collect(),
        offset: Vec3::new(200.0, 0.0, -50.0),
    });

    rooms.push(Room {
        name: String::from("Final Reward Room"),
        wall_height: 10.0,
        door_radius: 3.0,
        extension: -20.0,
        empty_side: [WallType::SOUTH].into_iter().collect(),
        door_side: HashSet::new(),
        offset: Vec3::new(200.0, 0.0, 10.0),
    });

    extract_room_bounds(commands, &rooms);

    for room in rooms {
        make_room(
            commands,
            meshes,
            materials,
            room.wall_height,
            room.door_radius,
            room.offset,
            room.extension,
            room.empty_side,
            room.door_side,
            asset_server,
        );
    }
}

pub fn despawn_map(commands: &mut Commands, query: Query<Entity, With<MapEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
