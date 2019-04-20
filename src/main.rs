extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate mpq;
extern crate image;
extern crate vecmath;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use crate::d2::D2;

mod d2;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut window: Window = WindowSettings::new("", [800, 600]).opengl(opengl)
                                                                         .exit_on_esc(true)
                                                                         .build()
                                                                         .unwrap();
    let mut d2 = D2::new(opengl);
    d2.init();
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            d2.render(&r);
        }

        if let Some(u) = e.update_args() {
            d2.update(&u)
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ds1_loads() {
        use mpq::Archive;
        use d2fileformats::ds1::Ds1;

        let archive_path = "D:\\Diablo II\\d2exp.mpq";
        let mut archive = Archive::open(archive_path).expect(&format!("can not find archive {}", archive_path));

        let filename = "data\\global\\tiles\\expansion\\Town\\townWest.ds1";
        let archive_file = archive.open_file(filename).expect(&format!("can't find {} in {}", filename, archive_path));
        let mut file_buffer = vec![0u8; archive_file.size() as usize];

        archive_file.read(&mut archive, &mut file_buffer).expect(&format!("failed to read {}", filename));
        let ds1 = Ds1::from(&file_buffer).expect("");
        println!("{}\n{:?}", filename, ds1);
    }
}
