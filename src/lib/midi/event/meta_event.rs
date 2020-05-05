use crate::utils::{FromStream, ToStream};
use crate::midi::VarLengthValue;
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom, Write};

#[derive(Debug, Deserialize, Serialize)]
pub enum MetaEvent {
    // 0x00
    SequenceNumber(Option<u16>),
    Text(String),
    Copyright(String),
    TrackName(String),
    Instrument(String),
    Lyric(String),
    Marker(String),
    CuePoint(String),
    ProgramName(String),
    DeviceName(String),
    // 0x20
    MidiChannelPrefix(u8),
    MidiPort(u8),
    // 0x2f
    EndOfTrack,
    // 0x51
    Tempo{ms_per_beat: u32},
    // 0x54
    SMPTEOffset{hr: u8, mn: u8,se: u8,fr: u8, ff:u8},
    // 0x58
    TimeSignature {
        numerator: u8,
        denominator: u8,
        clocks_per_metronome: u8,
        something: u8,
    },
    KeySignature{sf: i8,mi: u8},

    Unsupported(u8, Vec<u8>),
}

impl FromStream for MetaEvent {
    fn from_stream<R: Read + Seek>(reader: &mut R) -> Result<Self, Error> {
        let meta_type = reader.read_u8()?;
        let size = VarLengthValue::from_stream(reader)?;

        macro_rules! read_string {
            ($event:path) => {{
                let mut buffer = vec![0u8; size.0 as usize];
                reader.read_exact(&mut buffer)?;
                Ok($event(String::from_utf8(buffer).unwrap()))
            }};
        }

        match meta_type {
            0x00 => {
                if size.0 == 0 {
                    Ok(MetaEvent::SequenceNumber(None))
                } else if size.0 == 2 {
                    Ok(MetaEvent::SequenceNumber(Some(reader.read_u16::<BigEndian>()?)))
                } else {
                    Err(Error::from(ErrorKind::InvalidData))
                }
            }
            0x01 => read_string!(MetaEvent::Text),
            0x02 => read_string!(MetaEvent::Copyright),
            0x03 => read_string!(MetaEvent::TrackName),
            0x04 => read_string!(MetaEvent::Instrument),
            0x05 => read_string!(MetaEvent::Lyric),
            0x06 => read_string!(MetaEvent::Marker),
            0x07 => read_string!(MetaEvent::CuePoint),
            0x08 => read_string!(MetaEvent::ProgramName),
            0x09 => read_string!(MetaEvent::DeviceName),
            0x20 => {
                assert_eq!(size.0, 1);
                Ok(MetaEvent::MidiChannelPrefix(reader.read_u8()?))
            }
            0x21 => {
                assert_eq!(size.0, 1);
                Ok(MetaEvent::MidiPort(reader.read_u8()?))
            }
            0x2f => {
                assert_eq!(size.0, 0);
                Ok(MetaEvent::EndOfTrack)
            }
            0x51 => {
                assert_eq!(size.0, 3);
                Ok(MetaEvent::Tempo{
                    ms_per_beat: (
                        (reader.read_u8()? as u32) << 16
                        | (reader.read_u8()? as u32) << 8
                        | (reader.read_u8()? as u32)
                    )
                })
            }
            0x54 => {
                assert_eq!(size.0, 5);
                Ok(MetaEvent::SMPTEOffset {
                    hr: reader.read_u8()?,
                    mn: reader.read_u8()?,
                    se: reader.read_u8()?,
                    fr: reader.read_u8()?,
                    ff: reader.read_u8()?,
                })
            }
            0x58 => {
                if size.0 == 2 {
                    Ok(MetaEvent::TimeSignature {
                        numerator: reader.read_u8()?,
                        denominator: reader.read_u8()?,
                        clocks_per_metronome: 24,
                        something: 8,
                    })
                } else if size.0 == 4 {
                    Ok(MetaEvent::TimeSignature {
                        numerator: reader.read_u8()?,
                        denominator: reader.read_u8()?,
                        clocks_per_metronome: reader.read_u8()?,
                        something: reader.read_u8()?,
                    })
                } else {
                    panic!();
                }
            }
            0x59 => {
                assert_eq!(size.0, 2);
                Ok(MetaEvent::KeySignature {
                    sf: reader.read_i8()?,
                    mi: reader.read_u8()?,
                })
            }
            _ => {
                let mut buffer = vec![0u8; size.0 as usize];
                reader.read_exact(&mut buffer)?;
                Ok(MetaEvent::Unsupported(meta_type, buffer))
            }
        }
    }
}