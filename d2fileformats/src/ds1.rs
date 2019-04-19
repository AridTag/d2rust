use std::io::{Error, Cursor, Seek, SeekFrom, ErrorKind};
use byteorder::{ReadBytesExt, LittleEndian};
use std::cmp::min;
use std::fmt::{Debug, Formatter};
use crate::read_string::ReadString;
use ndarray::Array2;

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
    pub orientation: u8,
    pub bt_idx: i32,
    pub flags: u8
}

#[derive(Clone,Default)]
pub struct FloorCell {
    pub prop1: u8,
    pub prop2: u8,
    pub prop3: u8,
    pub prop4: u8,
    pub bt_idx: i32,
    pub flags: u8
}

#[derive(Clone,Default)]
pub struct TagCell {
    pub prop1: u32,
    pub flags: u8
}

#[derive(Clone,Default)]
pub struct ShadowCell {
    pub prop1: u8,
    pub prop2: u8,
    pub prop3: u8,
    pub prop4: u8,
    pub bt_idx: i32,
    pub flags: u8
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
                        return Err(Error::from(ErrorKind::Other));//, format!("No wall layer at index {}. this shouldn't happen", layer_index)));
                    }
                }

                5..=8 => {
                    let layer_index = (read_type - 5) as usize;
                    if let Some(layer) = wall_layers.get_mut(layer_index) {
                        Ds1::read_wall_cells_orientation(&mut reader, layer, version)?;
                    } else {
                        return Err(Error::from(ErrorKind::Other));//, "(Orientation) No wall layer at index {}. this shouldn't happen", layer_index));
                    }
                }

                9..=10 => {
                    let layer_index = (read_type - 9) as usize;
                    if let Some(layer) = floor_layers.get_mut(layer_index) {
                        Ds1::read_floor_cells(&mut reader, layer)?;
                    } else {
                        return Err(Error::from(ErrorKind::Other));//, "No floor layer at index {}. This shouldn't happen", layer_index));
                    }
                }

                11 => {
                    let layer_index = (read_type - 11) as usize;
                    if let Some(layer) = shadow_layers.get_mut(layer_index) {
                        Ds1::read_shadow_cells(&mut reader, layer)?;
                    } else {
                        return Err(Error::from(ErrorKind::Other));//, "No shadow layer at index {}. This shouldn't happen", layer_index));
                    }
                }

                12 => {
                    let layer_index = (read_type - 12) as usize;
                    if let Some(layer) = tag_layers.get_mut(layer_index) {
                        Ds1::read_tag_cells(&mut reader, layer)?;
                    } else {
                        return Err(Error::from(ErrorKind::Other));//, "No tag layer at index {}. This shouldn't happen", layer_index));
                    }
                }

                _ => {
                    panic!("Unknown layer type {}", read_type)
                }
            }
        }

        let objects = Ds1::read_objects(&mut reader, version)?;

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
            objects
        })
    }

    fn read_tag_cells(reader: &mut Cursor<&[u8]>, layer: &mut Layer<TagCell>) -> Result<(), Error> {
        for y in 0..layer.height {
            for x in 0..layer.width {
                if let Some(cell) = layer.cells.get_mut((x as usize, y as usize)) as Option<&mut TagCell> {
                    cell.prop1 = reader.read_u32::<LittleEndian>()?;
                } else {
                    return Err(Error::from(ErrorKind::Other));//, "No tag cell at {},{}. This shouldn't happen", x, y));
                }
            }
        }

        Ok(())
    }

    fn read_shadow_cells(reader: &mut Cursor<&[u8]>, layer: &mut Layer<ShadowCell>) -> Result<(), Error> {
        for y in 0..layer.height {
            for x in 0..layer.width {
                if let Some(cell) = layer.cells.get_mut((x as usize, y as usize)) as Option<&mut ShadowCell> {
                    cell.prop1 = reader.read_u8()?;
                    cell.prop2 = reader.read_u8()?;
                    cell.prop3 = reader.read_u8()?;
                    cell.prop4 = reader.read_u8()?;
                } else {
                    return Err(Error::from(ErrorKind::Other));//, "No shadow cell at {},{}. This shouldn't happen", x, y));
                }
            }
        }

        Ok(())
    }

    fn read_floor_cells(reader: &mut Cursor<&[u8]>, layer: &mut Layer<FloorCell>) -> Result<(), Error> {
        for y in 0..layer.height {
            for x in 0..layer.width {
                if let Some(cell) = layer.cells.get_mut((x as usize, y as usize)) as Option<&mut FloorCell> {
                    cell.prop1 = reader.read_u8()?;
                    cell.prop2 = reader.read_u8()?;
                    cell.prop3 = reader.read_u8()?;
                    cell.prop4 = reader.read_u8()?;
                } else {
                    return Err(Error::from(ErrorKind::Other));//, "No floor cell at {},{}. This shouldn't happen", x, y));
                }
            }
        }

        Ok(())
    }

    fn read_wall_cells(reader: &mut Cursor<&[u8]>, layer: &mut Layer<WallCell>) -> Result<(), Error> {
        for y in 0..layer.height {
            for x in 0..layer.width {
                if let Some(cell) = layer.cells.get_mut((x as usize, y as usize)) as Option<&mut WallCell> {
                    cell.prop1 = reader.read_u8()?;
                    cell.prop2 = reader.read_u8()?;
                    cell.prop3 = reader.read_u8()?;
                    cell.prop4 = reader.read_u8()?;
                } else {
                    return Err(Error::from(ErrorKind::Other));//, "No wall cell at {},{}. This shouldn't happen", x, y))
                }
            }
        }

        Ok(())
    }

    fn read_wall_cells_orientation(reader: &mut Cursor<&[u8]>, layer: &mut Layer<WallCell>, version: u32) -> Result<(), Error> {
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
                    return Err(Error::from(ErrorKind::Other));//, "(Orientation) No wall cell at {},{}. This shouldn't happen", x, y))
                }
            }
        }

        Ok(())
    }

    fn read_objects(reader: &mut Cursor<&[u8]>, version: u32) -> Result<Vec<Object>, Error> {
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
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
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
            write!(f, "      type  : {}\n", obj.type_)?;
            write!(f, "      x     : {}\n", obj.x)?;
            write!(f, "      y     : {}\n", obj.y)?;
            write!(f, "      flags : {}\n", obj.flags)?;
        }

        Ok(())
    }
}