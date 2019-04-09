use piston::input::*;
use opengl_graphics::{GlGraphics, OpenGL};
use graphics::types::Color;
use mpq::Archive;
use d2fileformats::Palette;

pub struct D2 {
    gl: GlGraphics
}

impl D2 {
    pub fn new(opengl: OpenGL) -> D2 {
        D2 {
            gl: GlGraphics::new(opengl)
        }
    }

    pub fn init(&mut self) {
        let mut archive = Archive::open("D:\\MedianXL\\d2data.mpq").expect("Where's the archive bro?");
        let file = archive.open_file("data\\global\\palette\\loading\\pal.dat").expect("where's the palette bro?");

        let mut buf: Vec<u8> = vec![0; file.size() as usize];

        file.read(&mut archive, &mut buf).expect("Failed to read palette bytes?");
        let palette = Palette::from(&buf[..]).expect("failed to load palette");

        for i in 0..palette.colors.len() {
            println!("[{}] - {:?}", i, palette.colors[i]);
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
        const RED:   Color = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation: f64 = 0.0;
        let (x, y) = (args.width / 2.0, args.height / 2.0);

        self.gl.draw(args.viewport(), |ctx, gl| {
            clear(GREEN, gl);

            let transform = ctx.transform.trans(x, y)
                               .rot_rad(rotation)
                               .trans(-25.0, -25.0);

            rectangle(RED, square, transform, gl);
        })
    }

    pub fn update(&mut self, _args: &UpdateArgs) {

    }
}