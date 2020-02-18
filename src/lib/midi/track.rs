use super::{TrackEvent, VarLengthValue};

#[derive(Debug)]
pub struct Track {
    delta_time: VarLengthValue,
    chunks: Vec<TrackEvent>,
}
