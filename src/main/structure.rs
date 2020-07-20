use std::borrow::Cow;
use tui::layout::{Constraint, Direction};
use tui::style::*;
use tui::widgets::{BorderType, Borders};

use crate::state::{BlockInfo, ContainerType, SubArea, WidgetType};

pub const APP: ContainerType = ContainerType::Divider {
    sub_areas: &[
        SubArea {
            component: ContainerType::Border {
                block_info: BlockInfo::new()
                    .title("Top Block")
                    .title_style(Style::new().fg(Color::Cyan))
                    .borders(Borders::ALL)
                    .border_style(Style::new().fg(Color::Magenta))
                    .border_type(BorderType::Rounded),
                component: Some(&FIRST_AREA),
            },
            constraint: Constraint::Percentage(20),
        },
        SubArea {
            component: ContainerType::Widget {
                widget_type: WidgetType::Paragraph {
                    parts: &[
                        (Cow::Borrowed("This is a sentence. "), Style::new()),
                        (
                            Cow::Borrowed("This is another sentence."),
                            Style::new().bg(Color::DarkGray),
                        ),
                    ],
                    style: Style::new().bg(Color::Green),
                },
            },
            constraint: Constraint::Percentage(80),
        },
    ],
    direction: Direction::Vertical,
    margin: (2, 1),
};

const FIRST_AREA: ContainerType = ContainerType::Widget {
    widget_type: WidgetType::List {
        parts: &[
            (Cow::Borrowed("first item"), Style::new()),
            (Cow::Borrowed("second item"), Style::new().fg(Color::Red)),
        ],
        style: Style::new().bg(Color::Blue),
    },
};
