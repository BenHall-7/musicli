#[derive(Debug)]
pub enum MidiEvent {
    NoteOff,
    NoteOn,
    NotePressure,
    Controller,
    Program,
    Pressure,
    PitchBend,
}