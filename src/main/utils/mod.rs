pub fn get_note_name(note_value: u8, use_sharps: bool) -> String {
    let octave = note_value / 12;
    if use_sharps {
        match note_value % 12 {
            0 => format!("C {}", octave),
            1 => format!("C# {}", octave),
            2 => format!("D {}", octave),
            3 => format!("D# {}", octave),
            4 => format!("E {}", octave),
            5 => format!("F {}", octave),
            6 => format!("F# {}", octave),
            7 => format!("G {}", octave),
            8 => format!("G# {}", octave),
            9 => format!("A {}", octave),
            10 => format!("A# {}", octave),
            11 => format!("B {}", octave),
            _ => unreachable!(),
        }
    } else {
        match note_value % 12 {
            0 => format!("C {}", octave),
            1 => format!("Db {}", octave),
            2 => format!("D {}", octave),
            3 => format!("Eb {}", octave),
            4 => format!("E {}", octave),
            5 => format!("F {}", octave),
            6 => format!("Gb {}", octave),
            7 => format!("G {}", octave),
            8 => format!("Ab {}", octave),
            9 => format!("A {}", octave),
            10 => format!("Bb {}", octave),
            11 => format!("B {}", octave),
            _ => unreachable!(),
        }
    }
}

pub fn is_accidental(note_value: u8, tonic: u8) -> bool {
    let note = (note_value - tonic) % 12;
    match note {
        1 | 3 | 6 | 8 | 10 => true,
        _ => false,
    }
}
