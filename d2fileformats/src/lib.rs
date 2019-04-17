extern crate ndarray;
extern crate byteorder;

pub mod palette;
pub mod dc6;
pub mod dcc;
pub mod ds1;
pub mod read_string;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
