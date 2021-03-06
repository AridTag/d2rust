use std::io::{Cursor, Seek, SeekFrom, ErrorKind};
use byteorder::{ReadBytesExt, LittleEndian};
use std::cmp::min;
use std::fmt::{Debug, Formatter};
use ndarray::Array2;
use crate::read_string::ReadString;
use crate::errors::*;

#[derive(Clone)]
pub struct Layer<T> {
    pub width: u32,
    pub height: u32,
    pub cells: Array2<T>
}

impl<T> Layer<T> where T: Clone + Default {
    fn new(width: u32, height: u32) -> Layer<T> {
        Layer {
            width,
            height,
            cells: Array2::from_elem((width as usize, height as usize), Default::default())
        }
    }
}

#[derive(Clone,Default)]
pub struct WallCell {
    pub prop1: u8,
    pub prop2: u8,
    pub prop3: u8,
    pub prop4: u8,
    pub orientation: u8
}

#[derive(Clone,Default)]
pub struct FloorCell {
    pub prop1: u8,
    pub prop2: u8,
    pub prop3: u8,
    pub prop4: u8,
}

#[derive(Clone,Default)]
pub struct TagCell {
    pub prop1: u32
}

#[derive(Clone,Default)]
pub struct ShadowCell {
    pub prop1: u8,
    pub prop2: u8,
    pub prop3: u8,
    pub prop4: u8
}

#[derive(Clone,Default)]
pub struct Object {
    pub type_: u32,
    pub id: u32,
    /// The objects x position in sub-cell coordinates
    pub x: u32,
    /// The objects y position in sub-cell coordinates
    pub y: u32,
    pub flags: u32,
    pub path_nodes: Vec<PathNode>,
}

#[derive(Clone,Default)]
pub struct TagGroup {
    pub tile_x: u32,
    pub tile_y: u32,
    pub width: u32,
    pub height: u32,
    pub unknown: u32,
}

#[derive(Clone,Default)]
pub struct PathNode {
    pub x: u32,
    pub y: u32,
    pub action: u32,
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

    /// The names of the files
    pub file_names: Vec<String>,

    pub wall_layers: Vec<Layer<WallCell>>,
    pub floor_layers: Vec<Layer<FloorCell>>,
    pub shadow_layers: Vec<Layer<ShadowCell>>,
    pub tag_layers: Vec<Layer<TagCell>>,

    pub objects: Vec<Object>,
    pub tag_groups: Vec<TagGroup>,
}

