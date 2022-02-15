use fltk::{
    app, draw,
    enums::{Align, Color, FrameType},
    frame::Frame,
    prelude::*,
    window::Window,
};

use std::f64::consts::PI;
use std::thread;
use std::time::Duration;

const SIZE: i32 = 1000;

const CENTER: Point = (SIZE / 2, SIZE / 2);

const TAU: f64 = PI * 2.;

type Point = (i32, i32);
type CircleMaker = Box<dyn FnMut() -> GraphicCircle>;

struct GraphicCircle {
    angle: f64,
    radius: i32,
    color: Color,
}

fn main() {
    let app = app::App::default();
    let mut window = Window::default()
        .with_size(SIZE, SIZE)
        .with_align(Align::Center);

    let mut frame = Frame::new(0, 0, SIZE, SIZE, "Circles");
    frame.set_frame(FrameType::DownBox);
    frame.set_color(Color::Black);

    window.end();
    window.show();

    frame.draw({
        let mut circles: Vec<CircleMaker> = Vec::new();
        {
            let circle_maker = |x| {
                let mut angle: f64 = x;

                let sin_color = |a: f64| ((a).sin() + 1.) * 127.5;
                let get_color = move |a: f64| {
                    Color::from_rgb(
                        sin_color(a) as u8,
                        sin_color(a + TAU / 3.) as u8,
                        sin_color(a + TAU / 3. * 2.) as u8,
                    )
                };

                Box::new(move || {
                    angle = (angle + TAU / 60.) % TAU;
                    GraphicCircle {
                        angle,
                        color: get_color(angle),
                        radius: ((angle.sin()+2.)*10.) as i32,
                    }
                })
            };

            (0..10)
                .into_iter()
                .for_each(|v| circles.push(circle_maker(TAU / 10. * v as f64)));
        }

        draw::set_draw_color(Color::White);
        move |_this| {
            draw::draw_rect_fill(0, 0, SIZE, SIZE, Color::Black);

            {
                let mut distance = 30;

                let mut circles_iter = circles.iter_mut();
                while let Some(circle) = circles_iter.next() {
                    let GraphicCircle {
                        angle,
                        color,
                        radius,
                    } = circle();

                    let hypotenuse = (distance + radius) as f64;

                    draw::set_draw_color(color);
                    draw::draw_pie(
                        (angle.cos() * hypotenuse) as i32 - radius + CENTER.0,
                        (angle.sin() * hypotenuse) as i32 - radius + CENTER.1,
                        radius * 2,
                        radius * 2,
                        0.,
                        360.,
                    );
                    draw::draw_circle(
                        CENTER.0 as f64,
                        CENTER.1 as f64,
                        (distance + radius * 2 + 2) as f64,
                    );

                    distance += radius * 2 + 5;
                }
            }
        }
    });

    while app.wait() {
        app.redraw();
        thread::sleep(Duration::from_millis(1000 / 60));
    }
}
