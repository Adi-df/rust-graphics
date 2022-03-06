use macroquad::prelude::*;
use std::f32::consts::PI;

const TAU: f32 = PI * 2.;
const STEPS: usize = 240;

static WHITE: Color = color_u8!(255, 255, 255, 255);

#[macroquad::main("Fourier")]
async fn main() {
    let step: f32 = TAU / STEPS as f32;

    let mut trigos_fns = vec![
        vec![(100., 1.), (50., 2.), (25., 3.)],
        (0..10)
            .map(|i| (150. / 2u32.pow(i) as f32, i as f32 + 1.))
            .collect(),
        vec![(50., 1.), (50., 2.), (50., 3.), (50., 4.)],
        vec![(150., 1.), (75., 5.), (37.5, -1.)],
        vec![(100., 1.), (100., -1.)],
        vec![(150., 1.), (150., -2.)],
        vec![(100., 1.), (100., -3.), (50., 5.)],
        (0..20).map(|i| (10., -1. * i as f32)).collect(),
        (0..20)
            .map(|i| (i as f32 * 1.7, i as f32 / 10. * 2.))
            .collect(),
    ]
    .into_iter()
    .cycle();

    let mut trigos: Vec<(f32, f32)> = trigos_fns.next().unwrap();

    let mut points: Vec<(f32, f32)> = Vec::new();

    let mut t = 0;

    loop {
        clear_background(BLACK);

        {
            let mut pos = (400., 300.);

            trigos.iter().for_each(|(m, tm)| {
                let next_pos = (
                    pos.0 + m * (tm * t as f32 * step).cos(),
                    pos.1 + m * (tm * t as f32 * step).sin(),
                );
                draw_circle_lines(pos.0, pos.1, *m, 1., WHITE);
                draw_line(pos.0, pos.1, next_pos.0, next_pos.1, 1., WHITE);
                pos = next_pos;
            });
            points.push((pos.0, pos.1));
        }

        {
            let mut points_iter = points.iter().peekable();
            while let Some(point) = points_iter.next() {
                if let Some(next_point) = points_iter.peek() {
                    draw_line(point.0, point.1, next_point.0, next_point.1, 2., WHITE)
                }
            }
        }

        t += 1;

        if t == STEPS {
            t = 0;
            points = Vec::new();
            trigos = trigos_fns.next().unwrap();
        }

        next_frame().await;
    }
}
