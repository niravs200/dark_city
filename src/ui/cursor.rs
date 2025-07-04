use bevy::{
    ecs::system::Query,
    window::{CursorGrabMode, Window},
};

pub fn hide_cursor(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut().unwrap();
    window.cursor_options.visible = false;
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
}

pub fn show_cursor(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut().unwrap();
    window.cursor_options.visible = true;
    window.cursor_options.grab_mode = CursorGrabMode::None;
}
