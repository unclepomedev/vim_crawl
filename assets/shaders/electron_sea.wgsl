#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import "shaders/math.wgsl"::{hash2, fbm, smin}

struct ElectronSeaMaterial {
    time: f32,
    tile_size: f32,
    offset: vec2<f32>,
    resolution: vec2<f32>,
    camera_pos: vec2<f32>,
}

@group(0) @binding(2)
var<uniform> sea_mat: ElectronSeaMaterial;

// coordinate space ========================================================
struct GridSpace {
    ci: vec2<f32>, // cell index
    cf: vec2<f32>, // inside cell coordinates (0.0 - 1.0)
}

fn get_grid_space(uv: vec2<f32>, res: vec2<f32>, cam: vec2<f32>, offset: vec2<f32>, tile_size: f32) -> GridSpace {
    let pixel_pos = (uv - 0.5) * res * vec2<f32>(1.0, -1.0);
    let world_pos = pixel_pos + cam;
    let safe_tile_size = max(abs(tile_size), 0.0001);
    let cell = vec2<f32>(
        (world_pos.x - offset.x) / safe_tile_size,
        (offset.y - world_pos.y) / safe_tile_size
    );
    return GridSpace(floor(cell), fract(cell));
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
        + 0.30 * cos(ci.y * 0.7 - t * 0.80)
        + 0.25 * sin((ci.x + ci.y) * 0.4 + t * 1.40);

    var energy = clamp((wave + 1.2) * 0.42, 0.0, 1.0);

    let center_dist = length(ci) * 0.05;
    let ripple = 0.5 + 0.5 * sin(center_dist * 6.0 - t * 2.5);
    energy = mix(energy, energy * ripple, 0.3);

    let pulse = 0.5 + 0.5 * sin(t * 2.8 + hash2(ci) * 6.2831);
    let ep = energy * (0.7 + 0.3 * pulse);

    return CellState(energy, pulse, ep);
}

// shape rendering =======================================================
fn get_line_alpha(cf: vec2<f32>, energy: f32) -> f32 {
    let line_w = 0.018 + energy * 0.012;
    let lineH = smin(abs(cf.y - 0.5), 0.0, line_w);
    let lineV = smin(abs(cf.x - 0.5), 0.0, line_w);
    return max(lineH, lineV);
}

fn get_dot_alpha(dot_d: f32, ep: f32) -> f32 {
    let dot_r = 0.045 + ep * 0.065;
    return smin(dot_d, dot_r - 0.01, dot_r + 0.01);
}

struct ElecState {
    active_pulse: f32,
    glow: f32,
}

fn get_electricity(ci: vec2<f32>, dot_d: f32, t: f32) -> ElecState {
    let elec_prob = hash2(ci + vec2<f32>(3.1, 7.3));
    let elec_phase = hash2(ci + vec2<f32>(13.7, 5.1)) * 6.2831;
    let elec_active = select(0.0, 1.0, elec_prob > 0.82);
    let elec_pulse = 0.5 + 0.5 * sin(t * 4.5 + elec_phase);

    let active_pulse = elec_active * elec_pulse;
    let glow = active_pulse * smin(dot_d, 0.0, 0.18);
    return ElecState(active_pulse, glow);
}

// post processing ===============================================
fn apply_post_process(col: vec3<f32>, uv: vec2<f32>, res: vec2<f32>) -> vec3<f32> {
    let scan_line = 0.92 + 0.08 * sin(uv.y * res.y * 1.5);
    let vign = 1.0 - smoothstep(0.4, 1.0, length(uv - vec2<f32>(0.5)) * 1.4);
    return col * scan_line * vign;
}

// main fragment shader
@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let t = sea_mat.time;

    let grid = get_grid_space(uv, sea_mat.resolution, sea_mat.camera_pos, sea_mat.offset, sea_mat.tile_size);

    let state = get_cell_state(grid.ci, t);

    let dot_d = length(grid.cf - vec2<f32>(0.5));

    let lines = get_line_alpha(grid.cf, state.energy);
    let dot = get_dot_alpha(dot_d, state.ep);
    let elec = get_electricity(grid.ci, dot_d, t);

    let c_cyan = vec3<f32>(0.0, 0.80, 1.0);
    let c_emerald = vec3<f32>(0.2, 1.0, 0.7);
    let c_purple = vec3<f32>(0.6, 0.3, 1.0);
    let c_white = vec3<f32>(1.0, 0.9, 1.0);

    var line_color = mix(c_cyan, c_emerald, state.energy);
    line_color = mix(line_color, c_purple, elec.active_pulse * 0.5);

    var dot_color = mix(c_emerald, c_cyan, state.energy);
    dot_color = mix(dot_color, c_white, elec.active_pulse * 0.8);

    let line_alpha = lines * (0.10 + state.energy * 0.55) * (0.7 + 0.3 * state.pulse);
    let dot_alpha = dot * (0.30 + state.ep * 0.70);
    let elec_alpha = elec.glow * 0.45;

    var col = vec3<f32>(0.008, 0.022, 0.06); // background color
    col = mix(col, line_color, line_alpha);
    col = mix(col, dot_color, dot_alpha);
    col += line_color * elec_alpha;

    col = apply_post_process(col, uv, sea_mat.resolution);

    return vec4<f32>(col, 1.0);
}
