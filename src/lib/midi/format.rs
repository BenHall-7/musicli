use super::Track;

#[derive(Debug)]
pub enum Format {
    SingleTrack(Track),
    // TODO:
    // there should always be a "Tempo" track followed by regular tracks
    MultipleTrack(Vec<Track>),
    // TODO:
    // just general information about this
    ParallelTrack(Vec<Track>),
}
