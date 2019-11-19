use crate::asset_formats::PaletteAsset;
use amethyst::assets::{Asset, Handle, ProcessingState, Format};
use amethyst::ecs::prelude::VecStorage;
use amethyst::renderer::{Sprite, types::TextureData, loaders::load_from_srgba, palette::Srgba};
use amethyst::{Error, Result};
use d2fileformats::dc6::Dc6;
use std::cmp::max;

/// Amethyst Format for loading from '.dc6' files
#[derive(Clone, Copy, Debug, Default)]
pub struct Dc6Format;

impl Format<Dc6Asset> for Dc6Format {
    fn name(&self) -> &'static str {
        "DC6"
    }

    fn import_simple(&self, bytes: Vec<u8>) -> Result<Dc6Asset> {
        if let Ok(dc6) = Dc6::from(&bytes) {
            return Ok(Dc6Asset(dc6));
        }

        Err(Error::from_string("failed to read dc6"))
    }
}

pub type Dc6Handle = Handle<Dc6Asset>;

#[derive(Clone, Debug)]
pub struct Dc6Asset(pub Dc6);

impl Dc6Asset {
    pub fn to_sprites(&self, palette: &PaletteAsset) -> (TextureData, Vec<Sprite>) {
        let dc6 = &self.0;

        let mut row_heights = vec![];
        let mut texture_width = 0;
        let mut texture_height = 0;
        for row in 0..dc6.header.directions {
            let mut row_height = 0;
            let mut row_width = 0;
            for frame_num in 0..dc6.header.frames {
                let frame_index = ((row * dc6.header.frames) + frame_num) as usize;
                let frame = dc6.frames.get(frame_index).unwrap();
                row_width += frame.header.width;
                row_height = max(row_height, frame.header.height);
            }
            texture_width = max(texture_width, row_width);
            texture_height += row_height;
            row_heights.push(row_height);
        }

        let mut sprites: Vec<Sprite> = vec![];

        let stride = texture_width * 4;
        let mut pixel_data = vec![0u8; (texture_width * texture_height * 4) as usize];
        let mut texture_starty: u32 = 0;
        for row in 0..dc6.header.directions {
            if row > 0 {
                let range = (0 as usize)..(row as usize);
                texture_starty = row_heights[range].iter().sum();
            }

            let mut texture_startx = 0;
            let mut sprite_start_x = 0;
            for frame_num in 0..dc6.header.frames {
                let frame_index = ((row * dc6.header.frames) + frame_num) as usize;
                let frame = dc6.frames.get(frame_index).unwrap();
                for y in 0..frame.header.height {
                    for x in 0..frame.header.width {
                        let palette_index = frame.pixels[(x as usize, y as usize)] as usize;
                        let pixel_color: [u8; 3] = palette.0.colors[palette_index];

                        let pixel_data_index =
                            (texture_starty + texture_startx + (x * 4) + (stride * y)) as usize;
                        pixel_data[pixel_data_index + 0] = pixel_color[2];
                        pixel_data[pixel_data_index + 1] = pixel_color[1];
                        pixel_data[pixel_data_index + 2] = pixel_color[0];
                        pixel_data[pixel_data_index + 3] = 255;
                    }
                }

                let sprite = Sprite::from_pixel_values(
                    texture_width,
                    texture_height,
                    frame.header.width,
                    frame.header.height,
                    sprite_start_x,
                    texture_starty,
                    [
                        frame.header.width as f32 / 2.0,
                        -(frame.header.height as f32) / 2.0,
                    ],
                    false,
                    false
                );
                sprites.push(sprite);

                sprite_start_x += frame.header.width;
                texture_startx += frame.header.width * 4;
            }
        }

        /*let metadata =
            TextureMetadata::srgb_scale().with_size(texture_width as u16, texture_height as u16);*/

        let mut textureBuilder = load_from_srgba(Srgba::new(0., 0., 0., 0.));
        textureBuilder.set_data_width(texture_width);
        textureBuilder.set_data_height(texture_height);

        return (TextureData::from(textureBuilder), sprites)
    }

    /// returns the texture width, height and pixel data
    pub fn to_texturedata(&self, palette: &PaletteAsset) -> (u32, u32, Vec<u8>) {
        let dc6 = &self.0;

        let mut row_heights = vec![];
        let mut texture_width = 0;
        let mut texture_height = 0;
        for row in 0..dc6.header.directions {
            let mut row_height = 0;
            let mut row_width = 0;
            for frame_num in 0..dc6.header.frames {
                let frame_index = ((row * dc6.header.frames) + frame_num) as usize;
                let frame = dc6.frames.get(frame_index).unwrap();
                row_width += frame.header.width;
                row_height = max(row_height, frame.header.height);
            }
            texture_width = max(texture_width, row_width);
            texture_height += row_height;
            row_heights.push(row_height);
        }

        let stride = texture_width * 4;
        let mut pixel_data = vec![0u8; (texture_width * texture_height * 4) as usize];
        let mut texture_starty: u32 = 0;
        for row in 0..dc6.header.directions {
            if row > 0 {
                let range = (0 as usize)..(row as usize);
                texture_starty = row_heights[range].iter().sum();
            }

            let mut texture_startx = 0;
            for frame_num in 0..dc6.header.frames {
                let frame_index = ((row * dc6.header.frames) + frame_num) as usize;
                let frame = dc6.frames.get(frame_index).unwrap();
                for y in 0..frame.header.height {
                    for x in 0..frame.header.width {
                        let palette_index = frame.pixels[(x as usize, y as usize)] as usize;
                        let pixel_color: [u8; 3] = palette.0.colors[palette_index];

                        let pixel_data_index =
                            (texture_starty + texture_startx + (x * 4) + (stride * y)) as usize;
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

impl From<Dc6Asset> for Result<ProcessingState<Dc6Asset>> {
    fn from(dc6: Dc6Asset) -> Result<ProcessingState<Dc6Asset>> {
        Ok(ProcessingState::Loaded(dc6))
    }
}
