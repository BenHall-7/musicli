use std::time::Duration;
use std::{
    io::{stdout, Cursor, Write},
    rc::Rc,
};

use binread::BinRead;
use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{SetTitle, EnterAlternateScreen, LeaveAlternateScreen, enable_raw_mode, disable_raw_mode};
use musiclib::midi::{File, Timing};
use tui::{backend::CrosstermBackend, Terminal};

mod error;
mod utils;
mod widgets;

use widgets::{Piano, PianoState};

const BEETHOVEN: &[u8] = include_bytes!("appass_3.mid");

fn main() -> Result<(), error::AppError> {
    execute!(
        stdout(),
        SetTitle("muslici - cli midi editor"),
        EnterAlternateScreen
    )?;
    enable_raw_mode()?;
    let mut t = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
    t.clear().unwrap();

    let mut cursor = Cursor::new(BEETHOVEN);
    let midi = File::read(&mut cursor).unwrap();
    let precision = if let Timing::Metrical { precision } = midi.timing {
        precision
    } else {
        panic!("Unexpected timing format for this example")
    };
    let tracks = if let musiclib::midi::Format::MultipleTrack(tracks) = midi.format {
        tracks
    } else {
        panic!("Unexpected format")
    };

    let mut piano_keys_state = PianoState::new(&Rc::new(tracks[1].clone()), precision);

    loop {
        t.draw(|f| {
            let area = f.size();
            f.render_stateful_widget(Piano, area, &mut piano_keys_state)
        })
        .unwrap();

        if poll(Duration::from_millis(0)).unwrap() {
            match read().unwrap() {
                Event::Key(k) => match k.code {
                    KeyCode::Esc => break,
                    KeyCode::Up => piano_keys_state.vscroll_by(1),
                    KeyCode::Down => piano_keys_state.vscroll_by(-1),
                    KeyCode::PageUp => piano_keys_state.vscroll_by(12),
                    KeyCode::PageDown => piano_keys_state.vscroll_by(-12),
                    KeyCode::Right => {
                        piano_keys_state.hscroll = piano_keys_state.next_group();
                    }
                    KeyCode::Left => {
                        piano_keys_state.hscroll = piano_keys_state.prev_group();
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;

    t.clear().unwrap();

    Ok(())
}
