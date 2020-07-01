use binread::BinRead;
use musiclib::error::Error;
use musiclib::midi::File;
use serde_yaml::to_string;
use std::env::args;
use std::fs::{read, write};
use std::io::Cursor;

fn main() -> Result<(), Error> {
    let args: Vec<String> = args().collect();
    if args.len() > 1 {
        let filename = &args[1];
        let mut cursor = Cursor::new(read(filename)?);
        match File::read(&mut cursor) {
            Ok(midi) => {
                let yaml = to_string(&midi).unwrap();
                write("output.yml", yaml)?;
                println!("Done!");
            }
            Err(e) => {
                println!("{:#?}", e);
            }
        }
    } else {
        println!("No args. Needed: <input file>");
    }

    Ok(())
}
