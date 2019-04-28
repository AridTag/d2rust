#[allow(dead_code)]
extern crate amethyst;
extern crate amethyst_imgui;
extern crate mpq;

use crate::dc6_format::Dc6Asset;
use crate::palette_format::PaletteAsset;
use crate::states::InitState;
use amethyst::assets::Processor;
use amethyst::core::transform::TransformBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage};
use amethyst::utils::application_root_dir;
use crate::d2::{SpriteCountComponent, SpriteAnimationComponent};
use amethyst_imgui::{imgui};

mod d2;
mod d2assetsource;
mod dc6_format;
mod palette_format;
mod states;

#[derive(Default, Clone, Copy)]
pub struct ImguiUseSystem;
impl<'s> amethyst::ecs::System<'s> for ImguiUseSystem {
    type SystemData = ();

    fn run(&mut self, _: Self::SystemData) {
        amethyst_imgui::with(|ui| {
            ui.window(imgui::im_str!("Hello world"))
                .size((300.0, 100.0), imgui::ImGuiCond::FirstUseEver)
                .build(|| {
                    ui.text(imgui::im_str!("Hello world!"));
                    ui.text(imgui::im_str!("こんにちは世界！"));
                    ui.text(imgui::im_str!("This...is...imgui-rs!"));
                    ui.separator();
                    let mouse_pos = ui.imgui().mouse_pos();
                    ui.text(imgui::im_str!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
                });

            ui.show_demo_window(&mut true);
        });
    }
}


fn main() -> amethyst::Result<()> {
    let mut logconfig = amethyst::LoggerConfig::default();
    logconfig.stdout = amethyst::StdoutLog::Off;
    amethyst::start_logger(logconfig);

    let root_dir = application_root_dir().expect("");
    let config_path = format!("{}/resources/display_config.ron", root_dir.to_str().unwrap());
    let display_config = DisplayConfig::load(&config_path);

    const BLACK: [f32;4] = [0.0, 0.0, 0.0, 1.0];
    const CORNFLOWER_BLUE: [f32;4] = [100.0 / 255.0, 149.0 / 255.0, 237.0 / 255.0, 1.0];
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target(BLACK, 1.0)
            .with_pass(DrawFlat2D::new())
            .with_pass(amethyst_imgui::DrawUi::default()),
    );

    let game_data = GameDataBuilder::default()
        .with(amethyst_imgui::BeginFrame::default(), "imgui_begin", &[])
        .with_barrier()
        .with(Processor::<Dc6Asset>::new(), "", &[])
        .with(Processor::<PaletteAsset>::new(), "", &[])
        .with_bundle(TransformBundle::new())?
        .with(ImguiUseSystem::default(), "imgui_use", &[])
        .with_bundle(RenderBundle::new(pipe, Some(display_config)).with_sprite_sheet_processor())?
        .with_barrier()
        .with(amethyst_imgui::EndFrame::default(), "imgui_end", &[]);

    let mut game = Application::build("./", InitState::new()).unwrap()
        .register::<SpriteCountComponent>()
        .register::<SpriteAnimationComponent>()
        .build(game_data)
        .expect("Failed to build Application");
    //let mut game = Application::new("./", InitState::new(), game_data)?;

    game.run();

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn ds1_loads() {
        use d2fileformats::ds1::Ds1;
        use mpq::Archive;

        let archive_path = "D:\\Diablo II\\d2exp.mpq";
        let mut archive =
            Archive::open(archive_path).expect(&format!("can not find archive {}", archive_path));

        let filename = "data\\global\\tiles\\expansion\\Town\\townWest.ds1";
        let archive_file = archive
            .open_file(filename)
            .expect(&format!("can't find {} in {}", filename, archive_path));
        let mut file_buffer = vec![0u8; archive_file.size() as usize];

        archive_file
            .read(&mut archive, &mut file_buffer)
            .expect(&format!("failed to read {}", filename));
        let ds1 = Ds1::from(&file_buffer).expect("");
        println!("{}\n{:?}", filename, ds1);
    }
}
