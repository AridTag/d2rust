use piston::input::*;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};
use graphics::types::Color;
use mpq::Archive;
use d2fileformats::palette::Palette;
use d2fileformats::dc6::Dc6;
use d2fileformats::ds1::Ds1;
use std::io::Error;
use image::{RgbaImage, ImageBuffer};
use std::mem;

pub struct D2 {
    gl: GlGraphics,
    texture: Option<Texture>
}

impl D2 {
    pub fn new(opengl: OpenGL) -> D2 {
        D2 {
            gl: GlGraphics::new(opengl),
            texture: None
        }
    }

    pub fn init(&mut self) {
        let mut archive = Archive::open("D:\\MedianXL\\d2data.mpq").expect("Where's the archive bro?");

        let file = archive.open_file("data\\global\\palette\\loading\\pal.dat").expect("where's the palette bro?");
        let mut buf: Vec<u8> = vec![0; file.size() as usize];
        let file2 = archive.open_file("data\\global\\ui\\loading\\loadingscreen.dc6").expect("Where's the dc6 bro?");
        let mut buf2 = vec![0u8; file2.size() as usize];
        file.read(&mut archive, &mut buf).expect("Failed to read palette bytes?");
        file2.read(&mut archive, &mut buf2).expect("Failed to read dc6 bytes?");
        let palette = Palette::from(&buf[..]).expect("failed to load palette");
        let loading_screen = Dc6::from(&buf2).expect("failed to load dc6");
        //println!("Frames: {}", loading_screen.header.frames);

        self.test_ds1(&mut archive);

        let texture = match self.create_texture(&loading_screen, &palette) {
            Ok(t) => t,
            Err(_) => panic!("eek!")
        };

        self.texture = Some(texture);
    }

    fn test_ds1(&self, archive: &mut Archive) {
        let file3 = archive.open_file("data\\global\\tiles\\ACT1\\TOWN\\townNE.ds1").expect("");
        let mut buf3 = vec![0u8; file3.size() as usize];

        file3.read(archive, &mut buf3).expect("");
        let town_ne = Ds1::from(&buf3).expect("");
        println!("{:?}", town_ne);
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const CF_BLUE: Color = [100.0 / 255.0, 149.0 / 255.0, 237.0 / 255.0, 1.0];

        let rotation: f64 = 0.0;
        let (x, y) = (args.width / 2.0, args.height / 2.0);

        let mut img = mem::replace(&mut self.texture, None).unwrap();
        self.gl.draw(args.viewport(), |ctx, gl| {
            clear(CF_BLUE, gl);

            let transform = ctx.transform.trans(x, y)
                               .rot_rad(rotation)
                               .trans(-(img.get_width() as f64 / 2.0), -(img.get_height() as f64 / 2.0));

            image(&img, transform, gl);
        });

        self.texture = Some(mem::replace(&mut img, Texture::empty(&TextureSettings::new()).unwrap()));
    }

    pub fn update(&mut self, _args: &UpdateArgs) {

    }

    fn create_texture(&mut self, dc: &Dc6, palette: &Palette) -> Result<Texture, Error> {

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
    }
}