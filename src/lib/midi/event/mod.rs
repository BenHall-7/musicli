mod meta_event;
mod midi_event;
mod sysex_event;

pub use meta_event::*;
pub use midi_event::*;
pub use sysex_event::*;

use super::VarLengthValue;
use crate::utils::{FromStream, FromStreamContext, ToStream};
use byteorder::{ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Error, ErrorKind, Read, Seek, SeekFrom, Write};

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
    Meta(MetaEvent),
    // SysEx(),
    Unsupported(u8, Vec<u8>),
}

impl FromStreamContext for Event {
    type Context = Option<u8>;

    fn from_stream_context<R: Read + Seek>(
        reader: &mut R,
        context: &mut Self::Context,
    ) -> Result<Self, Error> {
        let delta = VarLengthValue::from_stream(reader)?;
        let event_type = EventType::from_stream_context(reader, context)?;
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

impl FromStreamContext for EventType {
    type Context = Option<u8>;

    fn from_stream_context<R: Read + Seek>(
        reader: &mut R,
        event_num: &mut Option<u8>,
    ) -> Result<Self, Error> {
        let mut next_event = reader.read_u8()?;
        if next_event < 0x80 {
            next_event = event_num.unwrap();
            reader.seek(SeekFrom::Current(-1))?;
        }

        match next_event {
            0x00..=0x7f => unreachable!(),
            0x80..=0xef => {
                *event_num = Some(next_event);
                // TODO: This is ugly
                let midi_event = MidiEvent::from_stream_context(
                    reader,
                    event_num
                        .as_mut()
                        .ok_or(ErrorKind::InvalidData)
                        .map(|x| &mut *x)?,
                )?;
                Ok(Self::Midi(midi_event))
            }
            0xff => Ok(Self::Meta(MetaEvent::from_stream(reader)?)),
            _ => {
                reader.read_u8()?;
                let len = VarLengthValue::from_stream(reader)?;
                let mut bytes = vec![0u8; len.0 as usize];
                reader.read_exact(&mut bytes)?;
                Ok(Self::Unsupported(next_event, bytes))
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
            Self::Meta(_) => unimplemented!(),
            Self::Unsupported(num, vec) => {
                writer.write_u8(*num)?;
                VarLengthValue::from(vec.len() as u32).to_stream(writer)?;
                writer.write_all(&vec)?;
            }
        }
        Ok(())
    }
}
