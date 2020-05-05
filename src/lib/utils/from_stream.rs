use std::io::{Error, Read, Seek};

pub trait FromStreamContext: Sized {
    type Context;

    fn from_stream_context<R: Read + Seek>(
        reader: &mut R,
        context: &mut Self::Context,
    ) -> Result<Self, Error>;
}

pub trait FromStream: Sized {
    fn from_stream<R: Read + Seek>(reader: &mut R) -> Result<Self, Error>;
}