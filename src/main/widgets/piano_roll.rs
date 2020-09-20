use musiclib::midi::Track;
// use musiclib::midi::VarLengthValue; // use this for horizontal scroll
use crate::utils::get_note_name;
use musiclib::midi::event::{EventType, MidiEventType};
use tui::{widgets::StatefulWidget, layout::Rect, buffer::Buffer, style::Style};
use std::rc::Rc;

#[derive(Debug, Default, Copy, Clone)]
pub struct PianoRoll;

pub struct PianoRollState {
    pub track: Rc<Track>,
    pub note_number: usize,
    pub vertical_scroll: u32,
    pub horizontal_scroll: u32,
}

impl StatefulWidget for PianoRoll {
    type State = PianoRollState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let note = state.track.events.iter().filter_map(|e| {
            if let EventType::Midi(m) = &e.event_type {
                if let MidiEventType::NoteOn { note, velocity } = m.event_type {
                    return Some((note, velocity))
                }
            }
            None
        }).nth(state.note_number);
        if let Some(note) = note {
            let write = format!("note: {}, velocity: {}", get_note_name(note.0, true), note.1);
            buf.set_stringn(
                area.x,
                area.y,
                write,
                area.width as usize,
                Style::default()
            );
        }
    }
}
