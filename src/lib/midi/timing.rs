use crate::midi::SMPTETimecode;
use binread::io::{Error, ErrorKind, Read, Seek};
use binread::Endian::Big;
use binread::{BinRead, BinReaderExt, BinResult, ReadOptions};
use serde::{Deserialize, Serialize};

// todo: turn the enum comments back into doc comments when the bug is gone from binread
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum Timing {
    /// Indicates a subdivision of quarter notes.
    /// This timing is affected by tempo tracks, because it depends on the time of a quarter note.
    Metrical { precision: u16 },
    /// Indicates a subdivision of each second into frames.
    /// A frame is subdivided by the second value.
    /// This timing is not affected by tempo tracks.
    Real { fps: SMPTETimecode, precision: u8 },
}

impl BinRead for Timing {
    type Args = ();

    fn read_options<R: Read + Seek>(reader: &mut R, _: &ReadOptions, _: ()) -> BinResult<Self> {
        let short = reader.read_type::<i16>(Big)?;
        if short > 0 {
            Ok(Timing::Metrical {
                precision: short as u16,
            })
        } else {
            let timecode_amount = -((short >> 8) as i8);
            let timecode = SMPTETimecode::from(timecode_amount as u32)
                .or_else(|_| Err(Error::from(ErrorKind::InvalidData)))?;

            Ok(Timing::Real {
                fps: timecode,
                precision: (short & 0xff) as u8,
            })
        }
    }
}
