use crate::midi::VarLengthValue;
use binread::io::{Read, Seek, SeekFrom};
use binread::{BinRead, BinResult, Error, ReadOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, BinRead)]
#[serde(from = "MetaEventType")]
#[serde(into = "MetaEventType")]
pub struct MetaEvent {
    meta_type: u8,
    length: VarLengthValue,
    #[br(args(meta_type, length))]
    pub variant: MetaEventType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TimeSignature {
    numerator: u8,
    denominator: u8,
    clocks_per_metronome: u8,
    something: u8,
}

#[derive(Debug, Clone, Deserialize, Serialize, BinRead)]
#[br(import(ty: u8, len: VarLengthValue))]
pub enum MetaEventType {
    #[br(assert(ty == 0))]
    SequenceNumber(#[br(if(len.0 == 2))] Option<u16>),
    #[br(assert(ty == 1))]
    Text(
        #[br(count = len.0)]
        #[br(map = |v: Vec<u8>| String::from_utf8_lossy(&v).to_string())]
        String,
    ),
    #[br(assert(ty == 2))]
    Copyright(
        #[br(count = len.0)]
        #[br(map = |v: Vec<u8>| String::from_utf8_lossy(&v).to_string())]
        String,
    ),
    #[br(assert(ty == 3))]
    TrackName(
        #[br(count = len.0)]
        #[br(map = |v: Vec<u8>| String::from_utf8_lossy(&v).to_string())]
        String,
    ),
    #[br(assert(ty == 4))]
    Instrument(
        #[br(count = len.0)]
        #[br(map = |v: Vec<u8>| String::from_utf8_lossy(&v).to_string())]
        String,
    ),
    #[br(assert(ty == 5))]
    Lyric(
        #[br(count = len.0)]
        #[br(map = |v: Vec<u8>| String::from_utf8_lossy(&v).to_string())]
        String,
    ),
    #[br(assert(ty == 6))]
    Marker(
        #[br(count = len.0)]
        #[br(map = |v: Vec<u8>| String::from_utf8_lossy(&v).to_string())]
        String,
    ),
    #[br(assert(ty == 7))]
    CuePoint(
        #[br(count = len.0)]
        #[br(map = |v: Vec<u8>| String::from_utf8_lossy(&v).to_string())]
        String,
    ),
    #[br(assert(ty == 8))]
    ProgramName(
        #[br(count = len.0)]
        #[br(map = |v: Vec<u8>| String::from_utf8_lossy(&v).to_string())]
        String,
    ),
    #[br(assert(ty == 9))]
    DeviceName(
        #[br(count = len.0)]
        #[br(map = |v: Vec<u8>| String::from_utf8_lossy(&v).to_string())]
        String,
    ),
    #[br(assert(ty == 0x20))]
    MidiChannelPrefix(u8),
    #[br(assert(ty == 0x21))]
    MidiPort(u8),
    #[br(assert(ty == 0x2f))]
    EndOfTrack,
    #[br(assert(ty == 0x51))]
    Tempo {
        #[br(map = |v: [u8; 3]|
            ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | (v[2] as u32)
        )]
        ms_per_beat: u32,
    },
    #[br(assert(ty == 0x54))]
    SMPTEOffset {
        hours: u8,
        minutes: u8,
        seconds: u8,
        fr: u8,
        ff: u8,
    },
    #[br(assert(ty == 0x58))]
    TimeSignature(#[br(args(len))] TimeSignature),
    #[br(assert(ty == 0x59))]
    KeySignature { sf: i8, mi: u8 },
    Unsupported {
        #[br(calc = ty)]
        ty: u8,
        #[br(count = len.0)]
        data: Vec<u8>,
    },
}

impl Default for TimeSignature {
    fn default() -> Self {
        TimeSignature {
            numerator: 4,
            denominator: 2,
            clocks_per_metronome: 24,
            something: 8,
        }
    }
}

impl BinRead for TimeSignature {
    type Args = (VarLengthValue,);

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _: &ReadOptions,
        args: Self::Args,
    ) -> BinResult<Self> {
        if (args.0).0 == 2 {
            Ok(TimeSignature {
                numerator: u8::read(reader)?,
                denominator: u8::read(reader)?,
                ..Default::default()
            })
        } else if (args.0).0 == 4 {
            Ok(TimeSignature {
                numerator: u8::read(reader)?,
                denominator: u8::read(reader)?,
                clocks_per_metronome: u8::read(reader)?,
                something: u8::read(reader)?,
            })
        } else {
            Err(Error::Custom {
                err: Box::new("Cannot read time signature with length not equal to 2 or 4"),
                pos: reader.seek(SeekFrom::Current(0))? as usize,
            })
        }
    }
}

impl From<MetaEventType> for MetaEvent {
    fn from(f: MetaEventType) -> MetaEvent {
        MetaEvent {
            meta_type: 0,
            length: VarLengthValue::from(0),
            variant: f,
        }
    }
}

impl From<MetaEvent> for MetaEventType {
    fn from(f: MetaEvent) -> MetaEventType {
        f.variant
    }
}
