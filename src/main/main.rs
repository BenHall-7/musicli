use tui::layout::{Constraint, Direction, Layout};
use tui::style::*;
use tui::{backend::CrosstermBackend, Terminal};

mod state;

use state::{ContainerType, SubArea, WidgetType};

fn main() {
    let mut t = Terminal::new(CrosstermBackend::new(std::io::stdout())).unwrap();
    t.clear().unwrap();

    loop {
        t.draw(|mut f| {
            let size = f.size();

            let areas = ContainerType::Divider(vec![
                SubArea {
                    area: ContainerType::Widget(WidgetType::List {
                        parts: vec![
                            ("first item".into(), Style::default()),
                            ("second item".into(), Style::default().fg(Color::Red)),
                        ],
                        style: Style::default().bg(Color::Blue),
                    }),
                    constraint: Constraint::Percentage(20),
                },
                SubArea {
                    area: ContainerType::Widget(WidgetType::Paragraph {
                        parts: vec![
                            ("This is a sentence. ".into(), Style::default()),
                            (
                                "This is another sentence.".into(),
                                Style::default().bg(Color::LightRed),
                            ),
                        ],
                        style: Style::default().bg(Color::Green),
                    }),
                    constraint: Constraint::Percentage(80),
                },
            ]);
            areas.render(&mut f, size)
        })
        .unwrap()
    }
}
