pub struct Ds1 {
    version: u32,
    /// Number of tiles wide
    width: u32,
    /// Number of tiles tall
    height: u32,
    /// Determines which palette this map uses
    act: u32,
    /// If true there exists an unknown layer in the file (after wall, floor, and shadow layers)
    has_unknown_layer: u32,
    extra_files_count: u32
}

pub struct LayerHeader {
    /// Number of wall layers to use
    wall_layers: u32,
    /// Number of floor layers to use
    floor_layers: u32,
    /// Number of shadow layers to use
    shadow_layers: u32
}