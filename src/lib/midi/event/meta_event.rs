use crate::midi::VarLengthValue;
use binread::BinRead;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, BinRead)]
pub struct MetaEvent {
    meta_type: u8,
    length: VarLengthValue,
    #[br(args(meta_type, length))]
    pub variant: MetaEventType,
}

#[derive(Debug, Deserialize, Serialize, BinRead)]
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
    TimeSignature {
        numerator: u8,
        denominator: u8,
        clocks_per_metronome: u8,
        something: u8,
    },
    #[br(assert(ty == 0x59))]
    KeySignature {
        sf: i8,
        mi: u8,
    },
    Unsupported(#[br(count = len.0)] Vec<u8>),
}
