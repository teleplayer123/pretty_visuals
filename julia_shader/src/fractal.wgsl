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
    var uv = (in.clip_position.xy - 0.5 * res) / min(res.y, res.x);
    
    // Apply the breathing zoom from Rust
    uv = uv / ui.zoom;

    // THE KALEIDOSCOPE FOLD
    // This bounces the coordinates around the center, ensuring that even 
    // if the fractal tries to move away, it is mirrored back into view.
    uv = abs(uv);
    if (uv.x < uv.y) { uv = uv.yx; }

    var z = uv;
    
    // THE MORPHING CONSTANT
    // Lower the 'morph_speed' variable to slow down the writhing tentacles.
    // E.g., 0.05 is very slow and dreamy. 0.5 is fast and chaotic.
    let morph_speed = 0.05; 
    
    let c = vec2<f32>(
        -0.4 + 0.05 * cos(ui.time * morph_speed), 
         0.6 + 0.05 * sin(ui.time * (morph_speed * 1.33)) // slightly offset for organic movement
    );
    
    var iter: f32 = 0.0;
    let max_iter: f32 = 150.0; 
    
    for (var j = 0; j < 150; j++) {
        z = vec2<f32>(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
        let mag_sq = dot(z, z);
        
        if (mag_sq > 25.0) {
            // Smooth fractional escape for liquid gradients
            let log_zn = log(mag_sq) / 2.0;
            let nu = log(log_zn / log(2.0)) / log(2.0);
            iter = iter + 1.0 - nu;
            break;
        }
        iter = iter + 1.0;
    }

    // Instead of rendering the interior as pure black, we give it a deep, glowing purple
    // so there are no "dead" pixels on the screen.
    if (iter >= max_iter) { return vec4<f32>(0.05, 0.0, 0.1, 1.0); }

    // Slow down the color "current" to match the slow zoom
    let color_speed = 0.5; 
    let t = iter * 0.1 - ui.time * color_speed; 
    
    let r = 0.5 + 0.5 * sin(t * 1.0);
    let g = 0.5 + 0.5 * sin(t * 1.2 + 2.0);
    let b = 0.5 + 0.5 * sin(t * 1.4 + 4.0);

    return vec4<f32>(r, g, b, 1.0);
}