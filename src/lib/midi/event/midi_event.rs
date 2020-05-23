use binread::io::{Read, Seek};
use binread::{BinRead, BinResult, ReadOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MidiEvent {
    channel: u8,
    event_type: MidiEventType,
}

#[derive(Debug, Deserialize, Serialize, BinRead)]
#[br(import(event_num: u8))]
pub enum MidiEventType {
    #[br(assert(event_num == 0x8))]
    NoteOff { note: u8, velocity: u8 },
    #[br(assert(event_num == 0x9))]
    NoteOn { note: u8, velocity: u8 },
    #[br(assert(event_num == 0xa))]
    NotePressure { note: u8, pressure: u8 },
    #[br(assert(event_num == 0xb))]
    Controller { controller: u8, value: u8 },
    #[br(assert(event_num == 0xc))]
    Program { program: u8 },
    #[br(assert(event_num == 0xd))]
    Pressure { pressure: u8 },
    #[br(assert(event_num == 0xe))]
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