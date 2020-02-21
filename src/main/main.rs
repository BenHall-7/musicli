use tui::backend::{CrosstermBackend};
use tui::Terminal;

use musiclib::error::Error;

fn main() -> Result<(), Error> {
    let mut terminal = Terminal::new(CrosstermBackend::new())?;
    terminal.clear()?;

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
