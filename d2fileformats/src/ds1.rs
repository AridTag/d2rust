use std::io::{Error, Cursor, Seek, SeekFrom};
use byteorder::{ReadBytesExt, LittleEndian};
use std::cmp::min;
use std::fmt::{Debug, Formatter};
use crate::read_string::ReadString;
use ndarray::Array2;

pub struct Layer<T> {
    pub width: u32,
    pub height: u32,
    pub cells: Array2<T>
}

pub struct WallCell {
    pub prop1: u8,
    pub prop2: u8,
    pub prop3: u8,
    pub prop4: u8,
    pub orientation: u8,
    pub bt_idx: i32,
    pub flags: u8
}

pub struct FloorCell {
    pub prop1: u8,
    pub prop2: u8,
    pub prop3: u8,
    pub prop4: u8,
    pub bt_idx: i32,
    pub flags: u8
}

pub struct Ds1 {
    /// The ds1 format version
    pub version: u32,

    /// Number of tiles wide
    pub width: u32,

    /// Number of tiles tall
    pub height: u32,

    /// Determines which palette this map uses
    pub act: u32,

    /// Some sort of tag layer thing
    pub tag_type: u32,

    /// The number of files
    pub file_count: u32,

    /// The names of the files
    pub file_names: Vec<String>,

    /// The number of wall layers
    pub wall_layer_count: u32,

    /// The number of floor layers
    pub floor_layer_count: u32,

    /// The number of shadow layers
    pub shadow_layer_count: u32,

    /// The number of tag layers
    pub tag_layer_count: u32,

    pub wall_layers: Vec<Layer<WallCell>>,
    pub floor_layers: Vec<Layer<WallCell>>,
}

impl Ds1 {
    pub fn from(file_bytes: &[u8]) -> Result<Ds1, Error> {
        let mut reader = Cursor::new(file_bytes);

        let version = reader.read_u32::<LittleEndian>()?;
        let width = reader.read_u32::<LittleEndian>()? + 1;
        let height= reader.read_u32::<LittleEndian>()? + 1;

        let mut act = 1;
        if version >= 8 {
            act = min(reader.read_u32::<LittleEndian>()? + 1, 5);
        }

        let mut tag_type = 0;
        if version >= 10 {
            tag_type = reader.read_u32::<LittleEndian>()?;
        }

        let mut file_count = 0;
        let mut file_names = vec![];
        if version >= 3 {
            file_count = reader.read_u32::<LittleEndian>()?;
            for i in 0..file_count {
                file_names.push(reader.read_zstring()?);
            }
        }

        if version >= 9 && version <= 13 {
            // Skip 2 u32 for some reason
            reader.seek(SeekFrom::Current(8));
        }

        let mut shadow_layer_count = 1u32;
        let mut wall_layer_count   = 1u32;
        let mut floor_layer_count  = 1u32;
        let mut tag_layer_count    = 1u32;

        if version >= 4 {
            wall_layer_count = reader.read_u32::<LittleEndian>()?;

            if version >= 16 {
                floor_layer_count = reader.read_u32::<LittleEndian>()?;
            }
        }

        let mut layer_order = [1, 9, 5, 12, 11];
        let mut total_layers: u32 = 5;
        if version >= 4 {
            total_layers = (2 * wall_layer_count) + floor_layer_count + shadow_layer_count + tag_layer_count;
        }

        for i in 0..total_layers {
            for y in 0..height {
                for x in 0..width {

                }
            }
        }

        Ok(Ds1 {
            version,
            width,
            height,
            act,
            tag_type,
            file_count,
            file_names,
            wall_layer_count,
            floor_layer_count,
            shadow_layer_count,
            tag_layer_count,
            wall_layers: vec![],
            floor_layers: vec![]
        })
    }
}

impl Debug for Ds1 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "version       : {}\n", self.version)?;
        write!(f, "width         : {}\n", self.width)?;
        write!(f, "height        : {}\n", self.height)?;
        write!(f, "act           : {}\n", self.act)?;
        write!(f, "tag_type      : {}\n", self.tag_type)?;
        write!(f, "file_count    : {}\n", self.file_count)?;
        for file in &self.file_names {
            write!(f, "    {}\n", file)?;
        }
        write!(f, "wall_layers   : {}\n", self.wall_layer_count)?;
        write!(f, "floor_layers  : {}\n", self.floor_layer_count)?;
        write!(f, "shadow_layers : {}\n", self.shadow_layer_count)?;
        write!(f, "tag_layers    : {}\n", self.tag_layer_count)?;
        Ok(())
    }
}

pub struct LayerHeader {
    /// Number of wall layers to use
    pub wall_layers: u32,
    /// Number of floor layers to use
    pub floor_layers: u32,
    /// Number of shadow layers to use
    pub shadow_layers: u32
}