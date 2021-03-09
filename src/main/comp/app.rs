use super::{Component, FileSearch, Piano};
use binread::BinRead;
use musiclib::midi::{File, Format};
use std::io::Cursor;
use std::path::PathBuf;
use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Widget};

const BEETHOVEN: &[u8] = include_bytes!("appass_3.mid");

pub enum App {
    NotLoaded,
    Loading(FileSearch),
    Loaded(LoadedState),
}

pub struct LoadedState {
    play_state: PlayState,
    file_path: PathBuf,
    midi: File,
}

pub enum PlayState {
    Idle,
    Playing { position: u32 },
}

impl App {
    pub fn new() -> App {
        // temporary workaround until file loading is implemented
        let mut cursor = Cursor::new(BEETHOVEN);
        let midi = File::read(&mut cursor).unwrap();

        App::Loaded(LoadedState {
            play_state: PlayState::Idle,
            file_path: PathBuf::new(),
            midi,
        })
    }
}

impl Component for App {
    fn draw(&mut self, rect: Rect, buf: &mut Buffer) {
        match self {
            App::NotLoaded => {
                let block = Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Blue).add_modifier(Modifier::DIM));
                let inner = block.inner(rect);
            }
            App::Loading(search) => search.draw(rect, buf),
            App::Loaded(loaded) => {
                // TODO: should I put the Piano in here to avoid my lifetime annotation,
                // or in the actual struct?
                if let Format::MultipleTrack(tracks) = &loaded.midi.format {
                    let mut piano = Piano::new(&tracks[0]);
                    piano.draw(rect, buf);
                } else {
                    panic!("Built in midi file is in the wrong format. Replace this problem later")
                }
            }
        }
    }
}
