#![recursion_limit = "1024"]

extern crate ndarray;
extern crate byteorder;
#[macro_use]
extern crate error_chain;

pub mod palette;
pub mod d2s;
pub mod dc6;
pub mod dcc;
pub mod ds1;
pub mod dt1;
pub mod read_string;

mod errors {
    error_chain! {
        errors {

        }

        foreign_links {
            Io(std::io::Error);
        }
    }
}

pub use errors::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
