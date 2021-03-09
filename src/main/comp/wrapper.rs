use super::{App, Component};
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