impl Ds1 {
    pub fn from(file_bytes: &[u8]) -> Result<Ds1> {
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

        let mut file_names = vec![];
        if version >= 3 {
            let file_count = reader.read_u32::<LittleEndian>()?;
            for _ in 0..file_count {
                file_names.push(reader.read_zstring()?);
            }
        }

        if version >= 9 && version <= 13 {
            // Skip 2 u32 for some reason
            reader.seek(SeekFrom::Current(8))?;
        }

        let shadow_layer_count: u32 = 1;
        let mut wall_layer_count: u32 = 1;
        let mut floor_layer_count: u32 = 1;
        let mut tag_layer_count: u32 = 0;
        if version >= 10 && (tag_type == 1 || tag_type == 2) {
            tag_layer_count = 1;
        }

        if version >= 4 {
            wall_layer_count = reader.read_u32::<LittleEndian>()?;

            if version >= 16 {
                floor_layer_count = reader.read_u32::<LittleEndian>()?;
            }
        }

        // TODO: This whole "layer_order" thing is just terrible
        let mut layer_order = vec![1, 9, 5, 12, 11];
        if version >= 4 {
            layer_order = vec![];
            for i in 0..wall_layer_count {
                layer_order.push(1 + i); // cells for wall layer i
                layer_order.push(5 + i); // wall cells orientations
            }
            for i in 0..floor_layer_count {
                layer_order.push(9 + i); // cells for floor layer i
            }

            // TODO: Should we support multiple layers of these?
            if shadow_layer_count != 0 {
                layer_order.push(11); // cells for shadow layer i
            }
            if tag_layer_count != 0 {
                layer_order.push(12); // cells for tag layer i
            }
        }

        let mut wall_layers: Vec<Layer<WallCell>> = vec![Layer::<WallCell>::new(width, height); wall_layer_count as usize];
        let mut floor_layers: Vec<Layer<FloorCell>> = vec![Layer::<FloorCell>::new(width, height); floor_layer_count as usize];
        let mut shadow_layers: Vec<Layer<ShadowCell>> = vec![Layer::<ShadowCell>::new(width, height); shadow_layer_count as usize];
        let mut tag_layers: Vec<Layer<TagCell>> = vec![Layer::<TagCell>::new(width, height); tag_layer_count as usize];
        for read_type in layer_order {
            match read_type {
                1..=4 => {
                    let layer_index = (read_type - 1) as usize;
                    if let Some(layer) = wall_layers.get_mut(layer_index) {
                        Ds1::read_wall_cells(&mut reader, layer)?;
                    } else {
                        bail!("No wall layer at index {}. this shouldn't happen", layer_index);
                    }
                }

                5..=8 => {
                    let layer_index = (read_type - 5) as usize;
                    if let Some(layer) = wall_layers.get_mut(layer_index) {
                        Ds1::read_wall_cells_orientation(&mut reader, layer, version)?;
                    } else {
                        bail!("(Orientation) No wall layer at index {}. this shouldn't happen", layer_index);
                    }
                }

                9..=10 => {
                    let layer_index = (read_type - 9) as usize;
                    if let Some(layer) = floor_layers.get_mut(layer_index) {
                        Ds1::read_floor_cells(&mut reader, layer)?;
                    } else {
                        bail!("No floor layer at index {}. This shouldn't happen", layer_index);
                    }
                }

                11 => {
                    let layer_index = (read_type - 11) as usize;
                    if let Some(layer) = shadow_layers.get_mut(layer_index) {
                        Ds1::read_shadow_cells(&mut reader, layer)?;
                    } else {
                        bail!("No shadow layer at index {}. This shouldn't happen", layer_index);
                    }
                }

                12 => {
                    let layer_index = (read_type - 12) as usize;
                    if let Some(layer) = tag_layers.get_mut(layer_index) {
                        Ds1::read_tag_cells(&mut reader, layer)?;
                    } else {
                        bail!("No tag layer at index {}. This shouldn't happen", layer_index);
                    }
                }

                _ => {
                    bail!("Unknown layer type {}", read_type)
                }
            }
        }

        let mut objects: Vec<Object> = Ds1::read_objects(&mut reader, version)?;
        let tag_groups = Ds1::read_tag_groups(&mut reader, version, tag_type)?;

        if Ds1::is_reader_at_end(&mut reader) {
            // we need to return early as the file ended prematurely in tag groups
            return Ok(Ds1 {
                version,
                width,
                height,
                act,
                tag_type,
                file_names,
                wall_layers,
                floor_layers,
                shadow_layers,
                tag_layers,
                objects,
                tag_groups
            })
        }

        Ds1::read_npc_paths(&mut reader, version, &mut objects)?;

        Ok(Ds1 {
            version,
            width,
            height,
            act,
            tag_type,
            file_names,
            wall_layers,
            floor_layers,
            shadow_layers,
            tag_layers,
            objects,
            tag_groups,
        })
    }

    fn is_reader_at_end(reader: &mut Cursor<&[u8]>) -> bool {
        let current_pos = reader.position();
        if let Err(_) = reader.seek(SeekFrom::End(0)) {
            return true;
        }
        let end_pos = reader.position();
        reader.seek(SeekFrom::Start(current_pos)).expect("this shouldn't fail (famous last words)");

        return current_pos < end_pos;
    }

    fn read_npc_paths(reader: &mut Cursor<&[u8]>, version: u32, objects: &mut Vec<Object>) -> Result<()> {
        let npc_path_count = reader.read_u32::<LittleEndian>()?;
        for _ in 0..npc_path_count {
            let node_count = reader.read_u32::<LittleEndian>()?;
            let obj_x = reader.read_u32::<LittleEndian>()?;
            let obj_y = reader.read_u32::<LittleEndian>()?;

            let mut matching_objects = objects.iter_mut().filter(|o| o.x == obj_x && o.y == obj_y).collect::<Vec<&mut Object>>();
            if matching_objects.len() == 0 {
                // TODO: logging instead of println
                println!("WARNING: No objects at position {},{}", obj_x, obj_y);
                // TODO: This needs to skip the data
                continue;
            }

            for _ in 0..node_count {
                let node_x = reader.read_u32::<LittleEndian>()?;
                let node_y = reader.read_u32::<LittleEndian>()?;
                let mut action = 1;
                if version >= 15 {
                    action = reader.read_u32::<LittleEndian>()?;
                }

                for obj in matching_objects.iter_mut() {
                    if obj.path_nodes.len() > 0 {
                        // TODO: something should be done
                        println!("WARNING: obj already has path nodes");
                    }

                    let node = PathNode {
                        x: node_x,
                        y: node_y,
                        action
                    };
                    obj.path_nodes.push(node);
                }
            }
        }

        Ok(())
    }

