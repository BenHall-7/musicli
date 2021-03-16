use super::{Component, Event, FSResponse, FileSearch, TVResponse, TrackView};
use binread::BinRead;
use crossterm::event::{KeyCode, KeyEvent};
use musiclib::midi::{File, Format};
use std::fs::read;
use std::io::Cursor;
use std::path::PathBuf;
use tui::buffer::Buffer;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Widget};

pub enum App {
    NotLoaded,
    Loading(FileSearch),
    Loaded(LoadedState),
}

pub enum AppResponse {
    None,
    Exit,
}

pub struct LoadedState {
    play_state: PlayState,
    file_path: PathBuf,
    track_view: TrackView,
}

pub enum PlayState {
    Idle,
    Playing { position: u32 },
}

impl App {
    pub fn new() -> App {
        App::NotLoaded
    }
}

impl Component for App {
    type Response = AppResponse;

    fn handle_event(&mut self, event: Event) -> Self::Response {
        match self {
            App::NotLoaded => {
                if let Event::Key(key_event) = event {
                    match key_event.code {
                        KeyCode::Esc => return AppResponse::Exit,
                        KeyCode::Char(c) => {
                            if c == 'o' {
                                *self = App::Loading(FileSearch::new())
                            }
                        }
                        _ => {}
                    }
                }
            }
            App::Loading(fs) => {
                match fs.handle_event(event) {
                    FSResponse::Exit => *self = App::NotLoaded,
                    FSResponse::Open(path) => {
                        let mut cursor = Cursor::new(read(path.clone()).unwrap());
                        // TODO: handle errors
                        let midi = File::read(&mut cursor).unwrap();
                        *self = App::Loaded(LoadedState {
                            play_state: PlayState::Idle,
                            file_path: path,
                            track_view: TrackView::new(midi.format),
                        })
                    }
                    FSResponse::None => {}
                }
            }
            App::Loaded(loaded) => match loaded.track_view.handle_event(event) {
                TVResponse::Exit => *self = App::NotLoaded,
                TVResponse::None => {}
            },
        }
        AppResponse::None
    }

    fn draw(&mut self, rect: Rect, buf: &mut Buffer) {
        match self {
            App::NotLoaded => {
                let block = Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Blue).add_modifier(Modifier::DIM));
                let inner = block.inner(rect);
                block.render(rect, buf);
                let text = Paragraph::new(
                    "Welcome to musicli! Open a file with the 'o' key to get started",
                )
                .style(
                    Style::default()
                        .bg(Color::Blue)
                        .add_modifier(Modifier::BOLD),
                )
                .alignment(Alignment::Center);
                let mut text_area = inner;
                text_area.width /= 2;
                text_area.height /= 2;
                text_area.x += (inner.width as f32 / 4.0) as u16;
                text_area.y += (inner.height as f32 / 4.0) as u16;
                text.render(text_area, buf);
            }
            App::Loading(search) => search.draw(rect, buf),
            App::Loaded(loaded) => loaded.track_view.draw(rect, buf),
        }
    }
}
