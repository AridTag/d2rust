use d2fileformats::dc6;
use d2fileformats::dc6::Dc6;
use crate::palette_format::{PaletteHandle, PaletteAsset};
use amethyst::assets;
use amethyst::assets::{Asset, SimpleFormat, Handle, ProcessingState};
use amethyst::renderer::{Texture, TextureBuilder};
use amethyst::ecs::prelude::VecStorage;
use std::cmp::{max,min};

pub type Dc6Handle = Handle<Dc6Asset>;

#[derive(Clone,Debug)]
pub struct Dc6Asset(pub Dc6);

impl Dc6Asset {
    /// returns the texture width, height and pixel data
    pub fn to_texturedata(&self, palette: &PaletteAsset) -> (u32, u32, Vec<u8>) {
        let dc6 = &self.0;

        let mut row_heights = vec![];
        let mut texture_width = 0;
        let mut texture_height = 0;
        for row in 0..dc6.header.directions {
            let mut row_height = 0;
            let mut row_width = 0;
            for frame_num in 0..1 {//dc6.header.frames {
                let frame_index = ((row * dc6.header.frames) + frame_num) as usize;
                let frame = dc6.frames.get(frame_index).unwrap();
                row_width += frame.header.width;
                row_height = max(row_height, frame.header.height);
            }
            texture_width = max(texture_width, row_width);
            texture_height += row_height;
            row_heights.push(row_height);
        }

        let mut pixel_data = vec![0u8; (texture_width * texture_height * 4) as usize];
        let mut texture_starty: u32 = 0;
        for row in 0..dc6.header.directions {
            if row > 0 {
                let range = (0 as usize)..(row as usize);
                texture_starty = row_heights[range].iter().sum();
            }

            let mut texture_startx = 0;
            for frame_num in 0..1 {//dc6.header.frames {
                let frame_index = ((row * dc6.header.frames) + frame_num) as usize;
                let frame = dc6.frames.get(frame_index).unwrap();
                for y in 0..frame.header.height {
                    for x in 0..frame.header.width {
                        let palette_index = frame.pixels[(x as usize, y as usize)] as usize;
                        let pixel_color: [u8; 3] = palette.0.colors[palette_index];

                        let pixel_data_index = (texture_starty + texture_startx + (x * 4) + (texture_width * y)) as usize;
                        pixel_data[pixel_data_index + 0] = pixel_color[2];
                        pixel_data[pixel_data_index + 1] = pixel_color[1];
                        pixel_data[pixel_data_index + 2] = pixel_color[0];
                        pixel_data[pixel_data_index + 3] = 255;
                    }
                }
                texture_startx += frame.header.width * 4;
            }
        }

        return (texture_width, texture_height, pixel_data);
    }
}

impl Asset for Dc6Asset {
    const NAME: &'static str = "d2::Dc6";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

impl From<Dc6Asset> for assets::Result<ProcessingState<Dc6Asset>> {
    fn from(dc6: Dc6Asset) -> assets::Result<ProcessingState<Dc6Asset>> {
        Ok(ProcessingState::Loaded(dc6))
    }
}

/// Amethyst Format for loading from '.dc6' files
#[derive(Clone, Copy, Debug, Default)]
pub struct Dc6Format;

impl SimpleFormat<Dc6Asset> for Dc6Format {
    const NAME: &'static str = "DC6";
    type Options = ();

    fn import(&self, bytes: Vec<u8>, _: Self::Options) -> assets::Result<Dc6Asset> {
        if let Ok(dc6) = Dc6::from(&bytes) {
            return Ok(Dc6Asset(dc6));
        }

        Err(assets::Error::from_kind(assets::ErrorKind::Format("failed to read dc6")))
    }
}