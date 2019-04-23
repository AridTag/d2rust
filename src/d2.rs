use mpq::Archive;
use d2fileformats::palette::Palette;
use d2fileformats::dc6::Dc6;
use d2fileformats::ds1::Ds1;
use crate::d2assetsource;
use crate::d2assetsource::D2AssetSource;
use crate::dc6_format::{Dc6Format, Dc6Handle, Dc6Asset};
use std::io::Error;
use std::mem;
use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::assets::{ProgressCounter};
use amethyst::renderer::{
    Camera, PngFormat, Projection, SpriteRender, SpriteSheet,
    SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
};

pub const CAMERA_WIDTH: f32 = 800.0;
pub const CAMERA_HEIGHT: f32 = 600.0;

pub struct D2 {
    pub progress_counter: ProgressCounter,
    pub dc6_handle: Option<Dc6Handle>
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

        {
            let loader = &data.world.read_resource::<Loader>();

            let dc6_handle = loader.load_from(
                //"data\\global\\ui\\loading\\loadingscreen.dc6",
                "data\\global\\items\\inv2x3.DC6",
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
        if self.progress_counter.is_complete() {
            let dc6_assets = data.world.read_resource::<AssetStorage<Dc6Asset>>();
            let dc6 = dc6_assets
                .get(
                    self.dc6_handle
                        .as_ref()
                        .expect("Expected handle to be set."),
                )
                .expect("Expected dc6 to be loaded.");
            println!("Loaded: {:?}", dc6);
            return Trans::Quit;
        }

        Trans::None
    }
}

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world.create_entity()
         .with(Camera::from(Projection::orthographic(
             0.0,
             CAMERA_WIDTH,
             0.0,
             CAMERA_HEIGHT)))
         .with(transform)
         .build();
}

fn init_dc6test(world: &mut World) {
    let mut transform = Transform::default();

}

impl D2 {

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