use tui::layout::{Constraint, Direction};
use tui::style::*;
use tui::{backend::CrosstermBackend, Terminal};

mod state;
mod widgets;

use state::{ContainerType, SubArea, WidgetType};

fn main() {
    let mut t = Terminal::new(CrosstermBackend::new(std::io::stdout())).unwrap();
    t.clear().unwrap();

    loop {
        t.draw(|mut f| {
            let size = f.size();

            let areas = ContainerType::Divider {
                sub_areas: vec![
                    SubArea {
                        area: ContainerType::Widget {
                            widget_type: WidgetType::List {
                                parts: vec![
                                    ("first item".into(), Style::default()),
                                    ("second item".into(), Style::default().fg(Color::Red)),
                                ],
                                style: Style::default().bg(Color::Blue),
                            },
                        },
                        constraint: Constraint::Percentage(20),
                    },
                    SubArea {
                        area: ContainerType::Widget {
                            widget_type: WidgetType::Paragraph {
                                parts: vec![
                                    ("This is a sentence. ".into(), Style::default()),
                                    (
                                        "This is another sentence.".into(),
                                        Style::default().bg(Color::DarkGray),
                                    ),
                                ],
                                style: Style::default().bg(Color::Green),
                            },
                        },
                        constraint: Constraint::Percentage(80),
                    },
                ],
                direction: Direction::Horizontal,
                margin: (2, 0),
            };
            areas.render(&mut f, size)
        })
        .unwrap()
    }
}
