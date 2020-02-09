use std::io::{Error, Write};

pub trait ToStream {
    fn to_stream<W: Write>(&self, writer: &mut W) -> Result<(), Error>;
}
