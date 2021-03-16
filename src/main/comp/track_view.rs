use super::{Component, Event, Piano, PianoResponse, TPResponse, TrackPreview};
use binread::BinRead;
use crossterm::event::{KeyCode, KeyEvent};
use musiclib::midi::{Format, Track};
use std::cmp::{max, min};
use std::fs::read;
use std::io::Cursor;
use std::path::PathBuf;
use tui::buffer::Buffer;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Widget};

const TRACK_PREVIEW_HEIGHT: u16 = 8;

pub struct TrackView {
    format: TrackFormat,
    mode: TrackViewMode,
}

pub enum TrackFormat {
    Single,
    Multiple,
    Multiple2,
}

pub enum TrackViewMode {
    Preview {
        index: usize,
        vscroll: u16,
        tracks: Vec<TrackPreview>,
    },
    View {
        // this owns the current track
        piano: Option<Piano>,
        // the index the track was moved from
        index: usize,
        // the tracks not being currently edited
        other_tracks: Vec<Track>,
    },
}

impl TrackView {
    pub fn new(midi_format: Format) -> Self {
        match midi_format {
            Format::SingleTrack(t) => {
                TrackView {
                    format: TrackFormat::Single,
                    // for this we go directly to the track because there's no point selecting
                    // from a list with only 1 track in it
                    mode: TrackViewMode::View {
                        piano: Some(Piano::new(t)),
                        index: 0,
                        other_tracks: vec![],
                    },
                }
            }
            Format::MultipleTrack(mut tracks) => TrackView {
                format: TrackFormat::Multiple,
                mode: TrackViewMode::Preview {
                    index: 0,
                    vscroll: 0,
                    tracks: tracks
                        .drain(..)
                        .enumerate()
                        .map(|(i, t)| TrackPreview::new_search_name(t, format!("Track {}", i)))
                        .collect::<Vec<_>>(),
                },
            },
            Format::ParallelTrack(mut tracks) => TrackView {
                format: TrackFormat::Multiple2,
                mode: TrackViewMode::Preview {
                    index: 0,
                    vscroll: 0,
                    tracks: tracks
                        .drain(..)
                        .enumerate()
                        .map(|(i, t)| TrackPreview::new_search_name(t, format!("Track {}", i)))
                        .collect::<Vec<_>>(),
                },
            },
        }
    }
}

pub enum TVResponse {
    None,
    Exit,
}

impl Component for TrackView {
    type Response = TVResponse;

    fn handle_event(&mut self, event: Event) -> Self::Response {
        match &mut self.mode {
            TrackViewMode::Preview { index, tracks, .. } => {
                if let Event::Key(key_event) = event {
                    match key_event.code {
                        KeyCode::Up => {
                            if *index == 0 {
                                *index = tracks.len() - 1;
                            } else {
                                *index -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if *index == tracks.len() - 1 {
                                *index = 0;
                            } else {
                                *index += 1;
                            }
                        }
                        KeyCode::Esc => return TVResponse::Exit,
                        _ => {}
                    }
                }
                // now check events for the selected component
                match tracks[*index].handle_event(event) {
                    TPResponse::Focus => {
                        let mut midi_tracks = tracks.drain(..).map(|t| t.track).collect::<Vec<_>>();
                        let focused = midi_tracks.remove(*index);
                        self.mode = TrackViewMode::View {
                            piano: Some(Piano::new(focused)),
                            index: *index,
                            other_tracks: midi_tracks,
                        }
                    }
                    _ => {}
                }
            }
            TrackViewMode::View {
                piano,
                index,
                other_tracks,
            } => match piano.as_mut().unwrap().handle_event(event) {
                PianoResponse::Exit => {
                    other_tracks.insert(*index, piano.take().unwrap().track);
                    let previews = other_tracks
                        .drain(..)
                        .enumerate()
                        .map(|(i, t)| TrackPreview::new_search_name(t, format!("Track {}", i)))
                        .collect::<Vec<_>>();
                    self.mode = TrackViewMode::Preview {
                        index: *index,
                        vscroll: 0,
                        tracks: previews,
                    };
                }
                _ => {}
            },
        }
        TVResponse::None
    }

    fn draw(&mut self, rect: Rect, buf: &mut Buffer) {
        match &mut self.mode {
            TrackViewMode::Preview {
                index,
                vscroll,
                tracks,
            } => {
                let index = *index as u16;
                // the number of tracks that can be shown in the area
                let display_num = rect.height / TRACK_PREVIEW_HEIGHT;
                if display_num == 0 {
                    let p =
                        Paragraph::new("Vertical height is too small to display track previews");
                    p.render(rect, buf);
                    return;
                }

                if index < *vscroll {
                    *vscroll = index;
                } else if *vscroll + display_num <= index {
                    *vscroll = index - (display_num - 1)
                }

                for i in *vscroll..min(*vscroll + display_num, tracks.len() as u16) {
                    let preview_area = Rect {
                        x: rect.x,
                        y: rect.y + (i - *vscroll) * TRACK_PREVIEW_HEIGHT,
                        width: rect.width,
                        height: TRACK_PREVIEW_HEIGHT,
                    };
                    tracks[i as usize].selected = i == index;
                    tracks[i as usize].draw(preview_area, buf);
                }
            }
            TrackViewMode::View {
                piano,
                index,
                other_tracks,
            } => piano.as_mut().unwrap().draw(rect, buf),
        }
    }
}
