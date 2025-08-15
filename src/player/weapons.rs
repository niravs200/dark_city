use bevy::prelude::*;

use crate::player::player::Player;

#[derive(Component)]
pub struct Sword;

#[derive(Component)]
pub struct SwordSlash {
    pub start_time: f32,
    pub duration: f32,
}

#[derive(Component, Clone, Copy)]
pub struct SwordRestTransform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

fn initiate_sword(commands: &mut Commands, asset_server: Res<AssetServer>, camera_entity: &Entity) {
    let model_handle: Handle<Scene> = asset_server.load("player/sword.glb#Scene0");

    let rest_transform = Transform::from_xyz(0.3, 0.275, -0.8)
        .with_scale(Vec3::splat(0.8))
        .with_rotation(
            Quat::from_rotation_x(std::f32::consts::PI)
                * Quat::from_rotation_y(15.0_f32.to_radians()),
        );

    let model_entity = commands
        .spawn((
            SceneRoot(model_handle),
            rest_transform,
            Player,
            Sword,
            SwordRestTransform {
                translation: rest_transform.translation,
                rotation: rest_transform.rotation,
                scale: rest_transform.scale,
            },
        ))
        .id();

    commands.entity(*camera_entity).add_child(model_entity);
}

pub fn animate_sword_slash(
    mut commands: Commands,
    mut sword_query: Query<(Entity, &mut Transform, &SwordSlash, &SwordRestTransform), With<Sword>>,
    time: Res<Time>,
) {
    for (entity, mut transform, slash, rest) in sword_query.iter_mut() {
        let elapsed = time.elapsed_secs() - slash.start_time;

        if elapsed < slash.duration {
            let progress = elapsed / slash.duration;
            let start_pos = rest.translation + Vec3::new(0.2, 0.2, 0.0); // offset from rest
            let end_pos = rest.translation + Vec3::new(-0.5, -0.3, 0.0); // offset from rest

            transform.translation = start_pos.lerp(end_pos, progress);

            let vertical_rotation = Quat::from_rotation_x(15.0_f32.to_radians());
            let horizontal_rotation = Quat::from_rotation_y(90.0_f32.to_radians());
            transform.rotation = horizontal_rotation * vertical_rotation;
        } else {
            // Reset to base transform
            transform.translation = rest.translation;
            transform.rotation = rest.rotation;
            transform.scale = rest.scale;

            commands.entity(entity).remove::<SwordSlash>();
        }
    }
}
pub fn spawn_weapons(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    camera_entity: &Entity,
) {
    initiate_sword(commands, asset_server, camera_entity);
}
