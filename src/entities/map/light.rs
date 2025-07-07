use bevy::{
    color::Color,
    ecs::system::{Commands, ResMut},
    pbr::AmbientLight,
};

pub fn brighten_ambient(mut ambient: ResMut<AmbientLight>) {
    ambient.brightness = 900.0;
    ambient.color = Color::srgb(1.0, 1.0, 1.0);
}

pub fn remove_ambient(mut commands: Commands) {
    commands.remove_resource::<AmbientLight>();
}
