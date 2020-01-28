use amethyst::assets::{Asset, Handle, ProcessingState, Format};
use amethyst::ecs::prelude::VecStorage;
use d2fileformats::dt1::Dt1;
use amethyst::{Error, Result};

#[derive(Clone, Copy, Debug, Default)]
pub struct Dt1Format;

impl Format<Dt1Asset> for Dt1Format {
    fn name(&self) -> &'static str {
        "DT1"
    }

    fn import_simple(&self, bytes: Vec<u8>) -> Result<Dt1Asset> {
        if let Ok(dt1) = Dt1::from(&bytes) {
            return Ok(Dt1Asset(dt1));
        }

        Err(Error::from_string("failed to read dt1"))
    }
}

pub type Dt1Handle = Handle<Dt1Asset>;

#[derive(Clone, Debug)]
pub struct Dt1Asset(pub Dt1);

impl Asset for Dt1Asset {
    const NAME: &'static str = "d2::Dt1";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

impl From<Dt1Asset> for Result<ProcessingState<Dt1Asset>> {
    fn from(dt1: Dt1Asset) -> Result<ProcessingState<Dt1Asset>> {
        Ok(ProcessingState::Loaded(dt1))
    }
}