    fn read_tag_groups(reader: &mut Cursor<&[u8]>, version: u32, tag_type: u32) -> Result<Vec<TagGroup>> {
        if version < 12 || !(tag_type == 1 || tag_type == 2) {
            return Ok(vec![]);
        }

        if version >= 18 {
            // not sure why but we skip a dword
            reader.seek(SeekFrom::Current(4))?;
        }

        // Note: When reading tag groups it is entirely possible for the file to just suddenly end.
        // This is the case with data\global\tiles\act1\outdoors\trees.ds1 for example.

        let group_count = reader.read_u32::<LittleEndian>()?;
        let mut tag_groups: Vec<TagGroup> = vec![Default::default(); group_count as usize];
        for group in tag_groups.iter_mut() {
            let tile_x = reader.read_u32::<LittleEndian>();
            match tile_x {
                Ok(x) => {
                    group.tile_x = x;
                }

                Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => {
                    break;
                }

                Err(e) => {
                    bail!(e);
                }
            }

            let tile_y = reader.read_u32::<LittleEndian>();
            match tile_y {
                Ok(y) => {
                    group.tile_y = y;
                }

                Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => {
                    break;
                }

                Err(e) => {
                    bail!(e);
                }
            }

            let width = reader.read_u32::<LittleEndian>();
            match width {
                Ok(w) => {
                    group.width = w;
                }

                Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => {
                    break;
                }

                Err(e) => {
                    bail!(e);
                }
            }

            let height = reader.read_u32::<LittleEndian>();
            match height {
                Ok(h) => {
                    group.height = h;
                }

                Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => {
                    break;
                }

                Err(e) => {
                    bail!(e);
                }
            }

            if version >= 13 {
                let unknown = reader.read_u32::<LittleEndian>();
                match unknown {
                    Ok(u) => {
                        group.unknown = u;
                    }

                    Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => {
                        break;
                    }

                    Err(e) => {
                        bail!(e);
                    }
                }
            }
        }

        Ok(tag_groups)
    }

    fn read_tag_cells(reader: &mut Cursor<&[u8]>, layer: &mut Layer<TagCell>) -> Result<()> {
        for y in 0..layer.height {
            for x in 0..layer.width {
                if let Some(cell) = layer.cells.get_mut((x as usize, y as usize)) as Option<&mut TagCell> {
                    cell.prop1 = reader.read_u32::<LittleEndian>()?;
                } else {
                    bail!("No tag cell at {},{}. This shouldn't happen", x, y);
                }
            }
        }

        Ok(())
    }

    fn read_shadow_cells(reader: &mut Cursor<&[u8]>, layer: &mut Layer<ShadowCell>) -> Result<()> {
        for y in 0..layer.height {
            for x in 0..layer.width {
                if let Some(cell) = layer.cells.get_mut((x as usize, y as usize)) as Option<&mut ShadowCell> {
                    cell.prop1 = reader.read_u8()?;
                    cell.prop2 = reader.read_u8()?;
                    cell.prop3 = reader.read_u8()?;
                    cell.prop4 = reader.read_u8()?;
                } else {
                    bail!("No shadow cell at {},{}. This shouldn't happen", x, y);
                }
            }
        }

        Ok(())
    }

    fn read_floor_cells(reader: &mut Cursor<&[u8]>, layer: &mut Layer<FloorCell>) -> Result<()> {
        for y in 0..layer.height {
            for x in 0..layer.width {
                if let Some(cell) = layer.cells.get_mut((x as usize, y as usize)) as Option<&mut FloorCell> {
                    cell.prop1 = reader.read_u8()?;
                    cell.prop2 = reader.read_u8()?;
                    cell.prop3 = reader.read_u8()?;
                    cell.prop4 = reader.read_u8()?;
                } else {
                    bail!("No floor cell at {},{}. This shouldn't happen", x, y);
                }
            }
        }

        Ok(())
    }

    fn read_wall_cells(reader: &mut Cursor<&[u8]>, layer: &mut Layer<WallCell>) -> Result<()> {
        for y in 0..layer.height {
            for x in 0..layer.width {
                if let Some(cell) = layer.cells.get_mut((x as usize, y as usize)) as Option<&mut WallCell> {
                    cell.prop1 = reader.read_u8()?;
                    cell.prop2 = reader.read_u8()?;
                    cell.prop3 = reader.read_u8()?;
                    cell.prop4 = reader.read_u8()?;
                } else {
                    bail!("No wall cell at {},{}. This shouldn't happen", x, y);
                }
            }
        }

        Ok(())
    }

    fn read_wall_cells_orientation(reader: &mut Cursor<&[u8]>, layer: &mut Layer<WallCell>, version: u32) -> Result<()> {
        let orientation_lookup: [u8; 25] = [0x00, 0x01, 0x02, 0x01, 0x02, 0x03, 0x03, 0x05, 0x05, 0x06,
            0x06, 0x07, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10, 0x11, 0x12, 0x14];
        for y in 0..layer.height {
            for x in 0..layer.width {
                if let Some(cell) = layer.cells.get_mut((x as usize, y as usize)) as Option<&mut WallCell> {
                    let value = reader.read_u8()?;
                    if version < 7 {
                        cell.orientation = orientation_lookup[value as usize];
                    } else {
                        cell.orientation = value;
                    }
                    reader.seek(SeekFrom::Current(3))?; // skip 3 bytes?
                } else {
                    bail!("(Orientation) No wall cell at {},{}. This shouldn't happen", x, y);
                }
            }
        }

        Ok(())
    }

