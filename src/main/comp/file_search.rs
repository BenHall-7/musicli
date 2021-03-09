use super::Component;
use std::path::PathBuf;
use tui::buffer::Buffer;
use tui::layout::Rect;

// TODO, implement this fully

#[derive(Debug)]
pub struct FileSearch {
    path: PathBuf,
    selected: PathBuf,
    submitted: bool,
}

impl FileSearch {
    pub fn new() -> Self {
        Self {
            path: PathBuf::new(),
            selected: PathBuf::new(),
            submitted: false,
        }
    }
}

impl Component for FileSearch {
    fn draw(&mut self, rect: Rect, buf: &mut Buffer) {
        // ...
    }
}
