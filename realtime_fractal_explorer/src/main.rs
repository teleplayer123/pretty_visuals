use num_complex::Complex;
use pixels::{Pixels, SurfaceTexture};
use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 600;

struct World {
    zoom: f64,
    target_x: f64,
    target_y: f64,
    c: Complex<f64>,
    frame: u32,
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Rust Fractal VJ Tool")
        .with_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .build(&event_loop)
        .unwrap();

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };

    let mut world = World {
        zoom: 1.0,
        target_x: 0.0,
        target_y: 0.0,
        c: Complex::new(-0.8, 0.156),
        frame: 0,
    };

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.frame_mut());
            if pixels.render().is_err() { *control_flow = ControlFlow::Exit; return; }
        }

        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(key) = input.virtual_keycode {
                        let step = 0.1 / world.zoom;
                        match key {
                            VirtualKeyCode::W => world.zoom *= 1.05,
                            VirtualKeyCode::S => world.zoom /= 1.05,
                            VirtualKeyCode::Up => world.target_y -= step,
                            VirtualKeyCode::Down => world.target_y += step,
                            VirtualKeyCode::Left => world.target_x -= step,
                            VirtualKeyCode::Right => world.target_x += step,
                            VirtualKeyCode::A => world.c.re -= 0.01,
                            VirtualKeyCode::D => world.c.re += 0.01,
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
            window.request_redraw();
        }
        world.frame += 1;
    });
}

impl World {
    fn draw(&self, frame_buffer: &mut [u8]) {
        let scale = 3.0 / self.zoom;
        
        // We use standard loops here for simplicity, but Rayon could be added back
        for (i, pixel) in frame_buffer.chunks_exact_mut(4).enumerate() {
            let x = (i % WIDTH as usize) as f64;
            let y = (i / WIDTH as usize) as f64;

            // Coordinate transform with Kaleidoscope Fold
            let nx = ((x / WIDTH as f64 - 0.5) * scale + self.target_x).abs();
            let ny = ((y / HEIGHT as f64 - 0.5) * scale + self.target_y).abs();

            let mut z = Complex::new(nx, ny);
            let mut iter = 0;
            while iter < 64 && z.norm_sqr() <= 4.0 {
                z = z * z + self.c;
                iter += 1;
            }

            let r = (iter * 4) as u8;
            let g = (iter * 8) as u8;
            let b = (iter * 2) as u8;

            pixel.copy_from_slice(&[r, g, b, 255]); // RGBA
        }
    }
}
