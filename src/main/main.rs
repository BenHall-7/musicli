use musiclib::error::Error;
use musiclib::midi::File;
use musiclib::utils::FromStream;
use serde_yaml::to_string;
use std::env::args;
use std::io::Cursor;
// use tui::backend::CrosstermBackend;
// use tui::Terminal;

fn main() -> Result<(), Error> {
    let args: Vec<String> = args().collect();
    if args.len() > 1 {
        let filename = &args[1];
        let mut cursor = Cursor::new(std::fs::read(filename)?);
        match File::from_stream(&mut cursor, &mut ()) {
            Ok(midi) => {
                let yaml = to_string(&midi).unwrap();
                std::fs::write("output.yml", yaml)?;
                println!("Done!");
            }
            Err(e) => {
                println!("{}", e);
            }
        }
    } else {
        println!("No args. Needed: <input file>");
    }

    // let mut terminal = Terminal::new(CrosstermBackend::new())?;
    // terminal.clear()?;

    //let f = std::fs::read("C:/Users/Breakfast/Downloads/Pirate Island(Upgrade).mid")?;
    //let midi = Smf::parse_with_bytemap(raw: &[u8])

    //render(&mut terminal)

    Ok(())
}

// fn render<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Error> {
//     terminal.draw(|mut frame| {
//         let size = frame.size();
//         let chunks = Layout::default()
//             .direction(Direction::Horizontal)
//             .constraints([
//                 Constraint::Percentage(50),
//                 Constraint::Percentage(50),
//             ].as_ref())
//             .split(size);
//         Block::default()
//             .title("test")
//             .borders(Borders::ALL)
//             .render(&mut frame, chunks[1]);
//     })?;
//     Ok(())
// }
