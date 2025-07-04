use bevy::prelude::*;
use bevy::{
    asset::AssetServer,
    ecs::{
        children,
        component::Component,
        entity::Entity,
        query::With,
        system::{Commands, Query, Res},
    },
    ui::{Node, PositionType, UiTargetCamera, Val, widget::ImageNode},
    utils::default,
};

#[derive(Component)]
pub struct Crosshair;

pub fn spawn_crosshair(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    camera_entity: Entity,
) {
    let size = 16.0;
    let cross_hair = asset_server.load("ui/crosshair.png");

    commands.spawn((
        UiTargetCamera(camera_entity),
        Node {
            position_type: PositionType::Absolute,
            left: Val::Percent(50.0),
            top: Val::Percent(50.0),
            margin: UiRect::all(Val::Px(size / 2.0)),
            width: Val::Px(size),
            height: Val::Px(size),
            ..default()
        },
        Crosshair,
        children![ImageNode::new(cross_hair)],
    ));
}

pub fn despawn_crosshair(commands: &mut Commands, crosshair_q: Query<Entity, With<Crosshair>>) {
    if let Ok(entity) = crosshair_q.single() {
        commands.entity(entity).despawn();
    }
}
