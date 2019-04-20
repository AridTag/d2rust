extern crate mpq;
extern crate amethyst;

use crate::d2::D2;
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Event, Pipeline, RenderBundle, Stage, VirtualKeyCode};
use amethyst::core::transform::TransformBundle;
use amethyst::utils::application_root_dir;

mod d2;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let config_path = format!("{}/resources/display_config.ron", application_root_dir());
    let display_config = DisplayConfig::load(&config_path);

    let pipe = Pipeline::build()
        .with_stage(
            Stage::with_backbuffer()
                .clear_target([100.0 / 255.0, 149.0 / 255.0, 237.0 / 255.0, 1.0], 1.0)
                .with_pass(DrawFlat2D::new()),
        );

    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderBundle::new(pipe, Some(display_config))
                .with_sprite_sheet_processor()
        )?
        .with_bundle(TransformBundle::new())?;

    let mut game = Application::new("./", D2, game_data)?;

    game.run();

    /*let mut d2 = D2::new(opengl);
    d2.init();
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            d2.render(&r);
        }

        if let Some(u) = e.update_args() {
            d2.update(&u)
        }
    }*/

    Ok(())
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
