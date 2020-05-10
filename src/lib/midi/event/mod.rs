mod meta_event;
mod midi_event;
mod sysex_event;

pub use meta_event::*;
pub use midi_event::*;
pub use sysex_event::*;

use super::VarLengthValue;
use crate::error::Error;
use binread::{BinRead, BinReaderExt, BinResult, ReadOptions};
use binread::io::{Read, Seek, SeekFrom};
use std::pin::Pin;
use std::cell::RefMut;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub(crate) struct EventWithRet(pub Event, pub Option<u8>);

/// A top-level event struct in a Track, containing a delta time and one of 3 event types
#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    /// The number of Timing units since the last event
    delta: VarLengthValue,
    event_type: EventType,
}

#[derive(Debug)]
pub(crate) struct EventTypeWithRet(pub EventType, pub Option<u8>);

#[derive(Debug, Deserialize, Serialize)]
pub enum EventType {
    Midi(MidiEvent),
    Meta(MetaEvent),
    // SysEx(),
    Unsupported(u8, Vec<u8>),
}

impl BinRead for EventWithRet {
    type Args = Option<u8>;

    fn read_options<R: Read + Seek>(reader: &mut R, ro: &ReadOptions, running_status: Self::Args) -> BinResult<Self> {
        let delta = VarLengthValue::read(reader)?;
        let EventTypeWithRet(event_type, ret) = EventTypeWithRet::read_options(reader, ro, running_status)?;
        Ok(EventWithRet(Event {delta, event_type}, ret))
    }
}

impl BinRead for EventTypeWithRet {
    type Args = Option<u8>;

    fn read_options<R: Read + Seek>(reader: &mut R, ro: &ReadOptions, running_status: Self::Args) -> BinResult<Self> {
        let mut next_event = u8::read(reader)?;
        if next_event < 0x80 {
            next_event = running_status.expect("Event byte not found, and no running status is set");
            reader.seek(SeekFrom::Current(-1))?;
        }

        match next_event {
            0x00..=0x7f => unreachable!(),
            0x80..=0xef => {
                let midi_event = MidiEvent::from_stream_context(reader, event_num)?;
                Ok(EventTypeWithRet(EventType::Midi(midi_event), Some(next_event)))
            }
            0xff => {
                let meta_event = MetaEvent::from_stream_context(reader)?;
                Ok(EventTypeWithRet(EventType::Meta(meta_event), Some(next_event)))
            }
            _ => {
                u8::read(reader)?; // What was this about again?
                let len = VarLengthValue::read(reader)?;
                let mut bytes = vec![0u8; len.0 as usize];
                reader.read_exact(&mut bytes)?;
                Ok(EventTypeWithRet(EventType::Unsupported(next_event, bytes), running_status))
            }
        }
    }
}
