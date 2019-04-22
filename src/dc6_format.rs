use d2fileformats::dc6;
use amethyst::assets;
use amethyst::assets::{Asset, SimpleFormat, Handle, ProcessingState};
use amethyst::ecs::prelude::VecStorage;
use d2fileformats::dc6::Dc6;

pub type Dc6Handle = Handle<Dc6Asset>;

#[derive(Clone,Debug)]
pub struct Dc6Asset(pub Dc6);

impl Dc6Asset {
    pub fn to_spritesheet_ron(&self) -> String {
        let dc6 = &self.0;
        let texture_width: i32 = dc6.frames.iter().map(|f| f.header.width).sum();
        let texture_height: i32 = dc6.header.directions as i32 * dc6.frames.iter().map(|f| f.header.height).max().unwrap(); // TODO: This isn't exactly correct

        let mut result: String = String::new();
        result.push_str("(\n");
        result = format!("{}\tspritesheet_width: {},\n", result, texture_width);
        result = format!("{}\tspritesheet_height: {},\n", result, texture_height);
        // TODO: finish me
        return result;
    }
}

impl Asset for Dc6Asset {
    const NAME: &'static str = "d2::Dc6";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

impl From<Dc6Asset> for assets::Result<ProcessingState<Dc6Asset>> {
    fn from(dc6: Dc6Asset) -> assets::Result<ProcessingState<Dc6Asset>> {
        Ok(ProcessingState::Loaded(dc6))
    }
}

/// Amethyst Format for loading from '.dc6' files
#[derive(Clone, Copy, Debug, Default)]
pub struct Dc6Format;

impl SimpleFormat<Dc6Asset> for Dc6Format {
    const NAME: &'static str = "DC6";
    type Options = ();

    fn import(&self, bytes: Vec<u8>, _: Self::Options) -> assets::Result<Dc6Asset> {
        if let Ok(dc6) = Dc6::from(&bytes) {
            return Ok(Dc6Asset(dc6));
        }

        return Err(assets::Error::from_kind(assets::ErrorKind::Format("failed to read dc6")));
    }
}