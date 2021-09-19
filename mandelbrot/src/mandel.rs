// Rendering to a window
extern crate minifb;
use minifb::{Key, MouseMode, Window, WindowOptions};

// sleep between frames
use std::time::Instant;
use std::time::Duration;

use std::cmp::min;

fn main() {
    const IMGX: usize = 720;
    const IMGY: usize = 720;
    const MINX: f64 = -1.6;
    const MAXX: f64 =  1.6;
    const MINY: f64 = -1.6;
    const MAXY: f64 =  1.6;
    const SX: f64 = (MAXX-MINX) / IMGX as f64;
    const SY: f64 = (MAXY-MINY) / IMGY as f64;

    const RATIOX: f64 = 180.0 / IMGX as f64;
    const RATIOY: f64 = 180.0 / IMGY as f64;

    const ONE_FRAME: Duration = Duration::from_nanos(1_000_000_000 / 30);

    // Cool-looking values of c
    // let c = (-0.4, 0.598); // Use `i += 1;`
    // let c = (0.0, 0.8);
    // let c = (-0.835, -0.2321);
    // let c = (0.285, 0.0);
    // let c = (-0.70176, -0.3842); // Use `i += 3;`
    // let c = (-0.8, 0.156); // Looks cooler with better coloring
    // let c = (-0.7269, 0.1889); // ^^^
    // let c = (-0.1737, -1.06735);
    // let c = (0.33, 0.45);
    // let c = (0.43, 0.352);
    // let c = (-1.03, -0.39);
    // let c = (0.39, 0.12);
    // let c = (0.16, 0.62);
    // let c = (0.36, 0.644);
    // let c = (-0.14, 0.88);
    // let c = (0.33, -0.03);
    // let c = (-0.23, -0.8);
    // let c = (0.32, 0.06);
    // let c = (-0.125, 0.75);
    let c = (-0.044, 2./3.);

    let mut buffer: Vec<u32> = vec![0; IMGX * 2*IMGY];

    for x in 0..IMGX {
        for y in 0..IMGY {
            let cx = y as f64 * SX + MINX;
            let cy = x as f64 * SY + MINY;

            let mut z = (cx, cy);
            let mut i = 0_u32;
            while i < 255 {
                let (a, b) = z; // z = a + bi
                if a*a+b*b > 4.0 { break; }
                z = (
                    a*a - b*b + cx as f64,
                    2.0 * a*b + cy as f64
                );

                i += 3;
            }

            let d = min(IMGX-x, IMGY-y);
            let (r, b) = (x+d, y+d);

            let red = (r as f64 * RATIOX) as u32;
            let green = i;
            let blue = (b as f64 * RATIOY) as u32;

            // let red = (0.3 * x as f32) as u32;
            // let green = i;
            // let blue = (0.3 * y as f32) as u32;

            buffer[x*2*IMGY+y] =
                red   << 16 |
                green << 08 |
                blue  << 00;
        }
    }

    let mut window = Window::new(
        "Mandelbrot/Julia Sets",
        2*IMGY,
        IMGX,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });


    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(ONE_FRAME));

    // let mut frame = 0;
    let mut c = (0.0, 0.0);
    while window.is_open() && !window.is_key_down(Key::Escape) {

        if !window.is_key_down(Key::Space) {

            let start_frame = Instant::now();

            // let f = frame as f64 / 720.0;
            // let c = (0.7885*f.cos(), 0.7885*f.sin());

            window.get_mouse_pos(MouseMode::Discard).map(|(x, y)| {
                if x < IMGX as f32 {
                    let (x, y) = (x as f64, y as f64);
                    c = (x * SX + MINX, y * SY + MINY);
                }
            });

            for x in 0..IMGX {
                for y in 0..IMGY {
                    let cx = y as f64 * SX + MINX;
                    let cy = x as f64 * SY + MINY;

                    let mut z = (cx, cy);
                    let mut i = 0_u32;
                    while i < 255 && z.0*z.0+z.1*z.1 <= 4.0 {
                        let (a, b) = z; // z = a + bi
                        z = (
                            a*a - b*b + c.0,
                            2.0*a*b + c.1
                        );

                        // Use 3 or 5, but 5 looks better
                        i += 5;
                    }

                    let d = min(IMGX-x, IMGY-y);
                    let (r, b) = (x+d, y+d);

                    let red = (r as f64 * RATIOX) as u32;
                    let green = i;
                    let blue = (b as f64 * RATIOY) as u32;

                    // let red = (0.3 * x as f32) as u32;
                    // let green = i;
                    // let blue = (0.3 * y as f32) as u32;

                    buffer[x*2*IMGY+y+IMGX] =
                        red   << 16 |
                        green << 08 |
                        blue  << 00;
                }
            }

            eprintln!(
                "{:?} elapsed this frame.\tc = ({}, {})",
                start_frame.elapsed(),        c.0, c.1
            );
            
            // if window.is_key_down(Key::Comma) { frame -= 4; } // Shift + ',' --> '<'
            // else if window.is_key_down(Key::Period) { frame += 4; } // Shift + '.' --> '>'
            // else if window.is_key_down(Key::Left) { frame -= 1; }
            // else { frame += 1; }
        }
        
        window.update_with_buffer(&buffer, 2*IMGY, IMGX).unwrap();
    }
}