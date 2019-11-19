use amethyst::assets::{Asset, Handle, ProcessingState, Format};
use amethyst::ecs::prelude::VecStorage;
use amethyst::{Error, Result};
use d2fileformats::d2s::*;

pub type D2sHandle = Handle<D2sAsset>;

#[derive(Clone, Debug)]
pub struct D2sAsset(pub D2s);

impl Asset for D2sAsset {
    const NAME: &'static str = "d2::D2s";
    type Data = Self;
    type HandleStorage = VecStorage<D2sHandle>;
}

impl From<D2sAsset> for Result<ProcessingState<D2sAsset>> {
    fn from(d2s: D2sAsset) -> Result<ProcessingState<D2sAsset>> {
        Ok(ProcessingState::Loaded(d2s))
    }
}

/// Amethyst Format for loading from '.d2s' files
#[derive(Clone, Copy, Debug, Default)]
pub struct D2sFormat;

impl Format<D2sAsset> for D2sFormat {
    fn name(&self) -> &'static str {
        "D2s"
    }

    fn import_simple(&self, bytes: Vec<u8>) -> Result<D2sAsset> {
        if let Ok(d2s) = D2s::from(&bytes) {
            return Ok(D2sAsset(d2s));
        }

        Err(Error::from_string("failed to read d2s"))
    }
}
