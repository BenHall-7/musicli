use std::io::{Error, Read, Seek};

pub trait FromStream: Sized {
    fn from_stream<R: Read + Seek>(reader: &mut R) -> Result<Self, Error>;
}
