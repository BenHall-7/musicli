use std::io;
use std::path::{PathBuf, Path};

use midly::Smf;
use tui::backend::{Backend, CrosstermBackend};
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Widget};
use tui::Terminal;

use musiclib::error::Error;

struct App {
    pub project: Option<Project>,
}

struct Project {
    pub filename: Option<PathBuf>,
}

enum PlayStatus {
    Stopped,
    Playing,
}

fn main() -> Result<(), Error> {
    let mut terminal = Terminal::new(CrosstermBackend::new())?;
    terminal.clear()?;

    let f = std::fs::read("C:/Users/Breakfast/Downloads/Pirate Island(Upgrade).mid")?;
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