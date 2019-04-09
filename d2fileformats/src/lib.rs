use std::io::{Error, ErrorKind};

/// Represents a D2 palette
#[derive(Copy, Clone)]
pub struct Palette {
    /// The colors of this palette in RGBA
    pub colors: [[u8; 4]; 256]
}

impl Palette {
    /// Loads a Palette from the bytes of a palette file
    pub fn from(file_bytes: &[u8]) -> Result<Palette, Error> {
        if file_bytes.len() < 256 * 3 {
            return Err(Error::new(ErrorKind::InvalidInput, "Not enough bytes to decode palette"));
        }

        let mut colors: [[u8; 4]; 256] = [[0,0,0,0]; 256];
        for i in 0..colors.len() {
            let palette_index = i * 3;
            colors[i] = [file_bytes[palette_index + 2], file_bytes[palette_index + 1], file_bytes[palette_index], 255]
        }

        Ok(Palette {
            colors
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
