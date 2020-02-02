use amethyst::assets::{Asset, Handle, ProcessingState, Format};
use amethyst::ecs::prelude::VecStorage;
use d2fileformats::dt1::*;
use amethyst::{Error, Result};
use amethyst::renderer::{
    Sprite, types::TextureData, rendy::texture::TextureBuilder,
    rendy::hal::image::{
        Kind, ViewKind, SamplerInfo, WrapMode, Filter, Anisotropic, PackedColor
    }
};
use crate::asset_formats::PaletteAsset;
use std::cmp::max;
use std::io::{Cursor, Seek, SeekFrom, Read};
use byteorder::ReadBytesExt;

#[derive(Clone, Copy, Debug, Default)]
pub struct Dt1Format;

impl Format<Dt1Asset> for Dt1Format {
    fn name(&self) -> &'static str {
        "DT1"
    }

    fn import_simple(&self, bytes: Vec<u8>) -> Result<Dt1Asset> {
        if let Ok(dt1) = Dt1::from(&bytes) {
            return Ok(Dt1Asset(dt1));
        }

        Err(Error::from_string("failed to read dt1"))
    }
}

pub type Dt1Handle = Handle<Dt1Asset>;

#[derive(Clone, Debug)]
pub struct Dt1Asset(pub Dt1);

impl Asset for Dt1Asset {
    const NAME: &'static str = "d2::Dt1";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

impl From<Dt1Asset> for Result<ProcessingState<Dt1Asset>> {
    fn from(dt1: Dt1Asset) -> Result<ProcessingState<Dt1Asset>> {
        Ok(ProcessingState::Loaded(dt1))
    }
}

impl Dt1Asset {
    fn draw_isometric_subtile(pixel_data: &Vec<u8>, start_x: u32, start_y: u32, subtile_data: &Vec<u8>) {

    }

    fn draw_standard_subtile(pixel_data: &Vec<u8>, start_x: u32, start_y: u32, subtile_data: &Vec<u8>) {

    }

