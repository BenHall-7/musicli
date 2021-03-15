use super::{Component, Event};
use crossterm::event::{KeyCode, KeyModifiers};
use std::path::{Path, PathBuf};
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Widget};

// TODO, replace this implementation with a visual search, instead of typed

#[derive(Debug)]
pub struct FileSearch {
    path: String,
    error: Option<String>,
}

#[derive(Debug, Clone)]
pub enum FSResponse {
    None,
    Exit,
    Open(PathBuf),
}

impl FileSearch {
    pub fn new() -> Self {
        Self {
            path: String::new(),
            error: None,
        }
    }
}

impl Component for FileSearch {
    type Response = FSResponse;

    fn handle_event(&mut self, event: Event) -> Self::Response {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Char(c) => {
                    self.path.push(c);
                    FSResponse::None
                }
                KeyCode::Backspace => {
                    if !self.path.is_empty() {
                        self.path.pop();
                    }
                    FSResponse::None
                }
                KeyCode::Esc => FSResponse::Exit,
                KeyCode::Enter => {
                    let path = Path::new(&self.path);
                    if path.is_file() {
                        FSResponse::Open(path.to_path_buf())
                    } else {
                        self.error = Some(format!(
                            "The path '{}' either does not exist or is not a file",
                            path.to_string_lossy()
                        ));
                        FSResponse::None
                    }
                }
                _ => FSResponse::None,
            }
        } else {
            FSResponse::None
        }
    }

    fn draw(&mut self, rect: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("File Search")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue).add_modifier(Modifier::DIM));
        let inner = block.inner(rect);
        block.render(rect, buf);
        let text = format!(
            "Input the path of the file...\n{}\n> {}",
            self.error.as_ref().unwrap_or(&String::new()),
            self.path
        );
        let p = Paragraph::new(&text[..]).style(Style::default().add_modifier(Modifier::BOLD));
        p.render(inner, buf);
    }
}
