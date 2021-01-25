use std::rc::Rc;

use musiclib::midi::event::{Event, EventType, MidiEventType};
use musiclib::midi::Track;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::widgets::StatefulWidget;

use crate::utils::{get_note_name, is_accidental};

pub const KEY_WIDTH: u16 = 5;
// pub const CURRENT_EVENT_WIDTH: u16 = 15;

pub struct Piano;

pub struct PianoState {
    pub vscroll: u8,
    pub hscroll: usize,
    pub track: Rc<Track>,
    pub channel: u8,
    pub precision: u16,
    // active notes can be shown somehow...
}

impl PianoState {
    pub fn new(track: &Rc<Track>, precision: u16) -> Self {
        PianoState {
            vscroll: 0,
            hscroll: 0,
            track: track.clone(),
            channel: 0,
            precision,
        }
    }
}

impl StatefulWidget for Piano {
    type State = PianoState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // draw the vertical keyboard
        let note_start = state.vscroll;
        let note_end = state.vscroll + area.height as u8;
        for (position, note_value) in (note_start..note_end).rev().enumerate() {
            let style = if is_accidental(note_value, 0) {
                Style::default().bg(Color::Black).fg(Color::White)
            } else {
                Style::default().bg(Color::White).fg(Color::Black)
            };
            let area = Rect::new(area.x, area.y + position as u16, KEY_WIDTH, 1);
            buf.set_style(area, style);
            buf.set_string(area.x, area.y, get_note_name(note_value, true), style);
        }

        // draw the notes on the horizontal view
        // TODO: repeat process for future events until we can't fit them on screen

        for ev in state.get_current_events() {
            match &ev.event_type {
                // render MIDI events in the keyboard
                EventType::Midi(m) => {
                    if m.channel == state.channel {
                        match m.event_type {
                            MidiEventType::NoteOn { note, velocity } => {
                                if velocity > 0 {
                                    if note_end < 0xff && note >= note_end - 1 {
                                        // if there's a note above the range
                                        buf.set_string(
                                            area.x + KEY_WIDTH,
                                            area.y,
                                            "^^^",
                                            Style::default().bg(Color::Green),
                                        );
                                    } else if note_start > 0 && note < note_start + 1 {
                                        // if there's a note below the range
                                        buf.set_string(
                                            area.x + KEY_WIDTH,
                                            area.y + area.height - 1,
                                            "vvv",
                                            Style::default().bg(Color::Green),
                                        );
                                    } else {
                                        buf.set_string(
                                            area.x + KEY_WIDTH,
                                            area.y + area.height
                                                - 1
                                                - (note - state.vscroll) as u16,
                                            "   ",
                                            Style::default().bg(Color::Green),
                                        );
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

impl PianoState {
    pub fn get_current_events(&self) -> &[Event] {
        self.get_events_at(self.hscroll)
    }

    // TODO: match for midi events and the correct channel
    pub fn get_events_at(&self, at: usize) -> &[Event] {
        let to_end = self.track.events.len() - at;
        let current_size = self.track.events[at + 1..] // this slice to start after the current position
            .iter()
            .position(|v| v.delta.0 > 0)
            .map(|v| v + 1)
            .unwrap_or(to_end);
        &self.track.events[at..at + current_size]
    }

    pub fn next_group(&self) -> usize {
        self.track.events[self.hscroll + 1..] // this slice to start after the current position
            .iter()
            .position(|v| v.delta.0 > 0)
            .map(|v| v + self.hscroll + 1)
            .unwrap_or(self.hscroll) // if no next group, position doesn't change
    }

    pub fn prev_group(&self) -> usize {
        self.track.events[0..self.hscroll] // this slice to end before current position
            .iter()
            .rposition(|v| v.delta.0 > 0)
            .unwrap_or_default()
    }
}
