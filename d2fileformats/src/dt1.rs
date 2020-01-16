use std::io::{Cursor, Seek, SeekFrom};
use byteorder::{ReadBytesExt,LittleEndian};

pub struct Dt1 {
    pub version1: u32,
    pub version2: u32,
    pub tiles: Vec<Tile>,
}

pub struct Tile {
    pub direction: u32,
    pub roof_height: u16,
    pub material_flags: u16,
    pub height: u32,
    pub width: u32,
    pub tile_type: u32,
    pub style: u32,
    pub sequence: u32,
    pub rarity_frame_index: u32,
    pub subtile_flags: [u8;25],
    pub block_header_pointer: u32,
    pub block_header_size: u32,
    pub blocks: Vec<Block>
}

pub struct Block {
    pub x: u16,
    pub y: u16,
    pub grid_x: u8,
    pub grid_y: u8,
    pub format: u16,
    pub encoded_data: Vec<u8>,
    pub length: u32,
    pub file_offset: u32
}

impl Dt1 {
    pub fn from(file_bytes: &[u8]) -> Result<Dt1, std::io::Error> {
        let mut reader = Cursor::new(file_bytes);

        let version1 = reader.read_u32::<LittleEndian>()?;
        let version2 = reader.read_u32::<LittleEndian>()?;
        if version1 != 7 && version2 != 6 {
            panic!(format!("Unexpected dt1 version. Expected 7.6 but got {}.{}", version1, version2))
        }

        reader.seek(SeekFrom::Current(260))?;

        let num_tiles = reader.read_u32::<LittleEndian>()?;
        let mut tiles = Vec::with_capacity(num_tiles as usize);

        for _ in 0..num_tiles {
            let direction = reader.read_u32::<LittleEndian>()?;
            let roof_height = reader.read_u16::<LittleEndian>()?;
            let material_flags = reader.read_u16::<LittleEndian>()?;
            let height = reader.read_u32::<LittleEndian>()?;
            let width = reader.read_u32::<LittleEndian>()?;
            reader.seek(SeekFrom::Current(4))?;
            let tile_type = reader.read_u32::<LittleEndian>()?;
            let style = reader.read_u32::<LittleEndian>()?;
            let sequence = reader.read_u32::<LittleEndian>()?;
            let rarity_frame_index = reader.read_u32::<LittleEndian>()?;
            reader.seek(SeekFrom::Current(4))?;
            let mut subtile_flags = [0u8;25];
            for j in 0..subtile_flags.len() {
                subtile_flags[j] = reader.read_u8()?;
            }
            reader.seek(SeekFrom::Current(7))?;
            let block_header_pointer = reader.read_u32::<LittleEndian>()?;
            let block_header_size = reader.read_u32::<LittleEndian>()?;
            let num_blocks = reader.read_u32::<LittleEndian>()?;
            let blocks = Vec::with_capacity(num_blocks as usize);
            reader.seek(SeekFrom::Current(12))?;
            tiles.push(Tile {
                direction,
                roof_height,
                material_flags,
                height,
                width,
                tile_type,
                style,
                sequence,
                rarity_frame_index,
                subtile_flags,
                block_header_pointer,
                block_header_size,
                blocks
            })
        }

        Ok(Dt1 {
            version1,
            version2,
            tiles
        })
    }
}