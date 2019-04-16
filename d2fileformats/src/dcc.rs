
pub struct DccHeader {
    signature: u8,
    version: u8,
    num_directions: u8,
    frames_per_direction: u32,
    tag: u32,
    final_dc6_size: u32,
}

pub struct DccFrame {
    unknown1: u32,
    width: u32,
    height: u32,
    x_offset: i32,
    y_offset: i32,
    optional_data: u32,
    coded_data: u32,
    flipped: u32
}