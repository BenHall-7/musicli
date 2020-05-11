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

// impl ToStream for MidiEvent {
//     fn to_stream<W: Write + Seek>(&self, writer: &mut W) -> Result<(), Error> {
//         match self {
//             Self::NoteOff {
//                 channel,
//                 note,
//                 velocity,
//             } => {
//                 writer.write_u8(0x80 | (channel & 0xf))?;
//                 writer.write_u8(*note)?;
//                 writer.write_u8(*velocity)?;
//             }
//             Self::NoteOn {
//                 channel,
//                 note,
//                 velocity,
//             } => {
//                 writer.write_u8(0x90 | (channel & 0xf))?;
//                 writer.write_u8(*note)?;
//                 writer.write_u8(*velocity)?;
//             }
//             Self::NotePressure {
//                 channel,
//                 note,
//                 pressure,
//             } => {
//                 writer.write_u8(0xa0 | (channel & 0xf))?;
//                 writer.write_u8(*note)?;
//                 writer.write_u8(*pressure)?;
//             }
//             Self::Controller {
//                 channel,
//                 controller,
//                 value,
//             } => {
//                 writer.write_u8(0xb0 | (channel & 0xf))?;
//                 writer.write_u8(*controller)?;
//                 writer.write_u8(*value)?;
//             }
//             Self::Program { channel, program } => {
//                 writer.write_u8(0xc0 | (channel & 0xf))?;
//                 writer.write_u8(*program)?;
//             }
//             Self::Pressure { channel, pressure } => {
//                 writer.write_u8(0xd0 | (channel & 0xf))?;
//                 writer.write_u8(*pressure)?;
//             }
//             Self::PitchBend { channel, lsb, msb } => {
//                 writer.write_u8(0xe0 | (channel & 0xf))?;
//                 writer.write_u8(*lsb)?;
//                 writer.write_u8(*msb)?;
//             }
//         }
//         Ok(())
//     }
// }
