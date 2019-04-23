use d2fileformats::dc6;
use amethyst::assets;
use amethyst::assets::{Asset, SimpleFormat, Handle, ProcessingState};
use amethyst::ecs::prelude::VecStorage;
use serde::{Deserialize, Serialize};
use d2fileformats::dc6::Dc6;
use std::cmp::max;

/// Structure acting as scaffolding for serde when loading a spritesheet file.
/// Positions originate in the top-left corner (bitmap image convention).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct SpritePosition {
    /// Horizontal position of the sprite in the sprite sheet
    pub x: u32,
    /// Vertical position of the sprite in the sprite sheet
    pub y: u32,
    /// Width of the sprite
    pub width: u32,
    /// Height of the sprite
    pub height: u32
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct SerializedSpriteSheet {
    /// Width of the sprite sheet
    pub spritesheet_width: u32,
    /// Height of the sprite sheet
    pub spritesheet_height: u32,
    /// Description of the sprites
    pub sprites: Vec<SpritePosition>,
}

pub type Dc6Handle = Handle<Dc6Asset>;

#[derive(Clone,Debug)]
pub struct Dc6Asset(pub Dc6);

impl Dc6Asset {
    pub fn to_spritesheet_ron(&self) -> String {
        let dc6 = &self.0;

        let mut texture_width = 0;
        let mut texture_height = 0;
        for direction in 0..dc6.header.directions {
            let mut height = 0;
            let mut row_width = 0;
            for frame_num in 0..dc6.header.frames {
                let frame_index = ((direction * dc6.header.frames) + frame_num) as usize;
                let frame = dc6.frames.get(frame_index).unwrap();
                row_width += frame.header.width;
                height = max(height, frame.header.height);
            }
            texture_width = max(texture_width, row_width);
            texture_height += height;
        }

        let mut sprites: Vec<SpritePosition> = vec![];
        for direction in 0..dc6.header.directions {
            let mut current_x = 0;
            for frame_num in 0..dc6.header.frames {
                let frame_index = ((direction * dc6.header.frames) + frame_num) as usize;
                let frame = dc6.frames.get(frame_index).unwrap();

                current_x += frame.header.width;
            }
        }

        /*let mut result: String = String::new();
        result.push_str("(\n");
        result = format!("{}\tspritesheet_width: {},\n", result, texture_width);
        result = format!("{}\tspritesheet_height: {},\n", result, texture_height);
        result = format!("{}\tsprites: [\n", result);
        for direction in 0..dc6.header.directions {
            for frame_num in 0..header.frames {
                let frame_index = ((direction * header.frames) + frame_num) as usize;

                let frame = dc6.frames.get(frame_index).unwrap();
                result = format!("{}\t\t(\n", result);
                result = format!("{}\t\t\tx: {}\n", result, frame.header.);
                result = format!("{}\t\t\ty: {}\n", result, frame.header.offset_y);
                result = format!("{}\t\t\twidth: {}\n", result, frame.header.width);
                result = format!("{}\t\t\theight: {}\n", result, frame.header.height);
            }
        }*/

        return String::new();
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

        Err(assets::Error::from_kind(assets::ErrorKind::Format("failed to read dc6")))
    }
}