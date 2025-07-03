use bevy::prelude::*;
use bevy_rapier3d::{control::KinematicCharacterController, prelude::*};

#[derive(Component)]
pub struct Player;

pub fn setup_player(commands: &mut Commands) {
    commands
        .spawn((
            Player,
            Transform::from_xyz(0.0, 5.0, 0.0),
            Visibility::default(),
            Collider::round_cylinder(0.9, 0.3, 0.2),
            KinematicCharacterController {
                custom_mass: Some(5.0),
                up: Vec3::Y,
                offset: CharacterLength::Absolute(0.01),
                slide: true,
                autostep: Some(CharacterAutostep {
                    max_height: CharacterLength::Relative(0.3),
                    min_width: CharacterLength::Relative(0.5),
                    include_dynamic_bodies: false,
                }),
                max_slope_climb_angle: 45.0_f32.to_radians(),
                min_slope_slide_angle: 30.0_f32.to_radians(),
                apply_impulse_to_dynamic_bodies: true,
                snap_to_ground: None,
                ..default()
            },
        ))
        .with_children(|b| {
            b.spawn((Camera3d::default(), Transform::from_xyz(0.0, 0.2, -0.1)));
        });
}

pub fn despawn_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
