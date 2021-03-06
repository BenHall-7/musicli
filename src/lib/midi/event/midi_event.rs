use crate::midi::constants::Program;
use binread::io::{Read, Seek};
use binread::{BinRead, BinResult, ReadOptions};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MidiEvent {
    pub channel: u8,
    pub event_type: MidiEventType,
}

#[derive(Debug, Clone, Deserialize, Serialize, BinRead)]
#[br(import(event_num: u8))]
pub enum MidiEventType {
    #[br(pre_assert(event_num == 0x8))]
    NoteOff { note: u8, velocity: u8 },

    #[br(pre_assert(event_num == 0x9))]
    NoteOn { note: u8, velocity: u8 },

    #[br(pre_assert(event_num == 0xa))]
    NotePressure { note: u8, pressure: u8 },

    #[br(pre_assert(event_num == 0xb))]
    Controller { controller: u8, value: u8 },

    #[br(pre_assert(event_num == 0xc))]
    Program {
        #[br(map = |v: u8| Program::try_from(v).unwrap())]
        program: Program,
    },

    #[br(pre_assert(event_num == 0xd))]
    Pressure { pressure: u8 },

    #[br(pre_assert(event_num == 0xe))]
    PitchBend { lsb: u8, msb: u8 },
}

impl BinRead for MidiEvent {
    type Args = u8;

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        ro: &ReadOptions,
        status: Self::Args,
    ) -> BinResult<Self> {
        let event_num = status >> 4;
        let channel = status & 0xf;
        Ok(Self {
            channel,
            event_type: MidiEventType::read_options(reader, ro, (event_num,))?,
        })
    }
}
