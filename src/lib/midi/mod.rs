use std::fmt;

#[derive(Debug)]
pub struct File {
    pub format: Format,
    pub timing: Timing,
}

impl File {
    pub fn get_timing(&self) -> Timing {
        self.timing
    }

    // pub fn tracks(&self) -> Vec<&Track> {
    //     match self.format {
    //         Format::SingleTrack(ref t) => vec![t],
    //         Format::MultipleTrack(ref v) => v.iter().map(|t| t).collect(),
    //     }
    // }

    // pub fn tracks_mut(&mut self) -> Vec<&mut Track> {
    //     match self.format {
    //         Format::SingleTrack(ref mut t) => vec![t],
    //         Format::MultipleTrack(ref mut v) => v.iter_mut().map(|t| t).collect(),
    //     }
    // }
}

#[derive(Debug)]
pub enum Format {
    SingleTrack(Track),
    // TODO:
    // there should always be a "Tempo" track followed by regular tracks
    MultipleTrack(Vec<Track>),
    // TODO:
    // type 2
    // MultipleSong(),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Timing {
    /// Indicates a subdivision of quarter notes into a number of pulses.
    /// This timing is affected by tempo tracks.
    Metrical(u16),
    /// Indicates a subdivision of each second into frames.
    /// A frame is subdivided once again by the second value.
    /// This timing is not affected by tempo tracks.
    Real(u8, u8),
}

#[derive(Debug)]
pub struct Track {
    //u32 len
    chunks: Vec<TrackEvent>,
}

#[derive(Debug)]
pub struct TrackEvent {
    /// The number of Timing units since the last event
    /// This value is variable length-ed
    delta: u32,
}

#[derive(Debug)]
pub enum Event {
    
}
