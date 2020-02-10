use super::{VarLengthValue, TrackEvent};

#[derive(Debug)]
pub struct Track {
    delta_time: VarLengthValue,
    chunks: Vec<TrackEvent>,
}