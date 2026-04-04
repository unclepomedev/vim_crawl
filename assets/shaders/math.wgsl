fn hash2(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453);
}

fn vnoise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    return mix(
        mix(hash2(i), hash2(i + vec2<f32>(1.0, 0.0)), u.x),
        mix(hash2(i + vec2<f32>(0.0, 1.0)), hash2(i + vec2<f32>(1.0, 1.0)), u.x),
        u.y
    );
}

fn fbm(p: vec2<f32>) -> f32 {
    var val: f32 = 0.0;
    var amp: f32 = 0.5;
    var pp: vec2<f32> = p;
    for (var i: i32 = 0; i < 5; i++) {
        val += amp * vnoise(pp);
        pp *= 2.1;
        amp *= 0.5;
    }
    return val;
}

fn smin(a: f32, edge0: f32, edge1: f32) -> f32 {
    return 1.0 - smoothstep(edge0, edge1, a);
}
