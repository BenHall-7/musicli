// Could benefit from a macro

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SMPTETimecode {
    FPS24 = 24,
    FPS25 = 25,
    FPS29 = 29,
    FPS30 = 30,
}

impl SMPTETimecode {
    pub fn from(value: u32) -> Result<Self, ()> {
        match value {
            24 => Ok(SMPTETimecode::FPS24),
            25 => Ok(SMPTETimecode::FPS25),
            29 => Ok(SMPTETimecode::FPS29),
            30 => Ok(SMPTETimecode::FPS30),
            _ => Err(()),
        }
    }
}
