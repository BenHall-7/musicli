use crate::utils::Bounded;
use binread::io::{Read, Seek};
use binread::{BinRead, BinResult, ReadOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, BinRead)]
pub struct VarLengthValue(#[br(parse_with = read)] pub(crate) u32);

fn read<R: Read + Seek>(reader: &mut R, _: &ReadOptions, _: ()) -> BinResult<u32> {
    let mut value = 0u32;
    let max_bytes = 4;
    for _ in 0..max_bytes {
        let byte = u8::read(reader)? as u32;
        value |= byte & 0x7f;
        if (byte & 0x80) != 0 {
            value <<= 7;
        } else {
            break;
        }
    }

    Ok(VarLengthValue::bounded(value))
}

impl From<u32> for VarLengthValue {
    fn from(value: u32) -> VarLengthValue {
        Self(Self::bounded(value))
    }
}

impl Into<u32> for VarLengthValue {
    fn into(self) -> u32 {
        self.0
    }
}

impl Bounded<u32> for VarLengthValue {
    const MIN: u32 = 0;
    const MAX: u32 = 0x0fff_ffff;
}
