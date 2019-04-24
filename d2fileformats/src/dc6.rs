use std::io::{Error, Cursor, Seek, SeekFrom};
use byteorder::{ReadBytesExt, LittleEndian};
use ndarray::prelude::*;
use std::fmt::{Debug, Formatter};

/// Defines the header of a Dc6 image
#[derive(Clone)]
pub struct Dc6Header {
    /// Version of the file format (always 6)
    pub version: u32,
    /// Flags for this image
    /// 1 - celfile_serialized
    /// 4 - celfile_24bit
    pub flags: u32,
    /// Encoding (always 0)
    pub encoding: u32,
    /// Termination code (usually 0xEEEEEEEE or 0xCDCDCDCD
    /// Possibly can be 3 bytes for fonts?
    pub termination: u32,
    /// Number of directions
    pub directions: u32,
    /// Number of frames per direction
    pub frames: u32
}

impl Dc6Header {
    fn from(reader: &mut Cursor<&[u8]>) -> Result<Dc6Header, Error> {
        let version = reader.read_u32::<LittleEndian>()?;
        let flags = reader.read_u32::<LittleEndian>()?;
        let encoding = reader.read_u32::<LittleEndian>()?;
        let termination = reader.read_u32::<LittleEndian>()?;
        let directions = reader.read_u32::<LittleEndian>()?;
        let frames = reader.read_u32::<LittleEndian>()?;

        Ok(Dc6Header {
            version,
            flags,
            encoding,
            termination,
            directions,
            frames
        })
    }
}

#[derive(Clone)]
pub struct Dc6 {
    pub header: Dc6Header,
    pub frames: Vec<Dc6Frame>,
}

impl Dc6 {
    pub fn from(file_bytes: &[u8]) -> Result<Dc6, Error> {
        let mut reader = Cursor::new(file_bytes);

        let header = Dc6Header::from(&mut reader)?;
        let total_frames = (header.directions * header.frames) as usize;

        let mut frame_offsets: Vec<u64> = Vec::with_capacity(total_frames);
        for _ in 0..total_frames {
            let offset = reader.read_u32::<LittleEndian>()?;
            frame_offsets.push(offset as u64);
        }

        let mut frames: Vec<Dc6Frame> = Vec::with_capacity(total_frames);
        for direction in 0..header.directions {
            for frame_num in 0..header.frames {
                let frame_index = ((direction * header.frames) + frame_num) as usize;
                reader.seek(SeekFrom::Start(frame_offsets[frame_index]))?;

                let frame = Dc6Frame::from(&mut reader)?;
                frames.push(frame);
            }
        }

        Ok(Dc6 {
            header,
            frames
        })
    }
}

impl Debug for Dc6 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "\n")?;
        write!(f, "version       : {}\n", self.header.version)?;
        write!(f, "frames        : {}x{}\n", self.header.frames, self.header.directions)?;
        write!(f, "encoding      : {}\n", self.header.encoding)?;
        write!(f, "flags         : {}\n", self.header.flags)?;
        write!(f, "termination   : {}\n", self.header.termination)?;
        write!(f, "frames\n")?;
        for frame in &self.frames {
            write!(f, "--------------------\n")?;
            write!(f, "    flipped : {}\n", frame.header.flipped)?;
            write!(f, "    width   : {}\n", frame.header.width)?;
            write!(f, "    height  : {}\n", frame.header.height)?;
            write!(f, "    offset_x: {}\n", frame.header.offset_x)?;
            write!(f, "    offset_y: {}\n", frame.header.offset_y)?;
            write!(f, "    unknown : {}\n", frame.header.unknown)?;
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct Dc6FrameHeader {
    /// If flipped is 0 the pixels for this frame should be (de)serialized in the appropriate order
    /// 0 = bottom right to top left
    /// ?1 = top left to bottom right?
    pub flipped: u32,
    pub width: u32,
    pub height: u32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub unknown: u32,
    pub next_block: i32,
    pub length: u32
}

impl Dc6FrameHeader {
    fn from(reader: &mut Cursor<&[u8]>) -> Result<Dc6FrameHeader, Error> {
        let flipped = reader.read_u32::<LittleEndian>()?;
        let width = reader.read_u32::<LittleEndian>()?;
        let height = reader.read_u32::<LittleEndian>()?;
        let offset_x = reader.read_i32::<LittleEndian>()?;
        let offset_y = reader.read_i32::<LittleEndian>()?;
        let unknown = reader.read_u32::<LittleEndian>()?;
        let next_block = reader.read_i32::<LittleEndian>()?;
        let length = reader.read_u32::<LittleEndian>()?;

        Ok(Dc6FrameHeader {
            flipped,
            width,
            height,
            offset_x,
            offset_y,
            unknown,
            next_block,
            length
        })
    }
}

#[derive(Clone)]
pub struct Dc6Frame {
    /// The header information
    pub header: Dc6FrameHeader,
    /// The pixel palette indices for this frame
    /// This field is arranged such that [(0,0)] is top left
    /// When serialized it should be written in the order denoted by [header.flipped](Dc6FrameHeader::flipped)
    pub pixels: Array2<u8>
}

impl Dc6Frame {
    fn from(reader: &mut Cursor<&[u8]>) -> Result<Dc6Frame, Error> {
        let header = Dc6FrameHeader::from(reader)?;
        let pixels: Array2<u8> = Dc6Frame::decode_pixels(reader, &header)?;

        Ok(Dc6Frame {
            header,
            pixels
        })
    }

    fn decode_pixels(reader: &mut Cursor<&[u8]>, frame_header: &Dc6FrameHeader) -> Result<Array2<u8>, Error> {
        const TRANSPARENT_OPCODE: u8 = 0x80;

        let mut pixels: Array2<u8> = Array2::zeros((frame_header.width as usize, frame_header.height as usize));
        let mut x: usize = 0;
        let mut y: usize = 0;
        if frame_header.flipped == 0 {
            y = (frame_header.height - 1) as usize;
        }

        let mut i = 0;
        while i < frame_header.length {
            let opcode = reader.read_u8()?;
            if opcode == TRANSPARENT_OPCODE {
                // The rest of the current line is blank
                // If we are on the last line then just break now
                if i == frame_header.length - 1 {
                    break;
                }

                x = 0;
                if frame_header.flipped == 0 {
                    y -= 1;
                }
                else {
                    y += 1;
                }
            } else if (opcode & TRANSPARENT_OPCODE) > 0 {
                // 0x7F transparent pixels in a row
                x += (opcode & 0x7F) as usize;
            } else {
                // opcode number of palette indices in a row
                for _ in 0..opcode {
                    pixels[(x, y)] = reader.read_u8()?;
                    i += 1;
                    x += 1;
                }
            }

            i += 1;
        }

        return Ok(pixels);
    }
}