use crate::utils::Bounded;
use crate::utils::FromStream;
use crate::utils::ToStream;
use byteorder::{ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Error, Read, Seek, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct VarLengthValue(pub(crate) u32);

impl FromStream for VarLengthValue {
    fn from_stream<R: Read + Seek>(reader: &mut R) -> Result<Self, Error> {
        let mut value = 0u32;
        let max_bytes = 4;
        for _ in 0..max_bytes {
            let byte = reader.read_u8()? as u32;
            value |= byte & 0x7f;
            if (byte & 0x80) != 0 {
                value <<= 7;
            } else {
                break;
            }
        }

        Ok(VarLengthValue::from(value))
    }
}

impl ToStream for VarLengthValue {
    fn to_stream<W: Write + Seek>(&self, writer: &mut W) -> Result<(), Error> {
        let mut value = self.0;
        // small enough to avoid heap allocation, probably?
        let mut buffer = [0u8; 4];
        let mut bufferlen = 0;
        let mut byte = (value & 0x7f) as u8;

        for (ind, v) in buffer.iter_mut().enumerate() {
            *v = byte;
            bufferlen = ind;
            value >>= 7;
            if value == 0 {
                break;
            } else {
                byte = (value & 0x7f) as u8 | 0x80;
            }
        }
        for i in (0..=bufferlen).rev() {
            writer.write_u8(buffer[i])?;
        }

        Ok(())
    }
}

impl From<u32> for VarLengthValue {
    fn from(value: u32) -> VarLengthValue {
        Self(Self::clamp(value))
    }
}

impl Bounded<u32> for VarLengthValue {
    const MIN: u32 = 0;
    const MAX: u32 = 0x0fff_ffff;
}
