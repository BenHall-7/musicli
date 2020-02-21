mod midi_event;
mod meta_event;
mod sysex_event;

pub use midi_event::*;
pub use meta_event::*;
pub use sysex_event::*;

use super::VarLengthValue;

/// A top-level event in a Track, containing a delta time and one of 3 event types
#[derive(Debug)]
pub struct Event {
    /// The number of Timing units since the last event
    delta: VarLengthValue,
    event_type: EventType,
}

#[derive(Debug)]
pub enum EventType {
    Midi(MidiEvent),
    Meta(),
    SysEx(),
}