use mpq::Archive;
use d2fileformats::palette::Palette;
use d2fileformats::dc6::Dc6;
use d2fileformats::ds1::Ds1;
use crate::d2assetsource;
use crate::d2assetsource::D2AssetSource;
use crate::dc6_format::{Dc6Format, Dc6Handle, Dc6Asset};
use crate::palette_format::{PaletteFormat, PaletteHandle, PaletteAsset};
use std::io::Error;
use std::mem;
use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::assets::{ProgressCounter};
use amethyst::renderer::{ ScreenDimensions,
    Camera, PngFormat, Projection, SpriteRender, SpriteSheet,
    Sprite, SpriteSheetFormat, SpriteSheetHandle, Texture, TextureHandle, TextureData, TextureMetadata,
};

pub struct D2 {
    pub progress_counter: ProgressCounter,
    pub dc6_handle: Option<Dc6Handle>,
    pub palette_handle: Option<PaletteHandle>,
    is_initialized: bool,
    spawned_entity: bool,
    pub texture_handle: Option<TextureHandle>,
    pub spritesheet_handle: Option<SpriteSheetHandle>,
}

impl SimpleState for D2 {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        {
            let mut loader = data.world.write_resource::<Loader>();
            let mut mpq_source = D2AssetSource::new("D:\\Diablo II");
            mpq_source.add_mpq("d2data.mpq").expect("whoa");
            mpq_source.add_mpq("d2exp.mpq").expect("whoa");
            loader.add_source(d2assetsource::SOURCE_NAME, mpq_source);
        }

        {;
            let loader = &data.world.read_resource::<Loader>();

            let palette_handle = loader.load_from(
                "data\\global\\palette\\loading\\pal.dat",
                PaletteFormat,
                (),
                d2assetsource::SOURCE_NAME,
                &mut self.progress_counter,
                &data.world.read_resource::<AssetStorage<PaletteAsset>>()
            );

            self.palette_handle = Some(palette_handle);

            let dc6_handle = loader.load_from(
                "data\\global\\ui\\loading\\loadingscreen.dc6",
                Dc6Format,
                (),
                d2assetsource::SOURCE_NAME,
                &mut self.progress_counter,
                &data.world.read_resource::<AssetStorage<Dc6Asset>>(),
            );

            self.dc6_handle = Some(dc6_handle);
        }

        /*let mut archive = Archive::open("D:\\Diablo II\\d2data.mpq").expect("Where's the archive bro?");

        let file = archive.open_file("data\\global\\palette\\loading\\pal.dat").expect("where's the palette bro?");
        let mut buf: Vec<u8> = vec![0; file.size() as usize];
        let file2 = archive.open_file("data\\global\\ui\\loading\\loadingscreen.dc6").expect("Where's the dc6 bro?");
        let mut buf2 = vec![0u8; file2.size() as usize];
        file.read(&mut archive, &mut buf).expect("Failed to read palette bytes?");
        file2.read(&mut archive, &mut buf2).expect("Failed to read dc6 bytes?");
        let palette = Palette::from(&buf[..]).expect("failed to load palette");
        let loading_screen = Dc6::from(&buf2).expect("failed to load dc6");*/
        //println!("Frames: {}", loading_screen.header.frames);

        /*let texture = match self.create_texture(&loading_screen, &palette) {
            Ok(t) => t,
            Err(_) => panic!("eek!")
        };*/

        //self.texture = Some(texture);

        /*let mut archive2 = Archive::open("D:\\Diablo II\\d2exp.mpq").expect("Where's the archive bro?");
        let _ds1 = D2::load_ds1(&mut archive2, "data\\global\\tiles\\expansion\\Town\\townWest.ds1");*/

        let world = data.world;
        init_camera(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if !self.is_initialized && self.progress_counter.is_complete() {
            let dc6_assets = data.world.read_resource::<AssetStorage<Dc6Asset>>();
            let palette_assets = data.world.read_resource::<AssetStorage<PaletteAsset>>();
            let palette = palette_assets.get(self.palette_handle.as_ref().expect("Expected handle to be set")).expect("Wheres the palette?");
            let dc6 = dc6_assets.get(self.dc6_handle.as_ref().expect("Expected handle to be set.")).expect("Where's the dc6?");

            let (texture_width, texture_height, pixel_data) = dc6.to_texturedata(&palette);
            let loader = &data.world.read_resource::<Loader>();

            let metadata = TextureMetadata::srgb_scale().with_size(texture_width as u16, texture_height as u16);
            let texture_handle = loader.load_from_data(TextureData::U8(pixel_data, metadata),
                                                       &mut self.progress_counter,
                                                       &data.world.read_resource::<AssetStorage<Texture>>());

            let mut sprites = vec![];
            sprites.push(Sprite::from_pixel_values(texture_width, texture_height, 256, 256, 0, 0, [0.0, 0.0]));
            let s = SpriteSheet {
                texture: texture_handle.clone(),
                sprites,
            };
            let spritesheet_handle = loader.load_from_data(s, &mut self.progress_counter, &data.world.read_resource::<AssetStorage<SpriteSheet>>());
            self.texture_handle = Some(texture_handle);
            self.spritesheet_handle = Some(spritesheet_handle);
            self.is_initialized = true;

        } else if self.is_initialized && self.progress_counter.is_complete() && !self.spawned_entity {
            let handle = self.spritesheet_handle.clone().unwrap();
            let mut transform = Transform::default();
            transform.set_xyz(256.0, 512.0, 0.0);

            data.world.create_entity()
                .with(transform)
                //.with(self.texture_handle.clone().unwrap())
                .with(SpriteRender {
                    sprite_sheet: handle,
                    sprite_number: 0
                })
                .build();

            self.spawned_entity = true;
        }

        Trans::None
    }
}

fn init_camera(world: &mut World) {
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };

    let mut transform = Transform::default();
    transform.set_z(1.0);
    world.create_entity()
         .with(Camera::from(Projection::orthographic(
             0.0,
             width,
             0.0,
             height)))
         .with(transform)
         .build();
}

impl D2 {

    pub fn new() -> Self {
        D2 {
            progress_counter: ProgressCounter::new(),
            palette_handle: None,
            dc6_handle: None,
            is_initialized: false,
            spawned_entity: false,
            texture_handle: None,
            spritesheet_handle: None,
        }
    }

    fn load_ds1(archive: &mut Archive, filename: &str) -> Result<Ds1, Error> {
        let file3 = archive.open_file(filename)?;
        let mut buf3 = vec![0u8; file3.size() as usize];

        file3.read(archive, &mut buf3)?;

        Ds1::from(&buf3)
    }

    /*fn create_texture(&mut self, dc: &Dc6, palette: &Palette) -> Result<Texture, Error> {

        let frame = &dc.frames[0];
        let mut img: RgbaImage = ImageBuffer::new(frame.header.width as u32, frame.header.height as u32);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let palette_index = frame.pixels[(x as usize, y as usize)] as usize;
            let color: [u8; 3] = palette.colors[palette_index];
            pixel.data[0] = color[2];
            pixel.data[1] = color[1];
            pixel.data[2] = color[0];
            pixel.data[3] = 255;
        }

        Ok(Texture::from_image(&img, &TextureSettings::new()))
    }*/
}