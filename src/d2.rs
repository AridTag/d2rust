use crate::asset_formats::{
    D2sAsset, D2sFormat, D2sHandle, Dc6Asset, Dc6Format, Dc6Handle, PaletteAsset, PaletteFormat,
    PaletteHandle,
};
use crate::d2assetsource;
use amethyst::{
    assets::{AssetStorage, Loader, ProgressCounter},
    core::{timing::Time, transform::Transform},
    ecs::{join::Join, Component, DenseVecStorage, Read, ReadStorage, WriteStorage},
    prelude::*,
    renderer::{
        Camera, Projection, ScreenDimensions, SpriteRender, SpriteSheet, SpriteSheetHandle, Texture,
    },
};

pub struct SpriteAnimationComponent {
    pub update_rate: f64,
    pub last_update: f64,
}

impl Component for SpriteAnimationComponent {
    type Storage = DenseVecStorage<Self>;
}

pub struct SpriteCountComponent {
    pub count: usize,
}

impl Component for SpriteCountComponent {
    type Storage = DenseVecStorage<Self>;
}

pub struct D2 {
    progress_counter: ProgressCounter,
    is_initialized: bool,
    spawned_entity: bool,
    last_update: f64,
    dc6_palettes_to_convert: Vec<(Dc6Handle, PaletteHandle, f64, Transform)>,
    handles_to_spawn: Vec<(SpriteSheetHandle, f64, Transform)>,
}

impl SimpleState for D2 {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        {
            let (window_width, window_height) = {
                let dim = data.world.read_resource::<ScreenDimensions>();
                (dim.width(), dim.height())
            };

            let loader = &data.world.read_resource::<Loader>();

            let d2s_handle = loader.load_from(
                "C:\\Users\\jon\\Saved Games\\Diablo II\\Ass.d2s",
                D2sFormat,
                (),
                d2assetsource::SOURCE_NAME,
                &mut self.progress_counter,
                &data.world.read_resource::<AssetStorage<D2sAsset>>(),
            );

            /*let palette_handle = loader.load_from(
                "data\\global\\palette\\loading\\pal.dat",
                PaletteFormat,
                (),
                d2assetsource::SOURCE_NAME,
                &mut self.progress_counter,
                &data.world.read_resource::<AssetStorage<PaletteAsset>>());

            let dc6_handle = loader.load_from(
                "data\\global\\ui\\loading\\loadingscreen.dc6",
                Dc6Format,
                (),
                d2assetsource::SOURCE_NAME,
                &mut self.progress_counter,
                &data.world.read_resource::<AssetStorage<Dc6Asset>>());

            let mut transform = Transform::default();
            transform.set_xyz(window_width / 2.0, window_height / 2.0, 0.0);
            self.dc6_palettes_to_convert.push((dc6_handle, palette_handle, 0.4, transform));*/

            /*let dc6_handle = loader.load_from(
                "data\\global\\ui\\frontend\\ts11b.dc6",
                Dc6Format,
                (),
                d2assetsource::SOURCE_NAME,
                &mut self.progress_counter,
                &data.world.read_resource::<AssetStorage<Dc6Asset>>());

            let palette_handle = loader.load_from(
                "data\\global\\palette\\Sky\\pal.dat",
                PaletteFormat,
                (),
                d2assetsource::SOURCE_NAME,
                &mut self.progress_counter,
                &data.world.read_resource::<AssetStorage<PaletteAsset>>());*/

            /*let mut transform = Transform::default();
            transform.set_xyz(window_width / 2.0, window_height / 2.0, 0.0);
            self.dc6_palettes_to_convert.push((dc6_handle, palette_handle, 0.4, transform));

            let dc6_handle = loader.load_from(
                "data\\global\\ui\\FrontEnd\\D2logoFireLeft.DC6",
                Dc6Format,
                (),
                d2assetsource::SOURCE_NAME,
                &mut self.progress_counter,
                &data.world.read_resource::<AssetStorage<Dc6Asset>>());

            let palette_handle = loader.load_from(
                "data\\global\\palette\\Sky\\pal.dat",
                PaletteFormat,
                (),
                d2assetsource::SOURCE_NAME,
                &mut self.progress_counter,
                &data.world.read_resource::<AssetStorage<PaletteAsset>>());
            let mut transform = Transform::default();
            transform.set_xyz(window_width / 2.0 - 128.0, window_height / 2.0 + 256.0, 0.0);
            self.dc6_palettes_to_convert.push((dc6_handle, palette_handle, 0.1, transform));*/

