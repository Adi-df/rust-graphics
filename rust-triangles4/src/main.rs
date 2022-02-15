use fltk::{
    app, draw,
    enums::{Align, Color, FrameType},
    frame::Frame,
    prelude::*,
    window::Window,
};

use rand::prelude::*;

use std::ops::Range;
use std::sync::{atomic::AtomicBool, Arc};
use std::thread;
use std::time::Duration;

const WIDTH: i32 = 1200;
const HEIGHT: i32 = 1000;

const TAU: f64 = std::f64::consts::PI * 2.;
const CIRCLE_STEPS: f64 = 360. * 2.;
const FPS: u64 = 1000;

const SIDES_RANGE: Range<u8> = 3..7;
const SIZE_RANGE: Range<f64> = 30.0..50.0;
const LENGTH_RANGE: Range<u16> = 150..250;
const MAX_SHAPES: u16 = 7 * 200;

type Point = (f64, f64);

#[derive(Clone, Debug)]
struct Shape {
    pos: Point,
    sides: u8,
    size: f64,
    angle: f64,
    color: u16,
    length: u16,
}

impl Shape {
    fn new(pos: Point, sides: u8, size: f64, angle: f64, color: u16, length: u16) -> Self {
        Self {
            pos,
            sides,
            size,
            angle,
            color,
            length,
        }
    }

    fn new_head(pos: Point, sides: u8, size: f64, angle: f64, length: u16) -> Self {
        Self::new(pos, sides, size, angle, length, length)
    }
}

fn main() {
    let mut rng = thread_rng();
    let create_head = Arc::new(AtomicBool::new(false));

    let app = app::App::default();
    let mut window = Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_align(Align::Center);

    let mut frame = Frame::new(0, 0, WIDTH, HEIGHT, "Triangles");
    frame.set_frame(FrameType::DownBox);
    frame.set_color(Color::Black);

    window.end();
    window.show();

    frame.draw({
        let mut shapes: Vec<Shape> = vec![];

        /*
        fn wave_color<T: Into<f64> + Copy>(x: T) -> Color {
            Color::rgb_color(
                ((x.into() / 255. * TAU).sin() * 127.5 + 127.5) as u8,
                ((x.into() / 255. * TAU + TAU / 3.).sin() * 127.5 + 127.5) as u8,
                ((x.into() / 255. * TAU + TAU / 3. * 2.).sin() * 127.5 + 127.5) as u8,
            )
        }
        */
        fn shade_color(x: u16, l: u16) -> Color {
            let color = (x as f64 / l as f64 * f64::from(u8::MAX)) as u8;
            Color::from_rgb(color, color, color)
        }

        let create_head = Arc::clone(&create_head);
        move |_this| {
            {
                if create_head.load(std::sync::atomic::Ordering::SeqCst) {
                    create_head.store(false, std::sync::atomic::Ordering::SeqCst);

                    if shapes.len() < MAX_SHAPES as usize {
                        let size = rng.gen_range(SIZE_RANGE);
                        shapes.push(Shape::new_head(
                            (rng.gen_range((0.)..(WIDTH as f64)), -size),
                            rng.gen_range(SIDES_RANGE),
                            size,
                            0.,
                            rng.gen_range(LENGTH_RANGE),
                        ));
                    }
                }
            }

            draw::draw_rect_fill(0, 0, WIDTH, HEIGHT, Color::Black);

            shapes.iter().for_each(
                |Shape {
                     pos,
                     sides,
                     angle,
                     size,
                     color,
                     length,
                 }| {
                    draw::set_draw_color(shade_color(*color, *length));
                    //draw::set_draw_color(wave_color(*color));
                    draw::begin_polygon();

                    (0..*sides).into_iter().for_each(|p| {
                        draw::vertex(
                            pos.0
                                + (TAU / (*sides as f64) * (p as f64) + (*angle as f64)).cos()
                                    * size,
                            pos.1
                                + (TAU / (*sides as f64) * (p as f64) + (*angle as f64)).sin()
                                    * size,
                        )
                    });

                    draw::end_polygon();
                },
            );

            shapes.retain(|Shape { color, .. }| *color != 0);
            shapes.retain(|Shape { pos, size, .. }| pos.1 - size < HEIGHT as f64);
            shapes.iter_mut().for_each(|shape| {
                shape.color -= 1;
            });

            shapes.extend::<Vec<Shape>>(
                shapes
                    .iter()
                    .filter(|Shape { color, length, .. }| *color == *length - 1)
                    .map(
                        |Shape {
                             pos,
                             sides,
                             angle,
                             size,
                             length,
                             ..
                         }| {
                            Shape::new(
                                (pos.0, pos.1 + 1.),
                                *sides,
                                *angle + TAU / CIRCLE_STEPS,
                                *size,
                                *length,
                                *length,
                            )
                        },
                    )
                    .collect(),
            );
        }
    });

    {
        let create_head = Arc::clone(&create_head);
        thread::spawn(move || loop {
            create_head.store(true, std::sync::atomic::Ordering::SeqCst);
            thread::sleep(Duration::from_millis(500));
        });
    }

    while app.wait() {
        app.redraw();
        thread::sleep(Duration::from_millis(1000 / FPS));
    }
}
