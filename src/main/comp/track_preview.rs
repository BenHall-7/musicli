use super::{Component, Event, Piano, TrackView};
use binread::BinRead;
use crossterm::event::{KeyCode, KeyEvent};
use musiclib::midi::event::{EventType, MetaEventType};
use musiclib::midi::{Format, Track};
use std::fs::read;
use std::io::Cursor;
use std::path::PathBuf;
use tui::buffer::Buffer;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Widget};

pub struct TrackPreview {
    pub track: Track,
    pub name: String,
    pub selected: bool,
}

impl TrackPreview {
    pub fn new_search_name(track: Track, substitute_name: String) -> Self {
        let name = Self::search_track_name(&track).unwrap_or(substitute_name);
        TrackPreview {
            track,
            name,
            selected: false,
        }
    }

    pub fn new(track: Track, name: String) -> Self {
        TrackPreview {
            track,
            name,
            selected: false,
        }
    }

    pub fn search_track_name(track: &Track) -> Option<String> {
        for ev in &track.events {
            if let EventType::Meta(m) = &ev.event_type {
                if let MetaEventType::TrackName(name) = &m.variant {
                    return Some(name.clone());
                }
            }
        }
        None
    }
}

pub enum TPResponse {
    None,
    Focus,
}

impl Component for TrackPreview {
    type Response = TPResponse;

    fn handle_event(&mut self, event: Event) -> Self::Response {
        if let Event::Key(key_event) = event {
            if let KeyCode::Enter = key_event.code {
                return TPResponse::Focus;
            }
        }
        TPResponse::None
    }

    fn draw(&mut self, rect: Rect, buf: &mut Buffer) {
        let borders = Block::default()
            .borders(Borders::ALL)
            .border_style(if self.selected {
                Style::default().fg(Color::Green)
            } else {
                Style::default().fg(Color::Blue)
            })
            .title(self.name.clone());
        borders.render(rect, buf);
    }
}
