use tui::layout::{Constraint, Direction};
use tui::style::*;
use tui::widgets::Borders;
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

            ContainerType::Divider {
                sub_areas: &[
                    SubArea {
                        component: ContainerType::Border {
                            block: tui::widgets::Block::default()
                                .title("Top Block")
                                .title_style(Style::default().fg(Color::Cyan))
                                .borders(Borders::ALL),
                            component: Some(&ContainerType::Widget {
                                widget_type: WidgetType::List {
                                    parts: &[
                                        ("first item".into(), Style::default()),
                                        ("second item".into(), Style::default().fg(Color::Red)),
                                    ],
                                    style: Style::default().bg(Color::Blue),
                                },
                            })
                        },
                        constraint: Constraint::Percentage(20),
                    },
                    SubArea {
                        component: ContainerType::Widget {
                            widget_type: WidgetType::Paragraph {
                                parts: &[
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
                direction: Direction::Vertical,
                margin: (2, 1),
            }.render(&mut f, size)
        })
        .unwrap()
    }
}
