use tui::layout::{Constraint, Direction, Layout};
use tui::style::*;
use tui::{backend::CrosstermBackend, Terminal};

mod state;

use state::{Area, WidgetType};

fn main() {
    let mut t = Terminal::new(CrosstermBackend::new(std::io::stdout())).unwrap();
    t.clear().unwrap();

    loop {
        t.draw(|mut f| {
            let size = f.size();
            let layout = Layout::default()
                .margin(2)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(size);
            let areas = Area::SubArea(vec![
                (
                    layout[0],
                    Area::Widget(WidgetType::List(vec![
                        ("first item".into(), Style::default()),
                        ("second item".into(), Style::default().fg(Color::Red)),
                    ])),
                ),
                (
                    layout[1],
                    Area::Widget(WidgetType::Paragraph(vec![
                        ("This is a sentence.".into(), Style::default()),
                        (
                            "This is another sentence.".into(),
                            Style::default().bg(Color::Blue),
                        ),
                    ])),
                ),
            ]);
            areas.render(&mut f, size)
        })
        .unwrap()
    }
}
