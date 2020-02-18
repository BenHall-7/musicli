use crate::utils::{FromStream, SMPTETimecode, ToStream};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Error, ErrorKind, Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Timing {
    /// Indicates a subdivision of quarter notes into a number of pulses.
    /// This timing is affected by tempo tracks.
    Metrical(u16),
    /// Indicates a subdivision of each second into frames.
    /// A frame is subdivided once again by the second value.
    /// This timing is not affected by tempo tracks.
    Real(SMPTETimecode, u8),
}

impl FromStream for Timing {
    fn from_stream<R: Read>(reader: &mut R) -> Result<Self, Error> {
        let short = reader.read_i16::<BigEndian>()?;
        if short > 0 {
            Ok(Timing::Metrical(short as u16))
        } else {
            let timecode_amount = -((short >> 8) as i8);
            let timecode = SMPTETimecode::from(timecode_amount as u32)
                .or_else(|_| Err(Error::from(ErrorKind::InvalidData)))?;

            Ok(Timing::Real(
                timecode,
                (short & 0xff) as u8,
            ))
        }
    }
}

impl ToStream for Timing {
    fn to_stream<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        match self {
            Timing::Metrical(div) => {
                writer.write_u16::<BigEndian>(div & 0x7fff)?;
                Ok(())
            }
            Timing::Real(timecode, div) => {
                writer.write_i8(-(*timecode as i8))?;
                writer.write_u8(*div)?;
                Ok(())
            },
        }
    }
}
