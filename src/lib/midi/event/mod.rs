mod meta_event;
mod midi_event;
mod sysex_event;

pub use meta_event::*;
pub use midi_event::*;
pub use sysex_event::*;

use super::VarLengthValue;
use crate::utils::{FromStream, ToStream};
use byteorder::{ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Error, Read, Seek, SeekFrom, Write};

/// A top-level event struct in a Track, containing a delta time and one of 3 event types
#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    /// The number of Timing units since the last event
    delta: VarLengthValue,
    event_type: EventType,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum EventType {
    Midi(MidiEvent),
    // Meta(),
    // SysEx(),
    Unsupported(u8, Vec<u8>),
}

impl FromStream for Event {
    fn from_stream<R: Read + Seek>(reader: &mut R) -> Result<Self, Error> {
        let delta = VarLengthValue::from_stream(reader)?;
        let event_type = EventType::from_stream(reader)?;
        Ok(Event { delta, event_type })
    }
}

impl ToStream for Event {
    fn to_stream<W: Write + Seek>(&self, writer: &mut W) -> Result<(), Error> {
        self.delta.to_stream(writer)?;
        self.event_type.to_stream(writer)?;
        Ok(())
    }
}

impl FromStream for EventType {
    fn from_stream<R: Read + Seek>(reader: &mut R) -> Result<Self, Error> {
        let event_num = reader.read_u8()?;
        reader.seek(SeekFrom::Current(-1))?;
        match event_num {
            0x80..=0xef => {
                let midi_event = MidiEvent::from_stream(reader)?;
                Ok(Self::Midi(midi_event))
            }
            _ => {
                reader.read_u8()?;
                let len = VarLengthValue::from_stream(reader)?;
                let mut bytes = Vec::with_capacity(len.0 as usize);
                // TODO: does this actually work up to the capacity?
                reader.read_exact(&mut bytes)?;
                Ok(Self::Unsupported(event_num, bytes))
            }
        }
    }
}

impl ToStream for EventType {
    fn to_stream<W: Write + Seek>(&self, writer: &mut W) -> Result<(), Error> {
        match self {
            Self::Midi(midi) => {
                midi.to_stream(writer)?;
            }
            Self::Unsupported(num, vec) => {
                writer.write_u8(*num)?;
                VarLengthValue::from(vec.len() as u32).to_stream(writer)?;
                writer.write_all(&vec)?;
            }
        }
        Ok(())
    }
}
