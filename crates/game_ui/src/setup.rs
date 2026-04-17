use crate::components::MainCamera;
use crate::material::ElectronSeaMaterial;
use crate::resources::grid::GridRenderConfig;
use bevy::camera::ScalingMode;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use game_core::components::grid::GridPosition;
use game_core::components::player::Player;
use std::f32::consts::FRAC_PI_2;

const MODEL_OCCUPATION_RATE: f32 = 0.8;
pub fn sync_player_scale_on_grid_config_change(
    config: Res<GridRenderConfig>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if config.is_changed() {
        let scale = config.tile_size * MODEL_OCCUPATION_RATE;
        for mut transform in query.iter_mut() {
            transform.scale = Vec3::splat(scale);
        }
    }
}
pub fn setup_cameras_and_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    config: Res<GridRenderConfig>,
) {
    // for background shader
    commands.spawn((
        Camera2d,
        Camera {
            order: 0,
            ..default()
        },
        ElectronSeaMaterial::default(),
    ));

    // main camera
    commands.spawn((
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize,
            ..OrthographicProjection::default_3d()
        }),
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        MainCamera,
        Transform::from_xyz(0.0, 100.0, 0.0).looking_at(Vec3::ZERO, Vec3::NEG_Z),
    ));

    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(1.0, 3.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let scale = config.tile_size * MODEL_OCCUPATION_RATE;

    commands.spawn((
        SceneRoot(asset_server.load("models/turret.glb#Scene0")),
        Player,
        GridPosition { col: 0, row: 0 },
        Transform {
            translation: Vec3::ZERO,
            scale: Vec3::splat(scale),
            rotation: Quat::from_rotation_y(FRAC_PI_2) * Quat::from_rotation_z(-FRAC_PI_2 / 2.0),
        },
    ));
}

/// Recalculate [GridRenderConfig] when the window is resized.
pub fn recalculate_grid_on_window_resize(
    mut config: ResMut<GridRenderConfig>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let Ok(window) = windows.single() else { return };

    let w = window.width();
    let h = window.height();

    let new_tile = {
        let cols = (config.max_col + 1) as f32;
        let rows = (config.max_row + 1) as f32;
        let available_w = w - 80.0;
        let available_h = h - 120.0;
        (available_w / cols).min(available_h / rows).floor()
    };

    if (new_tile - config.tile_size).abs() > 0.5 {
        config.recalculate(w, h);
    }
}
