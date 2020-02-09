mod var_length_value;

pub use var_length_value::VarLengthValue;

#[derive(Debug)]
pub struct File {
    pub format: Format,
    pub timing: Timing,
}

impl File {
    pub fn get_timing(&self) -> Timing {
        self.timing
    }
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
    delta_time: VarLengthValue,
    chunks: Vec<TrackEvent>,
}

#[derive(Debug)]
pub struct TrackEvent {
    /// The number of Timing units since the last event
    /// This value is variable length-ed
    delta: u32,
}

#[derive(Debug)]
pub enum Event {}
