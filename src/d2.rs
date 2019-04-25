use crate::d2assetsource;
use crate::dc6_format::{Dc6Format, Dc6Handle, Dc6Asset};
use crate::palette_format::{PaletteFormat, PaletteHandle, PaletteAsset};
use amethyst::{
    prelude::*,
    assets::{AssetStorage, Loader, ProgressCounter},
    core::{transform::Transform, timing::Time},
    ecs::{Entities,Read,ReadStorage,WriteStorage,join::Join},
    renderer::{
        ScreenDimensions, Camera, Projection, SpriteRender,
        SpriteSheet, SpriteSheetHandle,
        Texture, TextureHandle},
};

pub struct D2 {
    pub progress_counter: ProgressCounter,
    pub dc6_handle: Option<Dc6Handle>,
    pub palette_handle: Option<PaletteHandle>,
    is_initialized: bool,
    spawned_entity: bool,
    last_update: f64,
    pub texture_handle: Option<TextureHandle>,
    pub spritesheet_handle: Option<SpriteSheetHandle>,
}

impl SimpleState for D2 {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        {
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

            let loader = &data.world.read_resource::<Loader>();
            let texture_storage = &data.world.read_resource::<AssetStorage<Texture>>();
            let spritesheet_storage = &data.world.read_resource::<AssetStorage<SpriteSheet>>();

            let (texture_data,sprites) = dc6.to_sprites(&palette);
            let texture_handle = loader.load_from_data(texture_data, &mut self.progress_counter, texture_storage);
            let spritesheet = SpriteSheet {
                texture: texture_handle.clone(),
                sprites
            };
            let spritesheet_handle = loader.load_from_data(spritesheet, &mut self.progress_counter, spritesheet_storage);

            self.texture_handle = Some(texture_handle);
            self.spritesheet_handle = Some(spritesheet_handle);
            self.is_initialized = true;
        } else if self.is_initialized && self.progress_counter.is_complete() && !self.spawned_entity {
            let (window_width, window_height) = {
                let dim = data.world.read_resource::<ScreenDimensions>();
                (dim.width(), dim.height())
            };

            let handle = self.spritesheet_handle.clone().unwrap();
            let mut transform = Transform::default();
            transform.set_xyz(window_width / 2.0, window_height / 2.0, 0.0);
            data.world.create_entity()
                .with(transform)
                .with(SpriteRender {
                    sprite_sheet: handle,
                    sprite_number: 0
                })
                .build();

            self.spawned_entity = true;
        } else if self.spawned_entity {
            let StateData { world, .. } = data;
            // Execute a pass similar to a system
            world.exec(
                |(entities, mut write_sprite, time): (
                    Entities,
                    WriteStorage<SpriteRender>,
                    Read<Time>
                )| {
                    for (entity, sprite) in (&entities, &mut write_sprite).join() {
                        if time.absolute_time_seconds() - self.last_update >= 2.0 {
                            self.last_update = time.absolute_time_seconds();

                            if sprite.sprite_number < 9 {
                                sprite.sprite_number += 1;
                            } else {
                                sprite.sprite_number = 0;
                            }

                            println!("{}", sprite.sprite_number);
                        }
                    }
                },
            );
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
            last_update: 0.0,
            texture_handle: None,
            spritesheet_handle: None,
        }
    }

    /*fn load_ds1(archive: &mut Archive, filename: &str) -> Result<Ds1, Error> {
        let file3 = archive.open_file(filename)?;
        let mut buf3 = vec![0u8; file3.size() as usize];

        file3.read(archive, &mut buf3)?;

        Ds1::from(&buf3)
    }*/

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