use super::event::{Event, EventWithRet};
use binread::io::{Read, Seek, SeekFrom};
use binread::Endian::Big;
use binread::{BinRead, BinReaderExt, BinResult, ReadOptions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, BinRead)]
#[br(magic = b"MTrk")]
pub struct Track {
    #[br(parse_with = read_events)]
    pub events: Vec<Event>,
}

fn read_events<R: Read + Seek>(reader: &mut R, ro: &ReadOptions, _: ()) -> BinResult<Vec<Event>> {
    let length = reader.read_type::<u32>(Big)?;
    let mut events = Vec::<Event>::new();
    let end = reader.seek(SeekFrom::Current(0))? as u32 + length;

    let mut running_status = None;

    while (reader.seek(SeekFrom::Current(0))? as u32) < end {
        let EventWithRet(event, status) = EventWithRet::read_options(reader, ro, running_status)?;
        running_status = status;
        events.push(event);
    }

    Ok(events)
}
