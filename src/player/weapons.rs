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

fn initiate_sword(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    camera_entity: &Entity,
) {
    let model_handle: Handle<Scene> = asset_server.load("player/sword.glb#Scene0");

    let rest_transform = Transform::from_xyz(0.3, 0.275, -0.8)
        .with_scale(Vec3::splat(0.8))
        .with_rotation(
            Quat::from_rotation_x(std::f32::consts::PI)
                * Quat::from_rotation_y(60.0_f32.to_radians())
                * Quat::from_rotation_z(-10.0_f32.to_radians()),
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

fn phase_progress(elapsed: f32, duration: f32, start_frac: f32, end_frac: f32) -> f32 {
    ((elapsed / duration).clamp(start_frac, end_frac) - start_frac) / (end_frac - start_frac)
}

pub fn animate_sword_slash(
    mut sword_query: Query<(Entity, &mut Transform, &SwordSlash, &SwordRestTransform), With<Sword>>,
    time: Res<Time>,
) {
    for (entity, mut transform, slash, rest) in sword_query.iter_mut() {
        let elapsed = time.elapsed_secs() - slash.start_time;
        let total_duration = slash.duration;
        let reset_duration = 0.5;

        if elapsed < total_duration {
            let raise_t = phase_progress(elapsed, total_duration, 0.0, 0.4);
            let max_rotation = -15.0_f32.to_radians();
            let rotation = Quat::from_rotation_z(max_rotation * raise_t)
                * Quat::from_rotation_x(20.0_f32.to_radians());

            let max_up = 0.25;
            let min_down = -0.75;
            let min_left = -0.5;

            let swing_t = phase_progress(elapsed, total_duration - reset_duration, 0.4, 1.0);

            let reset_t =
                ((elapsed - (total_duration - reset_duration)) / reset_duration).clamp(0.0, 1.0);

            let y_offset = (max_up * raise_t + min_down * swing_t) * (1.0 - reset_t);
            let x_offset = (min_left * swing_t) * (1.0 - reset_t);

            transform.translation = rest.translation + Vec3::new(x_offset, y_offset, 0.0);
            transform.rotation = rest.rotation * rotation.slerp(Quat::IDENTITY, reset_t);
        }
    }
}

pub fn spawn_weapons(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    camera_entity: &Entity,
) {
    initiate_sword(commands, asset_server, camera_entity);
}
