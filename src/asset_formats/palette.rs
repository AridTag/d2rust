use amethyst::assets::{Asset, Handle, ProcessingState, Format};
use amethyst::ecs::prelude::VecStorage;
use amethyst::{Error, Result};
use d2fileformats::palette::Palette;

pub type PaletteHandle = Handle<PaletteAsset>;

#[derive(Clone, Debug)]
pub struct PaletteAsset(pub Palette);

impl Asset for PaletteAsset {
    const NAME: &'static str = "d2::Palette";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

impl From<PaletteAsset> for Result<ProcessingState<PaletteAsset>> {
    fn from(palette: PaletteAsset) -> Result<ProcessingState<PaletteAsset>> {
        Ok(ProcessingState::Loaded(palette))
    }
}

/// Amethyst Format for loading from '.dc6' files
#[derive(Clone, Copy, Debug, Default)]
pub struct PaletteFormat;

impl Format<PaletteAsset> for PaletteFormat {
    fn name(&self) -> &'static str {
        "Palette"
    }

    fn import_simple(&self, bytes: Vec<u8>) -> Result<PaletteAsset> {
        if let Ok(pal) = Palette::from(&bytes) {
            return Ok(PaletteAsset(pal));
        }

        Err(Error::from_string("failed to read dc6"))
    }
}
