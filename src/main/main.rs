use std::{io::{stdout, Write, Cursor}, rc::Rc};
use std::time::Duration;
use tui::{backend::CrosstermBackend, Terminal};
use crossterm::{event::KeyModifiers, terminal::SetTitle};
use crossterm::event::{Event, KeyCode, poll, read};
use crossterm::{execute};
use binread::BinRead;
use musiclib::midi::{File, Timing};

mod widgets;
mod utils;

use widgets::{Piano, PianoState};

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
            f.render_stateful_widget(
                Piano,
                area,
                &mut piano_keys_state
            )
        })
        .unwrap();

        if poll(Duration::from_millis(0)).unwrap() {
            match read().unwrap() {
                Event::Key(k) => {
                    match k.code {
                        KeyCode::Esc => break,
                        KeyCode::Up => {
                            let change = if k.modifiers.contains(KeyModifiers::ALT) {
                                12
                            } else {
                                1
                            };
                            piano_keys_state.vscroll += change;
                        }
                        KeyCode::Down => {
                            let change = if k.modifiers.contains(KeyModifiers::ALT) {
                                12
                            } else {
                                1
                            };
                            if change > piano_keys_state.vscroll {
                                piano_keys_state.vscroll = 0;
                            } else {
                                piano_keys_state.vscroll -= change;
                            }
                        }
                        KeyCode::Right => {
                            piano_keys_state.hscroll = piano_keys_state.next_group();
                        }
                        KeyCode::Left => {
                            piano_keys_state.hscroll = piano_keys_state.prev_group();
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    t.clear().unwrap()
}
