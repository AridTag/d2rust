use crate::asset_formats::{
    D2sAsset, D2sFormat, D2sHandle, Dc6Asset, Dc6Format, Dc6Handle, PaletteAsset, PaletteFormat,
    PaletteHandle, Dt1Asset, Dt1Format, Dt1Handle
};
use crate::d2assetsource;
use amethyst::{
    assets::{Handle, AssetStorage, Loader, ProgressCounter},
    core::{transform::Transform},
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{
        Camera, SpriteRender, SpriteSheet, Texture
    },
    window::ScreenDimensions
};
use amethyst::shred::Fetch;

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
    dc6_palettes_to_convert: Vec<(Dc6Handle, PaletteHandle, f64, Transform)>,
    handles_to_spawn: Vec<(Handle<SpriteSheet>, f64, Transform)>,
    dt1_handle: Option<Dt1Handle>,
}

fn get_window_size(world: &mut World) -> (f32, f32) {
    let dim = world.read_resource::<ScreenDimensions>();
    (dim.width(), dim.height())
}


impl SimpleState for D2 {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        {
            //let (window_width, window_height) = get_window_size(data.world);

            let loader = &data.world.read_resource::<Loader>();

            /*let d2s_handle = loader.load_from(
                "C:\\Users\\jon\\Saved Games\\Diablo II\\Ass.d2s",
                D2sFormat,
                d2assetsource::SOURCE_NAME,
                &mut self.progress_counter,
                &data.world.read_resource::<AssetStorage<D2sAsset>>(),
            );*/

            let dt1_asset_storage = &data.world.read_resource::<AssetStorage<Dt1Asset>>();

            self.dt1_handle = Some(loader.load_from(
                "data\\global\\tiles\\expansion\\BaalLair\\throne.dt1",
                Dt1Format,
                d2assetsource::SOURCE_NAME,
                &mut self.progress_counter,
                dt1_asset_storage));


            let dc6_asset_storage = &data.world.read_resource::<AssetStorage<Dc6Asset>>();
            let palette_asset_storage = &data.world.read_resource::<AssetStorage<PaletteAsset>>();

            /*self.load_dc6(loader, dc6_asset_storage, palette_asset_storage,
                          "data\\global\\ui\\loading\\loadingscreen.dc6",
                          "data\\global\\palette\\loading\\pal.dat",
                          0.0, 0.0);*/

            self.load_dc6(loader, dc6_asset_storage, palette_asset_storage,
                          "data\\global\\ui\\MENU\\questdone.dc6",
                          "data\\global\\palette\\units\\pal.dat",
                          0.0, 0.0);

            /*self.load_dc6(loader, dc6_asset_storage, palette_asset_storage,
                          "data\\global\\ui\\FrontEnd\\D2logoFireLeft.DC6",
                          "data\\global\\palette\\units\\pal.dat",
                          -174.0, -100.0);
            self.load_dc6(loader, dc6_asset_storage, palette_asset_storage,
                          "data\\global\\ui\\FrontEnd\\D2logoFireRight.DC6",
                          "data\\global\\palette\\units\\pal.dat",
                          0.0, 0.0);*/
        }

        let world = data.world;
        init_camera(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let errors = self.progress_counter.errors();
        if !errors.is_empty() {
            for error in errors {
                println!("{} failed to load with error - {}", error.asset_name, error.error);
            }
        }

        if !self.is_initialized && self.progress_counter.is_complete() {
            let dc6_assets = data.world.read_resource::<AssetStorage<Dc6Asset>>();
            let palette_assets = data.world.read_resource::<AssetStorage<PaletteAsset>>();

            for (dc6_handle, palette_handle, update_rate, transform) in &self.dc6_palettes_to_convert
            {
                let palette = palette_assets
                    .get(&palette_handle)
                    .expect("Wheres the palette?");
                let dc6 = dc6_assets.get(&dc6_handle).expect("Where's the dc6?");

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
        }

        if self.progress_counter.is_complete() && self.handles_to_spawn.len() > 0 {
            for (spritesheet_handle, update_rate, transform) in &self.handles_to_spawn {
                spawn_animated_dc6(
                    data,
                    (*transform).clone(),
                    (*spritesheet_handle).clone(),
                    *update_rate,
                );
            }
            self.handles_to_spawn.clear();
        }

        if self.progress_counter.is_complete() && self.dt1_handle.is_some() {
            let dt1_assets = data.world.read_resource::<AssetStorage<Dt1Asset>>();
            let dt1 = dt1_assets.get(self.dt1_handle.as_ref().unwrap()).expect("wheres the dt1?");

            amethyst_imgui::with(|ui| {
                ui.text(format!("Tiles: {}", dt1.0.tiles.len()));
                for tile in &dt1.0.tiles {
                    ui.text(format!("blocks {}", tile.num_blocks))
                }
            });
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
    transform.set_translation_xyz(0.0, 0.0, 1.0);
    world.create_entity()
         .with(Camera::standard_2d(width, height))
         .with(transform)
         .build();
}

fn spawn_animated_dc6(
    data: &mut StateData<'_, GameData<'_, '_>>,
    transform: Transform,
    sprite_sheet: Handle<SpriteSheet>,
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
            dc6_palettes_to_convert: vec![],
            handles_to_spawn: vec![],
            dt1_handle: None,
        }
    }

    fn load_dc6<S: Into<String>>(&mut self, loader: &Fetch<Loader>, dc6_asset_storage: &Fetch<AssetStorage<Dc6Asset>>, palette_asset_storage: &Fetch<AssetStorage<PaletteAsset>>, dc6_path: S, palette_path: S, x: f32, y: f32) {
        let dc6_handle = loader.load_from(
            dc6_path,
            Dc6Format,
            d2assetsource::SOURCE_NAME,
            &mut self.progress_counter,
            dc6_asset_storage);

        let palette_handle = loader.load_from(
            palette_path,
            PaletteFormat,
            d2assetsource::SOURCE_NAME,
            &mut self.progress_counter,
            palette_asset_storage);

        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, 0.0);
        self.dc6_palettes_to_convert.push((dc6_handle, palette_handle, 0.2, transform));
    }
}
