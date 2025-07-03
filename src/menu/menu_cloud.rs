use bevy::prelude::*;
use rand::Rng;

use crate::menu::MenuAssets;

#[derive(Component)]
pub struct MainMenuCloud;

#[derive(Component)]
pub struct Cloud {
    pub speed: f32,
}

pub fn spawn_clouds(mut commands: Commands, menu_assets: Res<MenuAssets>, windows: Query<&Window>) {
    let window = windows.single().unwrap();
    let mut rng = rand::rng();

    for _ in 0..5 {
        let image = menu_assets.clouds[rng.random_range(0..menu_assets.clouds.len())].clone();
        let x = rng.random_range(-window.width() / 2.0..window.width() / 2.0);
        let y = rng.random_range(0.0..window.height() / 2.0);
        let speed = rng.random_range(20.0..40.0);

        commands.spawn((
            Sprite { image, ..default() },
            Transform::from_xyz(x, y, 0.0),
            GlobalTransform::default(),
            Visibility::Visible,
            InheritedVisibility::default(),
            ViewVisibility::default(),
            Cloud { speed },
            MainMenuCloud,
        ));
    }
}

pub fn animate_clouds(
    mut query: Query<(&mut Transform, &Cloud)>,
    time: Res<Time>,
    windows: Query<&Window>,
) {
    let window = windows.single().unwrap();
    let right_bound = window.width() / 2.0 + 100.0;
    let left_bound = -window.width() / 2.0 - 100.0;
    for (mut transform, cloud) in &mut query {
        transform.translation.x += cloud.speed * time.delta_secs();
        if transform.translation.x > right_bound {
            transform.translation.x = left_bound;
        }
    }
}

pub fn despawn_clouds(mut commands: Commands, query: Query<Entity, With<MainMenuCloud>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
