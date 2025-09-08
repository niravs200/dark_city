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

fn phase_progress(elapsed: f32, duration: f32, start_frac: f32, end_frac: f32) -> f32 {
    ((elapsed / duration).clamp(start_frac, end_frac) - start_frac) / (end_frac - start_frac)
}

pub fn animate_sword_slash(
    mut commands: Commands,
    mut sword_query: Query<(Entity, &mut Transform, &SwordSlash, &SwordRestTransform), With<Sword>>,
    time: Res<Time>,
) {
    for (entity, mut transform, slash, rest) in sword_query.iter_mut() {
        let elapsed = time.elapsed_secs() - slash.start_time;

        if elapsed < slash.duration {
            // Raising the sword
            let raise_t = phase_progress(elapsed, slash.duration, 0.0, 0.2);
            let max_rotation = -20.0_f32.to_radians();
            let rotation = Quat::from_rotation_z(max_rotation * raise_t);
            transform.rotation = rest.rotation * rotation;

            let max_up = 0.2;
            transform.translation = rest.translation + Vec3::new(0.0, max_up * raise_t, 0.0);

            // Rotate it a bit
            let tip_offset = Vec3::new(0.0, -20.0, 0.0);
            transform.translation -= tip_offset;

            let max_tip_rotation = 30.0_f32.to_radians();
            let tip_t = phase_progress(elapsed, slash.duration, 0.2, 1.0);
            if tip_t > 0.0 {
                let tip_rotation = Quat::from_rotation_z(max_tip_rotation * tip_t);
                transform.rotation = transform.rotation * tip_rotation;
            }
            transform.translation += tip_offset;
        } else {
            // Reset to rest
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
