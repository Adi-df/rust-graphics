#![feature(total_cmp)]

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

use rand::prelude::*;

use std::time::Duration;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

pub fn distance(p1: (i32, i32), p2: (i32, i32)) -> f64 {
    ((p2.0 - p1.0).pow(2) as f64 + (p2.1 - p1.1).pow(2) as f64).sqrt()
}

pub fn nearest(p: (i32, i32), list: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut distances = list
        .iter()
        .cloned()
        .map(|p2| (p2, distance(p, p2)))
        .collect::<Vec<((i32, i32), f64)>>();
    distances.sort_by(|(_, a), (_, b)| a.total_cmp(&b));
    distances.iter().map(|x| x.0).collect()
}

pub fn random_speeds(list: &mut Vec<(i32, i32)>) -> () {
    let mut rng = rand::thread_rng();
    list.iter_mut().for_each(|p| {
        p.0 = (p.0 + rng.gen_range(-1..=1)).min(3).max(-3);
        p.1 = (p.1 + rng.gen_range(-1..=1)).min(3).max(-3);
    });
}

pub fn move_points(points: &mut Vec<(i32, i32)>, speeds: &Vec<(i32, i32)>) -> () {
    points.iter_mut().enumerate().for_each(|(i, (px,py))| {
        *px = (*px + speeds[i].0).max(0).min(WIDTH);
        *py = (*py + speeds[i].1).max(0).min(HEIGHT);
    });
}

pub fn main() {
    let mut rng = rand::thread_rng();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut points: Vec<(i32, i32)> = Vec::new();
    let mut speeds: Vec<(i32, i32)> = Vec::new();

    (0..30).for_each(|_| {
        points.push((
            rng.gen_range(0..(WIDTH)),
            rng.gen_range(0..(HEIGHT)),
        ));
        speeds.push((
            rng.gen_range(-3..=3),
            rng.gen_range(-3..=3),
        ));
    });

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        random_speeds(&mut speeds);
        move_points(&mut points, &speeds);
        //points[0] = (0, 0);
        //points[1] = (WIDTH, 0);
        //points[2] = (0, HEIGHT);
        //points[3] = (WIDTH, HEIGHT as i32);

        canvas.set_draw_color(Color::RGB(255, 0, 0));
        points.iter().for_each(|p| canvas.draw_point(*p).unwrap());

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        points.iter().for_each(|p| {
            let nearest_points = nearest(*p, &points);
            canvas.draw_line(*p, nearest_points[0]).unwrap();
            canvas.draw_line(*p, nearest_points[1]).unwrap();
            canvas.draw_line(*p, nearest_points[2]).unwrap();
        });

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
