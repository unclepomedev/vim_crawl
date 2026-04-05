use crate::components::MainCamera;
use crate::material::ElectronSeaMaterial;
use bevy::prelude::*;
use game_core::components::grid::GridPosition;
use game_core::components::player::Player;

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
        GridPosition { col: 0, row: 0 },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}
