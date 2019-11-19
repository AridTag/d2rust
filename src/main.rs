#[allow(dead_code)]
extern crate amethyst;
extern crate amethyst_imgui;
extern crate mpq;

use crate::asset_formats::{D2sAsset, Dc6Asset, PaletteAsset};
use crate::d2::{SpriteAnimationComponent, SpriteCountComponent};
use crate::states::InitState;
use amethyst::assets::Processor;
use amethyst::core::transform::TransformBundle;
use amethyst::renderer::{
    plugins::{RenderToWindow},
    types::DefaultBackend,
    RenderingBundle
};
use amethyst::{
    input::{InputBundle, StringBindings},
};
use amethyst::prelude::*;
use amethyst::utils::application_root_dir;
use amethyst_imgui::RenderImgui;

mod asset_formats;
mod d2;
mod d2assetsource;
mod states;

#[derive(Default, Clone, Copy)]
pub struct ImguiUseSystem;
impl<'s> amethyst::ecs::System<'s> for ImguiUseSystem {
    type SystemData = ();

    fn run(&mut self, _: Self::SystemData) {
        amethyst_imgui::with(|ui| {
            /*ui.window(imgui::im_str!("Hello world"))
                .size((300.0, 100.0), imgui::ImGuiCond::FirstUseEver)
                .build(|| {
                    ui.text(imgui::im_str!("Hello world!"));
                    ui.text(imgui::im_str!("こんにちは世界！"));
                    ui.text(imgui::im_str!("This...is...imgui-rs!"));
                    ui.separator();
                    let mouse_pos = ui.imgui().mouse_pos();
                    ui.text(imgui::im_str!(
                        "Mouse Position: ({:.1},{:.1})",
                        mouse_pos.0,
                        mouse_pos.1
                    ));
                });*/

            ui.show_demo_window(&mut true);
        });
    }
}

fn main() -> amethyst::Result<()> {
    let mut logconfig = amethyst::LoggerConfig::default();
    logconfig.stdout = amethyst::StdoutLog::Off;
    amethyst::start_logger(logconfig);

    let root_dir = application_root_dir().expect("");
    let display_config_path = format!(
        "{}/resources/display_config.ron",
        root_dir.to_str().unwrap()
    );

    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    const CORNFLOWER_BLUE: [f32; 4] = [100.0 / 255.0, 149.0 / 255.0, 237.0 / 255.0, 1.0];

    let game_data = GameDataBuilder::default()
        .with_barrier()
        .with(Processor::<Dc6Asset>::new(), "", &[])
        .with(Processor::<PaletteAsset>::new(), "", &[])
        .with(Processor::<D2sAsset>::new(), "", &[])
        .with_bundle(TransformBundle::new())?
        .with(ImguiUseSystem::default(), "imgui_use", &[])
        .with_bundle(InputBundle::<StringBindings>::default())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderToWindow::from_config_path(display_config_path).with_clear(CORNFLOWER_BLUE))
                .with_plugin(RenderImgui::<StringBindings>::default()),
        )?;



    let mut game = Application::build("./", InitState::new())
        .unwrap()
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
