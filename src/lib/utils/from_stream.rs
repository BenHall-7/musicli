use std::io::{Error, Read, Seek};

pub trait FromStream: Sized {
    type Context;

    fn from_stream<R: Read + Seek>(
        reader: &mut R,
        context: &mut Self::Context,
    ) -> Result<Self, Error>;
}
