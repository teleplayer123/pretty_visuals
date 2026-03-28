use image::{Rgb, RgbImage};
use num_complex::Complex;
use rayon::prelude::*;
use std::fs;

// ffmpeg -framerate 30 -i frames/frame_%03d.png -c:v libx264 -pix_fmt yuv420p psychedelic_zoom.mp4

fn main() {
    let width = 800;
    let height = 800;
    let total_frames = 100;
    let output_dir = "frames";
    fs::create_dir_all(output_dir).unwrap();

    // The "Target" coordinates (a visually interesting area)
    let target_x = -0.743643887037158;
    let target_y = 0.131825904205311;
    let mut zoom = 1.0;

    println!("Generating {} psychedelic frames...", total_frames);

    for frame in 0..total_frames {
        let mut img = RgbImage::new(width, height);
        let scale = 2.0 / zoom;

        // Parallelize pixel calculation
        let pixels: Vec<(u32, u32, Rgb<u8>)> = (0..height)
            .into_par_iter()
            .flat_map(|y| {
                (0..width).into_par_iter().map(move |x| {
                    let cx = target_x + (x as f64 / width as f64 - 0.5) * scale;
                    let cy = target_y + (y as f64 / height as f64 - 0.5) * scale;
                    let color = get_color(cx, cy, frame);
                    (x, y, color)
                })
            })
            .collect();

        for (x, y, color) in pixels {
            img.put_pixel(x, y, color);
        }

        img.save(format!("{}/frame_{:03}.png", output_dir, frame)).unwrap();
        zoom *= 1.1; // Increase zoom for next frame
        println!("Frame {}/{} complete", frame + 1, total_frames);
    }
}

fn get_color(cx: f64, cy: f64, frame_count: u32) -> Rgb<u8> {
    let mut z = Complex::new(0.0, 0.0);
    let c = Complex::new(cx, cy);
    let max_iter = 255;
    let mut i = 0;

    while i < max_iter && z.norm_sqr() <= 4.0 {
        z = z * z + c;
        i += 1;
    }

    if i == max_iter {
        Rgb([0, 0, 0]) // Center of the set is black
    } else {
        // Psychedelic math: Use sine waves based on iteration count + frame shift
        let t = i as f64 + (frame_count as f64 * 0.5);
        let r = ((t * 0.1).sin() * 127.0 + 128.0) as u8;
        let g = ((t * 0.2).sin() * 127.0 + 128.0) as u8;
        let b = ((t * 0.3).sin() * 127.0 + 128.0) as u8;
        Rgb([r, g, b])
    }
}