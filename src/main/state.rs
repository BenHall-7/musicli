use std::borrow::Cow;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::Style;
use tui::widgets::{Block, BorderType, Borders, List, Paragraph, Text};
use tui::Frame;

#[derive(Debug)]
pub enum ContainerType<'a> {
    Widget {
        widget_type: WidgetType<'a>,
    },
    Border {
        block_info: BlockInfo<'a>,
        component: Option<&'a ContainerType<'a>>,
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
                block_info,
                component,
            } => {
                let block = if let Some(title) = block_info.title {
                    Block::default().title(title)
                } else {
                    Block::default()
                }
                .borders(block_info.borders)
                .border_type(block_info.border_type)
                .border_style(block_info.border_style)
                .title_style(block_info.title_style);
                // .style(block_info.style)
                frame.render_widget(block, rect);
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
            // TODO: Instead of ListState component, pass an optional state into render function?
            // WidgetType::StatefulList { parts, style, state } => {
            //     frame.render_stateful_widget(
            //         List::new(
            //             parts
            //                 .iter()
            //                 .map(|(s, style)| Text::Styled(s.to_owned(), *style)),
            //         )
            //         .style(*style),
            //         rect,
            //         &mut state.clone()
            //     )
            // }
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

/// a const compatible struct for representing a block
#[derive(Debug)]
pub struct BlockInfo<'a> {
    // Block apparently doesn't use the style property
    // style: Style,
    title: Option<&'a str>,
    title_style: Style,
    borders: Borders,
    border_type: BorderType,
    border_style: Style,
}

impl<'a> BlockInfo<'a> {
    pub const fn new() -> Self {
        Self {
            // style: Style::new(),
            title: None,
            title_style: Style::new(),
            borders: Borders::NONE,
            border_type: BorderType::Plain,
            border_style: Style::new(),
        }
    }

    // pub const fn style(mut self, style: Style) -> Self {
    //     self.style = style;
    //     self
    // }

    pub const fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    pub const fn title_style(mut self, title_style: Style) -> Self {
        self.title_style = title_style;
        self
    }

    pub const fn borders(mut self, borders: Borders) -> Self {
        self.borders = borders;
        self
    }

    pub const fn border_type(mut self, border_type: BorderType) -> Self {
        self.border_type = border_type;
        self
    }

    pub const fn border_style(mut self, border_style: Style) -> Self {
        self.border_style = border_style;
        self
    }
}
