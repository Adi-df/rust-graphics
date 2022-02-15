use std::io::stdout;
use std::ops::Range;
use std::time::Duration;

use rand::prelude::*;
use rand::thread_rng;

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event},
    style::{Color, Print, SetBackgroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, SetSize},
    ExecutableCommand, Result,
};

const BOUNCING_FORCE: Range<f32> = 0.25..0.5;
const DRIP_NUMBER: f64 = 1. / 8.;

type Pixel = (u16, u16, Color);

struct Drip {
    pos: (f32, f32),
    speed: (f32, f32),
    color: Color,
}

impl From<&Drip> for Pixel {
    fn from(drip: &Drip) -> Pixel {
        (drip.pos.0 as u16, drip.pos.1 as u16, drip.color)
    }
}

impl Drip {
    fn new(pos: (f32, f32), speed: (f32, f32), color: Color) -> Self {
        Self { pos, speed, color }
    }
}

fn draw_pixels(pixels: &Vec<Pixel>) -> () {
    stdout()
        .execute(SetBackgroundColor(Color::Black))
        .unwrap()
        .execute(Clear(ClearType::All))
        .unwrap();

    pixels.iter().for_each(|p| {
        stdout()
            .execute(MoveTo(p.0, p.1))
            .unwrap()
            .execute(SetBackgroundColor(p.2.clone()))
            .unwrap()
            .execute(Print(" "))
            .unwrap();
    });

    stdout()
        .execute(MoveTo(size().unwrap().0, size().unwrap().1))
        .unwrap();
}

fn random_color() -> Color {
    match thread_rng().gen_range(0..7) {
        1 => Color::Red,
        2 => Color::Blue,
        3 => Color::Green,
        4 => Color::Magenta,
        5 => Color::Cyan,
        6 => Color::Yellow,
        _ => Color::White,
    }
}

fn main() -> Result<()> {
    stdout()
        .execute(SetSize(30, 30))?
        .execute(SetBackgroundColor(Color::Black))?
        .execute(Clear(ClearType::All))?
        .execute(Hide)?;

    enable_raw_mode()?;

    let mut rain: Vec<Drip> = Vec::new();
    let mut rng = thread_rng();

    loop {
        let screen_size = size().unwrap_or((0, 0));
        rain = rain
            .into_iter()
            .filter(|d| {
                d.pos.1 >= 0.
                    && d.pos.1 < screen_size.1 as f32
                    && d.pos.0 >= 0.
                    && d.pos.0 < screen_size.0 as f32
            })
            .collect();

        rain.append(&mut rain.iter().fold(Vec::new(), |mut new, drip| {
            if drip.pos.1 >= screen_size.1 as f32 - 1. && drip.speed.1 > 1. {
                let bounce = drip.speed.1 * rng.gen_range(BOUNCING_FORCE);
                new.push(Drip::new(
                    drip.pos.clone(),
                    (-bounce, -bounce),
                    random_color(),
                ));
                new.push(Drip::new(
                    drip.pos.clone(),
                    (bounce, -bounce),
                    random_color(),
                ));
                new
            } else {
                new
            }
        }));

        rain.iter_mut().for_each(|drip| drip.speed.1 += 0.04);

        rain.iter_mut().for_each(|drip| {
            drip.pos.0 += drip.speed.0;
            drip.pos.1 += drip.speed.1;
        });

        if rng.gen_bool(DRIP_NUMBER) {
            rain.push(Drip::new(
                (rng.gen_range(0..screen_size.0) as f32, 0.),
                (0., 0.5),
                random_color(),
            ));
        }

        draw_pixels(&rain.iter().map(|d| Pixel::from(d)).collect());

        if poll(Duration::from_millis(25))? {
            match read()? {
                Event::Key(_) => break,
                _ => (),
            }
        }
    }

    disable_raw_mode()?;
    stdout().execute(Show)?;
    Ok(())
}
