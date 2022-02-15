use macroquad::prelude::*;

static BLACK: Color = Color {
    r: 0.,
    g: 0.,
    b: 0.,
    a: 255.,
};

type Point = (f32, f32);
type Pixel = (Point, Color);

struct Raimbow {
    hue: u16,
}
impl Raimbow {
    fn new(hue: u16) -> Self {
        Self { hue }
    }
}
impl Iterator for Raimbow {
    type Item = Color;

    fn next(&mut self) -> Option<Self::Item> {
        self.hue += 1;
        if self.hue >= 360 {
            self.hue = 0
        }

        Some(match self.hue {
            h if h < 60 => Color::from_rgba(255, h as u8 * 4, 0, 255),
            h if h < 120 => Color::from_rgba(255 - (h - 60) as u8 * 4, 255, 0, 255),
            h if h < 180 => Color::from_rgba(0, 255, (h - 120) as u8 * 4, 255),
            h if h < 240 => Color::from_rgba(0, 255 - (h - 180) as u8 * 4, 255, 255),
            h if h < 300 => Color::from_rgba((h - 240) as u8 * 4, 0, 255, 255),
            h => Color::from_rgba(255, 0, 255 - (h - 300) as u8 * 4, 255),
        })
    }
}

fn draw_pixel(pixel: Pixel) {
    draw_rectangle(pixel.0 .0, pixel.0 .1, 1., 1., pixel.1);
}

fn bezier(p0: Point, p1: Point, t: f32) -> Point {
    (p0.0 + (p1.0 - p0.0) * t, p0.1 + (p1.1 - p0.1) * t)
}

fn recurse_bezier(points: &[Point], t: f32) -> Point {
    let mut points = points.iter().peekable();

    let mut computed = Vec::new();
    while let Some(point) = points.next() {
        if let Some(next) = points.peek() {
            computed.push(bezier(*point, **next, t));
        }
    }

    match computed.len() {
        x if x > 1 => recurse_bezier(&computed, t),
        1 => computed[0],
        _ => (0., 0.),
    }
}

#[macroquad::main("Bezier curve")]
async fn main() {
    run().await;
}

async fn run() {
    let mut points: Vec<Point> = Vec::new();

    loop {
        draw_rectangle(0., 0., 800., 600., BLACK);

        if is_mouse_button_pressed(MouseButton::Left) {
            points.push(mouse_position());
        } else if is_mouse_button_pressed(MouseButton::Right) {
            points.pop();
        }

        if is_mouse_button_down(MouseButton::Middle) {
            points.push(mouse_position());
        }

        let mut color = Raimbow::new(1);
        (0..1000)
            .into_iter()
            .map(|x| x as f32 / 1000.)
            .for_each(|x| draw_pixel((recurse_bezier(&points, x), color.next().unwrap())));

        if is_mouse_button_down(MouseButton::Middle) {
            points.pop();
        }

        next_frame().await;
    }
}
