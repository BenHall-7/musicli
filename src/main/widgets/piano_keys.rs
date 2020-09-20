use tui::widgets::{StatefulWidget};
use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::style::{Style, Color};

use crate::utils::{get_note_name, is_accidental};

pub struct PianoKeys;

pub struct PianoKeysState {
    pub scroll: u8,
    // active notes can be shown somehow...
}

impl StatefulWidget for PianoKeys {
    type State = PianoKeysState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let keys = state.scroll..state.scroll + area.height as u8;
        for (position, note_value) in keys.rev().enumerate() {
            let style = if is_accidental(note_value, 0) {
                Style::default().bg(Color::Black).fg(Color::White)
            } else {
                Style::default().bg(Color::White).fg(Color::Black)
            };
            let area = Rect::new(
                area.x, 
                area.y + position as u16,
                5,
                1
            );
            buf.set_style(area, style);
            buf.set_string(area.x, area.y, get_note_name(note_value, true), style);
        }
    }
}
