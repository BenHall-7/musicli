use std::io::{stdout, Write, Cursor};
use std::time::Duration;
use tui::{backend::CrosstermBackend, buffer::Buffer, layout::Rect, Terminal};
use crossterm::terminal::SetTitle;
use crossterm::event::{Event, KeyCode, poll, read};
use crossterm::{execute};
use binread::BinRead;
use musiclib::midi::File;

mod widgets;
use widgets::{PianoRoll, PianoRollState};

const BEETHOVEN: &[u8] = include_bytes!("appass_3.mid");

fn main() {
    execute!(stdout(), SetTitle("muslici - cli midi editor")).unwrap_or_else(|_| {
        eprintln!("Unable to change title");
        ()
    });
    let mut t = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
    t.clear().unwrap();
    
    let mut cursor = Cursor::new(BEETHOVEN);
    let midi = File::read(&mut cursor).unwrap();
    let tracks = if let musiclib::midi::Format::MultipleTrack(tracks) = midi.format {
        tracks
    } else {
        panic!("Unexpected format")
    };

    let mut state = widgets::PianoRollState {
        track: std::rc::Rc::new(tracks[1].clone()),
        note_number: 0,
        horizontal_scroll: 0,
        vertical_scroll: 0,
    };

    loop {
        t.draw(|f| {
            let area = f.size();
            f.render_stateful_widget(
                PianoRoll,
                area,
                &mut state
            )
        })
        .unwrap();

        if poll(Duration::from_millis(0)).unwrap() {
            match read().unwrap() {
                Event::Key(k) => {
                    match k.code {
                        KeyCode::Esc => break,
                        KeyCode::Right => {
                            state.note_number += 1;
                        }
                        KeyCode::Left => {
                            state.note_number -= 1;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}