            let dc6_handle = loader.load_from(
                "data\\global\\ui\\FrontEnd\\D2logoFireRight.DC6",
                Dc6Format,
                (),
                d2assetsource::SOURCE_NAME,
                &mut self.progress_counter,
                &data.world.read_resource::<AssetStorage<Dc6Asset>>(),
            );

            let palette_handle = loader.load_from(
                "data\\global\\palette\\Sky\\pal.dat",
                PaletteFormat,
                (),
                d2assetsource::SOURCE_NAME,
                &mut self.progress_counter,
                &data.world.read_resource::<AssetStorage<PaletteAsset>>(),
            );
            let mut transform = Transform::default();
            transform.set_translation_xyz(
                window_width / 2.0 + 32.0,
                window_height / 2.0 + 256.0,
                0.0,
            );
            self.dc6_palettes_to_convert
                .push((dc6_handle, palette_handle, 0.1, transform));
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

            for (dc6_handle, palette_handle, update_rate, transform) in
                &self.dc6_palettes_to_convert
            {
                let palette = palette_assets
                    .get(&palette_handle)
                    .expect("Wheres the palette?");
                let dc6 = dc6_assets.get(&dc6_handle).expect("Where's the dc6?");

                //println!("{:?}", dc6.0);

                let loader = &data.world.read_resource::<Loader>();
                let texture_storage = &data.world.read_resource::<AssetStorage<Texture>>();
                let spritesheet_storage = &data.world.read_resource::<AssetStorage<SpriteSheet>>();

                let (texture_data, sprites) = dc6.to_sprites(&palette);
                let texture_handle = loader.load_from_data(
                    texture_data,
                    &mut self.progress_counter,
                    texture_storage,
                );
                let spritesheet = SpriteSheet {
                    texture: texture_handle.clone(),
                    sprites,
                };
                let spritesheet_handle = loader.load_from_data(
                    spritesheet,
                    &mut self.progress_counter,
                    spritesheet_storage,
                );

                self.handles_to_spawn
                    .push((spritesheet_handle, *update_rate, transform.clone()));
            }

            self.is_initialized = true;
        } else if self.is_initialized && self.progress_counter.is_complete() && !self.spawned_entity
        {
            for (spritesheet_handle, update_rate, transform) in &self.handles_to_spawn {
                spawn_animated_dc6(
                    data,
                    transform.clone(),
                    spritesheet_handle.clone(),
                    *update_rate,
                );
            }

            self.spawned_entity = true;
        } else if self.spawned_entity {
            let StateData { world, .. } = data;
            // Execute a pass similar to a system
            world.exec(
                |(mut write_sprite, read_spritecount, time): (
                    WriteStorage<SpriteRender>,
                    ReadStorage<SpriteCountComponent>,
                    Read<Time>,
                )| {
                    if time.absolute_time_seconds() - self.last_update >= 0.4 {
                        self.last_update = time.absolute_time_seconds();

                        for (sprite, sprite_count) in (&mut write_sprite, &read_spritecount).join()
                        {
                            if sprite.sprite_number < sprite_count.count - 1 {
                                sprite.sprite_number += 1;
                            } else {
                                sprite.sprite_number = 0;
                            }
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
    transform.set_translation_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0, width, 0.0, height,
        )))
        .with(transform)
        .build();
}

fn spawn_animated_dc6(
    data: &mut StateData<'_, GameData<'_, '_>>,
    transform: Transform,
    sprite_sheet: SpriteSheetHandle,
    update_rate: f64,
) {
    let sprite_count: usize = {
        let assets = &data.world.read_resource::<AssetStorage<SpriteSheet>>();
        let sheet = assets.get(&sprite_sheet).expect("This should be there");
        sheet.sprites.len()
    };

    data.world
        .create_entity()
        .with(transform)
        .with(SpriteRender {
            sprite_sheet,
            sprite_number: 0,
        })
        .with(SpriteCountComponent {
            count: sprite_count,
        })
        .with(SpriteAnimationComponent {
            last_update: 0.0,
            update_rate,
        })
        .build();
}

impl D2 {
    pub fn new() -> Self {
        D2 {
            progress_counter: ProgressCounter::new(),
            is_initialized: false,
            spawned_entity: false,
            last_update: 0.0,
            dc6_palettes_to_convert: vec![],
            handles_to_spawn: vec![],
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
