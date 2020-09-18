use musiclib::midi::Track;
use musiclib::midi::VarLengthValue;
use musiclib::midi::event::{Event, EventType, MidiEvent, MidiEventType};
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

fn get_note_name(note_value: u8) -> String {
    let octave = note_value / 12;
    match note_value % 12 {
        0 => format!("C {}", octave),
        1 => format!("C# {}", octave),
        2 => format!("D {}", octave),
        3 => format!("D# {}", octave),
        4 => format!("E {}", octave),
        5 => format!("F {}", octave),
        6 => format!("F# {}", octave),
        7 => format!("G {}", octave),
        8 => format!("G# {}", octave),
        9 => format!("A {}", octave),
        10 => format!("A# {}", octave),
        11 => format!("B {}", octave),
        _ => unreachable!(),
    }
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
            let write = format!("note: {}, velocity: {}", get_note_name(note.0), note.1);
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
