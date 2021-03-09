pub use super::TrackPreview;
pub use tui::widgets::StatefulWidget;

pub struct TrackView;

pub struct TrackViewState {
    widget_height: u16,
    scroll: usize,
}
