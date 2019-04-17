use std::io::{Read, Error};

pub trait ReadString {
    fn read_zstring(&mut self) -> Result<String, Error>;
}

impl <T: Read> ReadString for T {
    fn read_zstring(&mut self) -> Result<String, Error> {
        let mut buf = String::new();
        let mut current = [0u8; 1];
        loop {
            self.read(&mut current)?;

            if current[0] == 0 {
                break;
            }

            buf.push(current[0] as char);
        }

        Ok(buf)
    }
}