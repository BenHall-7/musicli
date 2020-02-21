use super::{event::Event, VarLengthValue};

#[derive(Debug)]
pub struct Track {
    delta_time: VarLengthValue,
    chunks: Vec<Event>,
}
