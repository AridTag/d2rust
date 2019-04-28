use std::fmt::{Debug, Formatter};
use crate::errors::*;

/// Represents a D2 palette
#[derive(Copy, Clone)]
pub struct Palette {
    /// The colors of this palette in BGR
    pub colors: [[u8; 3]; 256]
}

impl Debug for Palette {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Yep it's a palette\n")?;

        Ok(())
    }
}

impl Palette {
    /// Loads a Palette from the bytes of a palette file
    pub fn from(file_bytes: &[u8]) -> Result<Palette> {
        if file_bytes.len() < 256 * 3 {
            bail!("Not enough bytes to decode palette");
        }

        let mut colors: [[u8; 3]; 256] = [[0,0,0]; 256];
        for i in 0..colors.len() {
            let palette_index = i * 3;
            colors[i] = [file_bytes[palette_index], file_bytes[palette_index + 1], file_bytes[palette_index + 2]]
        }

        Ok(Palette {
            colors
        })
    }
}