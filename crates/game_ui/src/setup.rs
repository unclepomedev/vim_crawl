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

/// Returns the uniform world-space scale for a player model occupying one grid cell.
fn player_scale(config: &GridRenderConfig) -> f32 {
    // assume h <= w
    config.tile_h * MODEL_OCCUPATION_RATE
}

pub fn sync_player_scale_on_grid_config_change(
    config: Res<GridRenderConfig>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if config.is_changed() {
        let scale = player_scale(&config);
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

    commands.spawn((
        SceneRoot(asset_server.load("models/turret.glb#Scene0")),
        Player,
        GridPosition { col: 0, row: 0 },
        Transform {
            translation: Vec3::ZERO,
            scale: Vec3::splat(player_scale(&config)),
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

    // Compute the candidate tile_w without mutating config,
    // and skip the full recalculation if the change is negligible.
    let cols = (config.max_col + 1) as f32;
    let rows = (config.max_row + 1) as f32;
    let ratio = 3.0_f32 / 4.0;
    let available_w = w - 80.0;
    let available_h = h - 120.0;
    let tile_w_candidate = {
        let from_w = available_w / cols;
        let from_h = (available_h / rows) / ratio;
        from_w.min(from_h).floor()
    };

    if (tile_w_candidate - config.tile_w).abs() > 0.5 {
        config.recalculate(w, h);
    }
}
