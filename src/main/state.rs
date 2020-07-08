use std::borrow::Cow;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::Style;
use tui::widgets::{List, Paragraph, Text};
use tui::Frame;

// TODO: slice instead of vec.
// We want all areas to be pre-compiled. Right?
#[derive(Debug)]
pub enum ContainerType<'a> {
    Widget {
        widget_type: WidgetType<'a>,
    },
    // todo: Combine Layout and Vec<Area> into a single struct
    // to better handle possible length mismatches
    Divider {
        sub_areas: Vec<SubArea<'a>>,
        direction: Direction,
        margin: (u16, u16),
    },
}

impl<'a> ContainerType<'a> {
    pub fn render<B: Backend>(&self, frame: &mut Frame<B>, rect: Rect) {
        match self {
            ContainerType::Widget { widget_type } => widget_type.render(frame, rect),
            ContainerType::Divider {
                sub_areas,
                direction,
                margin,
            } => {
                // this is an OK structure but we may be able to do better
                let rects = Layout::default()
                    .direction(direction.clone())
                    .horizontal_margin(margin.0)
                    .vertical_margin(margin.1)
                    .constraints(sub_areas.iter().map(|a| a.constraint).collect::<Vec<_>>())
                    .split(rect);

                sub_areas.iter().zip(rects).for_each(|(area, rect)| {
                    area.area.render(frame, rect);
                })
            }
        }
    }
}

#[derive(Debug)]
pub struct SubArea<'a> {
    pub constraint: Constraint,
    pub area: ContainerType<'a>,
}

#[derive(Debug)]
pub enum WidgetType<'a> {
    List {
        parts: Vec<(Cow<'a, str>, Style)>,
        style: Style,
    },
    Paragraph {
        parts: Vec<(Cow<'a, str>, Style)>,
        style: Style,
    },
    // more...
}

impl<'a> WidgetType<'a> {
    pub fn render<B: Backend>(&self, frame: &mut Frame<B>, rect: Rect) {
        match self {
            WidgetType::List { parts, style } => {
                frame.render_widget(
                    List::new(
                        parts
                            .iter()
                            .map(|(s, style)| Text::Styled(s.to_owned(), *style)),
                    )
                    .style(*style),
                    rect,
                );
            }
            WidgetType::Paragraph { parts, style } => {
                frame.render_widget(
                    Paragraph::new(
                        parts
                            .iter()
                            .map(|(s, style)| Text::Styled(s.to_owned(), *style))
                            .collect::<Vec<Text>>() // this is dumb but apparently necessary
                            .iter(),
                    )
                    .style(*style),
                    rect,
                )
            }
        }
    }
}
