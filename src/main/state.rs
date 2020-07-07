use std::borrow::Cow;
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::{List, Paragraph, Text};
use tui::Frame;

// TODO: slice instead of vec.
// We want all areas to be pre-compiled. Right?
pub enum Area<'a> {
    Widget(WidgetType<'a>),
    // todo: Combine Layout and Vec<Area> into a single struct
    // to better handle possible length mismatches
    SubArea(Vec<(Rect, Area<'a>)>),
}

impl<'a> Area<'a> {
    pub fn render<B: Backend>(&self, frame: &mut Frame<B>, rect: Rect) {
        match self {
            Area::Widget(widget_type) => widget_type.render(frame, rect),
            Area::SubArea(sub_areas) => sub_areas
                .iter()
                .for_each(|(rect, sub_area)| sub_area.render(frame, *rect)),
        }
    }
}

pub enum WidgetType<'a> {
    List(Vec<(Cow<'a, str>, Style)>),
    Paragraph(Vec<(Cow<'a, str>, Style)>),
    // more...
}

impl<'a> WidgetType<'a> {
    pub fn render<B: Backend>(&self, frame: &mut Frame<B>, rect: Rect) {
        match self {
            WidgetType::List(text) => {
                frame.render_widget(
                    List::new(
                        text.iter()
                            .map(|(s, style)| Text::Styled(s.to_owned(), *style)),
                    ),
                    rect,
                );
            }
            WidgetType::Paragraph(text) => {
                frame.render_widget(
                    Paragraph::new(
                        text.iter()
                            .map(|(s, style)| Text::Styled(s.to_owned(), *style))
                            .collect::<Vec<Text>>() // this is dumb but apparently necessary
                            .iter(),
                    ),
                    rect,
                )
            }
        }
    }
}
