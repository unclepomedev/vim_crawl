#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import "shaders/math.wgsl"::{hash2, fbm}

// see material.rs
struct ElectronSeaMaterial {
    time: f32,
    tile_size: vec2<f32>,
    offset: vec2<f32>,
    resolution: vec2<f32>,
    camera_pos: vec2<f32>,
    bounds: vec4<f32>
}

@group(0) @binding(2)
var<uniform> sea_mat: ElectronSeaMaterial;

// coordinate space ========================================================
struct GridSpace {
    ci: vec2<f32>, // cell index
    cf: vec2<f32>, // inside cell coordinates (0.0 - 1.0)
}

fn get_grid_space(uv: vec2<f32>, res: vec2<f32>, cam: vec2<f32>, offset: vec2<f32>, tile_size: vec2<f32>) -> GridSpace {
    let pixel_pos = (uv - 0.5) * res;
    let world_pos = pixel_pos + cam;
    let safe_tile = max(abs(tile_size), vec2<f32>(0.0001, 0.0001));
    let cell = (world_pos - offset) / safe_tile;
    let draw_cell = cell + 0.5;
    return GridSpace(floor(draw_cell), fract(draw_cell));
}

// cell state ===============================================================
struct CellState {
    energy: f32, // base wave strength
    pulse: f32,  // flashing intensity
    ep: f32,     // composite energy coefficient for drawing
}

fn get_cell_state(ci: vec2<f32>, t: f32) -> CellState {
    let wave = fbm(ci * 0.18 + vec2<f32>(t * 0.22, t * 0.14))
        + 0.40 * sin(ci.x * 0.6 + t * 1.10)
        + 0.30 * cos(ci.y * 0.7 - t * 0.80);

    let energy = clamp((wave + 1.2) * 0.42, 0.0, 1.0);
    let pulse = 0.5 + 0.5 * sin(t * 2.8 + hash2(ci) * 6.2831);

    return CellState(energy, pulse, energy * (0.7 + 0.3 * pulse));
}

// shape rendering =======================================================
fn get_edge_line_alpha(cf: vec2<f32>, energy: f32) -> f32 {
    let thickness = 0.01 + energy * 0.01;
    let dist_to_edge = min(min(cf.x, 1.0 - cf.x), min(cf.y, 1.0 - cf.y));
    return 1.0 - smoothstep(0.0, thickness, dist_to_edge);
}

fn is_out_of_bounds(ci: vec2<f32>, bounds: vec4<f32>) -> bool {
    return ci.x < 0.0 || ci.x > bounds.x || ci.y < 0.0 || ci.y > bounds.y;
}

fn is_enemy_area(ci: vec2<f32>, bounds: vec4<f32>) -> bool {
    return ci.x > (bounds.x - bounds.z);
}

fn get_cell_color(state: CellState, enemy_area: bool) -> vec3<f32> {
    let c_cyan = vec3<f32>(0.0, 0.80, 1.0);
    let c_emerald = vec3<f32>(0.2, 1.0, 0.7);
    let c_red = vec3<f32>(1.0, 0.1, 0.1);

    var color = mix(c_cyan, c_emerald, state.energy);
    if (enemy_area) {
        color = mix(color, c_red, 0.5);
    }

    return color;
}

const DEFENSE_THICKNESS_PX: f32 = 20.0;
const C_DEFENSE: vec3<f32> = vec3<f32>(0.1, 1.0, 0.3);

// border rendering =====================================================

/// Returns the intensity of the left-edge defense line drawn at column 0.
/// `cf.x` is the fractional position within the current cell (0.0–1.0).
fn get_defense_line_alpha(ci: vec2<f32>, cf: vec2<f32>, tile_size: vec2<f32>) -> f32 {
    if ci.x != 0.0 {
        return 0.0;
    }
    let thickness = DEFENSE_THICKNESS_PX / tile_size.x;
    return 1.0 - smoothstep(0.0, thickness, cf.x);
}

// post processing ===============================================
fn apply_post_processing(col: vec3<f32>, uv: vec2<f32>, res_y: f32) -> vec3<f32> {
    let scan_line = 0.95 + 0.05 * sin(uv.y * res_y * 1.5);
    let vign = 1.0 - smoothstep(0.5, 1.5, length(uv - 0.5) * 2.0);
    return col * scan_line * vign;
}

// main fragment shader
@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let grid = get_grid_space(uv, sea_mat.resolution, sea_mat.camera_pos, sea_mat.offset, sea_mat.tile_size);
    let c_bg = vec3<f32>(0.008, 0.022, 0.06);

    if (is_out_of_bounds(grid.ci, sea_mat.bounds)) {
        return vec4<f32>(c_bg, 1.0);
    }

    let state = get_cell_state(grid.ci, sea_mat.time);
    let enemy_area = is_enemy_area(grid.ci, sea_mat.bounds);
    let base_color = get_cell_color(state, enemy_area);

    let lines = get_edge_line_alpha(grid.cf, state.energy);

    var col = c_bg;
    col = mix(col, base_color, lines * (0.2 + state.energy * 0.5));

    col = apply_post_processing(col, uv, sea_mat.resolution.y);

    let defense = get_defense_line_alpha(grid.ci, grid.cf, sea_mat.tile_size);
    col = mix(col, C_DEFENSE, defense * 0.9);

    return vec4<f32>(col, 1.0);
}