    pub fn to_sprites(&self, palette: &PaletteAsset) -> (TextureData, Vec<Sprite>) {
        let dt1 = &self.0;

        let mut row_heights = vec![];
        let mut texture_width: u32 = 0;
        let mut texture_height: u32 = 0;
        // TODO: calculate a better texture size
        for tile in &dt1.tiles {
            texture_height = max((-tile.height) as u32, texture_height);
            texture_width += tile.width as u32;
        }
        row_heights.push(texture_height);

        let mut sprites: Vec<Sprite> = vec![];

        let stride = texture_width * 4;
        let mut pixel_data = vec![0u8; (texture_width * texture_height * 4) as usize];
        let mut texture_starty: u32 = 0;
        let mut texture_row: u32 = 0; // TODO: When a better texture size is calculated this should be updated
        let mut sprite_start_x: u32 = 0;
        let mut texture_startx: u32 = 0;
        for (tile_index, tile) in dt1.tiles.iter().enumerate() {
            if texture_row > 0 {
                let range = (0 as usize)..(texture_row as usize);
                texture_starty = row_heights[range].iter().sum();
            }

            for (subtile_index, subtile) in tile.sub_tiles.iter().enumerate() {
                match subtile.format {
                    TileFormat::Isometric => {
                        //Dt1Asset::draw_isometric_subtile(&pixel_data, sprite_start_x + subtile.x, texture_starty + (-subtile.y), &subtile.encoded_data)
                        // isometric tile data is always 256 bytes
                        if subtile.encoded_data.len() != 256 {
                            // TODO: error
                            continue;
                        }

                        let x_jumps = [14, 12, 10, 8, 6, 4, 2, 0, 2, 4, 6, 8, 10, 12, 14];
                        let row_pixels = [4, 8, 12, 16, 20, 24, 28, 32, 28, 24, 20, 16, 12, 8, 4];

                        let mut data_index = 0;
                        for (y_index, pixel_count) in row_pixels.iter().enumerate() {
                            let mut x = x_jumps[y_index];
                            for p in 0..*pixel_count {
                                let pixel_x = sprite_start_x + subtile.x as u32 + x;
                                let pixel_y = texture_starty + subtile.y.abs() as u32 + y_index as u32;
                                let pixel_data_index = (texture_starty + texture_startx + (pixel_x * 4) + (stride * pixel_y)) as usize;
                                let palette_index = subtile.encoded_data[data_index];
                                let pixel_color: [u8; 3] = palette.0.colors[palette_index as usize];
                                pixel_data[pixel_data_index + 0] = pixel_color[2];
                                pixel_data[pixel_data_index + 1] = pixel_color[1];
                                pixel_data[pixel_data_index + 2] = pixel_color[0];
                                pixel_data[pixel_data_index + 3] = 255;

                                data_index += 1;
                                x += 1;
                            }
                        }
                    }

                    TileFormat::Standard(_) => {
                        //Dt1Asset::draw_standard_subtile(&pixel_data, sprite_start_x + subtile.x as u32, texture_starty + (-subtile.y) as u32, &subtile.encoded_data)
                        let mut x = 0u32;
                        let mut y = 0u32;

                        let mut reader = Cursor::new(&subtile.encoded_data);
                        while reader.position() < subtile.encoded_data.len() as u64 {
                            let b1 = reader.read_u8().expect("");

                            if reader.position() >= subtile.encoded_data.len() as u64 {
                                // TODO: Why are we hitting this condition?
                                break;
                            }
                            let b2 = reader.read_u8().expect("");
                            if b1 != 0 || b2 != 0 {
                                x += b1 as u32;
                                for i in 0..b2 {
                                    let pixel_x = sprite_start_x + subtile.x as u32 + x;
                                    let pixel_y = texture_starty + subtile.y.abs() as u32 + y as u32;
                                    let pixel_data_index = (texture_starty + texture_startx + (pixel_x * 4) + (stride * pixel_y)) as usize;
                                    if pixel_data_index >= pixel_data.len() {
                                        continue;
                                    }
                                    let palette_index = reader.read_u8().expect("");
                                    let pixel_color: [u8; 3] = palette.0.colors[palette_index as usize];
                                    pixel_data[pixel_data_index + 0] = pixel_color[2];
                                    pixel_data[pixel_data_index + 1] = pixel_color[1];
                                    pixel_data[pixel_data_index + 2] = pixel_color[0];
                                    pixel_data[pixel_data_index + 3] = 255;
                                }
                            } else {
                                x = 0;
                                y += 1;
                            }
                        }
                    }
                }
            }

            let sprite = Sprite::from_pixel_values(
                texture_width,
                texture_height,
                tile.width as u32,
                tile.height.abs() as u32,
                sprite_start_x,
                texture_starty,
                [
                    0f32,
                    0f32,
                ],
                false,
                false
            );
            sprites.push(sprite);

            sprite_start_x += tile.width as u32;
            texture_startx += tile.width as u32 * 4;
        }

        let texture_builder = TextureBuilder::new()
            .with_data_width(texture_width)
            .with_data_height(texture_height)
            .with_kind(Kind::D2(texture_width, texture_height, 1, 1))
            .with_view_kind(ViewKind::D2)
            .with_sampler_info(SamplerInfo {
                min_filter: Filter::Linear,
                mag_filter: Filter::Linear,
                mip_filter: Filter::Linear,
                wrap_mode: (WrapMode::Clamp, WrapMode::Clamp, WrapMode::Clamp),
                lod_bias: 0.0.into(),
                lod_range: std::ops::Range {
                    start: 0.0.into(),
                    end: 1000.0.into(),
                },
                comparison: None,
                border: PackedColor(0),
                anisotropic: Anisotropic::Off,
            }).with_raw_data(pixel_data, amethyst::renderer::Format::Rgba8Unorm);

        return (TextureData::from(texture_builder), sprites)
    }
}