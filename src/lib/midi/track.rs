use super::event::Event;
use crate::utils::{FromStream, ToStream};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Error, Read, Seek, SeekFrom, Write};

#[derive(Debug, Deserialize, Serialize)]
pub struct Track {
    chunks: Vec<Event>,
}

impl FromStream for Track {
    fn from_stream<R: Read + Seek>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        assert_eq!(b"MTrk", &buf);
        let length = reader.read_u32::<BigEndian>()?;

        let mut chunks = Vec::<Event>::new();

        let end = reader.seek(SeekFrom::Current(0))? as u32 + length;
        while (reader.seek(SeekFrom::Current(0))? as u32) < end {
            chunks.push(Event::from_stream(reader)?);
        }

        Ok(Self { chunks })
    }
}
