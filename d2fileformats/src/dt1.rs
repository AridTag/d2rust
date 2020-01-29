use std::io::{Cursor, Seek, SeekFrom, Read};
use byteorder::{ReadBytesExt,LittleEndian};

#[derive(Clone,Debug)]
pub struct Dt1 {
    pub version1: u32,
    pub version2: u32,
    pub tiles: Vec<Tile>,
}

#[derive(Clone,Debug)]
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
    pub sub_tiles_header_pointer: u32,
    pub sub_tiles_header_size: u32,
    pub num_sub_tiles: u32,
    pub sub_tiles: Vec<SubTile>
}

#[derive(Clone,Debug)]
pub struct SubTile {
    pub x: u16,
    pub y: u16,
    pub grid_x: u8,
    pub grid_y: u8,
    pub format: TileFormat,
    pub length: u32,
    pub file_offset: u32,
    pub encoded_data: Vec<u8>,
}

#[derive(Clone,Debug)]
pub enum TileFormat {
    Standard,
    Isometric,
    Unknown(u16),
}

impl TileFormat {
    pub fn from_u16(value: u16) -> TileFormat {
        match value {
            0 => TileFormat::Standard,
            1 => TileFormat::Isometric,
            x => TileFormat::Unknown(x)
        }
    }
}


impl Dt1 {
    pub fn from(file_bytes: &[u8]) -> Result<Dt1, crate::Error> {
        let mut reader = Cursor::new(file_bytes);

        let version1 = reader.read_u32::<LittleEndian>()?;
        let version2 = reader.read_u32::<LittleEndian>()?;
        if version1 != 7 && version2 != 6 {
            panic!(format!("Unexpected dt1 version. Expected 7.6 but got {}.{}", version1, version2))
        }

        reader.seek(SeekFrom::Current(260))?;

        let num_tiles = reader.read_u32::<LittleEndian>()?;
        let tiles = Dt1::read_tiles(&mut reader, num_tiles as usize)?;

        Ok(Dt1 {
            version1,
            version2,
            tiles
        })
    }

    fn read_tiles(reader: &mut Cursor<&[u8]>, num_tiles: usize) -> Result<Vec<Tile>, crate::Error> {
        let mut tiles = Vec::with_capacity(num_tiles);

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
            let sub_tiles_header_pointer = reader.read_u32::<LittleEndian>()?;
            let sub_tiles_header_size = reader.read_u32::<LittleEndian>()?;
            let num_sub_tiles = reader.read_u32::<LittleEndian>()?;
            let sub_tiles = Dt1::read_sub_tiles(reader, num_sub_tiles, sub_tiles_header_pointer, sub_tiles_header_size)?;

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
                sub_tiles_header_pointer,
                sub_tiles_header_size,
                num_sub_tiles,
                sub_tiles
            })
        }

        Ok(tiles)
    }

    fn read_sub_tiles(reader: &mut Cursor<&[u8]>, num_sub_tiles: u32, sub_tiles_header_pointer: u32, sub_tiles_header_size: u32) -> Result<Vec<SubTile>, crate::Error> {
        reader.seek(SeekFrom::Start(sub_tiles_header_pointer as u64))?;

        let mut sub_tiles = Vec::with_capacity(num_sub_tiles as usize);
        for _ in 0..num_sub_tiles {
            let x = reader.read_u16::<LittleEndian>()?;
            let y = reader.read_u16::<LittleEndian>()?;
            reader.seek(SeekFrom::Current(2))?;
            let grid_x = reader.read_u8()?;
            let grid_y = reader.read_u8()?;
            let format = reader.read_u16::<LittleEndian>()?;
            let length = reader.read_u32::<LittleEndian>()?;
            reader.seek(SeekFrom::Current(2))?;
            let file_offset = reader.read_u32::<LittleEndian>()?;
            let encoded_data = Dt1::read_block_data(reader, sub_tiles_header_pointer + file_offset, length)?;

            sub_tiles.push(SubTile {
                x,
                y,
                grid_x,
                grid_y,
                format: TileFormat::from_u16(format),
                length,
                file_offset,
                encoded_data
            });
        }

        Ok(sub_tiles)
    }

    fn read_block_data(reader: &mut Cursor<&[u8]>, data_offset: u32, length: u32) -> Result<Vec<u8>, crate::Error> {
        reader.seek(SeekFrom::Start(data_offset as u64))?;

        let mut block_data = vec![0u8; length as usize];
        reader.read_exact(&mut block_data)?;

        Ok(block_data)
    }
}