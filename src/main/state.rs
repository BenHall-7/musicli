use std::borrow::Cow;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::Style;
use tui::widgets::{Block, List, Paragraph, Text};
use tui::Frame;

#[derive(Debug)]
pub enum ContainerType<'a> {
    Widget {
        widget_type: WidgetType<'a>,
    },
    Border {
        block: Block<'a>,
        component: Option<&'a ContainerType<'a>>
    },
    Divider {
        sub_areas: &'a [SubArea<'a>],
        direction: Direction,
        margin: (u16, u16),
    },
}

impl<'a> ContainerType<'a> {
    pub fn render<B: Backend>(&self, frame: &mut Frame<B>, rect: Rect) {
        match self {
            ContainerType::Widget { widget_type } => widget_type.render(frame, rect),
            ContainerType::Border {
                block,
                component
            } => {
                frame.render_widget(*block, rect);
                if let Some(comp) = component {
                    comp.render(frame, block.inner(rect))
                }
            }
            ContainerType::Divider {
                sub_areas,
                direction,
                margin,
            } => {
                let rects = Layout::default()
                    .direction(direction.clone())
                    .horizontal_margin(margin.0)
                    .vertical_margin(margin.1)
                    // we use collect::<Vec<_>> because it's required by the constraints function
                    .constraints(sub_areas.iter().map(|a| a.constraint).collect::<Vec<_>>())
                    .split(rect);

                sub_areas.iter().zip(rects).for_each(|(area, rect)| {
                    area.component.render(frame, rect);
                })
            }
        }
    }
}

#[derive(Debug)]
pub struct SubArea<'a> {
    pub constraint: Constraint,
    pub component: ContainerType<'a>,
}

#[derive(Debug)]
pub enum WidgetType<'a> {
    List {
        parts: &'a [(Cow<'a, str>, Style)],
        style: Style,
    },
    Paragraph {
        parts: &'a [(Cow<'a, str>, Style)],
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
