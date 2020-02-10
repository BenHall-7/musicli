use super::{Timing, Format};

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