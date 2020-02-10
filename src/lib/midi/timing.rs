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