use macroquad::prelude::*;
use std::f32::consts::PI;

const TAU: f32 = PI * 2.;
const STEPS: usize = 240;

static WHITE: Color = color_u8!(255, 255, 255, 255);

#[macroquad::main("Fourier")]
async fn main() {
    let step: f32 = TAU / STEPS as f32;

    let trigos: Vec<(f32, f32)> = vec![(100., 1.), (50., 2.), (25., 3.)];

    // let trigos: Vec<(f32, f32)> = (0..10)
    //     .map(|i| (200. / 2u32.pow(i) as f32, i as f32 + 1.))
    //     .collect();

    // let trigos: Vec<(f32, f32)> = vec![(100., 1.), (100., 2.), (100., 3.), (100., 4.)];

    // let trigos: Vec<(f32, f32)> = vec![(200., 1.), (100., 5.), (50., -1.)];

    // let trigos: Vec<(f32, f32)> = vec![(200., 1.), (200., -1.)];

    // let trigos: Vec<(f32, f32)> = vec![(200., 1.), (200., -2.)];

    // let trigos: Vec<(f32, f32)> = vec![(150., 1.), (150., -3.), (75., 5.)];

    // let trigos: Vec<(f32, f32)> = (0..20).map(|i| (10., -1. * i as f32)).collect();

    // let trigos: Vec<(f32, f32)> = (0..20)
    //     .map(|i| (i as f32 * 2., i as f32 / 10. * 2.))
    //     .collect();

    let mut points: Vec<(f32, f32)> = Vec::new();

    let mut t = 0;

    loop {
        clear_background(BLACK);

        {
            let mut pos = (500., 500.);

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
        }

        next_frame().await;
    }
}
