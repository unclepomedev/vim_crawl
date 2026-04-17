use crate::components::MainCamera;
use crate::resources::grid::GridRenderConfig;
use bevy::render::render_graph::{InternedRenderLabel, RenderLabel};
use bevy::window::PrimaryWindow;
use bevy::{
    core_pipeline::{core_2d::graph::Node2d, fullscreen_material::FullscreenMaterial},
    prelude::*,
    reflect::TypePath,
    render::{extract_component::ExtractComponent, render_resource::ShaderType},
    shader::ShaderRef,
};

// The number of bytes must be a multiple of 16.
#[derive(Component, ExtractComponent, Clone, Copy, Default, ShaderType, TypePath)]
pub struct ElectronSeaMaterial {
    pub time: f32,        // 4 bytes
    pub tile_size: f32,   // 4 bytes
    pub offset: Vec2,     // 8 bytes
    pub resolution: Vec2, // 8 bytes
    pub camera_pos: Vec2, // 8 bytes
    pub bounds: Vec4,     // 16 bytes,  x: max_col, y: max_row, z: enemy_spawn_cols, w: unused
}

//noinspection ALL: suppress "Trait `WriteInto` is not implemented for `ElectronSeaMaterial`"
impl FullscreenMaterial for ElectronSeaMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/field.wgsl".into()
    }

    fn node_edges() -> Vec<InternedRenderLabel> {
        vec![
            Node2d::Tonemapping.intern(),
            Self::node_label().intern(),
            Node2d::EndMainPassPostProcessing.intern(),
        ]
    }
}

pub fn update_world_material(
    time: Res<Time>,
    config: Res<GridRenderConfig>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<&Transform, With<MainCamera>>,
    mut mat_q: Query<&mut ElectronSeaMaterial>,
) {
    let resolution = windows
        .single()
        .map(|w| Vec2::new(w.width(), w.height()))
        .unwrap_or(Vec2::new(1280.0, 720.0));

    let camera_pos = camera_q
        .single()
        .map(|t| Vec2::new(t.translation.x, -t.translation.z))
        .unwrap_or(Vec2::ZERO);

    for mut mat in &mut mat_q {
        mat.time = time.elapsed_secs();
        mat.resolution = resolution;
        mat.camera_pos = camera_pos;
        mat.tile_size = config.tile_size;
        mat.offset = Vec2::new(config.offset_x, config.offset_z);
        mat.bounds = Vec4::new(
            config.max_col as f32,
            config.max_row as f32,
            config.enemy_spawn_cols as f32,
            0.0,
        );
    }
}
