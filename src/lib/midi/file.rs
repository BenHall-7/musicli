use super::{Format, Timing};
use crate::midi::Track;
use crate::utils::{FromStream, ToStream};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Error, Read, Seek, Write};

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

impl FromStream for File {
    fn from_stream<R: Read + Seek>(reader: &mut R) -> Result<Self, Error> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        assert_eq!(b"MThd", &buf);

        let size = reader.read_u32::<BigEndian>()?;
        assert_eq!(size, 6);

        let format = reader.read_u16::<BigEndian>()?;
        let track_count = reader.read_u16::<BigEndian>()?;
        let timing = Timing::from_stream(reader)?;

        match format {
            0 => Ok(File {
                format: Format::SingleTrack(Track::from_stream(reader)?),
                timing,
            }),
            1 => {
                let mut tracks = Vec::with_capacity(track_count as usize);
                for _ in 0..track_count {
                    tracks.push(Track::from_stream(reader)?);
                }
                Ok(File {
                    format: Format::MultipleTrack(tracks),
                    timing,
                })
            }
            2 => {
                let mut tracks = Vec::with_capacity(track_count as usize);
                for _ in 0..track_count {
                    tracks.push(Track::from_stream(reader)?);
                }
                Ok(File {
                    format: Format::ParallelTrack(tracks),
                    timing,
                })
            }
            _ => panic!(),
        }
    }
}
