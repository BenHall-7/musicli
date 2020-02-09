use std::io::{Error, Read};

pub trait FromStream: Sized {
    fn from_stream<R: Read>(reader: &mut R) -> Result<Self, Error>;
}
