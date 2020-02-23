use std::io::{Error, Seek, Write};

pub trait ToStream {
    fn to_stream<W: Write + Seek>(&self, writer: &mut W) -> Result<(), Error>;
}
