use std::rc::Rc;

use musiclib::midi::{Track, event::EventType, event::MidiEventType};
use tui::widgets::{StatefulWidget};
use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::style::{Style, Color};

use crate::utils::{get_note_name, is_accidental};

const KEY_WIDTH: u16 = 5;

pub struct Piano;

pub struct PianoState {
    pub vscroll: u8,
    pub hscroll: u32,
    pub track: Rc<Track>,
    pub channel: u8,
    pub precision: u16,
    // active notes can be shown somehow...
}

impl StatefulWidget for Piano {
    type State = PianoState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let note_start = state.vscroll;
        let note_end = state.vscroll + area.height as u8;
        for (position, note_value) in (note_start..note_end).rev().enumerate() {
            let style = if is_accidental(note_value, 0) {
                Style::default().bg(Color::Black).fg(Color::White)
            } else {
                Style::default().bg(Color::White).fg(Color::Black)
            };
            let area = Rect::new(
                area.x, 
                area.y + position as u16,
                KEY_WIDTH,
                1
            );
            buf.set_style(area, style);
            buf.set_string(area.x, area.y, get_note_name(note_value, true), style);
        }
        let mut position: u32 = 0;
        state.track.events.iter().filter_map(|e| {
            position = position + e.delta.0;
            if let EventType::Midi(m) = &e.event_type {
                if m.channel == state.channel {
                    if let MidiEventType::NoteOn { note, velocity } = &m.event_type {
                        if *velocity > 0
                            && *note < note_end
                            && *note >= note_start
                            && position >= state.hscroll
                            && position < state.hscroll + ((area.width - KEY_WIDTH) as u32 * state.precision as u32) / 4
                        {
                            return Some((position, note, velocity))
                        }
                    }
                }   
            }
            return None
        }).for_each(|(time, note, _velocity)| {
            let x = (time - state.hscroll) * 4 / state.precision as u32;
            let y = note - state.vscroll;
            buf.set_string(
                area.x + KEY_WIDTH + x as u16,
                area.y + area.height - 1 - y as u16,
                "■",
                Style::default().fg(Color::Green)
            );
        });
    }
}
