#[test]
fn var_length_value_io() {
    use crate::midi::VarLengthValue;
    use binread::BinRead;
    use std::io::Cursor;

    let key_values = vec![
        (0x0000_0000, vec![0x00]),
        (0x0000_0040, vec![0x40]),
        (0x0000_007F, vec![0x7F]),
        (0x0000_0080, vec![0x81, 0x00]),
        (0x0000_2000, vec![0xC0, 0x00]),
        (0x0000_3FFF, vec![0xFF, 0x7F]),
        (0x0000_4000, vec![0x81, 0x80, 0x00]),
        (0x0010_0000, vec![0xC0, 0x80, 0x00]),
        (0x001F_FFFF, vec![0xFF, 0xFF, 0x7F]),
        (0x0020_0000, vec![0x81, 0x80, 0x80, 0x00]),
        (0x0800_0000, vec![0xC0, 0x80, 0x80, 0x00]),
        (0x0FFF_FFFF, vec![0xFF, 0xFF, 0xFF, 0x7F]),
    ];

    for (key, value) in key_values.iter() {
        // // writing to stream:
        // let var_value = VarLengthValue::from(*key);
        // let mut write_stream = Cursor::new(Vec::with_capacity(4));
        // var_value.to_stream(&mut write_stream).unwrap();
        // let actual = write_stream.into_inner();
        // assert_eq!(&actual, value);

        // reading from stream:
        let mut read_stream = Cursor::new(value);
        let new_var_value = VarLengthValue::read(&mut read_stream).unwrap();
        assert_eq!(new_var_value.0, *key);
    }
}

#[test]
fn timecode_io_metrical() {
    use crate::midi::Timing;
    use binread::BinRead;
    use std::io::Cursor;

    // // writing to stream:
    // let timing = Timing::Metrical(0x3c0);
    // let mut write_stream = Cursor::new(Vec::with_capacity(2));
    // timing.to_stream(&mut write_stream).unwrap();
    // assert_eq!(write_stream.into_inner(), vec![0x3, 0xc0]);

    // reading from stream:
    let mut read_stream = Cursor::new(vec![0x3, 0xc0]);
    let timing = Timing::read(&mut read_stream).unwrap();
    if let Timing::Metrical { precision } = timing {
        assert_eq!(precision, 0x3c0);
    } else {
        panic!();
    };
}

#[test]
fn timecode_io_smpte() {
    use crate::midi::{SMPTETimecode, Timing};
    use binread::BinRead;
    use std::io::Cursor;

    // // writing to stream:
    // let timing = Timing::Real(SMPTETimecode::FPS30, 80);
    // let mut write_stream = Cursor::new(Vec::with_capacity(2));
    // timing.to_stream(&mut write_stream).unwrap();
    // assert_eq!(write_stream.into_inner(), vec![0xe2, 0x50]);

    // reading from stream:
    let mut read_stream = Cursor::new(vec![0xe2, 0x50]);
    let timing = Timing::read(&mut read_stream).unwrap();
    if let Timing::Real { fps, precision } = timing {
        assert_eq!(fps, SMPTETimecode::FPS30);
        assert_eq!(precision, 80);
    } else {
        panic!();
    };
}
