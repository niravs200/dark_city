use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::{control::KinematicCharacterController, prelude::*};

use crate::{
    constants::player::{GRAVITY, GROUND_TIMER, JUMP_SPEED, MOUSE_SENSITIVITY, MOVEMENT_SPEED},
    entities::map::map::RoomBoundsData,
    ui::{
        EscButtonState, PauseOverlay, PauseState, despawn_pause_ui,
        hud::{RoomNameDisplay, update_room_display_text},
        spawn_pause_ui,
    },
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
    mut commands: Commands,
    pause_query: Query<Entity, With<PauseOverlay>>,
    mut esc_state: ResMut<EscButtonState>,
    time: Res<Time>,
    mut hold_timer: Local<f32>,
) {
    if pause_state.is_paused {
        if keyboard.just_pressed(KeyCode::Escape) {
            *hold_timer = 0.0;
            esc_state.progress = 0;
        }

        if keyboard.pressed(KeyCode::Escape) {
            *hold_timer += time.delta_secs();
            esc_state.progress = (*hold_timer * 5.0).clamp(0.0, 6.0) as u8;
        }

        if keyboard.just_released(KeyCode::Escape) {
            if *hold_timer < 0.5 {
                pause_state.is_paused = false;
                despawn_pause_ui(&mut commands, pause_query);
            }

            *hold_timer = 0.0;
            esc_state.progress = 0;
        }

        return;
    }

    if keyboard.just_released(KeyCode::Escape) {
        pause_state.is_paused = true;
        spawn_pause_ui(&mut commands);
        *hold_timer = 0.0;
        esc_state.progress = 0;
        return;
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
    bounds_data: Res<RoomBoundsData>,
    room_name_q: Query<&mut Text, With<RoomNameDisplay>>,
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

    calculate_player_room_location(&mut player, bounds_data, room_name_q);
}

fn calculate_player_room_location(
    player: &mut Query<(
        &mut Transform,
        &mut KinematicCharacterController,
        Option<&KinematicCharacterControllerOutput>,
    )>,
    bounds_data: Res<RoomBoundsData>,
    room_name_q: Query<&mut Text, With<RoomNameDisplay>>,
) {
    let Ok((player_transform, _, _)) = player.single_mut() else {
        return;
    };

    let player_pos = player_transform.translation;

    for bound in &bounds_data.bounds {
        if player_pos.x >= bound.min.x
            && player_pos.x <= bound.max.x
            && player_pos.y >= bound.min.y
            && player_pos.y <= bound.max.y
            && player_pos.z >= bound.min.z
            && player_pos.z <= bound.max.z
        {
            update_room_display_text(Some(&bound.name), room_name_q);
            return;
        }
    }

    update_room_display_text(None, room_name_q);
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
