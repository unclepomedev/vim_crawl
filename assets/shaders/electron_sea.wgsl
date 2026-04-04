#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import "shaders/math.wgsl"::{hash2, fbm, smin}

struct ElectronSeaMaterial {
    time: f32,
    resolution: vec2<f32>,
    camera_pos: vec2<f32>,
    padding: vec2<f32>,
}

@group(0) @binding(2)
var<uniform> sea_mat: ElectronSeaMaterial;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;
    let t = sea_mat.time;
    let res = sea_mat.resolution;
    let cam = sea_mat.camera_pos;

    let screen_pos = (uv - 0.5) * res;
    let world_pos = vec2<f32>(screen_pos.x + cam.x, -screen_pos.y + cam.y);

    let scale = 0.02;
    let cell = world_pos * scale;
    let ci = floor(cell);
    let cf = fract(cell);

    let wave = fbm(ci * 0.18 + vec2<f32>(t * 0.22, t * 0.14))
        + 0.40 * sin(ci.x * 0.6 + t * 1.10)
        + 0.30 * cos(ci.y * 0.7 - t * 0.80)
        + 0.25 * sin((ci.x + ci.y) * 0.4 + t * 1.40);

    var energy: f32 = clamp((wave + 1.2) * 0.42, 0.0, 1.0);

    let center_dist = length(ci) * 0.05;
    let ripple = 0.5 + 0.5 * sin(center_dist * 6.0 - t * 2.5);
    energy = mix(energy, energy * ripple, 0.3);

    let pulse = 0.5 + 0.5 * sin(t * 2.8 + hash2(ci) * 6.2831);
    let ep = energy * (0.7 + 0.3 * pulse);

    let line_w = 0.018 + energy * 0.012;
    let lineH = smin(abs(cf.y - 0.5), 0.0, line_w);
    let lineV = smin(abs(cf.x - 0.5), 0.0, line_w);
    let lines = max(lineH, lineV);

    let dot_r = 0.045 + ep * 0.065;
    let dot_d = length(cf - vec2<f32>(0.5));
    let dot = smin(dot_d, dot_r - 0.01, dot_r + 0.01);

    let elec_prob = hash2(ci + vec2<f32>(3.1, 7.3));
    let elec_phase = hash2(ci + vec2<f32>(13.7, 5.1)) * 6.2831;
    let elec_active = select(0.0, 1.0, elec_prob > 0.82);
    let elec_pulse = 0.5 + 0.5 * sin(t * 4.5 + elec_phase);
    let elec_glow = elec_active * elec_pulse * smin(dot_d, 0.0, 0.18);

    let c_cyan = vec3<f32>(0.0, 0.80, 1.0);
    let c_emerald = vec3<f32>(0.2, 1.0, 0.7);
    let c_purple = vec3<f32>(0.6, 0.3, 1.0);
    let c_white = vec3<f32>(1.0, 0.9, 1.0);

    var line_color = mix(c_cyan, c_emerald, energy);
    line_color = mix(line_color, c_purple, elec_active * elec_pulse * 0.5);

    var dot_color = mix(c_emerald, c_cyan, energy);
    dot_color = mix(dot_color, c_white, elec_active * elec_pulse * 0.8);

    let bg_color = vec3<f32>(0.008, 0.022, 0.06);

    let line_alpha = lines * (0.10 + energy * 0.55) * (0.7 + 0.3 * pulse);
    let dot_alpha = dot * (0.30 + ep * 0.70);
    let elec_alpha = elec_glow * 0.45;

    var col = bg_color;
    col = mix(col, line_color, line_alpha);
    col = mix(col, dot_color, dot_alpha);
    col += line_color * elec_alpha;

    let scan_line = 0.92 + 0.08 * sin(uv.y * res.y * 1.5);
    col *= scan_line;

    let vign = 1.0 - smoothstep(0.4, 1.0, length(uv - vec2<f32>(0.5)) * 1.4);
    col *= vign;

    return vec4<f32>(col, 1.0);
}
