use fltk::{
    app, draw,
    enums::{Align, Color, FrameType},
    frame::Frame,
    prelude::*,
    window::Window,
};

use rand::prelude::*;

use std::ops::Range;
use std::thread;
use std::time::Duration;

const WIDTH: i32 = 1200;
const HEIGHT: i32 = 1000;

const TAU: f64 = std::f64::consts::PI * 2.;
const QUARTER: f64 = TAU / 4.;
const FPS: u64 = 60;

const CLOCK_SIZE: (i32, i32) = (50, 50);
const CLOCK_NUMBER: (i32, i32) = (WIDTH / CLOCK_SIZE.0 / 2, HEIGHT / CLOCK_SIZE.1 / 2);
const CLOCK_MARGIN: (i32, i32) = (
    (WIDTH - CLOCK_NUMBER.0 * CLOCK_SIZE.0) / (CLOCK_NUMBER.0 + 2),
    (HEIGHT - CLOCK_NUMBER.1 * CLOCK_SIZE.1) / (CLOCK_NUMBER.1 + 2),
);

const SPEED_RANGE: Range<u8> = 1..10;
const DURATION: u64 = FPS * 3;

type Point = (i32, i32);

#[derive(Clone, Debug)]
struct Clock {
    pos: Point,
    sec: u8,
    min: u8,
    hour: u8,
    speed: u8,
}

impl Clock {
    fn new(pos: Point, speed: u8, sec: u8, min: u8, hour: u8) -> Self {
        Self {
            pos,
            speed,
            sec,
            min,
            hour,
        }
    }
}

fn main() {
    let mut rng = thread_rng();

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
        let mut clock: Vec<Clock> = vec![];

        (0..CLOCK_NUMBER.0).for_each(|x| {
            (0..CLOCK_NUMBER.1).for_each(|y| {
                let speed = rng.gen_range(SPEED_RANGE);

                let start_time = speed as f64 * DURATION as f64;

                clock.push(Clock::new(
                    (
                        CLOCK_MARGIN.0 * (x + 1) + CLOCK_SIZE.0 * x,
                        CLOCK_MARGIN.1 * (y + 1) + CLOCK_SIZE.1 * y,
                    ),
                    speed,
                    (60. - (start_time % 60.)).abs() as u8,
                    (60. - (start_time / 60. % 60.)).abs() as u8,
                    (12. - (start_time / 60. / 60. % 12.)).abs() as u8,
                ));
            })
        });

        move |_this| {
            draw::draw_rect_fill(0, 0, WIDTH, HEIGHT, Color::Black);

            clock.iter().for_each(
                |Clock {
                     pos,
                     sec,
                     min,
                     hour,
                     ..
                 }| {
                    draw::set_draw_color(Color::White);

                    let center = (
                        (pos.0 + CLOCK_SIZE.0 / 2) as f64,
                        (pos.1 + CLOCK_SIZE.1 / 2) as f64,
                    );
                    let radius = CLOCK_SIZE.0 as f64 / 2.;
                    draw::draw_circle(center.0, center.1, radius);

                    draw::draw_line(
                        center.0 as i32,
                        center.1 as i32,
                        center.0 as i32
                            + ((TAU / 60. * *sec as f64 - QUARTER).cos() * (radius - 1.)) as i32,
                        center.1 as i32
                            + ((TAU / 60. * *sec as f64 - QUARTER).sin() * (radius - 1.)) as i32,
                    );
                    draw::draw_line(
                        center.0 as i32,
                        center.1 as i32,
                        center.0 as i32
                            + ((TAU / 60. * *min as f64 - QUARTER).cos() * (radius - 3.)) as i32,
                        center.1 as i32
                            + ((TAU / 60. * *min as f64 - QUARTER).sin() * (radius - 3.)) as i32,
                    );
                    draw::draw_line(
                        center.0 as i32,
                        center.1 as i32,
                        center.0 as i32
                            + ((TAU / 12. * *hour as f64 - QUARTER).cos() * (radius - 3.)) as i32,
                        center.1 as i32
                            + ((TAU / 12. * *hour as f64 - QUARTER).sin() * (radius - 3.)) as i32,
                    );
                },
            );

            clock.iter_mut().for_each(
                |Clock {
                     sec,
                     min,
                     hour,
                     speed,
                     ..
                 }| {
                    *sec += *speed;
                    if *sec >= 60 {
                        *sec %= 60;
                        *min += 1;

                        if *min >= 60 {
                            *min %= 60;
                            *hour += 1;
                            if *hour >= 12 {
                                *hour %= 12;
                            }
                        }
                    }
                },
            );
        }
    });

    let mut frame = 0;
    while app.wait() {
        if frame < DURATION + 2 {
            frame += 1;
            app.redraw();
        }
        thread::sleep(Duration::from_millis(1000 / FPS));
    }
}
