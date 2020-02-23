pub enum MetaEvent {
    // 0x00
    SequenceNumber(Option<u16>),
    Text(String),
    Copyright(String),
    TrackName(String),
    Instrument(String),
    Lyric(String),
    Marker(String),
    CuePoint(String),
    ProgramName(String),
    DeviceName(String),
    // 0x20
    MidiChannelPrefix(u8),
    MidiPort(u8),
    // 0x2f
    EndOfTrack(),
    // 0x51
    Tempo(u32),
    // 0x54
    SMPTEOffset(u8, u8, u8, u8, u8),
    // 0x58
    TimeSignature {
        numerator: u8,
        denominator: u8,
        clocks_per_metronome: u8,
        something: u8,
    },
    KeySignature(u8, u8),

    Unsupported(Vec<u8>),
}
