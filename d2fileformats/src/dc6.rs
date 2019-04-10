use std::io::{Error, Cursor, ErrorKind, Seek, SeekFrom};
use byteorder::{ReadBytesExt, LittleEndian};
use ndarray::prelude::*;

/// Defines the header of a Dc6 image
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
        // TODO: Error handling
        let version = reader.read_u32::<LittleEndian>().unwrap();
        let flags = reader.read_u32::<LittleEndian>().unwrap();
        let encoding = reader.read_u32::<LittleEndian>().unwrap();
        let termination = reader.read_u32::<LittleEndian>().unwrap();
        let directions = reader.read_u32::<LittleEndian>().unwrap();
        let frames = reader.read_u32::<LittleEndian>().unwrap();

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

pub struct Dc6FrameHeader {
    /// If flipped is 0 the frames colors are arranged bottom right to top left
    pub flipped: u32,
    pub width: i32,
    pub height: i32,
    pub offset_x: i32,
    pub offset_y: i32,
    pub unknown: u32,
    pub next_block: i32,
    pub length: u32
}

impl Dc6FrameHeader {
    fn from(reader: &mut Cursor<&[u8]>) -> Result<Dc6FrameHeader, Error> {
        // TODO: Error handling
        let flipped = reader.read_u32::<LittleEndian>().unwrap();
        let width = reader.read_i32::<LittleEndian>().unwrap();
        let height = reader.read_i32::<LittleEndian>().unwrap();
        let offset_x = reader.read_i32::<LittleEndian>().unwrap();
        let offset_y = reader.read_i32::<LittleEndian>().unwrap();
        let unknown = reader.read_u32::<LittleEndian>().unwrap();
        let next_block = reader.read_i32::<LittleEndian>().unwrap();
        let length = reader.read_u32::<LittleEndian>().unwrap();

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

pub struct Dc6Frame {
    pub header: Dc6FrameHeader,
    /// The pixel palette indices for this frame
    pub pixels: Array2<u8>
}

const TRANSPARENT_OPCODE: u8 = 0x80;

impl Dc6Frame {
    fn from(reader: &mut Cursor<&[u8]>) -> Result<Dc6Frame, Error> {
        // TODO: Error handling
        let header = Dc6FrameHeader::from(reader).unwrap();
        let mut pixels: Array2<u8> = Array2::zeros((header.width as usize, header.height as usize));
        if header.flipped == 0 {
            Dc6Frame::decode_pixels_bottom_top(reader, &mut pixels, &header);
        } else {
            return Err(Error::new(ErrorKind::NotFound, "Top to bottom decode not implemented"));
        }

        Ok(Dc6Frame {
            header,
            pixels
        })
    }

    fn decode_pixels_bottom_top(reader: &mut Cursor<&[u8]>, pixels: &mut Array2<u8>, frame_header: &Dc6FrameHeader) {
        // TODO: Error handling
        let mut x: usize = 0;
        let mut y: usize = (frame_header.height - 1) as usize;

        let mut i = 0;
        while i < frame_header.length {
            let opcode = reader.read_u8().unwrap();
            if opcode == TRANSPARENT_OPCODE {
                // The rest of the current line is blank
                x = 0;
                y -= 1;
            } else if (opcode & TRANSPARENT_OPCODE) > 0 {
                // 0x7F transparent pixels in a row
                x += (opcode & 0x7F) as usize;
            } else {
                // opcode number of palette indices in a row
                for _ in 0..opcode {
                    pixels[(x, y)] = reader.read_u8().unwrap();
                    i += 1;
                    x += 1;
                }
            }

            i += 1;
        }
    }
}

pub struct Dc6 {
    pub header: Dc6Header,
    pub frames: Vec<Dc6Frame>,
}

impl Dc6 {
    pub fn from(file_bytes: &[u8]) -> Result<Dc6, Error> {
        let mut reader = Cursor::new(file_bytes);

        // TODO: Error handling
        let header = Dc6Header::from(&mut reader).unwrap();
        let total_frames = (header.directions * header.frames) as usize;

        let mut frame_offsets: Vec<u64> = Vec::with_capacity(total_frames);
        for _ in 0..total_frames {
            frame_offsets.push(reader.read_u32::<LittleEndian>().unwrap() as u64);
        }

        let mut frames: Vec<Dc6Frame> = Vec::with_capacity(total_frames);
        for direction in 0..header.directions {
            for frame_num in 0..header.frames {
                let frame_index = ((direction * header.frames) + frame_num) as usize;
                reader.seek(SeekFrom::Start(frame_offsets[frame_index])).unwrap();
                frames.push(Dc6Frame::from(&mut reader).unwrap());
            }
        }

        Ok(Dc6 {
            header,
            frames
        })
    }
}