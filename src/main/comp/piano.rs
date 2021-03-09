use std::rc::Rc;

use super::Component;
use musiclib::midi::event::{Event, EventType, MidiEventType};
use musiclib::midi::Track;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};

use crate::utils::{get_note_name, is_accidental};

pub const KEY_WIDTH: u16 = 6;
pub const CURRENT_EVENT_WIDTH: u16 = 12;
pub const NEXT_EVENT_WIDTH: u16 = 4;

pub struct Piano<'a> {
    pub vscroll: u8,
    pub hscroll: usize,
    pub track: &'a Track,
    pub channel: u8,
    // active notes can be shown somehow...
}

impl<'a> Piano<'a> {
    pub fn new(track: &'a Track) -> Self {
        Self {
            vscroll: 0,
            hscroll: 0,
            track,
            channel: 0,
        }
    }
}

impl<'a> Component for Piano<'a> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        // draw the vertical keyboard
        let note_start = self.vscroll;
        let note_end = self.vscroll + area.height as u8;
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
        let mut note_area = area;
        note_area.x += KEY_WIDTH;
        note_area.width = CURRENT_EVENT_WIDTH;
        let mut event_marker = self.hscroll;
        let mut events = self.get_events_at(event_marker);
        let mut background = false;

        macro_rules! advance {
            () => {
                note_area.x += note_area.width;
                note_area.width = NEXT_EVENT_WIDTH;
                event_marker += events.len();
                events = self.get_events_at(event_marker);
                background = !background;
            };
        }

        self.draw_events(note_area, buf, events, true, background);
        advance!();
        while note_area.x <= area.right() - NEXT_EVENT_WIDTH {
            if events.len() == 0 {
                break;
            }
            self.draw_events(note_area, buf, events, false, background);
            advance!();
        }
    }
}

impl<'a> Piano<'a> {
    fn draw_events(
        &self,
        area: Rect,
        buf: &mut Buffer,
        events: &[Event],
        detailed: bool,
        background: bool,
    ) {
        if background {
            buf.set_style(area, Style::default().bg(Color::Rgb(20, 20, 20)))
        }

        let note_start = self.vscroll;
        let note_end = self.vscroll + area.height as u8;

        for ev in events {
            match &ev.event_type {
                // render MIDI events in the keyboard
                EventType::Midi(m) => {
                    if m.channel == self.channel {
                        match m.event_type {
                            MidiEventType::NoteOn { note, velocity } => {
                                if velocity > 0 {
                                    if note_end < 0xff && note >= note_end - 1 {
                                        // if there's a note above the range
                                        buf.set_string(
                                            area.x,
                                            area.y,
                                            r" /\ ",
                                            Style::default().bg(Color::Green),
                                        );
                                    } else if note_start > 0 && note < note_start + 1 {
                                        // if there's a note below the range
                                        buf.set_string(
                                            area.x,
                                            area.y + area.height - 1,
                                            r" \/ ",
                                            Style::default().bg(Color::Green),
                                        );
                                    } else {
                                        buf.set_string(
                                            area.x,
                                            area.y + area.height - 1 - (note - self.vscroll) as u16,
                                            format!("{: >4}", velocity),
                                            Style::default().fg(Color::Black).bg(Color::Green),
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

    pub fn vscroll_by(&mut self, by: i16) {
        let to = self.vscroll as i16 + by;
        if to < 0 {
            self.vscroll = 0;
        } else if to > 127 {
            self.vscroll = 127;
        } else {
            self.vscroll = to as u8;
        }
    }
}
