use image::{Rgb, RgbImage};
use num_complex::Complex;
use rayon::prelude::*;
use noise::{NoiseFn, Perlin};
use std::fs;

// ffmpeg -framerate 60 -i frames/frame_%03d.png -c:v libx264 -crf 18 -pix_fmt yuv420p output.mp4

fn main() {
    let (width, height) = (800, 800);
    let total_frames = 600;
    let output_dir = "frames";
    fs::create_dir_all(output_dir).unwrap();

    let perlin = Perlin::new(1);
    let mut feedback_buffer: Vec<f32> = vec![0.0; (width * height * 3) as usize];

    println!("Generating Julia Set...");

    for frame in 0..total_frames {
        // Calculate Shimmer Factor
        // use Perlin noise to get a smooth, organic "pulse"
        let noise_val = perlin.get([frame as f64 * 0.1, 0.0, 0.0]); 
        let shimmer = (noise_val * 0.5 + 0.5) as f32; // Map to 0.0 - 1.0

        let angle = frame as f64 * 0.04;
        let c = Complex::new(0.355 + 0.1 * angle.cos(), 0.355 + 0.1 * angle.sin());

        // Render Frame
        let raw_pixels: Vec<u8> = (0..height).into_par_iter().flat_map(|y| {
            (0..width).into_par_iter().flat_map(move |x| {
                let z = Complex::new(
                    1.5 * (x as f64 - width as f64 / 2.0) / (0.4 * width as f64),
                    1.5 * (y as f64 - height as f64 / 2.0) / (0.4 * height as f64)
                );
                
                // Pass shimmer to the color function
                calculate_julia_pixel(z, c, frame, shimmer)
            })
        }).collect();

        // Temporal Blur & Bloom 
        let mut final_img = RgbImage::new(width, height);
        for i in 0..feedback_buffer.len() {
            // Shimmer also affects trail persistence (lower shimmer = shorter trails)
            let decay = 0.1 + (shimmer * 0.2); 
            feedback_buffer[i] = (raw_pixels[i] as f32 * (1.0 - decay)) + (feedback_buffer[i] * decay);
        }

        // Apply a quick glow and save
        let mut blurred: Vec<[u8; 3]> = feedback_buffer.chunks(3)
            .map(|c| [c[0] as u8, c[1] as u8, c[2] as u8]).collect();
        fastblur::gaussian_blur(&mut blurred, width as usize, height as usize, 3.0);

        for (i, rgb) in blurred.iter().enumerate() {
            let x = (i as u32) % width;
            let y = (i as u32) / width;
            let base_r = feedback_buffer[i*3] as u8;
            let base_g = feedback_buffer[i*3+1] as u8;
            let base_b = feedback_buffer[i*3+2] as u8;

            // Additive blend with a "shimmer boost"
            let r = base_r.saturating_add((rgb[0] as f32 * shimmer) as u8);
            let g = base_g.saturating_add((rgb[1] as f32 * shimmer) as u8);
            let b = base_b.saturating_add((rgb[2] as f32 * shimmer) as u8);
            
            final_img.put_pixel(x, y, Rgb([r, g, b]));
        }

        final_img.save(format!("{}/frame_{:03}.png", output_dir, frame)).unwrap();
        println!("Frame {} | Shimmer Intensity: {:.2}", frame, shimmer);
    }
}

fn calculate_julia_pixel(mut z: Complex<f64>, c: Complex<f64>, frame: u32, shimmer: f32) -> Vec<u8> {
    let mut i = 0;
    while i < 255 && z.norm_sqr() <= 4.0 {
        z = z * z + c;
        i += 1;
    }
    if i == 255 { return vec![0, 0, 0]; }

    // Use shimmer to expand/contract the color palette vibrancy
    let freq = 0.1 + (shimmer * 0.05) as f64;
    let r = ((i as f64 * freq + frame as f64 * 0.05).sin() * 127.0 + 128.0) as u8;
    let g = ((i as f64 * freq + 2.0).sin() * 127.0 + 128.0) as u8;
    let b = ((i as f64 * freq + 4.0).sin() * 127.0 + 128.0) as u8;
    
    // Apply a brightness multiplier based on shimmer
    let brightness = 0.7 + (shimmer * 0.3);
    vec![
        (r as f32 * brightness) as u8,
        (g as f32 * brightness) as u8,
        (b as f32 * brightness) as u8,
    ]
}