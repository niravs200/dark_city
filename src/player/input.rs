use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::{control::KinematicCharacterController, prelude::*};

use crate::{
    constants::player::{GRAVITY, GROUND_TIMER, JUMP_SPEED, MOUSE_SENSITIVITY, MOVEMENT_SPEED},
    ui::{PauseOverlay, PauseState, despawn_pause_ui, spawn_pause_ui},
};

#[derive(Default, Resource, Deref, DerefMut)]
pub struct MovementInput(Vec3);

#[derive(Default, Resource, Deref, DerefMut)]
pub struct LookInput(Vec2);

pub fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut movement: ResMut<MovementInput>,
    mut look: ResMut<LookInput>,
    mut mouse_events: EventReader<MouseMotion>,
    mut pause_state: ResMut<PauseState>,
    commands: Commands,
    pause_query: Query<Entity, With<PauseOverlay>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        pause_state.is_paused = !pause_state.is_paused;

        if pause_state.is_paused {
            spawn_pause_ui(commands);
        } else {
            despawn_pause_ui(commands, pause_query);
        }
    }

    if !pause_state.is_paused {
        if keyboard.pressed(KeyCode::KeyW) {
            movement.z -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            movement.z += 1.0
        }
        if keyboard.pressed(KeyCode::KeyA) {
            movement.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            movement.x += 1.0
        }
        **movement = movement.normalize_or_zero();
        if keyboard.pressed(KeyCode::ShiftLeft) {
            **movement *= 2.0;
        }
        if keyboard.pressed(KeyCode::Space) {
            movement.y = 1.0;
        }

        for event in mouse_events.read() {
            look.x -= event.delta.x * MOUSE_SENSITIVITY;
            look.y -= event.delta.y * MOUSE_SENSITIVITY;
            look.y = look.y.clamp(-89.9, 89.9);
        }
    }
}

pub fn player_movement(
    time: Res<Time>,
    mut input: ResMut<MovementInput>,
    mut player: Query<(
        &mut Transform,
        &mut KinematicCharacterController,
        Option<&KinematicCharacterControllerOutput>,
    )>,
    mut vertical_movement: Local<f32>,
    mut grounded_timer: Local<f32>,
) {
    let Ok((transform, mut controller, output)) = player.single_mut() else {
        return;
    };
    let delta_time = time.delta_secs();
    let mut movement = Vec3::new(input.x, 0.0, input.z) * MOVEMENT_SPEED;
    let jump_speed = input.y * JUMP_SPEED;
    **input = Vec3::ZERO;
    if output.map(|o| o.grounded).unwrap_or(false) {
        *grounded_timer = GROUND_TIMER;
        *vertical_movement = 0.0;
    }
    if *grounded_timer > 0.0 {
        *grounded_timer -= delta_time;
        if jump_speed > 0.0 {
            *vertical_movement = jump_speed;
            *grounded_timer = 0.0;
        }
    }
    movement.y = *vertical_movement;
    *vertical_movement += GRAVITY * delta_time * controller.custom_mass.unwrap_or(1.0);
    controller.translation = Some(transform.rotation * (movement * delta_time));
}

pub fn player_look(
    mut player: Query<&mut Transform, (With<KinematicCharacterController>, Without<Camera>)>,
    mut camera: Query<&mut Transform, With<Camera>>,
    input: Res<LookInput>,
) {
    let Ok(mut transform) = player.single_mut() else {
        return;
    };
    transform.rotation = Quat::from_axis_angle(Vec3::Y, input.x.to_radians());
    let Ok(mut transform) = camera.single_mut() else {
        return;
    };
    transform.rotation = Quat::from_axis_angle(Vec3::X, input.y.to_radians());
}
