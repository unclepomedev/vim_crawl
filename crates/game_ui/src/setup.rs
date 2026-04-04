use crate::components::{MainCamera, Player};
use crate::material::ElectronSeaMaterial;
use bevy::prelude::*;

pub fn setup_cameras_and_player(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            order: 0,
            ..default()
        },
        ElectronSeaMaterial::default(),
    ));

    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        MainCamera,
    ));

    commands.spawn((
        Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::new(20.0, 20.0)),
            ..default()
        },
        Player,
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
