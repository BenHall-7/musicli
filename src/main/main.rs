use std::io::{stdout, Write};
use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, SetTitle,
};
use tui::backend::{Backend, CrosstermBackend};
use tui::terminal::Frame;
use tui::Terminal;

mod comp;
mod error;
mod utils;

use comp::*;

fn main() -> Result<(), error::AppError> {
    execute!(
        stdout(),
        SetTitle("muslici - cli midi editor"),
        EnterAlternateScreen
    )?;
    enable_raw_mode()?;
    let mut t = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
    t.clear().unwrap();

    let mut app = App::new();

    loop {
        t.draw(|mut f| {
            let size = f.size();
            f.render_stateful_widget(Wrapper, size, &mut app);
        })
        .unwrap();

        if poll(Duration::from_millis(0)).unwrap() {
            let event = read().unwrap();
            let comp_event = match event {
                Event::Resize(..) => continue,
                Event::Mouse(m) => comp::Event::Mouse(m),
                Event::Key(k) => comp::Event::Key(k),
            };
            match app.handle_event(comp_event) {
                AppResponse::Exit => break,
                AppResponse::None => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;

    t.clear().unwrap();

    Ok(())
}
