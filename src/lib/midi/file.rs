use super::{Format, Timing};
use crate::midi::Track;
use binread::io::{Read, Seek};
use binread::Endian::Big;
use binread::{BinRead, BinReaderExt, BinResult, ReadOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub format: Format,
    timing: Timing,
}

impl File {
    pub fn get_timing(&self) -> Timing {
        self.timing
    }
}

impl BinRead for File {
    type Args = ();

    fn read_options<R: Read + Seek>(reader: &mut R, _: &ReadOptions, _: ()) -> BinResult<Self> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        assert_eq!(b"MThd", &buf);

        let size = reader.read_type::<u32>(Big)?;
        assert_eq!(size, 6);

        let format = reader.read_type::<u16>(Big)?;
        let track_count = reader.read_type::<u16>(Big)?;
        let timing = Timing::read(reader)?;

        match format {
            0 => Ok(File {
                format: Format::SingleTrack(Track::read(reader)?),
                timing,
            }),
            1 => {
                let mut tracks = Vec::with_capacity(track_count as usize);
                for _ in 0..track_count {
                    tracks.push(Track::read(reader)?);
                }
                Ok(File {
                    format: Format::MultipleTrack(tracks),
                    timing,
                })
            }
            2 => {
                let mut tracks = Vec::with_capacity(track_count as usize);
                for _ in 0..track_count {
                    tracks.push(Track::read(reader)?);
                }
                Ok(File {
                    format: Format::ParallelTrack(tracks),
                    timing,
                })
            }
            _ => panic!("MIDI format specifier only allows values 0, 1, or 2"),
        }
    }
}
