use crate::utils::{FromStream, ToStream};
use byteorder::{ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io::{Error, Read, Seek, Write};

#[derive(Debug, Deserialize, Serialize)]
pub enum MidiEvent {
    NoteOff(u8, u8, u8),
    NoteOn(u8, u8, u8),
    NotePressure(u8, u8, u8),
    Controller(u8, u8, u8),
    Program(u8, u8),
    Pressure(u8, u8),
    PitchBend(u8, u8, u8),
}

impl FromStream for MidiEvent {
    fn from_stream<R: Read + Seek>(reader: &mut R) -> Result<Self, Error> {
        let first_byte = reader.read_u8()?;
        let event_num = first_byte >> 4;
        let channel = first_byte & 0xf;
        match event_num {
            0x8 => Ok(Self::NoteOff(channel, reader.read_u8()?, reader.read_u8()?)),
            0x9 => Ok(Self::NoteOn(channel, reader.read_u8()?, reader.read_u8()?)),
            0xa => Ok(Self::NotePressure(
                channel,
                reader.read_u8()?,
                reader.read_u8()?,
            )),
            0xb => Ok(Self::Controller(
                channel,
                reader.read_u8()?,
                reader.read_u8()?,
            )),
            0xc => Ok(Self::Program(channel, reader.read_u8()?)),
            0xd => Ok(Self::Pressure(channel, reader.read_u8()?)),
            0xe => Ok(Self::PitchBend(
                channel,
                reader.read_u8()?,
                reader.read_u8()?,
            )),
            _ => unreachable!(),
        }
    }
}

impl ToStream for MidiEvent {
    fn to_stream<W: Write + Seek>(&self, writer: &mut W) -> Result<(), Error> {
        match self {
            Self::NoteOff(a, b, c) => {
                writer.write_u8(0x80 | (a & 0xf))?;
                writer.write_u8(*b)?;
                writer.write_u8(*c)?;
            }
            Self::NoteOn(a, b, c) => {
                writer.write_u8(0x90 | (a & 0xf))?;
                writer.write_u8(*b)?;
                writer.write_u8(*c)?;
            }
            Self::NotePressure(a, b, c) => {
                writer.write_u8(0xa0 | (a & 0xf))?;
                writer.write_u8(*b)?;
                writer.write_u8(*c)?;
            }
            Self::Controller(a, b, c) => {
                writer.write_u8(0xb0 | (a & 0xf))?;
                writer.write_u8(*b)?;
                writer.write_u8(*c)?;
            }
            Self::Program(a, b) => {
                writer.write_u8(0xc0 | (a & 0xf))?;
                writer.write_u8(*b)?;
            }
            Self::Pressure(a, b) => {
                writer.write_u8(0xd0 | (a & 0xf))?;
                writer.write_u8(*b)?;
            }
            Self::PitchBend(a, b, c) => {
                writer.write_u8(0xe0 | (a & 0xf))?;
                writer.write_u8(*b)?;
                writer.write_u8(*c)?;
            }
        }
        Ok(())
    }
}
