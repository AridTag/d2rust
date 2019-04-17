
pub struct DccHeader {
    pub signature: u8,
    pub version: u8,
    pub num_directions: u8,
    pub frames_per_direction: u32,
    pub tag: u32,
    pub final_dc6_size: u32,
}

pub struct DccFrame {
    pub unknown1: u32,
    pub width: u32,
    pub height: u32,
    pub x_offset: i32,
    pub y_offset: i32,
    pub optional_data: u32,
    pub coded_data: u32,
    pub flipped: u32
}