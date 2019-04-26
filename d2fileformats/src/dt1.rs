
pub struct Dt1 {
    pub version: u32,
    pub version2: u32,
    pub reserved: [u8;260],
    pub tiles: Vec<Tile>,
}

pub struct Tile {
    pub direction: u32,
    pub roof_height: u16,
    pub sound_index: u8,
    pub animated: u8,
    pub height: u32,
    pub width: u32,
    pub orientation: u32,
    pub main_index: u32,
    pub sub_index: u32,
    pub rarity_or_frame_index: u32,
    pub unknown1: u8,
    pub unknown2: u8,
    pub unknown3: u8,
    pub unknown4: u8,
    pub subtile_flags: [u8;25]
}