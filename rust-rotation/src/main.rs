use std::f32::consts::PI;

use macroquad::prelude::*;
use macroquad::rand::gen_range;

const DURATION: u16 = 600;
const RANDOMNISATION: u8 = 10;

type VecTexture = Vec<Vec<[u8; 4]>>;
type TextureSlice<'a> = Vec<&'a [[u8; 4]]>;

trait Texture {
    fn size(&self) -> (usize, usize);
    fn inline(&self) -> Vec<u8>;
    fn owned(self) -> VecTexture;
    fn texture2d(self) -> Texture2D;
    fn slice(&self, rect: (usize, usize, usize, usize)) -> TextureSlice;
    fn rotate(&self) -> VecTexture;
    fn paint(&mut self, painting: &VecTexture, pos: (usize, usize)) -> &mut Self;
}
trait IntoVecTexture {
    fn into_vec_texture(self) -> VecTexture;
}

impl Texture for VecTexture {
    fn size(&self) -> (usize, usize) {
        (self[0].len(), self.len())
    }
    fn inline(&self) -> Vec<u8> {
        self.iter()
            .flat_map(|row| row.iter().flatten())
            .cloned()
            .collect()
    }
    fn owned(self) -> VecTexture {
        self
    }
    fn texture2d(self) -> Texture2D {
        let (width, height) = self.size();
        Texture2D::from_rgba8(width as u16, height as u16, &self.inline())
    }
    fn slice(&self, rect: (usize, usize, usize, usize)) -> TextureSlice {
        (0..rect.3)
            .map(|y| &self[rect.1 + y][(rect.0)..(rect.0 + rect.2)])
            .collect()
    }

    fn rotate(&self) -> VecTexture {
        let (height, width) = self.size();
        let mut out_texture = vec![vec![[0; 4]; width]; height];

        self.iter().enumerate().for_each(|(y, row)| {
            row.iter()
                .cloned()
                .enumerate()
                .for_each(|(x, pix)| out_texture[x][width - y - 1] = pix)
        });
        out_texture
    }
    fn paint(&mut self, painting: &VecTexture, pos: (usize, usize)) -> &mut Self {
        painting.iter().enumerate().for_each(|(y, row)| {
            row.iter()
                .cloned()
                .enumerate()
                .for_each(|(x, pix)| self[pos.1 + y][pos.0 + x] = pix)
        });
        self
    }
}
impl<'a> Texture for TextureSlice<'a> {
    fn size(&self) -> (usize, usize) {
        (self[0].len(), self.len())
    }
    fn inline(&self) -> Vec<u8> {
        self.iter()
            .flat_map(|row| row.iter().flatten())
            .cloned()
            .collect()
    }
    fn owned(self) -> VecTexture {
        self.into_iter()
            .map(|s| s.into_iter().cloned().collect())
            .collect()
    }
    fn texture2d(self) -> Texture2D {
        let (width, height) = self.size();
        Texture2D::from_rgba8(width as u16, height as u16, &self.inline())
    }
    fn slice(&self, rect: (usize, usize, usize, usize)) -> TextureSlice {
        (0..rect.2)
            .map(|y| &self[rect.1 + y][(rect.0)..(rect.3)])
            .collect()
    }
    fn rotate(&self) -> VecTexture {
        let (height, width) = self.size();
        let mut out_texture = vec![vec![[0; 4]; width]; height];

        self.iter().enumerate().for_each(|(y, row)| {
            row.iter()
                .cloned()
                .enumerate()
                .for_each(|(x, pix)| out_texture[x][width - y - 1] = pix)
        });
        out_texture
    }
    fn paint(&mut self, painting: &VecTexture, pos: (usize, usize)) -> &mut Self {
        panic!("Can't paint on slice");
    }
}

impl IntoVecTexture for Image {
    fn into_vec_texture(self) -> VecTexture {
        let Image { bytes, width, .. } = self;
        bytes
            .chunks(4)
            .map(|x| {
                let mut color: [u8; 4] = [255; 4];
                for (i, v) in x.into_iter().enumerate() {
                    color[i] = *v;
                }
                color
            })
            .collect::<Vec<[u8; 4]>>()
            .chunks(width as usize)
            .map(|x| x.to_vec())
            .collect()
    }
}

#[macroquad::main("Rust-Rotation")]
async fn main() {
    // let from: VecTexture = vec![vec![vec![255; 4]; width as usize]; height as usize]
    //     .into_iter()
    //     .enumerate()
    //     .map(|(y, row)| {
    //         row.into_iter()
    //             .map(move |_| {
    //                 if y < height as usize / 2 {
    //                     [255, 255, 0, 255]
    //                 } else {
    //                     [0, 0, 255, 255]
    //                 }
    //             })
    //             .collect()
    //     })
    //     .collect();
    let from: VecTexture = load_image("images/in.png")
        .await
        .unwrap()
        .into_vec_texture();

    let (width, height) = from.size();
    let size_limit = width.min(height);

    let mut steps: Vec<(f32, f32, VecTexture)> = vec![(0., 0., from.clone())];
    let mut last = from.clone();
    for _ in 0..RANDOMNISATION {
        let size = gen_range(size_limit as u32 / 3, size_limit as u32);
        let pos = (
            gen_range(0, width as u32 - size) as usize,
            gen_range(0, height as u32 - size) as usize,
        );
        let rotated = last
            .slice((pos.0, pos.1, size as usize, size as usize))
            .rotate();
        last.paint(&rotated, pos);
        steps.push((pos.0 as f32, pos.1 as f32, rotated));
    }

    let steps: Vec<(f32, f32, Texture2D)> = steps
        .into_iter()
        .map(|(x, y, tex)| (x, y, tex.texture2d()))
        .collect();

    let mut layer = steps.len();
    let mut angle = 0.;
    loop {
        angle -= PI * 0.5 * (1. / (DURATION as f32 / 1000.)) * get_frame_time();
        if angle <= PI * -0.5 {
            if layer == 2 {
                break;
            } else {
                layer -= 1;
            }
            angle = 0.;
        }

        for (x, y, texture) in &steps[0..layer - 1] {
            draw_texture(*texture, *x, *y, Color::from_rgba(255, 255, 255, 255));
        }
        let (x, y, texture) = steps[layer - 1];
        draw_texture_ex(
            texture,
            x,
            y,
            Color::from_rgba(255, 255, 255, 255),
            DrawTextureParams {
                rotation: angle,
                ..Default::default()
            },
        );

        next_frame().await;
    }

    let mut end_t = 0.;
    while end_t < 2. {
        end_t += get_frame_time();
        draw_texture(steps[0].2, 0., 0., Color::from_rgba(255, 255, 255, 255));
        next_frame().await
    }
}
