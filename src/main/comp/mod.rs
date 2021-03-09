use tui::buffer::Buffer;
use tui::layout::Rect;

mod app;
mod file_search;
mod piano;
mod track_preview;
mod track_view;
mod wrapper;
pub use app::*;
pub use file_search::*;
pub use piano::*;
pub use track_preview::*;
pub use track_view::*;
pub use wrapper::*;

/// A component to enabling a nested layout of structs
pub trait Component {
    fn draw(&mut self, rect: Rect, buffer: &mut Buffer);
}
