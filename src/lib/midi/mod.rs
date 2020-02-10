mod file;
mod format;
mod timing;
mod track;
mod var_length_value;

pub use file::File;
pub use format::Format;
pub use timing::Timing;
pub use track::Track;
pub use var_length_value::VarLengthValue;

#[derive(Debug)]
pub struct TrackEvent {
    /// The number of Timing units since the last event
    /// This value is variable length-ed
    delta: u32,
}

#[derive(Debug)]
pub enum Event {}
