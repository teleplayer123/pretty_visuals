struct Uniforms {
    screen_size: vec2<f32>,
    time: f32,
    zoom: f32,
};

@group(0) @binding(0) var<uniform> ui: Uniforms;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(i32(in_vertex_index & 1u) << 2u) - 1.0;
    let y = f32(i32(in_vertex_index & 2u) << 1u) - 1.0;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let res = ui.screen_size;
    // 1. Center the coordinates perfectly
    var uv = (in.clip_position.xy - 0.5 * res) / min(res.y, res.x);
    
    // 2. Apply the zoom
    uv = uv / ui.zoom;

    // 3. The "Infinite Spiral" Coordinate
    // This specific point is deep inside a spiral arm that never ends.
    let target_coord = vec2<f32>(-0.743643887037151, 0.131825904205330);
    var z = uv + target_coord;

    // 4. Julia Constant 'c'
    // We keep this NEAR the Mandelbrot boundary for maximum complexity.
    let c = vec2<f32>(-0.8, 0.156) + vec2<f32>(cos(ui.time * 0.1), sin(ui.time * 0.1)) * 0.001;
    
    var iter: f32 = 0.0;
    let max_iter: f32 = 500.0; // High iterations for deep zoom clarity
    
    for (var j = 0; j < 500; j++) {
        // Complex math: z = z^2 + c
        z = vec2<f32>(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
        
        let mag_sq = dot(z, z);
        if (mag_sq > 100.0) {
            // Smooth coloring formula
            let log_zn = log(mag_sq) / 2.0;
            let nu = log(log_zn / log(2.0)) / log(2.0);
            iter = iter + 1.0 - nu;
            break;
        }
        iter = iter + 1.0;
    }

    if (iter >= max_iter) { return vec4<f32>(0.0, 0.0, 0.0, 1.0); }

    // 5. High-Frequency Psychedelic Colors
    // We multiply 'iter' to make the colors "zip" past as we zoom.
    let t = iter * 0.1 + ui.time * 0.5;
    let r = 0.5 + 0.5 * sin(t * 1.0);
    let g = 0.5 + 0.5 * sin(t * 1.3 + 2.0);
    let b = 0.5 + 0.5 * sin(t * 1.7 + 4.0);

    return vec4<f32>(r, g, b, 1.0);
}