    fn read_objects(reader: &mut Cursor<&[u8]>, version: u32) -> Result<Vec<Object>> {
        if version < 2 {
            // No objects prior to version 2
            return Ok(vec![]);
        }

        let object_count = reader.read_u32::<LittleEndian>()?;
        let mut objects: Vec<Object> = vec![Default::default(); object_count as usize];
        for obj in objects.iter_mut() {
            obj.type_ = reader.read_u32::<LittleEndian>()?;
            obj.id = reader.read_u32::<LittleEndian>()?;
            obj.x = reader.read_u32::<LittleEndian>()?;
            obj.y = reader.read_u32::<LittleEndian>()?;

            if version >= 5 {
                obj.flags = reader.read_u32::<LittleEndian>()?;
            }
        }

        Ok(objects)
    }
}

impl Debug for Ds1 {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "version       : {}\n", self.version)?;
        write!(f, "width         : {}\n", self.width)?;
        write!(f, "height        : {}\n", self.height)?;
        write!(f, "act           : {}\n", self.act)?;
        write!(f, "tag_type      : {}\n", self.tag_type)?;
        write!(f, "file_count    : {}\n", self.file_names.len())?;
        for file in &self.file_names {
            write!(f, "    {}\n", file)?;
        }
        write!(f, "wall_layers   : {}\n", self.wall_layers.len())?;
        for layer in &self.wall_layers {
            let first_cell = layer.cells.first();
            match first_cell {
                Some(c) => {
                    write!(f, "    First Cell\n")?;
                    write!(f, "      prop1 : {}\n", c.prop1)?;
                    write!(f, "      prop2 : {}\n", c.prop2)?;
                    write!(f, "      prop3 : {}\n", c.prop3)?;
                    write!(f, "      prop4 : {}\n", c.prop4)?;
                }

                None => {
                    write!(f, "    no cells\n")?;
                }
            }
        }
        write!(f, "floor_layers  : {}\n", self.floor_layers.len())?;
        for layer in &self.floor_layers {
            let first_cell = layer.cells.first();
            match first_cell {
                Some(c) => {
                    write!(f, "    First Cell\n")?;
                    write!(f, "      prop1 : {}\n", c.prop1)?;
                    write!(f, "      prop2 : {}\n", c.prop2)?;
                    write!(f, "      prop3 : {}\n", c.prop3)?;
                    write!(f, "      prop4 : {}\n", c.prop4)?;
                }

                None => {
                    write!(f, "    no cells\n")?;
                }
            }
        }
        write!(f, "shadow_layers : {}\n", self.shadow_layers.len())?;
        for layer in &self.shadow_layers {
            let first_cell = layer.cells.first();
            match first_cell {
                Some(c) => {
                    write!(f, "    First Cell\n")?;
                    write!(f, "      prop1 : {}\n", c.prop1)?;
                    write!(f, "      prop2 : {}\n", c.prop2)?;
                    write!(f, "      prop3 : {}\n", c.prop3)?;
                    write!(f, "      prop4 : {}\n", c.prop4)?;
                }

                None => {
                    write!(f, "    no cells\n")?;
                }
            }
        }
        write!(f, "tag_layers    : {}\n", self.tag_layers.len())?;
        for layer in &self.tag_layers {
            let first_cell = layer.cells.first();
            match first_cell {
                Some(c) => {
                    write!(f, "    First Cell\n")?;
                    write!(f, "      prop1 : {}\n", c.prop1)?;
                }

                None => {
                    write!(f, "    no cells\n")?;
                }
            }
        }
        write!(f, "object_count  : {}\n", self.objects.len())?;
        for obj in &self.objects {
            write!(f, "    Obj [id {}]\n", obj.id)?;
            write!(f, "      type       : {}\n", obj.type_)?;
            write!(f, "      x          : {}\n", obj.x)?;
            write!(f, "      y          : {}\n", obj.y)?;
            write!(f, "      flags      : {}\n", obj.flags)?;
            write!(f, "      path_nodes : {}\n", obj.path_nodes.len())?;
            for node in &obj.path_nodes {
                write!(f, "        ({},{}) action: {}\n", node.x, node.y, node.action)?;
            }
        }
        write!(f, "tag groups    : {}\n", self.tag_groups.len())?;
        for group in &self.tag_groups {
            write!(f, "    at ({},{}) size ({},{}) with unknown {}\n", group.tile_x, group.tile_y, group.width, group.height, group.unknown)?;
        }

        Ok(())
    }
}