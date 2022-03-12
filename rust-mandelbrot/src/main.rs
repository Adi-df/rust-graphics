use macroquad::prelude::*;
use num_complex::Complex;
use palette::{Hsv, IntoColor, Srgb};

const ITERATIONS: u32 = 50;
const BASE_SCALE: f64 = 4.;
const MOVE_SCALE: f64 = 8.;

#[macroquad::main("rust-mandelbrot")]
async fn main() {
    let mut scale = BASE_SCALE;
    let mut offset = (0., 0.);

    let mut last_scale = 0.;
    let mut last_offset = (-1., -1.);
    let mut last_mandelbrot_size = 0.;
    let mut last_mouse_position = (-1., -1.);

    let mut mandelbrot_texture = Texture2D::empty();
    let mut julia_texture = Texture2D::empty();
    loop {
        let (width, height) = (screen_width() as u32, screen_height() as u32);
        let mandelbrot_size = (width / 2).min(height) as f64;

        //Scroll
        {
            if mouse_wheel().1 == -1. {
                scale *= 2.;
            } else if mouse_wheel().1 == 1. {
                scale /= 2.;
            }
        }

        // Move
        {
            if is_key_down(KeyCode::Left) {
                offset.0 -= scale / MOVE_SCALE;
            } else if is_key_down(KeyCode::Right) {
                offset.0 += scale / MOVE_SCALE;
            }
            if is_key_down(KeyCode::Up) {
                offset.1 -= scale / MOVE_SCALE;
            } else if is_key_down(KeyCode::Down) {
                offset.1 += scale / MOVE_SCALE;
            }
        }

        // Draw mandelbrot set
        if scale != last_scale || mandelbrot_size != last_mandelbrot_size || offset != last_offset {
            mandelbrot_texture = {
                let bytes: Vec<u8> = (0..(mandelbrot_size.floor() as u32).pow(2))
                    .flat_map(|i| {
                        let (x, y) = (
                            i % mandelbrot_size.floor() as u32,
                            i / mandelbrot_size.floor() as u32,
                        );
                        let ni = {
                            let c = Complex::new(
                                x as f64 / mandelbrot_size * scale
                                    - scale / 2.
                                    - scale / MOVE_SCALE
                                    + offset.0,
                                y as f64 / mandelbrot_size * scale - scale / 2. + offset.1,
                            );
                            let mut z = Complex::new(0., 0.);

                            let mut i = 0;
                            while i < ITERATIONS && z.norm() < 2. {
                                i += 1;
                                z = z.powu(2) + c;
                            }
                            i
                        };

                        if ni == ITERATIONS {
                            [0, 0, 0, 255]
                        } else {
                            let rgb: Srgb =
                                Hsv::new(ni as f32 * 360. / ITERATIONS as f32, 1., 1.).into_color();
                            [
                                (rgb.red * 255.) as u8,
                                (rgb.green * 255.) as u8,
                                (rgb.blue * 255.) as u8,
                                255,
                            ]
                        }
                    })
                    .collect();
                Texture2D::from_rgba8(
                    mandelbrot_size.floor() as u16,
                    mandelbrot_size.floor() as u16,
                    &bytes,
                )
            };
        }

        // Draw julia
        if scale != last_scale
            || mandelbrot_size != last_mandelbrot_size
            || mouse_position() != last_mouse_position
            || offset != last_offset
        {
            julia_texture = {
                let (mouse_x, mouse_y) = mouse_position();
                let c = Complex::new(
                    mouse_x as f64 / mandelbrot_size * scale - scale / 2. - scale / MOVE_SCALE
                        + offset.0,
                    mouse_y as f64 / mandelbrot_size * scale - scale / 2. + offset.1,
                );
                let bytes: Vec<u8> = (0..(mandelbrot_size as u32).pow(2))
                    .flat_map(|i| {
                        let (x, y) = (
                            i % mandelbrot_size.floor() as u32,
                            i / mandelbrot_size.floor() as u32,
                        );
                        let ni = {
                            let mut z = Complex::new(
                                x as f64 / mandelbrot_size * BASE_SCALE - BASE_SCALE / 2.,
                                y as f64 / mandelbrot_size * BASE_SCALE - BASE_SCALE / 2.,
                            );

                            let mut i = 0;
                            while i < ITERATIONS && z.norm() < 2. {
                                i += 1;
                                z = z.powu(2) + c;
                            }
                            i
                        };

                        if ni == ITERATIONS {
                            [0, 0, 0, 255]
                        } else {
                            let rgb: Srgb =
                                Hsv::new(ni as f32 * 360. / ITERATIONS as f32, 1., 1.).into_color();
                            [
                                (rgb.red * 255.) as u8,
                                (rgb.green * 255.) as u8,
                                (rgb.blue * 255.) as u8,
                                255,
                            ]
                        }
                    })
                    .collect();
                Texture2D::from_rgba8(
                    mandelbrot_size.floor() as u16,
                    mandelbrot_size.floor() as u16,
                    &bytes,
                )
            };
        }

        last_scale = scale;
        last_offset = offset;
        last_mouse_position = mouse_position();
        last_mandelbrot_size = mandelbrot_size;

        draw_texture(mandelbrot_texture, 0., 0., WHITE);
        draw_texture(julia_texture, mandelbrot_size.ceil() as f32, 0., WHITE);

        // Number
        {
            let (mouse_x, mouse_y) = mouse_position();
            let c = Complex::new(
                mouse_x as f64 / mandelbrot_size * scale - scale / 2. + offset.0,
                mouse_y as f64 / mandelbrot_size * scale - scale / 2. + offset.1,
            );
            draw_text(
                &format!("c = {} + {}i", c.re, c.im),
                mandelbrot_size as f32,
                20.,
                16.,
                WHITE,
            )
        }

        next_frame().await
    }
}
