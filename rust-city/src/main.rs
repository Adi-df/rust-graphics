use fltk::{
    app, draw,
    enums::{Align, Color, FrameType},
    frame::Frame,
    prelude::*,
    window::Window,
};

use rand::prelude::*;
use rand::thread_rng;

use color_space::{Hsv, Rgb};

use std::ops::Range;
use std::sync::{atomic::AtomicBool, Arc};
use std::thread;
use std::time::Duration;

const SIZE: i32 = 1000;
const FPS: u64 = 60;

const BUILDING_WIDTH: i32 = 100;
const BUILDING_NUMBER: i32 = SIZE / BUILDING_WIDTH;
const WINDOW_SIZE: (i32, i32) = (BUILDING_WIDTH / 2, BUILDING_WIDTH * 3 / 4);
const WINDOW_MARGIN: (i32, i32) = ((BUILDING_WIDTH - WINDOW_SIZE.0) / 2, BUILDING_WIDTH / 4);
const BUILDING_HEIGHT_RANGE: Range<i32> = 2..10;

const REMIX_DURATION: Duration = Duration::from_millis(300);

#[derive(Clone, Debug)]
struct BuildingWindow {
    pos: i32,
    color: Hsv,
}

#[derive(Clone, Debug)]
struct Building {
    pos: i32,
    color: Hsv,
    height: i32,
    windows: Vec<BuildingWindow>,
}

impl Building {
    fn new(pos: i32, color: Hsv, height: i32, windows: Vec<BuildingWindow>) -> Self {
        Self {
            pos,
            color,
            height,
            windows,
        }
    }
}

fn hsv_to_rgba(hsv: Hsv) -> (u8, u8, u8, u8) {
    let rgb = Rgb::from(hsv);
    (rgb.r as u8, rgb.g as u8, rgb.b as u8, 255)
}

fn main() {
    let mut rng = thread_rng();

    let app = app::App::default();
    let mut window = Window::default()
        .with_size(SIZE, SIZE)
        .with_align(Align::Center);

    let mut frame = Frame::new(0, 0, SIZE, SIZE, "Circles");
    frame.set_frame(FrameType::DownBox);
    frame.set_color(Color::Black);

    window.end();
    window.show();

    let remix = Arc::new(AtomicBool::new(true));

    frame.draw({
        let mut buildings: Vec<Building> = Vec::new();

        let remix = Arc::clone(&remix);

        move |_this| {
            if remix.load(std::sync::atomic::Ordering::SeqCst) {
                buildings = (0..BUILDING_NUMBER)
                    .map(|i| {
                        let height = rng.gen_range(BUILDING_HEIGHT_RANGE);
                        Building::new(
                            i,
                            Hsv::new(rng.gen_range(0f64..360.), 0.5, 1.),
                            height,
                            (0..height)
                                .map(|i| BuildingWindow {
                                    pos: i,
                                    color: Hsv::new(rng.gen_range(0f64..360.), 0.5, 1.),
                                })
                                .collect(),
                        )
                    })
                    .collect();

                remix.store(false, std::sync::atomic::Ordering::SeqCst);
            }

            buildings.iter().for_each(|building| {
                let real_height =
                    (WINDOW_SIZE.1 + WINDOW_MARGIN.1) * building.height + WINDOW_MARGIN.1;

                draw::set_draw_color(Color::from_rgba_tuple(hsv_to_rgba(building.color)));
                draw::draw_rect(
                    building.pos * BUILDING_WIDTH,
                    SIZE - real_height,
                    BUILDING_WIDTH,
                    real_height,
                );

                building.windows.iter().for_each(|window| {
                    draw::set_draw_color(Color::from_rgba_tuple(hsv_to_rgba(window.color)));
                    draw::draw_rectf(
                        building.pos * BUILDING_WIDTH + WINDOW_MARGIN.0,
                        SIZE - real_height
                            + WINDOW_MARGIN.1
                            + (WINDOW_SIZE.1 + WINDOW_MARGIN.1) * window.pos,
                        WINDOW_SIZE.0,
                        WINDOW_SIZE.1,
                    );
                })
            });

            buildings.iter_mut().for_each(|building| {
                building.color.h += rng.gen_range(-1..1) as f64;
                building.color.h %= 360.;
                if building.color.h < 0. {
                    building.color.h += 360.;
                }

                building.windows.iter_mut().for_each(|window| {
                    window.color.h += rng.gen_range(-1..1) as f64;
                    window.color.h %= 360.;
                    if window.color.h < 0. {
                        window.color.h += 360.;
                    }
                })
            });
        }
    });

    thread::spawn(move || loop {
        remix.store(true, std::sync::atomic::Ordering::SeqCst);
        thread::sleep(REMIX_DURATION);
    });

    while app.wait() {
        app.redraw();
        thread::sleep(Duration::from_millis(1000 / FPS));
    }
}
