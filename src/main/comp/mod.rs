mod app;
mod file_search;
mod piano;
mod track_preview;
mod track_view;

pub use app::*;
pub use file_search::*;
pub use piano::*;
pub use track_preview::*;
pub use track_view::*;

use crossterm::event::{KeyEvent, MouseEvent};
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::StatefulWidget;

pub struct Wrapper;

impl StatefulWidget for Wrapper {
    type State = App;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        state.draw(area, buf);
    }
}

/// A trait enabling a nested layout of structs
pub trait Component {
    type Response;

    fn handle_event(&mut self, event: Event) -> Self::Response;

    fn draw(&mut self, rect: Rect, buffer: &mut Buffer);
}

pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
}
