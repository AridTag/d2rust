use amethyst::assets::Source;
use amethyst::{Result, Error};
use amethyst::utils::application_root_dir;
use mpq::Archive;
use std::path::{Path, PathBuf};

pub const SOURCE_NAME: &str = "D2AssetSource";

pub struct D2AssetSource {
    pub data_base_path: PathBuf,
    mpq_sources: Vec<PathBuf>,
}

impl D2AssetSource {
    pub fn new(data_base_path: &str) -> D2AssetSource {
        let mut path = Path::new(data_base_path).to_path_buf();
        if path.is_relative() {
            path = application_root_dir().unwrap().join(path);
        }

        // TODO: Should return an error if the path doesn't exist

        D2AssetSource {
            data_base_path: path,
            mpq_sources: vec![],
        }
    }

    pub fn add_mpq(&mut self, mpq_path: &str) -> std::io::Result<()> {
        let mut path = PathBuf::from(mpq_path);
        if path.is_relative() {
            path = self.data_base_path.join(path);
        }

        let path_str = path.into_os_string().into_string().unwrap();
        if let Ok(_) = Archive::open(path_str.clone()) {
            self.mpq_sources.push(PathBuf::from(path_str));
            return Ok(());
        }

        Err(std::io::Error::from(std::io::ErrorKind::NotFound))
    }
}

impl Source for D2AssetSource {
    fn modified(&self, _path: &str) -> Result<u64> {
        Ok(0)
    }

    fn load(&self, path_: &str) -> Result<Vec<u8>> {
        let path = Path::new(path_);
        if path.is_absolute() {
            if path.exists() {
                if let Ok(bytes) = std::fs::read(path.clone()) {
                    return Ok(bytes);
                }
            }

            return Err(Error::from_string("Absolute path not found"));
        }

        if path.is_relative() {
            // "loose" files will take priority over archives
            let data_path = self.data_base_path.join(path);
            if data_path.exists() {
                if let Ok(bytes) = std::fs::read(data_path.clone()) {
                    return Ok(bytes);
                } else {
                    return Err(Error::from_string("Relative path not found"));
                }
            }
        }

        let filename = path.to_str().unwrap();

        // mpqs loaded later take priority over earlier loaded ones
        for mpq_path in self.mpq_sources.iter().rev() {
            if let Ok(mut archive) = Archive::open(mpq_path) {
                if let Ok(file) = archive.open_file(filename) {
                    // Found it
                    let mut buf = vec![0u8; file.size() as usize];
                    if let Err(_) = file.read(&mut archive, &mut buf) {
                        return Err(Error::from_string("Failed to read file from mpq"));
                    }
                    return Ok(buf);
                }
            }
        }

        Err(Error::from_string("File not found"))
    }
}
