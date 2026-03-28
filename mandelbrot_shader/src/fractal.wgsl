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
    let ndc = (in.clip_position.xy - 0.5 * res) / min(res.y, res.x);
    
    // 1. The Zoom Point (Fixed Coordinate)
    // This is a famous "Double Spiral" location in the Mandelbrot set.
    let zoom_point = vec2<f32>(-0.743643887037151, 0.131825904205330);
    
    // 2. Coordinate calculation
    let c = (ndc / ui.zoom) + zoom_point;

    // 3. Mandelbrot Iteration (z = z^2 + c)
    var z = vec2<f32>(0.0, 0.0);
    var iter: f32 = 0.0;
    
    // Increase detail as we zoom
    let max_iter: f32 = 200.0 + (log2(ui.zoom) * 20.0); 
    
    for (var j = 0; j < 1000; j++) {
        if (f32(j) >= max_iter) { break; }
        
        // Complex square: (x+iy)^2 = x^2 - y^2 + i(2xy)
        z = vec2<f32>(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
        
        let mag_sq = dot(z, z);
        if (mag_sq > 100.0) {
            let log_zn = log(mag_sq) / 2.0;
            let nu = log(log_zn / log(2.0)) / log(2.0);
            iter = iter + 1.0 - nu;
            break;
        }
        iter = iter + 1.0;
    }

    if (iter >= max_iter) { return vec4<f32>(0.0, 0.0, 0.0, 1.0); }

    // 4. Color Palette (Psychedelic Sine Waves)
    let t = iter * 0.05 + ui.time * 0.2;
    let r = 0.5 + 0.5 * sin(t * 1.0);
    let g = 0.5 + 0.5 * sin(t * 1.3 + 2.0);
    let b = 0.5 + 0.5 * sin(t * 1.7 + 4.0);

    return vec4<f32>(r, g, b, 1.0);
}