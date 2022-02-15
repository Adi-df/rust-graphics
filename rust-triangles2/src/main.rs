#![feature(total_cmp)]

use fltk::{
    app, draw,
    enums::{Align, Color, FrameType},
    frame::Frame,
    prelude::*,
    window::Window,
};

use std::thread;
use std::time::Duration;

use rand::prelude::*;

const WIDTH: i32 = 1200;
const HEIGHT: i32 = 1000;

const POINT_NUMBER: u32 = 100;
const TRIANGLE_DISTANCE: f64 = 100.0;

type Point = (i32, i32);
type Speed = (i32, i32);

fn distance(p1: Point, p2: Point) -> f64 {
    ((p2.0 - p1.0).pow(2) as f64 + (p2.1 - p1.1).pow(2) as f64).sqrt()
}

fn compute_distance(p: Point, points: &Vec<Point>) -> Vec<(Point, f64)> {
    points
        .iter()
        .cloned()
        .map(|point| (point, distance(p, point)))
        .collect::<Vec<(Point, f64)>>()
}

fn sort_by_distance(p: Point, points: &Vec<Point>) -> Vec<(Point, f64)> {
    let mut distances = compute_distance(p, points);
    distances.sort_by(|p1, p2| p1.1.total_cmp(&p2.1));
    distances
}

fn main() {
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
        let mut rng = rand::thread_rng();

        let mut points: Vec<(Point, Speed)> = Vec::new();

        (0..POINT_NUMBER).for_each(|_x| {
            points.push((
                (rng.gen_range(0..=WIDTH), rng.gen_range(0..=HEIGHT)),
                (rng.gen_range(-1..=1), rng.gen_range(-1..=1)),
            ));
        });

        move |_this| {
            draw::draw_rect_fill(0, 0, WIDTH, HEIGHT, Color::Black);

            points
                .iter()
                .for_each(|p| draw::draw_rect_fill(p.0 .0, p.0 .1, 1, 1, Color::White));

            points.iter_mut().for_each(|(p, s)| {
                p.0 = (p.0 + s.0).min(WIDTH).max(0);
                p.1 = (p.1 + s.1).min(HEIGHT).max(0);

                s.0 = (s.0 + rng.gen_range(-1..=1)).min(3).max(-3);
                s.1 = (s.1 + rng.gen_range(-1..=1)).min(3).max(-3);
            });

            draw::set_draw_color(Color::White);
            points
                .iter()
                .map(|point| {
                    (
                        point,
                        sort_by_distance(point.0, &points.iter().map(|p| p.0).collect()),
                    )
                })
                .for_each(|(point, nearest)| {
                    if nearest[1].1 < TRIANGLE_DISTANCE
                        && nearest[2].1 < TRIANGLE_DISTANCE
                        && distance(nearest[1].0, nearest[2].0) < TRIANGLE_DISTANCE
                    {
                        draw::draw_polygon(
                            point.0.0,
                            point.0.1,
                            nearest[1].0.0,
                            nearest[1].0.1,
                            nearest[2].0.0,
                            nearest[2].0.1,
                        );
                    }
                    draw::draw_line(point.0.0, point.0.1, nearest[1].0.0, nearest[1].0.1);
                });
        }
    });

    while app.wait() {
        app.redraw();
        thread::sleep(Duration::from_millis(1000 / 60));
    }
}
